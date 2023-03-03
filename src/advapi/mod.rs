#![cfg_attr(docsrs, doc(cfg(feature = "advapi")))]

pub(in crate::advapi) mod ffi;
pub(crate) mod privs;
pub mod co;
pub mod guard;

mod enums;
mod funcs;
mod handles;
mod structs;

pub mod decl {
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
