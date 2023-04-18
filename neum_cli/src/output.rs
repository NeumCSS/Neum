use crate::{html_parse, neum_parse, ARGS};
use itertools::Itertools;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Component;
use std::sync::Mutex;
use std::time::Instant;

lazy_static::lazy_static! {
    static ref REAL_DEFAULTS: neum::Neum = neum::Neum::default();
    static ref DEFAULTS: Mutex<neum::Neum> = Mutex::new(neum::Neum::default());
}

pub fn update(refresh: bool) {
    let now = Instant::now();
    let mut output = String::from("/* auto generated by Neum https://github.com/AMTitan/Neum */\n");
    let html = html_parse::HTML_FILES.lock().unwrap();
    let neum_files = neum_parse::NEUM_FILES.lock().unwrap();

    let mut total_classes = Vec::new();
    for (_, i) in html.iter() {
        total_classes.append(&mut i.clone());
    }
    total_classes = total_classes
        .iter()
        .unique()
        .cloned()
        .collect::<Vec<String>>();

    let mut total_neum = DEFAULTS.lock().unwrap();
    if refresh {
        let mut libraries = neum::Neum::empty();
        let mut other = neum::Neum::empty();

        for (path, neum) in neum_files.iter() {
            if path
                .as_path()
                .components()
                .any(|x| x == Component::Normal(OsStr::new(".neum")))
            {
                libraries = libraries.combine_priority(neum.clone());
            } else {
                other = other.combine_priority(neum.clone());
            }
        }

        *total_neum = REAL_DEFAULTS
            .clone()
            .combine_priority(libraries)
            .combine_priority(other);

        total_neum.refresh();
    }

    for i in total_classes {
        if let Some(mut x) = total_neum.convert(i.clone()) {
            while x.starts_with('.') || x.starts_with('@') {
                let period = x.starts_with('.');
                let mut split = x.split('}').collect::<Vec<_>>();
                let mut new_css = format!("{}}}", split.remove(0));
                if period && new_css.split('{').next().unwrap().contains(':') {
                    let mut vec = new_css.split(':').collect::<Vec<_>>();
                    let first = &format!(".{i}");
                    vec[0] = first;
                    new_css = vec.join(":");
                }
                output.push_str(&new_css);
                x = split.join("}");
            }
            if x != ";" {
                output.push_str(&format!(".{i}{{{x}}}"));
            }
        }
    }
    let mut file = File::create(ARGS.output.clone()).unwrap();
    file.write_all(output.as_bytes()).unwrap();
    if ARGS.verbose {
        println!("Generated css in {:?}", now.elapsed());
    }
}
