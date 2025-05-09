#![cfg(feature = "shell")]

mod com_impls;
mod com_interfaces;
mod enums;
mod funcs;
mod handles;
mod structs;

pub mod co;
pub(in crate::shell) mod ffi;
pub mod guard;
pub(in crate::shell) mod iterators;
pub mod messages;
pub(crate) mod privs;
pub(in crate::shell) mod vts;

pub mod decl {
	pub use super::com_impls::decl::*;
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
