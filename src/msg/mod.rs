//! Parameters of
//! [window messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
//!
//! Each message struct defines the parameters it receives and also its return
//! type.

#[macro_use]
mod macros;

mod message;
pub use message::{MsgSend, MsgSendRecv, WndMsg};

pub mod bm;
pub mod cb;
pub mod dtm;
pub mod hdm;
pub mod lb;
pub mod lvm;
pub mod sb;
pub mod stm;
pub mod wm;
