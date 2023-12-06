use std::path::Path;

#[derive(Debug)]
pub struct Environment {
    /// Path to the mods' JS entrypoint.
    ///
    /// e.g. "c:\users\megu\vencord\patcher.js"
    pub mod_entrypoint: String,

    /// Path to check for to revert to default app.asar behaviour after the mod has loaded.
    ///
    /// e.g. "vencord\patcher.js"
    pub toggle_query: Option<String>,

    /// Custom name for AppData profile.
    ///
    /// e.g. "MyCustomProfile"
    pub custom_data_dir: Option<String>,

    /// ModHook ASAR replacement.
    ///
    /// e.g. "c:\users\megu\vencord\app.asar"
    pub asar_path: Option<String>,

    /// Modded ASAR filename.
    ///
    /// This is the file that the mod (e.g. Vencord) loads
    /// to return to the original Discord context.
    ///
    /// ModHook will redirect calls to this file to the original app.asar (e.g. _app.asar -> app.asar)
    ///
    /// e.g. "_app.asar"
    pub modded_asar_filename: Option<String>,

    /// Whether or not the mod is the moonlight mod.
    ///
    /// Moonlight uses `require(entrypoint).inject(asarPath);`
    /// instead of the usual `require(entrypoint);`
    pub is_moonlight: bool,
}

#[allow(dead_code)]
impl Environment {
    /// Creates a new Environment struct from the current environment variables.
    pub fn from_env() -> Self {
        let mut env = Environment {
            asar_path: None,
            mod_entrypoint: std::env::var("MODHOOK_MOD_ENTRYPOINT").unwrap(),
            toggle_query: None,
            custom_data_dir: None,
            modded_asar_filename: None,
            is_moonlight: false,
        };

        if let Ok(path) = std::env::var("MODHOOK_ASAR_PATH") {
            env.asar_path = Some(path);
        } else {
            let absolute_path = Path::new("app.asar").canonicalize().unwrap();
            env.asar_path = Some(absolute_path.to_str().unwrap().to_string());
        }

        if let Ok(path) = std::env::var("MODHOOK_TOGGLE_QUERY") {
            env.toggle_query = Some(path.to_lowercase());
        } else {
            env.toggle_query = Some(env.mod_entrypoint.clone());
        }

        if let Ok(path) = std::env::var("MODHOOK_CUSTOM_DATA_DIR") {
            env.custom_data_dir = Some(path);
        }

        if let Ok(file) = std::env::var("MODHOOK_MOD_ASAR_FILENAME") {
            env.modded_asar_filename = Some(file);
        } else {
            env.modded_asar_filename = Some("_app.asar".to_string());
        }

        if let Ok(is_moonlight) = std::env::var("MODHOOK_IS_MOONLIGHT") {
            env.is_moonlight = is_moonlight == "true";
        }

        env
    }

    /// Applies the environment variables to the current process.
    pub fn apply(&self) {
        if let Some(path) = &self.asar_path {
            std::env::set_var("MODHOOK_ASAR_PATH", path);
        }

        std::env::set_var("MODHOOK_MOD_ENTRYPOINT", &self.mod_entrypoint);

        if let Some(path) = &self.custom_data_dir {
            std::env::set_var("MODHOOK_CUSTOM_DATA_DIR", path);
        }

        if let Some(query) = &self.toggle_query {
            std::env::set_var("MODHOOK_TOGGLE_QUERY", query.to_lowercase());
        } else {
            std::env::set_var("MODHOOK_TOGGLE_QUERY", &self.mod_entrypoint);
        }

        if let Some(file) = &self.modded_asar_filename {
            std::env::set_var("MODHOOK_MOD_ASAR_FILENAME", file);
        } else {
            std::env::set_var("MODHOOK_MOD_ASAR_FILENAME", "_app.asar");
        }

        // Disable auto patching of the Discord client.
        // Currently supported mods:
        // - Vencord
        std::env::set_var("DISABLE_UPDATER_AUTO_PATCHING", "true");

        if self.is_moonlight {
            std::env::set_var("MODHOOK_IS_MOONLIGHT", "true");
        }
    }
}
