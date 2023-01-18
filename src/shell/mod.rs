#![cfg_attr(docsrs, doc(cfg(feature = "shell")))]

pub(in crate::shell) mod ffi;
pub(crate) mod privs;
pub mod co;
pub mod messages;

mod com_interfaces;
mod funcs;
mod handles;
mod structs;

pub mod decl {
	pub use super::com_interfaces::decl::*;
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
	pub use super::handles::traits::*;
}

pub mod vt {
	pub use super::com_interfaces::vt::*;
}
