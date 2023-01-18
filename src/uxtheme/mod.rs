#![cfg_attr(docsrs, doc(cfg(feature = "uxtheme")))]

pub(in crate::uxtheme) mod ffi;
pub mod co;
pub mod guard;

mod funcs;
mod handles;

pub mod decl {
	pub use super::funcs::*;
	pub use super::handles::decl::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
