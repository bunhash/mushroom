#![cfg(target_os = "windows")]
//! Launches MapleStory and injects mapledev.dll

use std::ffi::CString;
use std::path::Path;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::handleapi::CloseHandle;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::processthreadsapi::{
    CreateProcessA, CreateRemoteThread, OpenProcess, ResumeThread, PROCESS_INFORMATION,
    STARTUPINFOA,
};
use winapi::um::winnt::LPCSTR;

// MapleStory EXE
const MAPLESTORY: &str = "./GMSv83_4GB_docker.exe";

// DLL to inject
const INJECT_DLL: &str = "mapledev.dll";

// CREATE_SUSPENDED
const CREATION_FLAGS: DWORD = 0x4;

// PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE |
// PROCESS_QUERY_INFORMATION
const ACCESS_FLAGS: DWORD = 0x2 | 0x8 | 0x10 | 0x20 | 0x400;

// MEM_COMMIT | MEM_RESERVE
const MEM_FLAGS: DWORD = 0x1000 | 0x2000;

// PAGE_EXECUTE_READWRITE
const PAGE_FLAGS: DWORD = 0x40;

/// Launcher Errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Path(String),
    ProcessFailed,
    CStringFailed(String),
    ProcessNotFound,
    Kernel32NotFound,
    Kernel32Loading,
    OpenProcess,
    MemoryAllocation,
    Injection,
    Loading,
}

fn get_pid(name: &str) -> Result<Pid, Error> {
    let mut system = System::new();
    system.refresh_processes();
    let mut it = system.processes_by_name(name);
    Ok(it.next().ok_or(Error::ProcessNotFound)?.pid())
}

unsafe fn inject_dll(pid: DWORD, dll: CString) -> Result<(), Error> {
    // Get Kernel32.dll handle
    let kernel32 = CString::new("Kernel32.dll")
        .map_err(|_| Error::CStringFailed("Kernel32.dll".to_string()))?;
    let module = GetModuleHandleA(kernel32.as_ptr() as LPCSTR);
    if module == ::std::ptr::null_mut() {
        return Err(Error::Kernel32NotFound);
    }

    // Load Kernel32.dll and grab the LoadLibraryA function
    let load_library_fn = CString::new("LoadLibraryA")
        .map_err(|_| Error::CStringFailed("LoadLibraryA".to_string()))?;
    let load_addr = GetProcAddress(module, load_library_fn.as_ptr() as LPCSTR);
    if load_addr == ::std::ptr::null_mut() {
        return Err(Error::Kernel32Loading);
    }

    // Cast the function ptr
    let load_addr: Option<unsafe extern "system" fn(LPVOID) -> DWORD> =
        Some(::std::mem::transmute(load_addr));

    // Open the process with extra privileges
    let handle = OpenProcess(ACCESS_FLAGS, 1, pid);
    if handle == ::std::ptr::null_mut() {
        return Err(Error::OpenProcess);
    }

    // Extend the virtual memory
    let address = VirtualAllocEx(
        handle,
        ::std::ptr::null_mut(),
        dll.as_bytes().len(),
        MEM_FLAGS,
        PAGE_FLAGS,
    );
    if address == ::std::ptr::null_mut() {
        return Err(Error::MemoryAllocation);
    }

    // Inject
    if WriteProcessMemory(
        handle,
        address,
        dll.as_ptr() as LPVOID,
        dll.as_bytes().len(),
        ::std::ptr::null_mut(),
    ) == 0
    {
        return Err(Error::Injection);
    }

    // Load DLL with LoadLibraryA
    if CreateRemoteThread(
        handle,
        ::std::ptr::null_mut(),
        0,
        load_addr,
        address,
        0,
        ::std::ptr::null_mut(),
    ) == ::std::ptr::null_mut()
    {
        return Err(Error::Loading);
    }

    Ok(())
}

pub fn main() -> Result<(), Error> {
    let ms_exe =
        CString::new(MAPLESTORY).map_err(|_| Error::CStringFailed(MAPLESTORY.to_string()))?;
    let mut si: STARTUPINFOA = unsafe { ::std::mem::zeroed() };
    let mut pi: PROCESS_INFORMATION = unsafe { ::std::mem::zeroed() };
    if unsafe {
        CreateProcessA(
            ms_exe.as_ptr(),
            ::std::ptr::null_mut(),
            ::std::ptr::null_mut(),
            ::std::ptr::null_mut(),
            0,
            CREATION_FLAGS,
            ::std::ptr::null_mut(),
            ::std::ptr::null_mut(),
            &mut si,
            &mut pi,
        )
    } == 0
    {
        Err(Error::ProcessFailed)
    } else {
        let pname = Path::new(MAPLESTORY)
            .file_name()
            .ok_or(Error::Path(MAPLESTORY.to_string()))?
            .to_str()
            .ok_or(Error::Path(MAPLESTORY.to_string()))?;
        let pid = get_pid(pname)?.as_u32() as DWORD;
        unsafe {
            inject_dll(
                pid,
                CString::new(INJECT_DLL)
                    .map_err(|_| Error::CStringFailed(INJECT_DLL.to_string()))?,
            )?;
            ResumeThread(pi.hThread);
            CloseHandle(pi.hThread);
            CloseHandle(pi.hProcess);
        }
        Ok(())
    }
}
