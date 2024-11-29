use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigFile {
    Instance(Instance),
    Mod(Mod),
}

#[derive(Debug)]
pub struct Config {
    pub mods: HashMap<String, Mod>,
    pub instances: HashMap<String, Instance>,
}

impl Config {
    pub fn validate(&mut self) {
        for (_, mod_) in self.mods.iter_mut() {
            let entrypoint_exists = std::path::Path::new(&mod_.path)
                .join(&mod_.entrypoint)
                .exists();

            mod_.is_valid = entrypoint_exists;
        }

        for (_, instance) in self.instances.iter_mut() {
            if let Some(target_mod) = self.mods.get(&instance.r#mod) {
                let asar_exists: bool = std::path::Path::new(&instance.path)
                    .parent()
                    .expect("Expected Discord executable path to have a parent.")
                    .join("resources")
                    .join("app.asar")
                    .exists();

                instance.is_valid = target_mod.is_valid && asar_exists;
            } else {
                instance.is_valid = false;
            }
        }
    }

    pub fn init() -> Config {
        let profiles_dir = dirs::data_dir()
            .unwrap()
            .join("discord-modloader")
            .join("profiles");

        println!("Loading from profile directory: {}", profiles_dir.display());

        if !profiles_dir.exists() {
            std::fs::create_dir_all(&profiles_dir).expect("Unable to create profiles directory.");
        }

        let configs_dir = dirs::config_local_dir().unwrap().join("discord-modloader");

        let instances_dir = configs_dir.join("instances");
        let mods_dir = configs_dir.join("mods");

        if !instances_dir.exists() {
            std::fs::create_dir_all(&instances_dir).expect("Unable to create instances directory.");
        }

        if !mods_dir.exists() {
            std::fs::create_dir_all(&mods_dir).expect("Unable to create mods directory.");
        }

        let instances = std::fs::read_dir(&instances_dir).unwrap();
        let mods = std::fs::read_dir(&mods_dir).unwrap();

        println!("Loading instances from: {:?}", &instances_dir);

        let mut mod_configs: HashMap<String, Mod> = HashMap::new();
        for mod_file in mods {
            let mod_path = mod_file.unwrap();
            let mod_file = ConfigFile::from_file(mod_path.path().to_str().unwrap());
            let mod_name = mod_path
                .path()
                .with_extension("")
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            if let ConfigFile::Mod(mut mod_file) = mod_file {
                mod_file.config_path = mod_path
                    .path()
                    .canonicalize()
                    .expect("Unable to locate config file.")
                    .to_str()
                    .unwrap()
                    .to_string();
                mod_configs.insert(mod_name, mod_file);
            } else {
                panic!("Instance file found in mods directory.");
            }
        }

        let mut instance_configs: HashMap<String, Instance> = HashMap::new();
        for instance_file in instances {
            let instance_path = instance_file.unwrap();
            let instance_file = ConfigFile::from_file(instance_path.path().to_str().unwrap());
            let instance_name = instance_path
                .path()
                .with_extension("")
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            if let ConfigFile::Instance(mut instance_file) = instance_file {
                instance_file.config_path = instance_path
                    .path()
                    .canonicalize()
                    .expect("Unable to locate config file.")
                    .to_str()
                    .unwrap()
                    .to_string();

                instance_file.profile_path = instance_file
                    .profile
                    .as_ref()
                    .map(|profile| profiles_dir.join(profile).to_str().unwrap().to_string());

                instance_configs.insert(instance_name, instance_file);
            } else {
                panic!("Mod file found in instances directory.");
            }
        }

        Config {
            mods: mod_configs,
            instances: instance_configs,
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

#[derive(Debug, Deserialize)]
pub struct Instance {
    /// The display name of the instance. (e.g. "Vencord", "Moonlight", "My Custom Instance")
    ///
    /// Can be duplicate, but not recommended for clarity.
    pub name: String,

    /// A path to the icon to use for the mod.
    pub icon: Option<String>,

    /// The custom profile to use.
    /// This will run a unique chrome profile for the mod,
    /// allowing for multiple instances of Discord to run at the same time.
    pub profile: Option<String>,

    /// The identifier of the mod to use for this instance.
    pub r#mod: String,

    // pub description: String,
    pub path: String,

    #[serde(skip, default)]
    pub is_valid: bool,

    #[serde(skip, default)]
    pub config_path: String,

    #[serde(skip, default)]
    pub profile_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Mod {
    /// The display name of the mod. (e.g. "Vencord", "Moonlight", "BetterDiscord")
    ///
    /// Can be duplicate, but not recommended for clarity.
    pub name: String,

    /// A path to the icon to use for the mod.
    pub icon: Option<String>,

    /// The path to the mod's dist folder. (e.g. "/path/to/moonlight")
    pub path: String,

    /// The entrypoint of the mod. (e.g. "injector.js", "patcher.js")
    pub entrypoint: String,

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
#[derive(Debug, Deserialize)]
pub struct ModLoader {
    pub prefix: Option<String>,
    pub profile: Option<String>,
    #[serde(default = "default_require")]
    pub require: Option<String>,
    pub suffix: Option<String>,
}

fn default_require() -> Option<String> {
    Some(r#"require("__MOD_ENTRYPOINT_FILE__")"#.to_string())
}
