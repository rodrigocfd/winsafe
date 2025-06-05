#![cfg(feature = "psapi")]

mod funcs;
mod handles;
mod structs;

pub(in crate::psapi) mod ffi;

pub mod decl {
	pub use super::funcs::*;
	pub use super::structs::*;
}
