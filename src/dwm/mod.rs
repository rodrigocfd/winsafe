#![cfg_attr(docsrs, doc(cfg(feature = "dwm")))]

pub(in crate::dwm) mod ffi;

mod funcs;
mod handles;

pub mod decl {
	pub use super::funcs::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
