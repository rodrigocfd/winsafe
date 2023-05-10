#![cfg_attr(docsrs, doc(cfg(feature = "ole")))]

pub(in crate::ole) mod ffi;
pub(crate) mod privs;
pub mod co;
pub mod guard;

mod aliases;
mod com_interfaces;
mod funcs;
mod handles;
mod structs;

pub mod decl {
	pub use super::aliases::*;
	pub use super::com_interfaces::decl::*;
	pub use super::funcs::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
	pub use super::handles::traits::*;
}

pub mod vt {
	pub use super::com_interfaces::vt::*;
}
