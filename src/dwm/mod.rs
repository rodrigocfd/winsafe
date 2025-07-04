#![cfg(feature = "dwm")]

mod enums;
mod funcs;
mod handles;

pub mod co;
pub(in crate::dwm) mod ffi;

pub mod decl {
	pub use super::enums::*;
	pub use super::funcs::*;
}
