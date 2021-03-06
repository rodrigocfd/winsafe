//! Win32 handles.

#[macro_use]
mod macros;

mod any_handles;
mod haccel;
mod hbrush;
mod hcursor;
mod hdc;
mod hdrop;
mod hdwp;
mod hfont;
mod hhook;
mod hicon;
mod himagelist;
mod hinstance;
mod hkey;
mod hlocal;
mod hmenu;
mod hrgn;
mod htheme;
mod hwnd;

pub use any_handles::*;
pub use haccel::HACCEL;
pub use hbrush::HBRUSH;
pub use hcursor::HCURSOR;
pub use hdc::HDC;
pub use hdrop::HDROP;
pub use hdwp::HDWP;
pub use hfont::HFONT;
pub use hhook::HHOOK;
pub use hicon::HICON;
pub use himagelist::HIMAGELIST;
pub use hinstance::HINSTANCE;
pub use hkey::HKEY;
pub use hlocal::HLOCAL;
pub use hmenu::HMENU;
pub use hrgn::HRGN;
pub use htheme::HTHEME;
pub use hwnd::HWND;
