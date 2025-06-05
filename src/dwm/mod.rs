#![cfg(feature = "dwm")]

mod funcs;
mod handles;

pub mod co;
pub(in crate::dwm) mod ffi;

pub mod decl {
	pub use super::funcs::*;
}
