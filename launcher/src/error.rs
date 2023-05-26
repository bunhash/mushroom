//! Launcher/DLL errors

use std::fmt;

/// Launcher and DLL errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    CStringFailed(String),
    Path(String),
    ProcessNotFound(String),
    ModuleNotLoaded(String),
    ModuleNotFound(String),
    SymbolNotFound(String, String),
    ProcessFailed,
    ProcessNotOpened,
    VMemAllocFailed,
    InjectionFailed,
    ThreadFailed,
    HookInitializeFailed(String),
    HookEnableFailed(String),
    AddressFormat,
    Unknown(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::CStringFailed(s) => write!(f, "CString failed `{}`", s),
            Self::Path(s) => write!(f, "Path error: {}", s),
            Self::ProcessNotFound(n) => write!(f, "Could not find process `{}`", n),
            Self::ModuleNotLoaded(n) => write!(f, "Could not load `{}`", n),
            Self::ModuleNotFound(n) => write!(f, "Could not find module `{}`", n),
            Self::SymbolNotFound(m, s) => write!(f, "Could not find `{}` in {}", s, m),
            Self::ProcessFailed => write!(f, "Process could not be started"),
            Self::ProcessNotOpened => write!(f, "Could not open process"),
            Self::VMemAllocFailed => write!(f, "Could not allocate vmem"),
            Self::InjectionFailed => write!(f, "Injection failed"),
            Self::ThreadFailed => write!(f, "Remote thread failed"),
            Self::HookInitializeFailed(func) => write!(f, "Could not hook `{}`", func),
            Self::HookEnableFailed(func) => write!(f, "Hook initialization failed `{}`", func),
            Self::AddressFormat => write!(f, "Address could not be formatted"),
            Self::Unknown(s) => write!(f, "Unknown: {}", s),
        }
    }
}
