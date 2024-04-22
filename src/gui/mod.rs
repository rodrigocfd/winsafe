//! High-level GUI abstractions for user windows and native controls. They can
//! be created programmatically or by loading resources from a `.res` file.
//! These files can be created with a WYSIWYG
//! [resource editor](https://en.wikipedia.org/wiki/Resource_(Windows)#Resource_software).
//!
//! You'll probably want to start your GUI application using the
//! [`WindowMain`].

#![cfg(feature = "gui")]

mod iterators;
mod native_controls;
mod privs_gui;
mod proc;
mod traits_gui;
mod windows;

pub mod events;

pub(in crate::gui) mod privs {
	pub(in crate::gui) use super::events::privs::*;
	pub(in crate::gui) use super::native_controls::privs::*;
	pub(in crate::gui) use super::privs_gui::*;
	pub(in crate::gui) use super::windows::privs::*;
}

pub use native_controls::decl::*;
pub use windows::decl::*;

pub(crate) mod traits {
	pub use super::traits_gui::*;
}

pub mod spec {
	//! Structs which expose specialized methods of controls.

	pub use super::native_controls::spec::*;
}
