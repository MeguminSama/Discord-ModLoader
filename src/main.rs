use clap::Parser;

use discord_modloader::{config, init_current_cache};

#[derive(clap::Parser, Debug)]
struct Args {
    #[clap(short, long)]
    pub profile: Option<String>,

    #[clap(short, long)]
    pub instance: Option<String>,
}

#[cfg(target_os = "macos")]
fn main() {
    println!("macOS is not supported yet. Feel free to submit a PR.");
    println!("https://github.com/MeguminSama/Discord-Modloader");
}

#[cfg(not(target_os = "macos"))]
fn main() {
    // TODO: Check if args are provided. If yes, don't load GUI.
    let args: Args = Args::parse();

    if let (Some(profile_id), Some(instance_id)) = (args.profile, args.instance) {
        let mut config = config::Config::init();
        config.validate();

        unsafe { load_profile(&config, &profile_id, &instance_id) };
    } else {
        gui::start_gui();
    }
}

#[cfg(target_os = "linux")]
unsafe fn load_profile(config: &config::Config, profile_id: &str, instance_id: &str) {
    use discord_modloader::paths::{self, ensure_dir};

    let profile = config
        .profiles
        .get(profile_id)
        .expect(&format!("Failed to find profile '{}'.", profile_id));

    // Try to use the local instance of libdiscord_modloader.so first.
    let current_exe = std::env::current_exe().unwrap();
    let mut shared_object = current_exe.with_file_name("libdiscord_modloader.so");

    // If it doesn't exist, try to use the system-wide one.
    if !shared_object.exists() {
        let lib_path = std::path::PathBuf::from("/usr/lib/discord-modloader");
        shared_object = lib_path.join("libdiscord_modloader.so");
        if !shared_object.exists() {
            panic!("libdiscord_modloader.so not found. Please report this on GitHub.");
        }
    }

    let asar_path = init_current_cache(config, profile_id, instance_id);

    let working_dir = if profile.profile.use_default_profile {
        std::path::Path::new(&profile.discord.executable)
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    } else {
        let profile_dir = ensure_dir(paths::data_profiles_dir().join(profile_id));
        profile_dir.to_str().unwrap().to_string()
    };

    let mut target = std::process::Command::new(&profile.discord.executable)
        .current_dir(working_dir)
        .env("LD_PRELOAD", shared_object.to_str().unwrap())
        .env("MODLOADER_ASAR_PATH", asar_path)
        // .args(["--trace-warnings"])
        .spawn()
        .expect("Failed to launch instance.");

    target
        .wait()
        .expect("Failed to wait for instance to finish.");
}

#[cfg(target_os = "windows")]
unsafe fn load_profile(config: &config::Config, instance: &config::Instance) {
    use detours_sys::{DetourCreateProcessWithDllExA, _PROCESS_INFORMATION, _STARTUPINFOA};
    use libdiscordmodloader::discord::get_discord_exe;
    use winapi::um::{
        handleapi::CloseHandle,
        processthreadsapi::ResumeThread,
        winbase::CREATE_SUSPENDED,
        winuser::{MessageBoxA, MB_ICONERROR},
    };

    println!("Loading Instance: {}", instance.name);
    if let Some(ref profile_path) = instance.profile_path {
        println!("On profile: {}", profile_path)
    }

    let asar_path = init_current_cache(instance, config.mods.get(&instance.r#mod).unwrap());

    let current_exe = std::env::current_exe().unwrap();
    let lp_current_directory = current_exe.parent().unwrap().to_str().unwrap();
    let dll = current_exe.with_file_name("libdiscordmodloader.dll");

    if !dll.exists() {
        MessageBoxA(
            std::ptr::null_mut(),
            c"libdiscordmodloader.dll not found.\nPlease verify your installation.".as_ptr(),
            c"Error loading modloader".as_ptr(),
            MB_ICONERROR,
        );
        panic!("libdiscordmodloader.dll not found.");
    }

    let discord_exe = get_discord_exe(&instance.path).expect("Failed to get Discord executable.");

    std::env::set_var("MODLOADER_ASAR_PATH", asar_path);
    std::env::set_var("MODLOADER_DLL_PATH", &dll);

    let dll = std::ffi::CString::new(dll.to_str().unwrap()).unwrap();
    let lp_current_directory = std::ffi::CString::new(lp_current_directory).unwrap();

    let mut process_info: _PROCESS_INFORMATION = unsafe { std::mem::zeroed() };
    let mut startup_info: _STARTUPINFOA = unsafe { std::mem::zeroed() };
    let discord_exe = std::ffi::CString::new(discord_exe.to_str().unwrap()).unwrap();

    let result = DetourCreateProcessWithDllExA(
        std::ptr::null_mut(),
        discord_exe.as_ptr() as *mut i8,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        0,
        CREATE_SUSPENDED,
        std::ptr::null_mut(),
        lp_current_directory.as_ptr(),
        &raw mut startup_info,
        &raw mut process_info,
        dll.as_ptr(),
        None,
    );

    if result == 0 {
        MessageBoxA(
            std::ptr::null_mut(),
            c"Failed to inject DLL into Discord".as_ptr(),
            c"Error launching Discord".as_ptr(),
            MB_ICONERROR,
        );
        panic!("Failed to create process with DLL.");
    }

    ResumeThread(process_info.hThread);

    CloseHandle(process_info.hProcess);
    CloseHandle(process_info.hThread);
}
