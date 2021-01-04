//! Raw FFI bindings.

// The aliases below are simplified versions of Win32 types.
// They are supposed to be used only as syntactic sugar in the FFI calls.

pub type BOOL = i32;
pub type HANDLE = *mut std::ffi::c_void;
pub type PCSTR = *const u16;
pub type PCVOID = *const std::ffi::c_void;
pub type PFUNC = *const std::ffi::c_void;
pub type PSTR = *mut u16;
pub type PVOID = *mut std::ffi::c_void;

pub mod advapi32;
pub mod comctl32;
pub mod gdi32;
pub mod kernel32;
pub mod ole32;
pub mod shell32;
pub mod user32;
pub mod uxtheme;
