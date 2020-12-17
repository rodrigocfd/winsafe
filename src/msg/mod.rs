//! Parameters of
//! [window messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).

#[macro_use]
mod macros;

mod wm_any;
mod wm_notify;
mod wm_structs;

pub use wm_any::*;
pub use wm_notify::*;
pub use wm_structs::*;