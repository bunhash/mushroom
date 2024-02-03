//! user32.dll window hooks
//!
//! Initialization occurs at 504030D3 in gr2d_dx8
//!
//! MapleStory.exe calls it with (probably):
//!
//!   init(user32.75C30003, 0xDB100003, 0x4003, 258, 320, 0x4a7e01c)
//!

use crate::utils;
use retour::static_detour;
use std::ffi::{CStr, CString};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPINT, LPVOID, UINT, ULONG, WORD};
use winapi::shared::windef::{HMENU, HWND};
use winapi::um::processthreadsapi::ExitProcess;
use winapi::um::winnt::{LONG, LPCSTR};

/// The Name of the Window
const WINDOW_NAME: &str = "MapleDev";

static_detour! {
    /// CreateWindowExA hook structure
    static CreateWindowExAHook: unsafe extern "system" fn(DWORD, LPCSTR, LPCSTR, DWORD, c_int, c_int, c_int, c_int, HWND, HMENU, HINSTANCE, LPVOID) -> HWND;
}

static_detour! {
    /// SetWindowLongA hook structure
    static SetWindowLongAHook: unsafe extern "system" fn(HWND, c_int, LONG) -> LONG;
}

static_detour! {
    /// SetWindowPos hook structure
    static SetWindowPosHook: unsafe extern "system" fn(HWND, HWND, c_int, c_int, c_int, c_int, UINT) -> BOOL;
}

type CreateWindowExAFn = unsafe extern "system" fn(
    DWORD,
    LPCSTR,
    LPCSTR,
    DWORD,
    c_int,
    c_int,
    c_int,
    c_int,
    HWND,
    HMENU,
    HINSTANCE,
    LPVOID,
) -> HWND;
type SetWindowLongFn = unsafe extern "system" fn(HWND, c_int, LONG) -> LONG;
type SetWindowPosFn =
    unsafe extern "system" fn(HWND, HWND, c_int, c_int, c_int, c_int, UINT) -> BOOL;

const BORDERED: LONG = 0xc80000;

#[allow(non_snake_case)]
fn CreateWindowExA_detour(
    dwExStyle: DWORD,
    lpClassName: LPCSTR,
    lpWindowName: LPCSTR,
    dwStyle: DWORD,
    x: c_int,
    y: c_int,
    nWidth: c_int,
    nHeight: c_int,
    hWndParent: HWND,
    hMenu: HMENU,
    hInstance: HINSTANCE,
    lpParam: LPVOID,
) -> HWND {
    let window_name = unsafe { CStr::from_ptr(lpWindowName) }
        .to_string_lossy()
        .to_string();
    winlog!(
        "[CreateWindowExA] Name: {}, Style: {:x}, x: {}, y: {}, width: {}, height: {}",
        window_name,
        dwStyle,
        x,
        y,
        nWidth,
        nHeight
    );
    unsafe { ExitProcess(3424) };
    panic!();

    if nWidth != 800 || nHeight != 600 {
        // This needs to occur sooner... I should probably just hijack 0x9f1c04 to do my patching
        //
        // ...
        // 009F18C9 | 55                       | push ebp |
        // 009F18CA | 8BEC                     | mov ebp,esp |
        // 009F18CC | 81EC 10020000            | sub esp,210 |
        // 009F18D2 | 80A5 F0FDFFFF 00         | and byte ptr ss:[ebp-210],0 |
        // 009F18D9 | 8D85 F0FDFFFF            | lea eax,dword ptr ss:[ebp-210] |
        // 009F18DF | 68 B8F2B300              | push maplestory.B3F2B8 | B3F2B8:"http://Ingameweb.nexon.net/maplestory/client/launcher.html"
        // 009F18E4 | 50                       | push eax | eax:&"WvsClientMtx"
        // 009F18E5 | E8 660C0700              | call maplestory.A62550 |
        // 009F18EA | 59                       | pop ecx |
        // 009F18EB | 59                       | pop ecx |
        // 009F18EC | 6A 00                    | push 0 |
        // 009F18EE | C745 F0 76020000         | mov dword ptr ss:[ebp-10],276 |
        // 009F18F5 | C745 F4 94020000         | mov dword ptr ss:[ebp-C],294 |
        // 009F18FC | C745 F8 05000000         | mov dword ptr ss:[ebp-8],5 |
        // 009F1903 | FF15 8802BF00            | call dword ptr ds:[<&GetModuleHandleA>] |
        // 009F1909 | 8945 FC                  | mov dword ptr ss:[ebp-4],eax |
        // 009F190C | 8D85 F0FDFFFF            | lea eax,dword ptr ss:[ebp-210] |
        // 009F1912 | 50                       | push eax | eax:&"WvsClientMtx"
        // 009F1913 | E8 5892D8FF              | call maplestory.77AB70 |
        // 009F1918 | 59                       | pop ecx |
        // 009F1919 | C9                       | leave |
        // 009F191A | C3                       | ret |
        // ...
        // 009F1C04 | E8 C0FCFFFF              | call maplestory.9F18C9 |
        unsafe {
            // Patch
            utils::patch(0x9f1c04, &mut [0x90; 5]);
            // 009F4E84 | EB 10                    | jmp gmsv83_4gb.9F4E96 |
            // 009F4E86 | 90                       | nop |
            // ...
            // 009F4E96 | 8B01                     | mov eax,dword ptr ds:[ecx] | ecx:EntryPoint
            // 009F4E98 | 8B55 08                  | mov edx,dword ptr ss:[ebp+8] | edx:EntryPoint
            // 009F4E9B | 0FB61417                 | movzx edx,byte ptr ds:[edi+edx] | edx:EntryPoint
            // 009F4E9F | 8365 14 00               | and dword ptr ss:[ebp+14],0 |
            // 009F4EA3 | 89C3                     | mov ebx,eax |
            // 009F4EA5 | 21F3                     | and ebx,esi | esi:EntryPoint
            // 009F4EA7 | 31DA                     | xor edx,ebx | edx:EntryPoint
            // 009F4EA9 | 8B1495 7C16BF00          | mov edx,dword ptr ds:[edx*4+BF167C] | edx:EntryPoint
            // 009F4EB0 | C1E8 08                  | shr eax,8 |
            // 009F4EB3 | 31C2                     | xor edx,eax | edx:EntryPoint
            // 009F4EB5 | 8B45 10                  | mov eax,dword ptr ss:[ebp+10] |
            // 009F4EB8 | 8911                     | mov dword ptr ds:[ecx],edx | ecx:EntryPoint, edx:EntryPoint
            // 009F4EBA | C700 2B030000            | mov dword ptr ds:[eax],32B |
            // 009F4EC0 | 8B01                     | mov eax,dword ptr ds:[ecx] | ecx:EntryPoint
            // 009F4EC2 | 40                       | inc eax |
            // 009F4EC3 | EB 10                    | jmp gmsv83_4gb.9F4ED5 |
            // 009F4EC5 | 90                       | nop |
            // ...
            // 009F4ED5 | EB 20                    | jmp gmsv83_4gb.9F4EF7 |
            // 009F4ED7 | 8B11                     | mov edx,dword ptr ds:[ecx] | edx:EntryPoint, ecx:EntryPoint
            // 009F4ED9 | 8B5D 08                  | mov ebx,dword ptr ss:[ebp+8] |
            utils::patch(
                0x9f4e84,
                &mut [
                    0xeb, 0x10, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
                    0x90, 0x90, 0x90, 0x90, 0x90, 0x8b, 0x01, 0x8b, 0x55, 0x08, 0x0f, 0xb6, 0x14,
                    0x17, 0x83, 0x65, 0x14, 0x00, 0x89, 0xc3, 0x21, 0xf3, 0x31, 0xda, 0x8b, 0x14,
                    0x95, 0x7c, 0x16, 0xbf, 0x00, 0xc1, 0xe8, 0x08, 0x31, 0xc2, 0x8b, 0x45, 0x10,
                    0x89, 0x11, 0xc7, 0x00, 0x2b, 0x03, 0x00, 0x00, 0x8b, 0x01, 0x40, 0xeb, 0x10,
                    0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
                    0x90, 0x90, 0x90, 0xeb, 0x20, 0x8b, 0x11, 0x8b, 0x5d, 0x08,
                ],
            );
        }
    } else {
        // do stuff
    }
    let window_name = CString::new(WINDOW_NAME).unwrap_or_else(|e| {
        winlog!("[CreateWindowExA] ERROR: {:?}", e);
        unsafe { ExitProcess(3424) };
        panic!();
    });
    unsafe {
        CreateWindowExAHook.call(
            dwExStyle,
            lpClassName,
            window_name.as_ptr(),
            dwStyle,
            x,
            y,
            nWidth,
            nHeight,
            hWndParent,
            hMenu,
            hInstance,
            lpParam,
        )
    }
}

#[allow(non_snake_case)]
fn SetWindowLongA_detour(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> LONG {
    winlog!(
        "[SetWindowLongA] Index: {:?}, Value: {:x}",
        nIndex,
        dwNewLong
    );
    if nIndex == -4 {
        /*
        winlog!(
        "[SetWindowLongA] overriding GWL_WNDPROC: {:x} -> 9fe8c9",
        dwNewLong
        );
        unsafe { SetWindowLongAHook.call(hWnd, nIndex, 0x9fe8c9) }
        */
        unsafe { SetWindowLongAHook.call(hWnd, nIndex, dwNewLong) }
    } else {
        unsafe { SetWindowLongAHook.call(hWnd, nIndex, dwNewLong) }
    }
}

#[allow(non_snake_case)]
fn SetWindowPos_detour(
    hWnd: HWND,
    hWndInsertAfter: HWND,
    X: c_int,
    Y: c_int,
    cx: c_int,
    cy: c_int,
    uFlags: UINT,
) -> BOOL {
    winlog!(
        "[SetWindowPos] X: {:?}, Y: {:?}, cx: {:?}, cy: {:?}, uFlags: {:x}",
        X,
        Y,
        cx,
        cy,
        uFlags
    );
    unsafe { SetWindowPosHook.call(hWnd, hWndInsertAfter, X, Y, cx, cy, uFlags) }
}

/// Sets up user32.dll hooks
pub(crate) unsafe fn main() {
    let user32 = utils::load_module("user32.dll").unwrap_or_else(|e| {
        winlog!("[window::main] ERROR: {:?}", e);
        unsafe { ExitProcess(3424) };
        panic!();
    });

    // Hook CreateWindowExA
    let cwea = utils::get_symbol(user32, "CreateWindowExA").unwrap_or_else(|e| {
        winlog!("[window::main] ERROR: {:?}", e);
        unsafe { ExitProcess(3424) };
        panic!();
    });
    let cwea: CreateWindowExAFn = ::std::mem::transmute(cwea);
    CreateWindowExAHook
        .initialize(cwea, CreateWindowExA_detour)
        .unwrap_or_else(|e| {
            winlog!("[window::main] ERROR: {:?}", e);
            unsafe { ExitProcess(3424) };
            panic!();
        })
        .enable()
        .unwrap_or_else(|e| {
            winlog!("[window::main] ERROR: {:?}", e);
            unsafe { ExitProcess(3424) };
            panic!();
        });

    // Hook SetWindowLongA
    let swla = utils::get_symbol(user32, "SetWindowLongA").unwrap_or_else(|e| {
        winlog!("[window::main] ERROR: {:?}", e);
        unsafe { ExitProcess(3424) };
        panic!();
    });
    let swla: SetWindowLongFn = ::std::mem::transmute(swla);
    SetWindowLongAHook
        .initialize(swla, SetWindowLongA_detour)
        .unwrap_or_else(|e| {
            winlog!("[window::main] ERROR: {:?}", e);
            unsafe { ExitProcess(3424) };
            panic!();
        })
        .enable()
        .unwrap_or_else(|e| {
            winlog!("[window::main] ERROR: {:?}", e);
            unsafe { ExitProcess(3424) };
            panic!();
        });

    // Hook SetWindowPos
    let swp = utils::get_symbol(user32, "SetWindowPos").unwrap_or_else(|e| {
        winlog!("[window::main] ERROR: {:?}", e);
        unsafe { ExitProcess(3424) };
        panic!();
    });
    let swp: SetWindowPosFn = ::std::mem::transmute(swp);
    SetWindowPosHook
        .initialize(swp, SetWindowPos_detour)
        .unwrap_or_else(|e| {
            winlog!("[window::main] ERROR: {:?}", e);
            unsafe { ExitProcess(3424) };
            panic!();
        })
        .enable()
        .unwrap_or_else(|e| {
            winlog!("[window::main] ERROR: {:?}", e);
            unsafe { ExitProcess(3424) };
            panic!();
        });
}
