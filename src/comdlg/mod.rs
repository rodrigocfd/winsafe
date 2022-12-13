#![cfg_attr(docsrs, doc(cfg(feature = "comdlg")))]

pub(in crate::comdlg) mod ffi;

mod aliases;
mod funcs;
mod structs;

pub mod co;

pub mod decl {
	pub use super::aliases::*;
	pub use super::funcs::*;
	pub use super::structs::*;
}
