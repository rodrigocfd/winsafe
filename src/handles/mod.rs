//! Win32 handles.

#[macro_use]
mod macros;

mod haccel;
mod hbitmap;
mod hbrush;
mod hcursor;
mod hdc;
mod hicon;
mod hinstance;
mod hlocal;
mod hmenu;
mod hrgn;
mod hwnd;

pub use haccel::*;
pub use hbitmap::*;
pub use hbrush::*;
pub use hcursor::*;
pub use hdc::*;
pub use hicon::*;
pub use hinstance::*;
pub use hlocal::*;
pub use hmenu::*;
pub use hrgn::*;
pub use hwnd::*;