static mut MOD_DONE_LOADING: bool = false;

pub unsafe fn file_name_handler(path: &str) -> String {
    let asar_toggle_query = r"patcher.js";

    let path = path.to_lowercase();

    if path.contains(asar_toggle_query) {
        MOD_DONE_LOADING = true;
    }

    if path.ends_with("_app.asar") {
        let new_path = path.replace("_app.asar", "app.asar");
        return new_path;
    }

    if MOD_DONE_LOADING {
        return path.to_string();
    }

    if path.ends_with("app.asar") {
        return std::env::var("MODHOOK_ASAR_PATH").unwrap();
    }

    path.to_string()
}
