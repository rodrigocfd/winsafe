mod ifiledialog;
mod ifileopendialog;
mod ifilesavedialog;
mod imodalwindow;
mod ishellitem;
mod ishellitemarray;
mod itaskbarlist;
mod itaskbarlist2;
mod itaskbarlist3;
mod itaskbarlist4;

pub mod decl {
	pub use super::ifiledialog::IFileDialog;
	pub use super::ifileopendialog::IFileOpenDialog;
	pub use super::ifilesavedialog::IFileSaveDialog;
	pub use super::imodalwindow::IModalWindow;
	pub use super::ishellitem::IShellItem;
	pub use super::ishellitemarray::IShellItemArray;
	pub use super::itaskbarlist::ITaskbarList;
	pub use super::itaskbarlist2::ITaskbarList2;
	pub use super::itaskbarlist3::ITaskbarList3;
	pub use super::itaskbarlist4::ITaskbarList4;
}

pub mod traits {
	pub use super::ifiledialog::ShellIFileDialog;
	pub use super::ifileopendialog::ShellIFileOpenDialog;
	pub use super::ifilesavedialog::ShellIFileSaveDialog;
	pub use super::imodalwindow::ShellIModalWindow;
	pub use super::ishellitem::ShellIShellItem;
	pub use super::ishellitemarray::ShellIShellItemArray;
	pub use super::itaskbarlist::ShellITaskbarList;
	pub use super::itaskbarlist2::ShellITaskbarList2;
	pub use super::itaskbarlist3::ShellITaskbarList3;
	pub use super::itaskbarlist4::ShellITaskbarList4;
}

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
