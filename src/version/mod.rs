#![cfg(feature = "version")]

mod guards;
mod handles;
mod structs;

pub mod co;
pub(in crate::version) mod ffi;

pub mod decl {
	pub use super::guards::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}
