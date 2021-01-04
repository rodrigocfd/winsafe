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
mod htheme;
mod hwnd;

pub use haccel::HACCEL;
pub use hbitmap::HBITMAP;
pub use hbrush::HBRUSH;
pub use hcursor::HCURSOR;
pub use hdc::HDC;
pub use hdrop::HDROP;
pub use hfont::HFONT;
pub use hhook::HHOOK;
pub use hicon::HICON;
pub use hinstance::HINSTANCE;
pub use hkey::HKEY;
pub use hlocal::HLOCAL;
pub use hmenu::HMENU;
pub use hpen::HPEN;
pub use hrgn::HRGN;
pub use htheme::HTHEME;
pub use hwnd::HWND;
