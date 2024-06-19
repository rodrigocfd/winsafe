#![cfg(feature = "dxgi")]

mod com_interfaces;
mod funcs;
mod structs;

pub(in crate::dxgi) mod ffi;
pub(in crate::dxgi) mod iterators;
pub(in crate::dxgi) mod vts;
pub mod co;

pub mod decl {
	pub use super::com_interfaces::decl::*;
	pub use super::funcs::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
}
