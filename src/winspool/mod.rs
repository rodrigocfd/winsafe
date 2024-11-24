#![cfg(feature = "winspool")]

mod funcs;
mod structs;

pub(in crate::winspool) mod ffi;
pub mod co;

pub mod decl {
	pub use super::funcs::*;
	pub use super::structs::*;
}
