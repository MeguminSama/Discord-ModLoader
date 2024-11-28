pub mod config;
mod unix;

#[cfg(target_os = "linux")]
#[allow(unused_imports)]
pub use unix::*;

pub fn get_or_write_cache(instance: &config::Instance, mod_: &config::Mod) -> String {
    let cache_dir = dirs::cache_dir().unwrap().join("discord-modloader");
    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir).unwrap();
    }

    let asar_dir = cache_dir.join("asar");
    if !asar_dir.exists() {
        std::fs::create_dir_all(&asar_dir).unwrap();
    }

    let cached_asar_path = asar_dir.join(format!("{}-{}.asar", instance.r#mod, mod_.name));
    // TODO: Figure out a way to cache, while also regenerating if the mod or instance config changes.
    // if cached_asar_path.exists() {
    //     return cached_asar_path.to_str().unwrap().to_owned();
    // }

    static ASAR_CUSTOM_PROFILE_JS: &str = include_str!("./asar/custom_profile.js");

    if let Some(ref profile) = instance.profile_path {
        let profile_path = std::path::Path::new(profile);
        if !profile_path.exists() {
            std::fs::create_dir_all(&profile_path).unwrap();
        }
    }

    let mod_entrypoint = std::path::Path::new(&mod_.path).join(&mod_.entrypoint);
    let mod_entrypoint = mod_entrypoint.to_str().unwrap();

    let profile_dir = instance.profile_path.clone().unwrap_or_default();

    let profile_loader = instance.profile.as_ref().map(|_| {
        ASAR_CUSTOM_PROFILE_JS
            .replace("__CUSTOM_PROFILE_DIR__", &profile_dir)
            .replace("__MOD_ENTRYPOINT_FILE__", mod_entrypoint)
    });

    let custom_loader = if let Some(loader) = &mod_.loader {
        let prefix = loader
            .prefix
            .as_ref()
            .map(|p| {
                p.replace("__CUSTOM_PROFILE_DIR__", &profile_dir)
                    .replace("__MOD_ENTRYPOINT_FILE__", mod_entrypoint)
            })
            .unwrap_or_default();

        let custom_profile = loader
            .profile
            .as_ref()
            .map(|p| {
                p.replace("__CUSTOM_PROFILE_DIR__", &profile_dir)
                    .replace("__MOD_ENTRYPOINT_FILE__", mod_entrypoint)
            })
            .unwrap_or(profile_loader.unwrap_or_default());

        let require = loader
            .require
            .as_ref()
            .map(|r| {
                r.replace("__CUSTOM_PROFILE_DIR__", &profile_dir)
                    .replace("__MOD_ENTRYPOINT_FILE__", mod_entrypoint)
            })
            .unwrap_or(format!(r#"require("{}")"#, mod_entrypoint));

        let suffix = loader
            .suffix
            .as_ref()
            .map(|s| {
                s.replace("__CUSTOM_PROFILE_DIR__", &profile_dir)
                    .replace("__MOD_ENTRYPOINT_FILE__", mod_entrypoint)
            })
            .unwrap_or_default();

        format!("{}\n{}\n{}\n{}", prefix, custom_profile, require, suffix)
    } else {
        format!(
            "{}\nrequire(\"{}\")",
            profile_loader.unwrap_or_default(),
            mod_entrypoint
        )
    };

    let mut asar = asar::AsarWriter::new();
    asar.write_file("index.js", custom_loader, false).unwrap();
    asar.write_file("package.json", include_bytes!("./asar/package.json"), false)
        .unwrap();

    asar.finalize(std::fs::File::create(&cached_asar_path).unwrap())
        .unwrap();

    cached_asar_path.to_str().unwrap().to_owned()
}
