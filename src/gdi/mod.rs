#![cfg_attr(docsrs, doc(cfg(feature = "gdi")))]

pub(in crate::gdi) mod ffi;
pub(crate) mod privs;
pub mod co;
pub mod messages;

mod funcs;
mod handles;
mod structs;

pub mod decl {
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}

pub mod guard {
	pub use super::handles::guard::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
