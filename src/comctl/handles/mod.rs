mod himagelist;
mod hwnd;

pub mod decl {
	pub use super::himagelist::HIMAGELIST;

	impl_handle! { HTREEITEM: "comctl";
		/// Handle to an
		/// [tree view item](https://learn.microsoft.com/en-us/windows/win32/controls/tree-view-controls).
	}
}

pub mod guard {
	pub use super::himagelist::HimagelistDragGuard;
}

pub mod traits {
	pub use super::himagelist::comctl_Himagelist;
	pub use super::hwnd::comctl_Hwnd;
}