#![cfg(all(target_arch = "x86", target_os = "windows"))]
//! DLL to inject

#[macro_use]
extern crate lazy_static;

use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::winnt::DLL_PROCESS_ATTACH;

mod sockhook;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
    _module: HINSTANCE,
    call_reason: DWORD,
    _reserved: LPVOID,
) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        sockhook::main().is_ok() as BOOL
    } else {
        TRUE
    }
}
