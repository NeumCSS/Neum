use crate::ARGS;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn watch() {
    if let Some(neum_folder) = &ARGS.neum_folder {
        thread::spawn(move || {
            let (tx, rx) = std::sync::mpsc::channel();

            let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();

            debouncer
                .watcher()
                .watch(Path::new(&neum_folder), RecursiveMode::Recursive)
                .unwrap();

            for events in rx {
                if let Ok(event) = events {
                    for e in event {
                        if e.path.ends_with(".neum") {
                            // todo
                        }
                    }
                }
            }
        });
    }
    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();

    debouncer
        .watcher()
        .watch(Path::new(&ARGS.source_code), RecursiveMode::Recursive)
        .unwrap();

    for events in rx {
        if let Ok(event) = events {
            for e in event {
                if e.path.ends_with(".html") || e.path.ends_with(".htm") {
                    // todo
                } else if e.path.ends_with(".neum") && ARGS.neum_folder.is_none() {
                    // todo
                }
            }
        }
    }
}
