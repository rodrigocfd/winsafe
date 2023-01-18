#![cfg_attr(docsrs, doc(cfg(feature = "version")))]

pub(in crate::version) mod ffi;
pub mod co;

mod funcs;
mod structs;
mod utilities;

pub mod decl {
	pub use super::funcs::*;
	pub use super::structs::*;
	pub use super::utilities::*;
}
