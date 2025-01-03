#![cfg(feature = "winspool")]

mod handles;
mod funcs;
mod structs;

pub(in crate::winspool) mod ffi;
pub mod co;
pub mod guard;

pub mod decl {
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
