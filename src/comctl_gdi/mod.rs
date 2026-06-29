#![cfg(all(feature = "comctl", feature = "gdi"))]

mod handles;
mod structs;

pub(in crate::comctl_gdi) mod ffi;
pub mod messages_dtm;

pub mod decl {
	pub use super::structs::*;
}
