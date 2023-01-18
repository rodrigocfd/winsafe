#![cfg_attr(docsrs, doc(cfg(feature = "gdi")))]

pub(in crate::gdi) mod ffi;
pub(crate) mod privs;
pub mod co;
pub mod guard;
pub mod messages;

mod funcs;
mod handles;
mod structs;

pub mod decl {
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
