//! Raw FFI bindings.

/// Placeholder to any Win32 handle.
pub type HANDLE = *mut std::ffi::c_void;

pub mod advapi32;
pub mod comctl32;
pub mod gdi32;
pub mod kernel32;
pub mod ole32;
pub mod user32;