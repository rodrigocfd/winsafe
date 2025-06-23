#![cfg(feature = "wininet")]

mod handles;

pub mod co;
pub(in crate::wininet) mod ffi;
pub mod guards;

pub mod decl {
	pub use super::handles::decl::*;
}
