#![cfg(feature = "shell")]

mod com_interfaces;
mod enums;
mod funcs;
mod handles;
mod structs;

pub(in crate::shell) mod ffi;
pub(in crate::shell) mod iterators;
pub(in crate::shell) mod vts;
pub(crate) mod privs;
pub mod co;
pub mod guard;
pub mod messages;

pub mod decl {
	pub use super::com_interfaces::decl::*;
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
	pub use super::handles::traits::*;
}
