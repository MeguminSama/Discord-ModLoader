use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::paths::{self, ensure_dir};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigFile {
    Instance(Instance),
    Mod(Mod),
}

#[derive(Debug, Clone)]
pub struct Config {
    // pub instances: HashMap<String, Instance>,
    pub profiles: HashMap<String, ProfileConfig>,
    pub mods: HashMap<String, Mod>,
}

impl Config {
    pub fn validate(&mut self) {
        // for (_, mod_) in self.mods.iter_mut() {
        //     let entrypoint_exists = std::path::Path::new(&mod_.path)
        //         .join(&mod_.entrypoint)
        //         .exists();

        //     mod_.is_valid = entrypoint_exists;
        // }

        // for (_, instance) in self.instances.iter_mut() {
        //     if let Some(target_mod) = self.mods.get(&instance.r#mod) {
        //         let asar_exists: bool = std::path::Path::new(&instance.path)
        //             .parent()
        //             .expect("Expected Discord executable path to have a parent.")
        //             .join("resources")
        //             .join("app.asar")
        //             .exists();

        //         instance.is_valid = target_mod.is_valid && asar_exists;
        //     } else {
        //         instance.is_valid = false;
        //     }
        // }
    }

    pub fn init() -> Config {
        let profiles_config_dir = ensure_dir(paths::config_profile_dir());
        let mods_config_dir = ensure_dir(paths::config_mods_dir());

        let mut profile_configs = HashMap::new();

        for profile in paths::read_dir(&profiles_config_dir) {
            if !profile.path().to_string_lossy().ends_with(".toml") {
                continue;
            }
            let id = profile.file_name().to_string_lossy().replace(".toml", "");

            let profile = std::fs::read_to_string(profile.path()).unwrap();
            let profile = toml::from_str::<ProfileConfig>(&profile).unwrap();

            profile_configs.insert(id, profile);
        }

        let mut mod_configs = HashMap::new();

        for mod_ in paths::read_dir(&mods_config_dir) {
            if !mod_.path().to_string_lossy().ends_with(".toml") {
                continue;
            }
            let id = mod_.file_name().to_string_lossy().replace(".toml", "");

            let mod_ = std::fs::read_to_string(mod_.path()).unwrap();
            let mod_ = toml::from_str::<ModConfig>(&mod_).unwrap();

            mod_configs.insert(id, mod_.r#mod);
        }

        dbg!(&profile_configs);

        Config {
            profiles: profile_configs,
            mods: mod_configs,
        }
    }
}

impl ConfigFile {
    fn from_file(path: &str) -> Self {
        let file = std::fs::read_to_string(path).unwrap();
        let config: ConfigFile = toml::from_str(&file).unwrap();
        config
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstanceConfig {
    pub instance: Instance,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Instance {
    pub id: String,
    /// The display name of the instance. (e.g. "Vencord", "Moonlight", "My Custom Instance")
    ///
    /// Can be duplicate, but not recommended for clarity.
    pub name: String,

    /// A path to the icon to use for the mod.
    pub icon: Option<String>,

    /// The identifier of the mod to use for this instance.
    pub mod_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModConfig {
    pub r#mod: Mod,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mod {
    /// The display name of the mod. (e.g. "Vencord", "Moonlight", "BetterDiscord")
    ///
    /// Can be duplicate, but not recommended for clarity.
    pub name: String,

    /// The path to the mod's dist folder. (e.g. "/path/to/moonlight")
    pub path: String,

    /// The entrypoint of the mod. (e.g. "injector.js", "patcher.js")
    pub entrypoint: String,

    /// A path to the icon to use for the mod.
    pub icon: Option<String>,

    /// Provide custom loader JS to be injected into the ASAR index.js.
    pub loader: Option<ModLoader>,

    #[serde(skip, default)]
    pub is_valid: bool,

    #[serde(skip, default)]
    pub config_path: String,
}

/// The loader configuration for the mod.
/// You can use this to specify custom JS to be in your ASAR's index.js.
///
/// The following variables can be used:
///
/// - `__CUSTOM_PROFILE_DIR__`: The directory of the custom profile.
/// - `__MOD_ENTRYPOINT_FILE__`: The entrypoint file of the mod.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModLoader {
    pub prefix: Option<String>,
    #[serde(default = "default_require")]
    pub require: Option<String>,
    pub suffix: Option<String>,
}

fn default_require() -> Option<String> {
    Some(r#"require("__MOD_ENTRYPOINT_FILE__")"#.to_string())
}

// [profile]
// name = "test"
// [[instance]]
// ...

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileConfig {
    pub profile: Profile,

    #[serde(rename = "instance")]
    pub instances: Vec<Instance>,

    pub discord: Discord,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    #[serde(default)]
    pub use_default_profile: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Discord {
    pub executable: String,
}
