#![cfg_attr(docsrs, doc(cfg(feature = "kernel")))]

pub(in crate::kernel) mod ffi;
pub(in crate::kernel) mod iterators;
pub(crate) mod ffi_types;
pub(crate) mod privs;
pub mod co;
pub mod guard;

mod aliases;
mod base_traits;
mod enums;
mod funcs;
mod handles;
mod structs;
mod utilities;

pub mod decl {
	pub use super::aliases::*;
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
	pub use super::utilities::*;
}

pub mod traits {
	pub use super::base_traits::*;
	pub use super::handles::traits::*;
}
