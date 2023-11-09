static mut MOD_DONE_LOADING: bool = false;

pub fn is_renderer() -> bool {
    std::env::args().any(|arg| arg.contains("--type=renderer"))
}

pub fn mod_done_loading() -> bool {
    unsafe { MOD_DONE_LOADING }
}

pub unsafe fn file_name_handler(path: &str) -> String {
    let asar_toggle_query = r"patcher.js";

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

    if path.contains(r"\app.asar") {
        // let remainder = path.split(r"\app.asar").collect::<Vec<&str>>()[1];
        return std::env::var("MODHOOK_ASAR_PATH").unwrap();
        return r"c:\Users\megu\AppData\Roaming\DiscordModHook\Profiles\8369af24-9984-4070-b5d3-0ad65c3cadd9.asar".to_string();
    }

    path.to_string()
}
