//! High-level GUI abstractions for user windows and native controls. They can
//! be created programmatically or by loading resources from a `.res` file.
//! These files can be created with a WYSIWYG
//! [resource editor](https://en.wikipedia.org/wiki/Resource_(Windows)#Resource_software).
//!
//! You'll probably want to start your GUI application using the
//! [`WindowMain`](crate::gui::WindowMain).

mod base;
mod dlg_base;
mod dlg_control;
mod dlg_main;
mod dlg_modal;
mod native_controls;
mod privs;
mod raw_base;
mod raw_control;
mod raw_main;
mod raw_modal;
mod resizer;
mod traits_sealed;
mod traits;
mod very_unsafe_cell;
mod window_control;
mod window_main;
mod window_modal;

pub mod events;

pub use native_controls::*;
pub use raw_control::WindowControlOpts;
pub use raw_main::WindowMainOpts;
pub use raw_modal::WindowModalOpts;
pub use resizer::{Horz, Vert};
pub use traits::*;
pub use window_control::WindowControl;
pub use window_main::WindowMain;
pub use window_modal::WindowModal;

pub(crate) mod prelude {
	pub use super::events::prelude::*;
	pub use super::traits::*;
}
