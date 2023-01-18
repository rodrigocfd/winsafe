#![cfg_attr(docsrs, doc(cfg(feature = "comdlg")))]

pub(in crate::comdlg) mod ffi;
pub mod co;

mod aliases;
mod funcs;
mod structs;

pub mod decl {
	pub use super::aliases::*;
	pub use super::funcs::*;
	pub use super::structs::*;
}
