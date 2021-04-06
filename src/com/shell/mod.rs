//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! COM interfaces.

pub mod clsid;
pub mod vt;

mod any_structs;
mod ifiledialog;
mod ifileopendialog;
mod imodalwindow;
mod ishellitem;
mod ishellitemarray;
mod itaskbarlist;
mod itaskbarlist2;
mod itaskbarlist3;

pub use any_structs::*;
pub use ifiledialog::IFileDialog;
pub use ifileopendialog::IFileOpenDialog;
pub use imodalwindow::IModalWindow;
pub use ishellitem::IShellItem;
pub use ishellitemarray::IShellItemArray;
pub use itaskbarlist::ITaskbarList;
pub use itaskbarlist2::ITaskbarList2;
pub use itaskbarlist3::ITaskbarList3;
