//! Launcher using Windows detours

use detours_sys::{
    DetourCreateProcessWithDllExA, BOOL, DWORD, LPCSTR, LPSECURITY_ATTRIBUTES, LPSTR, LPVOID,
    _PROCESS_INFORMATION, _STARTUPINFOA,
};
use std::ffi::CString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ProcessFailed,
    CStringFailed(String),
}

// MapleStory EXE
const MAPLESTORY: &str = "./GMSv83_4GB.exe";

// DLL to inject
const INJECT_DLL: &str = "./mapledev.dll";

// CREATE_SUSPENDED
const CREATION_FLAGS: DWORD = 4;

fn main() -> Result<(), Error> {
    let exe = CString::new(MAPLESTORY).map_err(|_| Error::CStringFailed(MAPLESTORY.to_string()))?;
    let dll = CString::new(INJECT_DLL).map_err(|_| Error::CStringFailed(INJECT_DLL.to_string()))?;

    let mut si: _STARTUPINFOA = unsafe { ::std::mem::zeroed() };
    let mut pi: _PROCESS_INFORMATION = unsafe { ::std::mem::zeroed() };

    if unsafe {
        DetourCreateProcessWithDllExA(
            exe.as_ptr() as LPCSTR,
            0 as LPSTR,
            0 as LPSECURITY_ATTRIBUTES,
            0 as LPSECURITY_ATTRIBUTES,
            0 as BOOL,
            CREATION_FLAGS,
            0 as LPVOID,
            0 as LPCSTR,
            &mut si,
            &mut pi,
            dll.as_ptr() as LPCSTR,
            None,
        )
    } == 0
    {
        return Err(Error::ProcessFailed);
    }
    Ok(())
}
