#![cfg(all(feature = "comctl", feature = "gdi"))]

mod handles;
mod structs;

pub(in crate::comctl_gdi) mod ffi;
pub mod messages;

pub mod decl {
	pub use super::structs::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
