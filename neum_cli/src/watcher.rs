use crate::output::update;
use crate::ARGS;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use walkdir::WalkDir;

use crate::html_parse;
use crate::neum_parse;

pub fn watch() {
    let current = std::env::current_dir().unwrap();

    if let Some(neum_folder) = &ARGS.neum_folder {
        let current = current.clone();
        thread::spawn(move || {
            let (tx, rx) = std::sync::mpsc::channel();

            let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();

            debouncer
                .watcher()
                .watch(neum_folder, RecursiveMode::Recursive)
                .unwrap();

            for event in rx.into_iter().flatten() {
                let mut changed = false;
                for e in event {
                    let e = e.path.strip_prefix(current.clone()).unwrap().to_path_buf();
                    if e.ends_with(".neum") {
                        if let Err(e) = neum_parse::update_neum(e.clone()) {
                            eprintln!("{e}");
                        }
                        changed = true;
                    }
                }
                if changed {
                    update(true);
                }
            }
        });
    }
    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();

    debouncer
        .watcher()
        .watch(
            &ARGS
                .source_code
                .clone()
                .unwrap_or_else(|| PathBuf::from(".")),
            RecursiveMode::Recursive,
        )
        .unwrap();

    for event in rx.into_iter().flatten() {
        let mut changed = false;
        let mut refresh = false;
        for e in event {
            if !excludes(e.path.clone()) {
                let e = e.path.strip_prefix(current.clone()).unwrap().to_path_buf();
                if let Some(extension) = e.extension() {
                    if extension == "html" || extension == "htm" || extension == "xhtml" {
                        if html_parse::update_html(e.clone()).is_err() {
                            eprintln!("Failded to parse {}", e.display());
                        }
                        changed = true;
                    } else if extension == "neum" && ARGS.neum_folder.is_none() {
                        if let Err(e) = neum_parse::update_neum(e.clone()) {
                            eprintln!("{e}");
                        }
                        changed = true;
                        refresh = true;
                    }
                }
            }
        }
        if changed {
            update(refresh);
        }
    }
}

pub fn init() {
    if let Some(neum_folder) = &ARGS.neum_folder {
        for e in WalkDir::new(neum_folder).into_iter().flatten() {
            if !excludes(e.path().to_path_buf()) {
                if let Some(extension) = e.path().extension() {
                    if extension == "neum" && ARGS.neum_folder.is_none() {
                        if let Err(e) = neum_parse::update_neum(e.path().to_path_buf()) {
                            eprintln!("{e}");
                        }
                    }
                }
            }
        }
    }

    for e in WalkDir::new(
        &ARGS
            .source_code
            .clone()
            .unwrap_or_else(|| PathBuf::from(".")),
    )
    .into_iter()
    .flatten()
    {
        if !excludes(e.path().to_path_buf()) {
            if let Some(extension) = e.path().extension() {
                if extension == "html" || extension == "htm" || extension == "xhtml" {
                    if html_parse::update_html(e.path().to_path_buf()).is_err() {
                        eprintln!("Failded to parse {}", e.path().display());
                    }
                } else if extension == "neum" && ARGS.neum_folder.is_none() {
                    if let Err(e) = neum_parse::update_neum(e.path().to_path_buf()) {
                        eprintln!("{e}");
                    }
                }
            }
        }
    }
    update(false);
}

fn excludes(path: PathBuf) -> bool {
    for i in &ARGS.exclude {
        if path.starts_with(i) {
            return true;
        }
    }
    false
}
