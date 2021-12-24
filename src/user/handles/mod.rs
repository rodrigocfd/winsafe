mod haccel;
mod hcursor;
mod hdc;
mod hdwp;
mod hhook;
mod hicon;
mod hinstance;
mod hmenu;
mod hmonitor;
mod hprocess;
mod hwnd;

pub mod decl {
	pub use super::haccel::HACCEL;
	pub use super::hcursor::HCURSOR;
	pub use super::hdc::HDC;
	pub use super::hdwp::HDWP;
	pub use super::hhook::HHOOK;
	pub use super::hicon::HICON;
	pub use super::hmenu::HMENU;
	pub use super::hmonitor::HMONITOR;
	pub use super::hwnd::HWND;

	impl_handle! { HBITMAP: "user";
		/// Handle to a
		/// [bitmap](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbitmap).
	}

	impl_handle! { HBRUSH: "user";
		/// Handle to a
		/// [brush](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbrush).
	}

	impl_handle! { HRGN: "user";
		/// Handle to a
		/// [region](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hrgn)
		/// GDI object.
	}
}

pub mod traits {
	pub use super::haccel::UserHaccel;
	pub use super::hcursor::UserHcursor;
	pub use super::hdc::UserHdc;
	pub use super::hdwp::UserHdwp;
	pub use super::hhook::UserHhook;
	pub use super::hicon::UserHicon;
	pub use super::hinstance::UserHinstance;
	pub use super::hmenu::UserHmenu;
	pub use super::hmonitor::UserHmonitor;
	pub use super::hprocess::UserHprocess;
	pub use super::hwnd::UserHwnd;
}
