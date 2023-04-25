use crate::ARGS;
use lazy_static::lazy_static;
use neum::Neum;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

lazy_static! {
    pub static ref NEUM_FILES: Arc<Mutex<HashMap<PathBuf, Neum>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub fn update_neum(path: PathBuf) -> Result<(), neum::error::NeumError> {
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
    if let Ok(content) = &fs::read_to_string(path.clone()) {
        let neum = Neum::new(content.clone(), Some(path.display().to_string()))?;
        let neum_files = NEUM_FILES.clone();
        let mut neum_files = neum_files.lock().unwrap();
        if let Some(i) = neum_files.get_mut(&path) {
            *i = neum;
        } else {
            neum_files.insert(path, neum);
        }
    } else {
        let neum_files = NEUM_FILES.clone();
        let mut neum_files = neum_files.lock().unwrap();
        neum_files.remove(&path);
    }
    if ARGS.verbose {
        println!(" in {:?}", now.elapsed());
    }
    Ok(())
}
