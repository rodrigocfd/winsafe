#![cfg(feature = "taskschd")]

mod com_interfaces;

pub mod co;
pub(in crate::taskschd) mod vts;

pub mod decl {
	pub use super::com_interfaces::decl::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
}
