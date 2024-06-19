#![cfg(feature = "mf")]

mod com_interfaces;
mod funcs;
mod structs;

pub(in crate::mf) mod ffi;
pub(crate) mod privs;
pub(crate) mod vts;
pub mod co;

pub mod decl {
	pub use super::com_interfaces::decl::*;
	pub use super::funcs::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
}
