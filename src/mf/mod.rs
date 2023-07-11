#![cfg_attr(docsrs, doc(cfg(feature = "mf")))]

pub(in crate::mf) mod ffi;
pub(crate) mod privs;
pub mod co;

mod com_interfaces;
mod funcs;

pub mod decl {
	pub use super::com_interfaces::decl::*;
	pub use super::funcs::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
}

pub mod vt {
	pub use super::com_interfaces::vt::*;
}
