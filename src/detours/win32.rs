use std::ffi::{c_void, CString};

use detours_sys::{DetourAttach, DetourCreateProcessWithDllW};
use widestring::U16CString;
use winapi::{
    shared::{
        minwindef::{BOOL, DWORD, LPVOID},
        ntdef::LPCWSTR,
    },
    um::{
        fileapi::{CreateFileW, GetFileAttributesW},
        minwinbase::LPSECURITY_ATTRIBUTES,
        processthreadsapi::{CreateProcessW, ResumeThread, LPPROCESS_INFORMATION, LPSTARTUPINFOW},
        winnt::{HANDLE, LPWSTR},
    },
};

use super::utils::file_name_handler;

/// Path to the libmodhook DLL.
static DLL_PATH: &str = "libmodhook.dll";

static mut O_CREATE_FILE_W: *mut c_void = 0 as _;
static mut O_GET_FILE_ATTRIBUTES_W: *mut c_void = 0 as _;
static mut O_CREATE_PROCESS_W: *mut c_void = 0 as _;

pub unsafe fn init_detours() {
    O_CREATE_FILE_W = CreateFileW as *mut c_void;
    DetourAttach(&raw mut O_CREATE_FILE_W, create_file_w as _);

    O_GET_FILE_ATTRIBUTES_W = GetFileAttributesW as *mut c_void;
    DetourAttach(&raw mut O_GET_FILE_ATTRIBUTES_W, get_file_attributes_w as _);

    O_CREATE_PROCESS_W = CreateProcessW as *mut c_void;
    DetourAttach(&raw mut O_CREATE_PROCESS_W, create_process_w as _);
}

unsafe fn create_file_w(
    lp_file_name: LPCWSTR,
    dw_desired_access: DWORD,
    dw_share_mode: DWORD,
    lp_security_attributes: LPSECURITY_ATTRIBUTES,
    dw_creation_disposition: DWORD,
    dw_flags_and_attributes: DWORD,
    h_template_file: HANDLE,
) -> HANDLE {
    let file_name = U16CString::from_ptr_str(lp_file_name).to_string().unwrap();

    let new_file_name = file_name_handler(&file_name);
    let new_file_name = U16CString::from_str(new_file_name).unwrap();

    let create_file_w: extern "C" fn(
        lp_file_name: LPCWSTR,
        dw_desired_access: DWORD,
        dw_share_mode: DWORD,
        lp_security_attributes: LPSECURITY_ATTRIBUTES,
        dw_creation_disposition: DWORD,
        dw_flags_and_attributes: DWORD,
        h_template_file: HANDLE,
    ) -> HANDLE = std::mem::transmute(O_CREATE_FILE_W);

    create_file_w(
        new_file_name.as_ptr(),
        dw_desired_access,
        dw_share_mode,
        lp_security_attributes,
        dw_creation_disposition,
        dw_flags_and_attributes,
        h_template_file,
    )
}

unsafe fn get_file_attributes_w(lp_file_name: LPCWSTR) -> DWORD {
    let get_file_attributes_w: extern "C" fn(lp_file_name: LPCWSTR) -> DWORD =
        std::mem::transmute(O_GET_FILE_ATTRIBUTES_W);

    let file_name = U16CString::from_ptr_str(lp_file_name).to_string().unwrap();

    let new_file_name = file_name_handler(&file_name);
    let new_file_name = U16CString::from_str(new_file_name).unwrap();

    get_file_attributes_w(new_file_name.as_ptr())
}

#[allow(clippy::too_many_arguments)]
unsafe fn create_process_w(
    lp_application_name: LPCWSTR,
    lp_command_line: LPWSTR,
    lp_process_attributes: LPSECURITY_ATTRIBUTES,
    lp_thread_attributes: LPSECURITY_ATTRIBUTES,
    b_inherit_handles: BOOL,
    dw_creation_flags: DWORD,
    lp_environment: LPVOID,
    lp_current_directory: LPCWSTR,
    lp_startup_info: LPSTARTUPINFOW,
    lp_process_information: LPPROCESS_INFORMATION,
) -> BOOL {
    let create_process_w: unsafe extern "C" fn(
        lp_application_name: LPCWSTR,
        lp_command_line: LPWSTR,
        lp_process_attributes: LPSECURITY_ATTRIBUTES,
        lp_thread_attributes: LPSECURITY_ATTRIBUTES,
        b_inherit_handles: BOOL,
        dw_creation_flags: DWORD,
        lp_environment: LPVOID,
        lp_current_directory: LPCWSTR,
        lp_startup_info: LPSTARTUPINFOW,
        lp_process_information: LPPROCESS_INFORMATION,
    ) -> BOOL = std::mem::transmute(O_CREATE_PROCESS_W);

    let command_line = U16CString::from_ptr_str(lp_command_line)
        .to_string()
        .unwrap();

    if !command_line.contains("--type=renderer") {
        return create_process_w(
            lp_application_name,
            lp_command_line,
            lp_process_attributes,
            lp_thread_attributes,
            b_inherit_handles,
            dw_creation_flags,
            lp_environment,
            lp_current_directory,
            lp_startup_info,
            lp_process_information,
        );
    }

    let dll_path = CString::new(DLL_PATH.to_string()).unwrap();

    let success = DetourCreateProcessWithDllW(
        lp_application_name,
        lp_command_line,
        lp_process_attributes as _,
        lp_thread_attributes as _,
        b_inherit_handles,
        dw_creation_flags,
        lp_environment as _,
        lp_current_directory,
        lp_startup_info as _,
        lp_process_information as _,
        dll_path.as_ptr(),
        Some(std::mem::transmute(O_CREATE_PROCESS_W)),
    );

    if success != 1 {
        println!("[ModHook] Failed to create process");
        return success;
    }

    ResumeThread((*lp_process_information).hThread as _);

    success
}
