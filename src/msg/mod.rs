//! Parameters of
//! [window messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).

#[macro_use]
mod macros;

mod message;
mod wm_structs_bcm;
mod wm_structs_cb;
mod wm_structs_dtm;
mod wm_structs_hdm;
mod wm_structs_lvm;
mod wm_structs_sb;
mod wm_structs_stm;
mod wm_structs;

pub use message::{Message, MessageHandleable};
pub use wm_structs_bcm::*;
pub use wm_structs_cb::*;
pub use wm_structs_dtm::*;
pub use wm_structs_hdm::*;
pub use wm_structs_lvm::*;
pub use wm_structs_sb::*;
pub use wm_structs_stm::*;
pub use wm_structs::*;
