#![cfg(feature = "version")]

mod funcs;
mod structs;
mod utilities;

pub(in crate::version) mod ffi;
pub mod co;

pub mod decl {
	pub use super::funcs::*;
	pub use super::structs::*;
	pub use super::utilities::*;
}
