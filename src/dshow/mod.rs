#![cfg(feature = "dshow")]

mod com_interfaces;
mod structs;

pub (in crate::dshow) mod iterators;
pub (in crate::dshow) mod vts;
pub mod co;

pub mod decl {
	pub use super::com_interfaces::decl::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
}
