//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! COM interfaces.

pub mod clsid;
pub mod guid;
pub mod vt;

#[macro_use] mod imodalwindow;
#[macro_use] mod ishellitem;
#[macro_use] mod ishellitemarray;
#[macro_use] mod itaskbarlist;

#[macro_use] mod ifiledialog;
#[macro_use] mod itaskbarlist2;

#[macro_use] mod ifileopendialog;
#[macro_use] mod ifilesavedialog;
#[macro_use] mod itaskbarlist3;

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
