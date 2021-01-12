//! High-level GUI abstractions.

#[macro_use]
mod macros;

mod dialog_base;
mod globals;
mod main_loop;
mod window_base;

pub mod events;

mod dialog_main;
mod native_controls;
mod traits;
mod window_control;
mod window_main;

pub use dialog_main::DialogMain;
pub use native_controls::*;
pub use traits::{Child, create_children, Parent};
pub use window_control::{WindowControl, WindowControlOpts};
pub use window_main::{WindowMain, WindowMainOpts};
