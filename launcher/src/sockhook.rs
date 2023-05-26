//! mswsock.dll hooks

use crate::error::Error;
use crate::utils;
use retour::static_detour;
use std::ffi::{CStr, CString};
use std::sync::Mutex;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LPINT, ULONG, WORD};
use winapi::shared::ws2def::{LPSOCKADDR, LPWSABUF, SOCKADDR_IN};
use winapi::um::processthreadsapi::ExitProcess;
use winapi::um::winsock2::{
    inet_addr, inet_ntoa, ntohs, LPQOS, LPSOCKADDR_IN, LPWSAPROTOCOL_INFOW, SOCKET,
};
use winapi::um::ws2spi::{LPWSPDATA, LPWSPPROC_TABLE, WSPUPCALLTABLE};

/// The IP to redirect INET traffic to
const IP: &str = "172.17.112.1";

static_detour! {
    /// WSPStartup hook structure
    static WSPStartupHook: unsafe extern "system" fn(WORD, LPWSPDATA, LPWSAPROTOCOL_INFOW, WSPUPCALLTABLE, LPWSPPROC_TABLE) -> c_int;
}

/// WSPStartup function definition
type WSPStartupFn = unsafe extern "system" fn(
    WORD,
    LPWSPDATA,
    LPWSAPROTOCOL_INFOW,
    WSPUPCALLTABLE,
    LPWSPPROC_TABLE,
) -> c_int;

/// WSPGetPeerName function definition
type WSPGetPeerNameFn = unsafe extern "system" fn(SOCKET, LPSOCKADDR, LPINT, LPINT) -> c_int;

/// WSPConnect function definition
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
    /// Original WSPGetPeerName function
    static ref WSPGETPEERNAME: Mutex<Option<WSPGetPeerNameFn>> = Mutex::new(None);
}

lazy_static! {
    /// Original WSPConnect function
    static ref WSPCONNECT: Mutex<Option<WSPConnectFn>> = Mutex::new(None);
}

lazy_static! {
    /// The encoded address to redirect INET traffic to
    static ref REROUTED_ADDR: Mutex<ULONG> = Mutex::new(unsafe { ::std::mem::zeroed() });
}

lazy_static! {
    /// The original address the client was trying to reach
    static ref LAST_CONNECT: Mutex<ULONG> = Mutex::new(unsafe { ::std::mem::zeroed() });
}

/// Wrapped static function
#[allow(non_snake_case)]
unsafe fn WSPGetPeerName(sock: SOCKET, name: LPSOCKADDR, namelen: LPINT, lpErrno: LPINT) -> c_int {
    WSPGETPEERNAME
        .lock()
        .unwrap_or_else(|e| {
            winlog!("[WSPGetPeerName] ERROR: {:?}", e);
            ExitProcess(3424);
            panic!();
        })
        .unwrap_or_else(|| {
            winlog!("[WSPGetPeerName] ERROR: WSPGetPeerName null");
            ExitProcess(3424);
            panic!();
        })(sock, name, namelen, lpErrno)
}

/// Wrapped static function
#[allow(non_snake_case)]
unsafe fn WSPConnect(
    sock: SOCKET,
    name: LPSOCKADDR,
    namelen: c_int,
    lpCallerData: LPWSABUF,
    lpCalleeData: LPWSABUF,
    lpSQOS: LPQOS,
    lpGQOS: LPQOS,
    lpErrno: LPINT,
) -> c_int {
    WSPCONNECT
        .lock()
        .unwrap_or_else(|e| {
            winlog!("[WSPConnect] ERROR: {:?}", e);
            ExitProcess(3424);
            panic!();
        })
        .unwrap_or_else(|| {
            winlog!("[WSPConnect] ERROR: WSPConnect null");
            ExitProcess(3424);
            panic!();
        })(
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

/// WSPGetPeerName Detour
#[allow(non_snake_case)]
unsafe extern "system" fn WSPGetPeerName_detour(
    sock: SOCKET,
    name: LPSOCKADDR,
    namelen: LPINT,
    lpErrno: LPINT,
) -> c_int {
    let ret = WSPGetPeerName(sock, name, namelen, lpErrno);

    let from_addr: LPSOCKADDR_IN = ::std::mem::transmute(name);

    let port = ntohs((*from_addr).sin_port);

    // Only if this is the login portal
    if port >= 8000 && port < 9000 {
        let mut to_addr: SOCKADDR_IN = ::std::mem::zeroed();
        *to_addr.sin_addr.S_un.S_addr_mut() = *LAST_CONNECT.lock().unwrap_or_else(|e| {
            winlog!("[WSPGetPeerName] ERROR: {:?}", e);
            ExitProcess(3424);
            panic!();
        });

        // Debug
        let from_ip: String = CStr::from_ptr(inet_ntoa((*from_addr).sin_addr))
            .to_string_lossy()
            .into();
        let port = ntohs((*from_addr).sin_port);
        let to_ip: String = CStr::from_ptr(inet_ntoa(to_addr.sin_addr))
            .to_string_lossy()
            .into();
        winlog!(
            "[WSPGetPeerName] Replaced: {}:{} -> {}:{}",
            from_ip,
            port,
            to_ip,
            port,
        );

        // Overwrite response
        (*from_addr).sin_addr = to_addr.sin_addr;
    }

    ret
}

/// WSPConnect Detour
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
    let from_addr: LPSOCKADDR_IN = ::std::mem::transmute(name);

    let port = ntohs((*from_addr).sin_port);

    // Only if this is the login portal
    if port >= 8000 && port < 9000 {
        // Debug
        let from_ip: String = CStr::from_ptr(inet_ntoa((*from_addr).sin_addr))
            .to_string_lossy()
            .into();
        winlog!(
            "[WSPConnect] Replaced: {}:{} -> {}:{}",
            from_ip,
            port,
            IP,
            port,
        );

        // Save original routing information
        *LAST_CONNECT.lock().unwrap_or_else(|e| {
            winlog!("[WSPConnect] ERROR: {:?}", e);
            ExitProcess(3424);
            panic!();
        }) = *(*from_addr).sin_addr.S_un.S_addr();

        // Overwrite destination
        *(*from_addr).sin_addr.S_un.S_addr_mut() = *REROUTED_ADDR.lock().unwrap_or_else(|e| {
            winlog!("[WSPConnect] ERROR: {:?}", e);
            ExitProcess(3424);
            panic!();
        });
    }

    WSPConnect(
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

/// WSPStartup Detour
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
        *WSPGETPEERNAME.lock().unwrap_or_else(|e| {
            winlog!("[WSPStartup] ERROR: {:?}", e);
            unsafe { ExitProcess(3424) };
            panic!();
        }) = unsafe { (*lpProcTable).lpWSPGetPeerName };
        unsafe { (*lpProcTable).lpWSPGetPeerName = Some(WSPGetPeerName_detour) };

        // Hook WSPConnect
        *WSPCONNECT.lock().unwrap_or_else(|e| {
            winlog!("[WSPStartup] ERROR: {:?}", e);
            unsafe { ExitProcess(3424) };
            panic!();
        }) = unsafe { (*lpProcTable).lpWSPConnect };
        unsafe { (*lpProcTable).lpWSPConnect = Some(WSPConnect_detour) };
    }
    ret
}

/// Sets up mswsock.dll hooks
pub(crate) unsafe fn main() -> Result<(), Error> {
    let ip = CString::new(IP).map_err(|_| Error::CStringFailed(IP.into()))?;
    *REROUTED_ADDR
        .lock()
        .map_err(|e| Error::Unknown(format!("{:?}", e)))? = inet_addr(ip.as_ptr());
    let address = utils::load_module_symbol("mswsock.dll", "WSPStartup")?;
    let target: WSPStartupFn = ::std::mem::transmute(address);
    WSPStartupHook
        .initialize(target, WSPStartup_detour)
        .map_err(|_| Error::HookInitializeFailed("WSPStartup".into()))?
        .enable()
        .map_err(|_| Error::HookEnableFailed("WSPStartup".into()))?;
    Ok(())
}
