#![cfg(feature = "winusb")]

mod handles;

pub(in crate::winusb) mod ffi;
pub mod guards;

pub mod decl {
	pub use super::handles::decl::*;
}
