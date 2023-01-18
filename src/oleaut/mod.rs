#![cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]

pub(in crate::oleaut) mod ffi;
pub mod co;

mod com_interfaces;
mod funcs;
mod structs;

pub mod decl {
	pub use super::com_interfaces::decl::*;
	pub use super::funcs::*;
	pub use super::structs::decl::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
	pub use super::structs::traits::*;
}

pub mod vt {
	pub use super::com_interfaces::vt::*;
}
