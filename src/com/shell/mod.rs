//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! COM interfaces.

pub mod clsid;
pub mod co;

mod itaskbarlist;
mod itaskbarlist2;
mod itaskbarlist3;

pub use itaskbarlist::{ITaskbarList, ITaskbarListVtbl};
pub use itaskbarlist2::{ITaskbarList2, ITaskbarList2Vtbl};
pub use itaskbarlist3::{ITaskbarList3, ITaskbarList3Vtbl};
