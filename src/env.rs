use std::path::Path;

pub struct Environment {
    /// ModHook ASAR replacement.
    pub asar_path: Option<String>,
    /// Path to the mods' JS entrypoint.
    pub mod_entrypoint: String,
    /// Path to check for to revert to default app.asar behaviour
    /// after the mod has loaded.
    /// This defaults to Environment.asar_path.
    pub toggle_query: Option<String>,
    /// Custom path for AppData location.
    /// e.g. for multiple profiles, or running multiple clients at once.
    pub custom_data_dir: Option<String>,
}

impl Environment {
    pub fn from_env() -> Self {
        let mut env = Environment {
            asar_path: None,
            mod_entrypoint: std::env::var("MODHOOK_MOD_ENTRYPOINT").unwrap(),
            toggle_query: None,
            custom_data_dir: None,
        };

        if let Ok(path) = std::env::var("MODHOOK_ASAR_PATH") {
            env.asar_path = Some(path);
        } else {
            let absolute_path = Path::new("app.asar").canonicalize().unwrap();
            env.asar_path = Some(absolute_path.to_str().unwrap().to_string());
        }

        if let Ok(path) = std::env::var("MODHOOK_TOGGLE_QUERY") {
            env.toggle_query = Some(path);
        } else {
            env.toggle_query = Some(env.mod_entrypoint.clone());
        }

        if let Ok(path) = std::env::var("MODHOOK_CUSTOM_DATA_DIR") {
            env.custom_data_dir = Some(path);
        }

        env
    }

    pub fn apply(&self) {
        if let Some(path) = &self.asar_path {
            std::env::set_var("MODHOOK_ASAR_PATH", path);
        }

        std::env::set_var("MODHOOK_MOD_ENTRYPOINT", &self.mod_entrypoint);

        if let Some(path) = &self.custom_data_dir {
            std::env::set_var("MODHOOK_CUSTOM_DATA_DIR", path);
        }

        if let Some(query) = &self.toggle_query {
            std::env::set_var("MODHOOK_TOGGLE_QUERY", query);
        } else {
            std::env::set_var("MODHOOK_TOGGLE_QUERY", &self.mod_entrypoint);
        }
    }
}
