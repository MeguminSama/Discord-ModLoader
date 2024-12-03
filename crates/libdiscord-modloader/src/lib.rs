pub mod config;
pub mod discord;
pub mod paths;

use libc::MOD_FREQUENCY;
use paths::ensure_dir;

#[cfg(target_os = "linux")]
mod unix;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
#[allow(unused_imports)]
pub use unix::*;

#[cfg(target_os = "windows")]
#[allow(unused_imports)]
pub use windows::*;

pub fn init_current_cache(cfg: &config::Config, profile_id: &str, instance_id: &str) -> String {
    static ASAR_CUSTOM_PROFILE_JS: &str = include_str!("./asar/custom_profile.js");
    static ASAR_PACKAGE_JSON: &str = include_str!("./asar/package.json");

    let asar_dir = ensure_dir(paths::data_asar_dir());

    let profile = cfg
        .profiles
        .get(profile_id)
        .expect(&format!("Failed to find profile '{}'.", profile_id));

    let instance = profile
        .instances
        .iter()
        .find(|instance| instance.id == instance_id)
        .expect(&format!("Failed to find instance '{}'.", instance_id));

    let r#mod = cfg
        .mods
        .get(&instance.mod_id)
        .expect(&format!("Failed to find mod '{}'.", instance.mod_id));

    let asar_cache_path = asar_dir.join(format!(
        "{}-{}-{}.asar",
        profile_id, instance_id, instance.mod_id
    ));

    let profile_dir = if profile.profile.use_default_profile {
        let profile_dir = ensure_dir(paths::data_profiles_dir().join(profile_id));
        let profile_dir = profile_dir.to_str().unwrap().replace("\\", "\\\\");
        Some(profile_dir)
    } else {
        None
    };

    let mod_entrypoint = std::path::Path::new(&r#mod.path).join(&r#mod.entrypoint);
    let mod_entrypoint = mod_entrypoint.to_str().unwrap().replace("\\", "\\\\");

    let mut custom_loader = String::from("console.log(\"Launching with Discord Modloader.\");\n");

    // If using a custom profile directory, insert this.
    if let Some(ref profile_dir) = profile_dir {
        let data = ASAR_CUSTOM_PROFILE_JS
            .replace("__CUSTOM_PROFILE_DIR__", &profile_dir)
            .replace("__MOD_ENTRYPOINT_FILE__", &mod_entrypoint);

        custom_loader.push_str(&data);
    }

    if let Some(ref loader) = r#mod.loader {
        // Users can provide a custom prefix in the profile TOML.
        if let Some(ref prefix) = loader.prefix {
            let mut prefix = prefix.replace("__MOD_ENTRYPOINT_FILE__", &mod_entrypoint);

            if let Some(ref profile_dir) = profile_dir {
                prefix = prefix.replace("__CUSTOM_PROFILE_DIR__", &profile_dir);
            }

            custom_loader.push_str(&prefix);
        }

        // If the user provides a custom require, use that instead of the default.
        if let Some(ref require) = loader.require {
            let mut require = require.replace("__MOD_ENTRYPOINT_FILE__", &mod_entrypoint);

            if let Some(ref profile_dir) = profile_dir {
                require = require.replace("__CUSTOM_PROFILE_DIR__", &profile_dir);
            }

            custom_loader.push_str(&require);
        } else {
            custom_loader.push_str(&format!(r#"require("{}")"#, &mod_entrypoint));
        }

        // If the user provices a custom suffix, insert it.
        if let Some(ref suffix) = loader.suffix {
            let mut suffix = suffix.replace("__MOD_ENTRYPOINT_FILE__", &mod_entrypoint);

            if let Some(ref profile_dir) = profile_dir {
                suffix = suffix.replace("__CUSTOM_PROFILE_DIR__", &profile_dir);
            }

            custom_loader.push_str(&suffix);
        }
    } else {
        custom_loader.push_str(&format!(r#"require("{}");"#, &mod_entrypoint));
    }

    let mut asar = asar::AsarWriter::new();

    asar.write_file("index.js", custom_loader, false).unwrap();
    asar.write_file("package.json", ASAR_PACKAGE_JSON, false)
        .unwrap();

    asar.finalize(std::fs::File::create(&asar_cache_path).unwrap())
        .unwrap();

    asar_cache_path.to_str().unwrap().to_owned()
}
