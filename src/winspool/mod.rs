#![cfg(feature = "winspool")]

mod funcs;
mod handles;
mod structs;

pub mod co;
pub(in crate::winspool) mod ffi;
pub mod guards;

pub mod decl {
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}
