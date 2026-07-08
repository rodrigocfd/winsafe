#![cfg(feature = "winmm")]

mod enums;
mod funcs;

pub mod co;
pub(in crate::winmm) mod ffi;

pub mod decl {
	pub use super::enums::*;
	pub use super::funcs::*;
}
