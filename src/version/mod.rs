#![cfg(feature = "version")]

mod handles;
mod structs;

pub mod co;
pub(in crate::version) mod ffi;
pub mod guard;

pub mod decl {
	pub use super::handles::decl::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
