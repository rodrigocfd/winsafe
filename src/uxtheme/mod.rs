#![cfg(feature = "uxtheme")]

mod funcs;
mod handles;
mod structs;

pub mod co;
pub(in crate::uxtheme) mod ffi;
pub mod guards;

pub mod decl {
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
