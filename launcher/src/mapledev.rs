#![cfg(all(target_arch = "x86", target_os = "windows"))]
//! DLL to inject

#[macro_use]
extern crate lazy_static;

use winapi::shared::minwindef::{BOOL, DWORD, FALSE, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

pub mod error;

#[macro_use]
#[allow(dead_code)]
pub(crate) mod utils;

mod sockhook;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
    hinstDLL: HINSTANCE,
    fdwReason: DWORD,
    _reserved: LPVOID,
) -> BOOL {
    if fdwReason == DLL_PROCESS_ATTACH {
        DisableThreadLibraryCalls(hinstDLL);
        winlog!("[DllMain] Injected mapledev.dll");
        match sockhook::main() {
            Ok(_) => TRUE,
            Err(e) => {
                winlog!("[DllMain] {:?}", e);
                FALSE
            }
        }
    } else {
        TRUE
    }
}
