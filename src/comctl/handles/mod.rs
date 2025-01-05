mod himagelist;
mod hpropsheetpage;
mod hwnd;

pub mod decl {
	pub use super::himagelist::HIMAGELIST;
	pub use super::hpropsheetpage::HPROPSHEETPAGE;

	impl_handle! { HTREEITEM;
		/// Handle to a
		/// [tree view item](https://learn.microsoft.com/en-us/windows/win32/controls/tree-view-controls).
	}
}

pub mod traits {
	pub use super::himagelist::comctl_Himagelist;
	pub use super::hpropsheetpage::comctl_Hpropsheetpage;
	pub use super::hwnd::comctl_Hwnd;
}
