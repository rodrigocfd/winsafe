mod haccel;
mod hclipboard;
mod hcursor;
mod hdc;
mod hdesk;
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
	pub use super::hclipboard::HCLIPBOARD;
	pub use super::hcursor::HCURSOR;
	pub use super::hdc::HDC;
	pub use super::hdesk::HDESK;
	pub use super::hdwp::HDWP;
	pub use super::hhook::HHOOK;
	pub use super::hicon::HICON;
	pub use super::hmenu::HMENU;
	pub use super::hmonitor::HMONITOR;
	pub use super::hwnd::HWND;

	handle! { HBITMAP;
		/// Handle to a
		/// [bitmap](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbitmap).
	}

	handle! { HBRUSH;
		/// Handle to a
		/// [brush](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbrush).
	}

	handle! { HPALETTE;
		/// Handle to a
		/// [palette](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hpalette).
	}

	handle! { HRGN;
		/// Handle to a
		/// [region](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hrgn)
		/// GDI object.
	}
}
