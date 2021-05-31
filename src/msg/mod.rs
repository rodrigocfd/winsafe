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
//! control. This can be done by sending it an
//! [`LVM_DELETEITEM`](crate::msg::lvm::DeleteItem) message via
//! [`SendMessage`](crate::HWND::SendMessage). The message itself is a struct,
//! which is initialized with the specific message parameters.
//!
//! The message struct also defines the data type returned by `SendMessage`. In
//! the example below, `LVM_DELETEITEM` returns `WinResult<()>`.
//!
//! ```rust,ignore
//! use winsafe::{HWND, msg::lvm};
//!
//! let hlistview: HWND; // initialized somewhere
//!
//! hlistview.SendMessage({
//!     lvm::DeleteItem {
//!         index: 2,
//!     },
//! }).expect("Failed to delete item 2.");
//! ```
//!
//! Messages are organized into modules according to their prefixes:
//! [`wm`](crate::msg::wm) (window messages), [`lvm`](crate::msg::lvm) (list
//! view messages), and so on.

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
pub mod pbm;
pub mod sb;
pub mod stm;
pub mod trbm;
pub mod tvm;
pub mod wm;
