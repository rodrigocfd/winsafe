#![doc = include_str!("gui.md")]
#![cfg(feature = "gui")]

mod events;
mod globals_priv;
mod globals_pub;
mod native_controls;
mod traits_gui;
mod windows;

pub(in crate::gui) mod privs {
	pub(in crate::gui) use super::events::privs::*;
	pub(in crate::gui) use super::globals_priv::*;
	pub(in crate::gui) use super::native_controls::privs::*;
	pub(in crate::gui) use super::windows::privs::*;
}

pub(crate) mod traits {
	pub use super::events::traits::*;
	pub use super::traits_gui::*;
}

pub use globals_pub::*;
pub use native_controls::{collections, decl::*};
pub use windows::decl::*;
