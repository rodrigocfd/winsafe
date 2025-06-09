#![cfg(feature = "oleaut")]

mod com_interfaces;
mod enums;
mod funcs;
mod structs;

pub mod co;
pub(in crate::oleaut) mod ffi;
pub(in crate::oleaut) mod iterators;
pub(crate) mod vts;

pub mod decl {
	pub use super::com_interfaces::decl::*;
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::structs::decl::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
}
