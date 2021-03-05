//! Parameters of
//! [window messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
//!
//! [`WndMsg`](crate::msg::WndMsg) is the generic message, with `WPARAM` and
//! `LPARAM` fields. Other messages belong to a module according to its prefix,
//! for example, [`BM_CLICK`](crate::msg::bm::Click) can be found in
//! [`bm`](crate::msg::bm) module.
//!
//! # Examples
//!
//! We want to delete the 3rd element of a [`ListView`](crate::gui::ListView)
//! control. This can be done by sending it a
//! [`LVM_DELETEITEM`](crate::msg::lvm::DeleteItem) message. Note how the
//! `index` parameter is set when initializing the message struct, and how
//! [`SendMessage`](crate::HWND::SendMessage) returns a
//! [`WinResult`](crate::WinResult).
//!
//! ```rust,ignore
//! use winsafe::{HWND, msg::lvm};
//!
//! let hlistview: HWND; // initialize it somewhere...
//!
//! hlistview.SendMessage({
//!     lvm::DeleteItem {
//!         index: 2,
//!     },
//! }).expect("Failed to delete item 2.");
//! ```
//!
//! Each message has its own set of parameters. Also, each message has its own
//! result type, which will be returned by `SendMessage`.

#[macro_use]
mod macros;

mod message;
pub use message::{MsgSend, MsgSendRecv, WndMsg};

pub mod bm;
pub mod cb;
pub mod dtm;
pub mod em;
pub mod hdm;
pub mod lb;
pub mod lvm;
pub mod mcm;
pub mod sb;
pub mod stm;
pub mod wm;
