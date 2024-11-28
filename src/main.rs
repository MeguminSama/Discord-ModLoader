use clap::Parser;

use discord_modloader_lib::{config, get_or_write_cache};

#[derive(clap::Parser, Debug)]
struct Args {
    #[clap(short, long)]
    pub instance: String,
}

#[cfg(target_os = "macos")]
fn main() {
    println!("macOS is not supported yet. Feel free to submit a PR.");
    println!("https://github.com/MeguminSama/Discord-Modloader");
}

#[cfg(not(target_os = "macos"))]
fn main() {
    let mut config = config::Config::init();
    config.validate();

    let args: Args = Args::parse();

    if let Some(instance) = config.instances.get(&args.instance) {
        dbg!(&instance);
        load_profile(&config, instance);
    } else {
        println!("Instance not found. Make sure it exists in the instances directory.");
    }
}

#[cfg(target_os = "linux")]
fn load_profile(config: &config::Config, instance: &config::Instance) {
    println!("Loading Instance: {}", instance.name);
    if let Some(ref profile_path) = instance.profile_path {
        println!("On profile: {}", profile_path)
    }

    let asar_path = get_or_write_cache(instance, config.mods.get(&instance.r#mod).unwrap());

    let mut target = std::process::Command::new(instance.path.clone())
        .current_dir(std::path::Path::new(&instance.path).parent().unwrap())
        // TODO: move libmodhook.so into global libs dir
        .env(
            "LD_PRELOAD",
            "/home/megu/Workspace/Discord/Discord-ModLoader/target/debug/libdiscord_modloader.so",
        )
        .env("MODLOADER_ASAR_PATH", asar_path)
        .args(["--trace-warnings"])
        .spawn()
        .expect("Failed to launch instance.");

    target
        .wait()
        .expect("Failed to wait for instance to finish.");
}

#[cfg(target_os = "windows")]
fn load_profile(config: &config::Config, instance: &config::Instance) {
    println!("Loading Instance: {}", instance.name);
    if let Some(ref profile_path) = instance.profile_path {
        println!("On profile: {}", profile_path)
    }

    // let asar_path = get_or_write_cache(instance, config.mods.get(&instance.r#mod).unwrap());

    let mut target = std::process::Command::new(instance.path.clone())
        .current_dir(std::path::Path::new(&instance.path).parent().unwrap())
        .args(["--trace-warnings"])
        .spawn()
        .expect("Failed to launch instance.");

    target
        .wait()
        .expect("Failed to wait for instance to finish.");
}
