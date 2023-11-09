#![windows_subsystem = "windows"]
use std::{ffi::CString, mem::MaybeUninit, ptr::null_mut};

use clap::Parser;
use detours_sys::{DetourCreateProcessWithDllExA, _PROCESS_INFORMATION, _STARTUPINFOA};
use env::Environment;
use winapi::um::{
    handleapi::CloseHandle, processthreadsapi::ResumeThread, winbase::CREATE_SUSPENDED,
};

mod discord;
mod env;

static DLL_PATH: &str = "libmodhook.dll";

#[derive(Parser, Debug)]
#[command(name = "modhook", verbatim_doc_comment)]
/// Discord ModHook
/// For more information, visit: https://github.com/MeguminSama/ModHook
pub struct Args {
    /// Path to the Discord folder.
    /// Example: --discord-path "c:\users\megu\appdata\roaming\discordptb"
    #[arg(short, long, verbatim_doc_comment)]
    pub discord_path: String,

    /// Path to the mods' JS entrypoint.
    /// Example: --mod-entrypoint "c:\users\megu\vencord\patcher.js"
    #[arg(short, long, verbatim_doc_comment)]
    pub mod_entrypoint: String,

    /// Path to check for to revert to default app.asar behaviour after the mod has loaded.
    /// Example: --toggle-query "vencord\patcher.js"
    #[arg(short, long, verbatim_doc_comment)]
    pub toggle_query: Option<String>,

    /// Custom name for AppData profile.
    /// Example: --custom-data-dir "MyCustomProfile"
    #[arg(short, long, verbatim_doc_comment)]
    pub custom_data_dir: Option<String>,

    /// ModHook ASAR replacement.
    /// Example: --asar-path "c:\users\megu\vencord\app.asar"
    #[arg(short, long, verbatim_doc_comment)]
    pub asar_path: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    let target_exe = discord::get_discord_executable(&args.discord_path).unwrap();
    let target_exe = target_exe.to_str().unwrap();

    let mut asar_path = std::env::current_dir().unwrap();

    asar_path.push("app.asar");

    let asar_path = asar_path.to_str().unwrap().to_string();

    let mut dll_path = std::env::current_dir().unwrap();
    dll_path.push(DLL_PATH);

    let environment = Environment {
        asar_path: Some(asar_path),
        mod_entrypoint: args.mod_entrypoint,
        toggle_query: args.toggle_query,
        custom_data_dir: args.custom_data_dir,
    };

    unsafe {
        inject(dll_path.to_str().unwrap(), target_exe, &environment).unwrap();
    }
}

/// # Safety
/// This function is unsafe because it calls the WinAPI.
pub unsafe fn inject(
    dll_path: &str,
    target_exe: &str,
    environment: &Environment,
) -> std::io::Result<()> {
    let cstr_target_exe = CString::new(target_exe)?;
    let cstr_dll_path = CString::new(dll_path)?;

    let mut process_info: _PROCESS_INFORMATION = MaybeUninit::zeroed().assume_init();
    let mut startup_info: _STARTUPINFOA = MaybeUninit::zeroed().assume_init();

    environment.apply();

    let result = DetourCreateProcessWithDllExA(
        null_mut(),
        cstr_target_exe.as_ptr() as *mut i8,
        null_mut(),
        null_mut(),
        0,
        CREATE_SUSPENDED,
        null_mut(),
        null_mut(),
        &mut startup_info as *mut _,
        &mut process_info as *mut _,
        cstr_dll_path.as_ptr(),
        None,
    );

    if result == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to create process",
        ));
    }

    ResumeThread(process_info.hThread as _);

    CloseHandle(process_info.hProcess as _);
    CloseHandle(process_info.hThread as _);

    Ok(())
}
