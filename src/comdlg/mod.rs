#![cfg(feature = "comdlg")]

mod aliases;
mod funcs;
mod structs;

pub(in crate::comdlg) mod ffi;
pub mod co;

pub mod decl {
	pub use super::aliases::*;
	pub use super::funcs::*;
	pub use super::structs::*;
}
