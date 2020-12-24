//! Parameters of
//! [window messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).

#[macro_use]
mod macros;

mod ret_wm;
mod wm_any;
mod wm_notify;
mod wm_structs;
mod wm_structs_lvm;

pub use ret_wm::RetWm;
pub use wm_any::{Wm, WmAny};
pub use wm_notify::{Nm, WmNotify};
pub use wm_structs::*;
pub use wm_structs_lvm::*;