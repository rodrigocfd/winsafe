//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! COM interfaces.

pub mod clsid;
pub mod co;
pub mod vt;

mod ifiledialog;
mod imodalwindow;
mod itaskbarlist;
mod itaskbarlist2;
mod itaskbarlist3;

pub use ifiledialog::IFileDialog;
pub use imodalwindow::IModalWindow;
pub use itaskbarlist::ITaskbarList;
pub use itaskbarlist2::ITaskbarList2;
pub use itaskbarlist3::ITaskbarList3;
