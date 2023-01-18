#![cfg_attr(docsrs, doc(cfg(feature = "ktm")))]

pub(in crate::ktm) mod ffi;
pub mod co;

mod handles;

pub mod decl {
	pub use super::handles::decl::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
