use crate::ARGS;
use html_parser::{Dom, Node};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

lazy_static! {
    pub static ref HTML_FILES: Arc<Mutex<HashMap<PathBuf, Vec<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub fn update_html(path: PathBuf) -> anyhow::Result<()> {
    print!(
        "Updating: {}{}",
        path.display(),
        match ARGS.verbose {
            true => "",
            false => "\n",
        }
    );
    io::stdout().flush().unwrap();
    let now = Instant::now();
    let mut real_classes = Vec::new();
    if let Ok(content) = fs::read_to_string(path.clone()) {
        let classes = Dom::parse(&content)?
            .children
            .iter()
            .map(|x| get_classes(x.clone()))
            .collect::<Vec<Vec<String>>>();
        for i in &classes {
            real_classes.append(&mut i.clone());
        }
        real_classes = real_classes
            .iter()
            .unique()
            .cloned()
            .collect::<Vec<String>>();
        let html_files = HTML_FILES.clone();
        let mut html_files = html_files.lock().unwrap();
        if let Some(i) = html_files.get_mut(&path) {
            *i = real_classes;
        } else {
            html_files.insert(path, real_classes);
        }
    } else {
        let html_files = HTML_FILES.clone();
        let mut html_files = html_files.lock().unwrap();
        html_files.remove(&path);
    }
    if ARGS.verbose {
        println!(" in {:?}", now.elapsed());
    }
    Ok(())
}

fn get_classes(node: Node) -> Vec<String> {
    if let Some(element) = node.element() {
        let mut classes = element.classes.clone();
        for i in &element.children {
            classes.append(&mut get_classes(i.clone()));
        }
        return classes;
    }
    Vec::new()
}
