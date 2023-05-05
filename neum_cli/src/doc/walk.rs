use walkdir::WalkDir;

pub fn walk_neum_folder() {
    for e in WalkDir::new(doc!().neum_folder.clone().unwrap_or(".".into())).into_iter().flatten() {
            if let Some(extension) = e.path().extension() {
                if extension == "neum" {
                    //if let Err(e) = crate::doc::build(e.path().to_path_buf()) {
                    //    eprintln!("{e}");
                    //}
                }
            }
    }
}
