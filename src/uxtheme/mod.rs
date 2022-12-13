#![cfg_attr(docsrs, doc(cfg(feature = "uxtheme")))]

pub(in crate::uxtheme) mod ffi;

mod funcs;
mod handles;

pub mod co;

pub mod decl {
	pub use super::funcs::*;
	pub use super::handles::decl::*;
}

pub mod guard {
	pub use super::handles::guard::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
