use std::{ffi::CString, mem::MaybeUninit, ptr::null_mut};

use detours_sys::{DetourCreateProcessWithDllExA, _PROCESS_INFORMATION, _STARTUPINFOA};
use env::Environment;
use winapi::um::{
    handleapi::CloseHandle, processthreadsapi::ResumeThread, winbase::CREATE_SUSPENDED,
};

mod env;

fn main() {
    let dll_path = "libmodhook_rs";

    let target_exe = r"c:\Users\megu\AppData\Local\DiscordPTB\app-1.0.1042\DiscordPTB.exe";

    let mut asar_path = std::env::current_dir().unwrap();

    asar_path.push("app.asar");

    let asar_path = asar_path.to_str().unwrap().to_string();

    let environment = Environment {
        asar_path: Some(asar_path),
        mod_entrypoint: r"c:\Users\megu\Workspace\Discord\Vencord\dist\patcher.js".to_string(),
        toggle_query: None,
        custom_data_dir: None,
    };

    unsafe {
        inject(dll_path, target_exe, &environment).unwrap();
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
        dbg!(result);
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
