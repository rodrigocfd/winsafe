//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! COM interfaces, structs and constants.
//!
//! To enable the Shell COM module, use:
//!
//! ```toml
//! [dependencies]
//! winsafe = { version = "0.0.4", features = ["shell"] }
//! ```

pub mod co;
pub mod clsid;
pub mod guid;

#[macro_use] mod imodalwindow; // 2nd level interfaces
#[macro_use] mod ishellitem;
#[macro_use] mod ishellitemarray;
#[macro_use] mod itaskbarlist;

#[macro_use] mod ifiledialog; // 3rd level interfaces
#[macro_use] mod itaskbarlist2;

#[macro_use] mod ifileopendialog; // 4th level interfaces
#[macro_use] mod ifilesavedialog;
#[macro_use] mod itaskbarlist3;

#[macro_use] mod itaskbarlist4; // 5th level interface

mod any_structs;

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
