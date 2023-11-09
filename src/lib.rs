use std::{ffi::c_void, ptr::null_mut};

use detours_sys::{
    DetourAttach, DetourGetEntryPoint, DetourIsHelperProcess, DetourRestoreAfterWith,
    DetourTransactionBegin, DetourTransactionCommit, DetourUpdateThread,
};

use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID},
    um::{processthreadsapi::GetCurrentThread, winnt::DLL_PROCESS_ATTACH},
};

#[cfg(debug_assertions)]
use winapi::um::consoleapi::AllocConsole;
// use crate::detours::fs::create_asar_in_memory;

mod detours;
mod env;

static mut O_ENTRYPOINT: *mut c_void = 0 as _;

#[no_mangle]
extern "cdecl" fn ExportedFunction() {}

#[no_mangle]
unsafe extern "stdcall" fn DllMain(
    _hinst_dll: HINSTANCE,
    fwd_reason: DWORD,
    _lpv_reserved: LPVOID,
) -> BOOL {
    if DetourIsHelperProcess() == 1 {
        return 1;
    }

    if fwd_reason != DLL_PROCESS_ATTACH {
        return 1;
    }

    DetourRestoreAfterWith();

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread() as _);

    O_ENTRYPOINT = DetourGetEntryPoint(null_mut());
    DetourAttach(&mut O_ENTRYPOINT, main as _);

    detours::win32::init_detours();

    DetourTransactionCommit();

    1
}

unsafe fn main() {
    #[cfg(debug_assertions)]
    {
        AllocConsole();
        println!("[ModHook] Process Hooked");
    }
    let start_discord: extern "C" fn() = std::mem::transmute(O_ENTRYPOINT);
    start_discord();
}
