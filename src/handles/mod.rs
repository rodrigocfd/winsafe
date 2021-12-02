//! Win32
//! [handles](https://docs.microsoft.com/en-us/windows/win32/sysinfo/handles-and-objects).

#[macro_use] mod macros;

mod haccel;
mod hbitmap;
mod hbrush;
mod hcursor;
mod hdc;
mod hdrop;
mod hdwp;
mod hfile;
mod hfilemap;
mod hfindfile;
mod hfont;
mod hglobal;
mod hhook;
mod hicon;
mod himagelist;
mod hinstance;
mod hkey;
mod hlocal;
mod hmenu;
mod hmonitor;
mod hpen;
mod hpipe;
mod hprocess;
mod hprocesslist;
mod hrgn;
mod htheme;
mod hthread;
mod hupdatersrc;
mod hwnd;
mod traits;
mod without_impl;

pub use haccel::HACCEL;
pub use hbitmap::HBITMAP;
pub use hbrush::HBRUSH;
pub use hcursor::HCURSOR;
pub use hdc::HDC;
pub use hdrop::HDROP;
pub use hdwp::HDWP;
pub use hfile::HFILE;
pub use hfilemap::{HFILEMAP, HFILEMAPVIEW};
pub use hfindfile::HFINDFILE;
pub use hfont::HFONT;
pub use hglobal::HGLOBAL;
pub use hhook::HHOOK;
pub use hicon::HICON;
pub use himagelist::HIMAGELIST;
pub use hinstance::HINSTANCE;
pub use hkey::HKEY;
pub use hlocal::HLOCAL;
pub use hmenu::HMENU;
pub use hmonitor::HMONITOR;
pub use hpen::HPEN;
pub use hpipe::HPIPE;
pub use hprocess::HPROCESS;
pub use hprocesslist::HPROCESSLIST;
pub use hrgn::HRGN;
pub use htheme::HTHEME;
pub use hthread::HTHREAD;
pub use hupdatersrc::HUPDATERSRC;
pub use hwnd::HWND;
pub use without_impl::*;

pub(crate) mod prelude {
	pub use super::traits::*;
}
