//! High-level GUI abstractions.
//!
//! You'll probably want to start your GUI application using the
//! [`CustomMain`](crate::gui::CustomMain).

pub mod events;

mod base;
mod custom_control;
mod custom_main;
mod custom_modal;
mod dialog_base;
mod dialog_control;
mod dialog_main;
mod dialog_modal;
mod immut;
mod main_loop;
mod native_controls;
mod privs;
mod resizer;
mod traits;
mod window_base;
mod window_control;
mod window_main;
mod window_modal;

pub use custom_control::CustomControl;
pub use custom_main::CustomMain;
pub use custom_modal::CustomModal;
pub use native_controls::*;
pub use resizer::{Resizer, Resz};
pub use traits::{Child, Parent};
pub use window_control::CustomControlOpts;
pub use window_main::CustomMainOpts;
pub use window_modal::CustomModalOpts;
