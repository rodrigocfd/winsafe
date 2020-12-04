//! Win32 handles.

#[macro_use]
mod macros;

mod hinstance;
mod hlocal;
mod hmenu;
mod hwnd;

pub use hinstance::*;
pub use hlocal::*;
pub use hmenu::*;
pub use hwnd::*;