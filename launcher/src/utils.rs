//! Shared MapleDev DLL functions

use crate::error::Error;
use std::ffi::CString;
use std::fmt;
use winapi::shared::minwindef::HINSTANCE;
use winapi::um::debugapi::OutputDebugStringA;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress, LoadLibraryA};
use winapi::um::processthreadsapi::ExitProcess;

pub unsafe fn windows_log(args: fmt::Arguments) {
    let formatted = format!("[MapleDev] {}", args);
    let output = to_cstring(&formatted);
    OutputDebugStringA(output.as_ptr());
}

#[macro_export]
macro_rules! winlog {
    ( $( $args:tt )+ ) => {
        unsafe { crate::utils::windows_log(format_args!($( $args )*)) }
    }
}

/// Creates a CString or exits the process
pub fn to_cstring(s: &str) -> CString {
    match CString::new(s) {
        Ok(cs) => cs,
        Err(e) => {
            winlog!("[to_cstring] ERROR: {:?}", e);
            unsafe { ExitProcess(3424) };
            panic!();
        }
    }
}

/// Finds the symbol of an already loaded module
pub unsafe fn get_symbol(module: &str, symbol: &str) -> Result<usize, Error> {
    let lpmodule = CString::new(module).map_err(|_| Error::CStringFailed(module.into()))?;
    let lpsymbol = CString::new(symbol).map_err(|_| Error::CStringFailed(module.into()))?;
    let handle = GetModuleHandleA(lpmodule.as_ptr());
    if handle == ::std::ptr::null_mut() {
        return Err(Error::ModuleNotLoaded(module.into()));
    }
    let address = GetProcAddress(handle, lpsymbol.as_ptr());
    if address == ::std::ptr::null_mut() {
        return Err(Error::SymbolNotFound(module.into(), symbol.into()));
    }
    Ok(address as usize)
}

/// Loads a module
pub unsafe fn load_module(module: &str) -> Result<HINSTANCE, Error> {
    let lpmodule = CString::new(module).map_err(|_| Error::CStringFailed(module.into()))?;
    let handle = LoadLibraryA(lpmodule.as_ptr());
    if handle == ::std::ptr::null_mut() {
        Err(Error::ModuleNotLoaded(module.into()))
    } else {
        Ok(handle)
    }
}

/// Loads a module and finds a symbol
pub unsafe fn load_module_symbol(module: &str, symbol: &str) -> Result<usize, Error> {
    let handle = load_module(module)?;
    let lpsymbol = CString::new(symbol).map_err(|_| Error::CStringFailed(module.into()))?;
    let address = GetProcAddress(handle, lpsymbol.as_ptr());
    if address == ::std::ptr::null_mut() {
        return Err(Error::SymbolNotFound(module.into(), symbol.into()));
    }
    Ok(address as usize)
}
