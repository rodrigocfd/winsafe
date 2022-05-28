mod ibindctx;
mod ifiledialog;
mod ifileopendialog;
mod ifilesavedialog;
mod imodalwindow;
mod ipersist;
mod isequentialstream;
mod ishellitem;
mod ishellitemarray;
mod ishelllink;
mod istream;
mod itaskbarlist;
mod itaskbarlist2;
mod itaskbarlist3;
mod itaskbarlist4;

pub mod decl {
	pub use super::ibindctx::IBindCtx;
	pub use super::ifiledialog::IFileDialog;
	pub use super::ifileopendialog::IFileOpenDialog;
	pub use super::ifilesavedialog::IFileSaveDialog;
	pub use super::imodalwindow::IModalWindow;
	pub use super::ipersist::IPersist;
	pub use super::isequentialstream::ISequentialStream;
	pub use super::ishellitem::IShellItem;
	pub use super::ishellitemarray::IShellItemArray;
	pub use super::ishelllink::IShellLink;
	pub use super::istream::IStream;
	pub use super::itaskbarlist::ITaskbarList;
	pub use super::itaskbarlist2::ITaskbarList2;
	pub use super::itaskbarlist3::ITaskbarList3;
	pub use super::itaskbarlist4::ITaskbarList4;
}

pub mod traits {
	pub use super::ibindctx::ShellIBindCtx;
	pub use super::ifiledialog::ShellIFileDialog;
	pub use super::ifileopendialog::ShellIFileOpenDialog;
	pub use super::ifilesavedialog::ShellIFileSaveDialog;
	pub use super::imodalwindow::ShellIModalWindow;
	pub use super::ipersist::ShellIPersist;
	pub use super::isequentialstream::ShellISequentialStream;
	pub use super::ishellitem::ShellIShellItem;
	pub use super::ishellitemarray::ShellIShellItemArray;
	pub use super::ishelllink::ShellIShellLink;
	pub use super::istream::ShellIStream;
	pub use super::itaskbarlist::ShellITaskbarList;
	pub use super::itaskbarlist2::ShellITaskbarList2;
	pub use super::itaskbarlist3::ShellITaskbarList3;
	pub use super::itaskbarlist4::ShellITaskbarList4;
}

pub mod vt {
	pub use super::ibindctx::IBindCtxVT;
	pub use super::ifiledialog::IFileDialogVT;
	pub use super::ifileopendialog::IFileOpenDialogVT;
	pub use super::ifilesavedialog::IFileSaveDialogVT;
	pub use super::imodalwindow::IModalWindowVT;
	pub use super::ipersist::IPersistVT;
	pub use super::isequentialstream::ISequentialStreamVT;
	pub use super::ishellitem::IShellItemVT;
	pub use super::ishellitemarray::IShellItemArrayVT;
	pub use super::ishelllink::IShellLinkVT;
	pub use super::istream::IStreamVT;
	pub use super::itaskbarlist::ITaskbarListVT;
	pub use super::itaskbarlist2::ITaskbarList2VT;
	pub use super::itaskbarlist3::ITaskbarList3VT;
	pub use super::itaskbarlist4::ITaskbarList4VT;
}
