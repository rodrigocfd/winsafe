#![cfg(feature = "winspool")]

mod funcs;

pub(in crate::winspool) mod ffi;

pub mod decl {
	pub use super::funcs::*;
}
