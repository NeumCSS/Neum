use crate::{html_parse, neum_parse, ARGS};
use itertools::Itertools;
use std::fs::File;
use std::io::Write;
use std::ffi::OsStr;
use std::path::Component;

pub fn update() {
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

    let mut total_neum = neum::Neum::default();
    let mut libraries = neum::Neum::new("", None).unwrap();
    let mut other = neum::Neum::new("", None).unwrap();

    for (path, neum) in neum_files.iter() {
        if !path.as_path().components().filter(|x| *x == Component::Normal(OsStr::new(".neum"))).collect::<Vec<_>>().is_empty() {
            libraries = libraries.combine_priority(neum.clone());
        }
        else {
            other = other.combine_priority(neum.clone());
        }
    }

    total_neum = total_neum.combine_priority(libraries).combine_priority(other);

    for i in total_classes {
        if let Some(mut x) = total_neum.convert(i.clone()) {
            while x.starts_with('.') || x.starts_with('@') {
                let period = x.starts_with('.');
                let mut split = x.split('}').collect::<Vec<_>>();
                let mut new_css = format!("{}}}", split.remove(0));
                if period {
                    let mut vec = new_css.split(':').collect::<Vec<_>>();
                    let first = &format!(".{i}");
                    vec[0] = first;
                    new_css = vec.join(":");
                }
                output.push_str(&new_css);
                x = split.join("}");
            }
            output.push_str(&format!(".{i}{{{x}}}"));
        }
    }
    let mut file = File::create(ARGS.output.clone()).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
