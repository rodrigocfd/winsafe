mod himagelist;
mod hpropsheetpage;
mod hwnd;

pub mod decl {
	pub use super::himagelist::HIMAGELIST;
	pub use super::hpropsheetpage::HPROPSHEETPAGE;
	use crate::macros::*;

	handle! { HTREEITEM;
		/// Handle to a
		/// [tree view item](https://learn.microsoft.com/en-us/windows/win32/controls/tree-view-controls).
	}
}
