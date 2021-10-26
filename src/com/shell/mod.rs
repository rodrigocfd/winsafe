//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! COM interfaces, structs and constants.
//!
//! To enable the Shell COM module, use:
//!
//! ```toml
//! [dependencies]
//! winsafe = { version = "0.0.6", features = ["shell"] }
//! ```

pub mod clsid;
pub mod co;
pub mod guid;

pub(in crate::com) mod any_structs;
pub(in crate::com) mod ifiledialog;
pub(in crate::com) mod ifileopendialog;
pub(in crate::com) mod ifilesavedialog;
pub(in crate::com) mod imodalwindow;
pub(in crate::com) mod ishellitem;
pub(in crate::com) mod ishellitemarray;
pub(in crate::com) mod itaskbarlist;
pub(in crate::com) mod itaskbarlist2;
pub(in crate::com) mod itaskbarlist3;
pub(in crate::com) mod itaskbarlist4;

pub use any_structs::*;
pub use ifiledialog::IFileDialog;
pub use ifileopendialog::IFileOpenDialog;
pub use ifilesavedialog::IFileSaveDialog;
pub use imodalwindow::IModalWindow;
pub use ishellitem::IShellItem;
pub use ishellitemarray::IShellItemArray;
pub use itaskbarlist::ITaskbarList;
pub use itaskbarlist2::ITaskbarList2;
pub use itaskbarlist3::ITaskbarList3;
pub use itaskbarlist4::ITaskbarList4;

pub(crate) mod prelude {
	pub use super::ifiledialog::IFileDialogT;
	pub use super::ifileopendialog::IFileOpenDialogT;
	pub use super::ifilesavedialog::IFileSaveDialogT;
	pub use super::imodalwindow::IModalWindowT;
	pub use super::ishellitem::IShellItemT;
	pub use super::ishellitemarray::IShellItemArrayT;
	pub use super::itaskbarlist::ITaskbarListT;
	pub use super::itaskbarlist2::ITaskbarList2T;
	pub use super::itaskbarlist3::ITaskbarList3T;
	pub use super::itaskbarlist4::ITaskbarList4T;
}

/// [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/) COM
/// virtual tables.
pub mod vt {
	pub use super::ifiledialog::IFileDialogVT;
	pub use super::ifileopendialog::IFileOpenDialogVT;
	pub use super::ifilesavedialog::IFileSaveDialogVT;
	pub use super::imodalwindow::IModalWindowVT;
	pub use super::ishellitem::IShellItemVT;
	pub use super::ishellitemarray::IShellItemArrayVT;
	pub use super::itaskbarlist::ITaskbarListVT;
	pub use super::itaskbarlist2::ITaskbarList2VT;
	pub use super::itaskbarlist3::ITaskbarList3VT;
	pub use super::itaskbarlist4::ITaskbarList4VT;
}
