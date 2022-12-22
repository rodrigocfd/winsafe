#![cfg_attr(docsrs, doc(cfg(feature = "ktm")))]

pub(in crate::ktm) mod ffi;

mod handles;

pub mod co;

pub mod decl {
	pub use super::handles::decl::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
