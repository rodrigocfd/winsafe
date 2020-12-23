//! Win32 handles.

#[macro_use]
mod macros;

mod haccel;
mod hbitmap;
mod hbrush;
mod hcursor;
mod hdc;
mod hdrop;
mod hfont;
mod hhook;
mod hicon;
mod hinstance;
mod hkey;
mod hlocal;
mod hmenu;
mod hpen;
mod hrgn;
mod hwnd;

pub use haccel::*;
pub use hbitmap::*;
pub use hbrush::*;
pub use hcursor::*;
pub use hdc::*;
pub use hdrop::*;
pub use hfont::*;
pub use hhook::*;
pub use hicon::*;
pub use hinstance::*;
pub use hkey::*;
pub use hlocal::*;
pub use hmenu::*;
pub use hpen::*;
pub use hrgn::*;
pub use hwnd::*;