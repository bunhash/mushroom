#![cfg(all(target_arch = "x86", target_os = "windows"))]
//! Launches MapleStory and injects mapledev.dll

use std::ffi::CString;
use std::path::Path;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use winapi::shared::minwindef::{DWORD, FALSE, LPVOID};
use winapi::um::handleapi::CloseHandle;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::processthreadsapi::{
    CreateProcessA, CreateRemoteThread, OpenProcess, ResumeThread, PROCESS_INFORMATION,
    STARTUPINFOA,
};

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
    let module_name = CString::new("Kernel32.dll").unwrap();
    let m_handle = GetModuleHandleA(module_name.as_ptr());
    if m_handle == ::std::ptr::null_mut() {
        return Err(Error::Kernel32NotFound);
    }

    // Load Kernel32.dll and grab the LoadLibraryA function
    let symbol_name = CString::new("LoadLibraryA").unwrap();
    let symbol_addr = GetProcAddress(m_handle, symbol_name.as_ptr());
    if symbol_addr == ::std::ptr::null_mut() {
        return Err(Error::Kernel32Loading);
    }

    // Cast the function ptr
    let symbol_addr: Option<unsafe extern "system" fn(LPVOID) -> DWORD> =
        Some(::std::mem::transmute(symbol_addr));

    // Open the process with extra privileges
    let phandle = OpenProcess(ACCESS_FLAGS, 1, pid);
    if phandle == ::std::ptr::null_mut() {
        return Err(Error::OpenProcess);
    }

    // Extend the virtual memory
    let address = VirtualAllocEx(
        phandle,
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
        phandle,
        address,
        dll.as_ptr() as LPVOID,
        dll.as_bytes().len(),
        ::std::ptr::null_mut(),
    ) == FALSE
    {
        return Err(Error::Injection);
    }

    // Load DLL with LoadLibraryA
    if CreateRemoteThread(
        phandle,
        ::std::ptr::null_mut(),
        0,
        symbol_addr,
        address,
        0,
        ::std::ptr::null_mut(),
    ) == ::std::ptr::null_mut()
    {
        return Err(Error::Loading);
    }

    // wait for remote thread?

    Ok(())
}

pub fn main() -> Result<(), Error> {
    let ms_exe = CString::new(MAPLESTORY).unwrap();
    let mut si: STARTUPINFOA = unsafe { ::std::mem::zeroed() };
    let mut pi: PROCESS_INFORMATION = unsafe { ::std::mem::zeroed() };
    if unsafe {
        CreateProcessA(
            ms_exe.as_ptr(),
            ::std::ptr::null_mut(),
            ::std::ptr::null_mut(),
            ::std::ptr::null_mut(),
            FALSE,
            CREATION_FLAGS,
            ::std::ptr::null_mut(),
            ::std::ptr::null_mut(),
            &mut si,
            &mut pi,
        )
    } == FALSE
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
            inject_dll(pid, CString::new(INJECT_DLL).unwrap())?;
            ResumeThread(pi.hThread);
            CloseHandle(pi.hThread);
            CloseHandle(pi.hProcess);
        }
        Ok(())
    }
}
