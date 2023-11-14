use std::path::PathBuf;

static mut MOD_DONE_LOADING: bool = false;

const KNOWN_ASAR_NAMES: &[&str] = &["_app.asar", "app.orig.asar"];

/// Map original filenames to our modified filenames.
pub unsafe fn file_name_handler(path: &str) -> String {
    let asar_toggle_query = std::env::var("MODHOOK_TOGGLE_QUERY").unwrap();
    let path = path.to_lowercase();
    let pathbuf = PathBuf::from(&path.clone());

    if path.contains(&asar_toggle_query) {
        MOD_DONE_LOADING = true;
    }

    if let Some(filename) = pathbuf.file_name() {
        let filename = filename.to_str().unwrap();

        // If we have a custom asar filename, replace it with the original.
        if let Ok(asar) = std::env::var("MODHOOK_MOD_ASAR_FILENAME") {
            if filename.eq(&asar.to_lowercase()) {
                return path.replace(&asar, "app.asar");
            }
        }

        // Otherwise, default to the known asar names and replace them with the original.
        if let Some(asar) = KNOWN_ASAR_NAMES.iter().find(|&&x| filename.eq(x)) {
            let new_path = path.replace(asar, "app.asar");
            return new_path;
        }
    }

    if MOD_DONE_LOADING {
        return path.to_string();
    }

    if path.ends_with("app.asar") {
        return std::env::var("MODHOOK_ASAR_PATH").unwrap();
    }

    path.to_string()
}
