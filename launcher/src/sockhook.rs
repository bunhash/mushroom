//! winsock2.dll and ws2spi.dll hooks

use retour::static_detour;
use std::ffi::CString;
use std::sync::Mutex;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LPINT, WORD};
use winapi::shared::ws2def::{LPSOCKADDR, LPWSABUF};
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};
use winapi::um::winsock2::{inet_addr, LPQOS, LPSOCKADDR_IN, LPWSAPROTOCOL_INFOW, SOCKET};
use winapi::um::ws2spi::{LPWSPDATA, LPWSPPROC_TABLE, WSPUPCALLTABLE};

static_detour! {
    static WSPStartupHook: unsafe extern "system" fn(WORD, LPWSPDATA, LPWSAPROTOCOL_INFOW, WSPUPCALLTABLE, LPWSPPROC_TABLE) -> c_int;
}

type WSPStartupFn = unsafe extern "system" fn(
    WORD,
    LPWSPDATA,
    LPWSAPROTOCOL_INFOW,
    WSPUPCALLTABLE,
    LPWSPPROC_TABLE,
) -> c_int;
type WSPGetPeerNameFn = unsafe extern "system" fn(SOCKET, LPSOCKADDR, LPINT, LPINT) -> c_int;
type WSPConnectFn = unsafe extern "system" fn(
    SOCKET,
    LPSOCKADDR,
    c_int,
    LPWSABUF,
    LPWSABUF,
    LPQOS,
    LPQOS,
    LPINT,
) -> c_int;

lazy_static! {
    static ref LOCAL_SERVER: CString = CString::new("127.0.0.1").unwrap();
}

lazy_static! {
    static ref WSPGETPEERNAME: Mutex<Option<WSPGetPeerNameFn>> = Mutex::new(None);
}

lazy_static! {
    static ref WSPCONNECT: Mutex<Option<WSPConnectFn>> = Mutex::new(None);
}

pub enum Error {
    ModuleNotLoaded,
    SymbolNotFound,
    HookInitialize,
    HookEnable,
}

#[allow(non_snake_case)]
unsafe extern "system" fn WSPGetPeerName_detour(
    sock: SOCKET,
    name: LPSOCKADDR,
    namelen: LPINT,
    lpErrno: LPINT,
) -> c_int {
    WSPGETPEERNAME.lock().unwrap().unwrap()(sock, name, namelen, lpErrno)
}

#[allow(non_snake_case)]
unsafe extern "system" fn WSPConnect_detour(
    sock: SOCKET,
    name: LPSOCKADDR,
    namelen: c_int,
    lpCallerData: LPWSABUF,
    lpCalleeData: LPWSABUF,
    lpSQOS: LPQOS,
    lpGQOS: LPQOS,
    lpErrno: LPINT,
) -> c_int {
    let sockadder_in: LPSOCKADDR_IN = ::std::mem::transmute(name);
    *(*sockadder_in).sin_addr.S_un.S_addr_mut() = inet_addr(LOCAL_SERVER.as_ptr());
    WSPCONNECT.lock().unwrap().unwrap()(
        sock,
        name,
        namelen,
        lpCallerData,
        lpCalleeData,
        lpSQOS,
        lpGQOS,
        lpErrno,
    )
}

#[allow(non_snake_case)]
fn WSPStartup_detour(
    wVersionRequested: WORD,
    lpWSPData: LPWSPDATA,
    lpProtocolInfo: LPWSAPROTOCOL_INFOW,
    UpcallTable: WSPUPCALLTABLE,
    lpProcTable: LPWSPPROC_TABLE,
) -> c_int {
    let ret = unsafe {
        WSPStartupHook.call(
            wVersionRequested,
            lpWSPData,
            lpProtocolInfo,
            UpcallTable,
            lpProcTable,
        )
    };
    if ret == 0 {
        // Hook WSPGetPeerName
        *WSPGETPEERNAME.lock().unwrap() = unsafe { (*lpProcTable).lpWSPGetPeerName };
        unsafe { (*lpProcTable).lpWSPGetPeerName = Some(WSPGetPeerName_detour) };

        // Hook WSPConnect
        *WSPCONNECT.lock().unwrap() = unsafe { (*lpProcTable).lpWSPConnect };
        unsafe { (*lpProcTable).lpWSPConnect = Some(WSPConnect_detour) };
    }
    ret
}

unsafe fn load_module_symbol(module: &str, symbol: &str) -> Result<usize, Error> {
    let module = CString::new(module).unwrap();
    let symbol = CString::new(symbol).unwrap();
    let handle = LoadLibraryA(module.as_ptr());
    if handle == ::std::ptr::null_mut() {
        return Err(Error::ModuleNotLoaded);
    }
    let address = GetProcAddress(handle, symbol.as_ptr());
    if address == ::std::ptr::null_mut() {
        return Err(Error::SymbolNotFound);
    }
    Ok(address as usize)
}

pub(crate) unsafe fn main() -> Result<(), Error> {
    let address = load_module_symbol("mswsock.dll", "WSPStartup")?;
    let target: WSPStartupFn = ::std::mem::transmute(address);
    WSPStartupHook
        .initialize(target, WSPStartup_detour)
        .map_err(|_| Error::HookInitialize)?
        .enable()
        .map_err(|_| Error::HookEnable)?;
    Ok(())
}
