//! Win32 handles.

#[macro_use]
mod macros;

mod hinstance;
mod hlocal;
mod hwnd;

pub use hinstance::*;
pub use hlocal::*;
pub use hwnd::*;