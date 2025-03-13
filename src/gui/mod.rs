#![doc = include_str!("gui.md")]
#![cfg(feature = "gui")]

mod dpi;
mod globals;
mod iterators;
mod msg_error;
mod native_controls;
mod windows;

pub mod events;
pub(crate) mod traits;

pub(in crate::gui) mod privs {
	pub(in crate::gui) use super::globals::*;
	pub(in crate::gui) use super::iterators::*;
	pub(in crate::gui) use super::native_controls::privs::*;
	pub(in crate::gui) use super::windows::privs::*;
}

pub use dpi::*;
pub use msg_error::MsgError;
pub use native_controls::collections;
pub use native_controls::decl::*;
pub use windows::decl::*;
