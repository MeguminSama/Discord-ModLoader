// use std::{ffi::CString, path::Path, ptr::null_mut};

// use widestring::U16CString;
// use winapi::{
//     self,
//     ctypes::c_void,
//     um::{
//         fileapi::{CreateFileW, WriteFile, CREATE_ALWAYS},
//         handleapi::CloseHandle,
//         namedpipeapi::{CreateNamedPipeW, CreatePipe},
//         winbase::{
//             FILE_FLAG_FIRST_PIPE_INSTANCE, PIPE_ACCESS_DUPLEX, PIPE_TYPE_BYTE,
//             PIPE_UNLIMITED_INSTANCES,
//         },
//         winnt::{FILE_ATTRIBUTE_NORMAL, GENERIC_READ, GENERIC_WRITE, HANDLE},
//     },
// };

// static ASAR_DATA: &[u8] = include_bytes!("../../app.asar");
// static PIPE_NAME: &str = r"\\.\pipe\discord_modhook";
// static mut ASAR_EXISTS: bool = false;

// pub unsafe fn create_asar_in_memory() -> HANDLE {
//     let pipe_name = U16CString::from_str(PIPE_NAME).unwrap();

//     if ASAR_EXISTS {
//         return 0 as HANDLE;
//     }

//     dbg!("create_asar_in_memory: pipe");

//     let handle = CreateNamedPipeW(
//         pipe_name.as_ptr(),
//         PIPE_ACCESS_DUPLEX,
//         PIPE_TYPE_BYTE,
//         PIPE_UNLIMITED_INSTANCES,
//         0,
//         ASAR_DATA.len() as u32 + 1024,
//         0,
//         null_mut(),
//     );

//     dbg!("create_asar_in_memory: pipe created");

//     // CreateFileW for handle and write ASAR_DATA to it

//     // let write_handle = CreateFileW(
//     //     pipe_name.as_ptr(),
//     //     GENERIC_READ | GENERIC_WRITE,
//     //     0,
//     //     null_mut(),
//     //     CREATE_ALWAYS,
//     //     FILE_ATTRIBUTE_NORMAL,
//     //     null_mut(),
//     // );

//     dbg!("create_asar_in_memory: write handle created");

//     WriteFile(
//         handle,
//         ASAR_DATA.as_ptr() as _,
//         ASAR_DATA.len() as _,
//         null_mut(),
//         null_mut(),
//     );

//     dbg!("create_asar_in_memory: wrote to pipe");

//     // CloseHandle(write_handle);

//     dbg!("create_asar_in_memory: closed write handle");

//     ASAR_EXISTS = true;

//     handle

//     // let read_handle = CreateFileW(
//     //     pipe_name.as_ptr(),
//     //     GENERIC_READ,
//     //     0,
//     //     null_mut(),
//     //     CREATE_ALWAYS,
//     //     FILE_ATTRIBUTE_NORMAL,
//     //     null_mut(),
//     // );

//     // read_handle
// }
