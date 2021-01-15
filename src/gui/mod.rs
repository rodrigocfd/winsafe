//! High-level GUI abstractions.

pub mod events;

mod base;
mod custom_control;
mod custom_main;
mod dialog_base;
mod dialog_control;
mod dialog_main;
mod globals;
mod immut;
mod main_loop;
mod native_controls;
mod parent;
mod window_base;
mod window_control;
mod window_main;

pub use custom_control::CustomControl;
pub use custom_main::CustomMain;
pub use native_controls::*;
pub use parent::Parent;
pub use window_control::CustomControlOpts;
pub use window_main::CustomMainOpts;
