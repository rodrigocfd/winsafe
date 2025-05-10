#![cfg(feature = "mf")]

mod com_impls;
mod com_interfaces;
mod funcs;
mod structs;

pub mod co;
pub(in crate::mf) mod ffi;
pub mod guards;
pub(in crate::mf) mod iterators;
pub(crate) mod privs;
pub(crate) mod vts;

pub mod decl {
	pub use super::com_impls::decl::*;
	pub use super::com_interfaces::decl::*;
	pub use super::funcs::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
}
