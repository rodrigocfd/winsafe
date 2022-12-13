#![cfg_attr(docsrs, doc(cfg(feature = "advapi")))]

pub(in crate::advapi) mod ffi;
pub(crate) mod privs;

pub mod co;

mod enums;
mod funcs;
mod handles;

pub mod decl {
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::handles::decl::*;
}

pub mod guard {
	pub use super::handles::guard::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
