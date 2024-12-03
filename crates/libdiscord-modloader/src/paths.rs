static APP_NAME: &str = "discord-modloader";

/// Create a directory if it doesn't exist.
pub fn ensure_dir(dir: std::path::PathBuf) -> std::path::PathBuf {
    if !dir.exists() {
        std::fs::create_dir_all(&dir).expect(&format!(
            "Failed to create path. Make sure you have permissions. {}",
            dir.display()
        ));
    }
    dir
}

/// The config directory for this application.
pub fn configs_dir() -> std::path::PathBuf {
    dirs::config_local_dir()
        .expect("Failed to get config dir")
        .join(APP_NAME)
}

/// The profiles config directory.
pub fn config_profile_dir() -> std::path::PathBuf {
    configs_dir().join("profiles")
}

/// The mods config directory.
pub fn config_mods_dir() -> std::path::PathBuf {
    configs_dir().join("mods")
}

/// The data directory for this application.
pub fn data_dir() -> std::path::PathBuf {
    dirs::data_dir()
        .expect("Failed to get data dir")
        .join(APP_NAME)
}

/// The asar cache directory.
pub fn data_asar_dir() -> std::path::PathBuf {
    data_dir().join("asar")
}

/// The asar path for a profile, instance, and mod.
pub fn data_asar_path(profile_id: &str, instance_id: &str, mod_id: &str) -> std::path::PathBuf {
    data_asar_dir().join(format!("{}-{}-{}.asar", profile_id, instance_id, mod_id))
}

/// The profiles data directory.
pub fn data_profiles_dir() -> std::path::PathBuf {
    data_dir().join("profiles")
}

/// Read a directory and return a vector of DirEntry. Panic if it fails.
pub fn read_dir(path: &std::path::Path) -> Vec<std::fs::DirEntry> {
    std::fs::read_dir(path)
        .expect(&format!("Failed to read directory: {}", path.display()))
        .collect::<Result<Vec<_>, _>>()
        .expect(&format!("Failed to read directory: {}", path.display()))
}
