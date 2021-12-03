//! Raw FFI bindings.

// The aliases below are simplified versions of Win32 types.
// They are supposed to be used only as syntactic sugar in the FFI calls.

#[macro_use] mod macros;

pub(crate) type BOOL = i32;
pub(crate) type HANDLE = *mut std::ffi::c_void;
pub(crate) type HRES = u32; // originally declared as i32
pub(crate) type PCSTR = *const u16;
pub(crate) type PCVOID = *const std::ffi::c_void;
pub(crate) type PFUNC = *const std::ffi::c_void;
pub(crate) type PSTR = *mut u16;
pub(crate) type PVOID = *mut std::ffi::c_void;

pub(crate) mod advapi32;
pub(crate) mod comctl32;
pub(crate) mod comdlg32;
pub(crate) mod gdi32;
pub(crate) mod kernel32;
pub(crate) mod msimg32;
pub(crate) mod ole32;
pub(crate) mod oleaut32;
pub(crate) mod shell32;
pub(crate) mod shlwapi;
pub(crate) mod user32;
pub(crate) mod uxtheme;
pub(crate) mod version;
