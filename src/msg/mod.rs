//! Parameters of
//! [window messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).

#[macro_use]
mod macros;

mod wm_structs;
mod wm_structs_bcm;
mod wm_structs_lvm;

pub use wm_structs::*;
pub use wm_structs_bcm::*;
pub use wm_structs_lvm::*;
