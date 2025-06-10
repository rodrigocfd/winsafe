mod ienumshellitems;
mod ifiledialog;
mod ifileopendialog;
mod ifileoperation;
mod ifilesavedialog;
mod imodalwindow;
mod ishellfolder;
mod ishellitem;
mod ishellitem2;
mod ishellitemarray;
mod ishelllink;
mod itaskbarlist;
mod itaskbarlist2;
mod itaskbarlist3;
mod itaskbarlist4;

pub mod decl {
	pub use super::ienumshellitems::IEnumShellItems;
	pub use super::ifiledialog::IFileDialog;
	pub use super::ifileopendialog::IFileOpenDialog;
	pub use super::ifileoperation::IFileOperation;
	pub use super::ifilesavedialog::IFileSaveDialog;
	pub use super::imodalwindow::IModalWindow;
	pub use super::ishellfolder::IShellFolder;
	pub use super::ishellitem::IShellItem;
	pub use super::ishellitem2::IShellItem2;
	pub use super::ishellitemarray::IShellItemArray;
	pub use super::ishelllink::IShellLink;
	pub use super::itaskbarlist::ITaskbarList;
	pub use super::itaskbarlist2::ITaskbarList2;
	pub use super::itaskbarlist3::ITaskbarList3;
	pub use super::itaskbarlist4::ITaskbarList4;
}

pub mod traits {
	pub use super::ienumshellitems::shell_IEnumShellItems;
	pub use super::ifiledialog::shell_IFileDialog;
	pub use super::ifileopendialog::shell_IFileOpenDialog;
	pub use super::ifileoperation::shell_IFileOperation;
	pub use super::ifilesavedialog::shell_IFileSaveDialog;
	pub use super::imodalwindow::shell_IModalWindow;
	pub use super::ishellfolder::shell_IShellFolder;
	pub use super::ishellitem::shell_IShellItem;
	pub use super::ishellitem2::shell_IShellItem2;
	pub use super::ishellitemarray::shell_IShellItemArray;
	pub use super::ishelllink::shell_IShellLink;
	pub use super::itaskbarlist::shell_ITaskbarList;
	pub use super::itaskbarlist2::shell_ITaskbarList2;
	pub use super::itaskbarlist3::shell_ITaskbarList3;
	pub use super::itaskbarlist4::shell_ITaskbarList4;
}
