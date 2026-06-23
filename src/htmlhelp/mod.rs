#![cfg(feature = "htmlhelp")]

mod enums;
mod handles;

pub mod co;
pub(in crate::htmlhelp) mod ffi;

pub mod decl {
	pub use super::enums::*;
}
