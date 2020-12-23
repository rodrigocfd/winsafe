//! Parameters of
//! [window messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).

mod wm_any;
mod wm_notify;
mod wm_structs;

pub use wm_any::{Wm, WmAny};
pub use wm_notify::{Nm, WmNotify};
pub use wm_structs::*;