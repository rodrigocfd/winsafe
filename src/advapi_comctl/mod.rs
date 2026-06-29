#![cfg(all(feature = "advapi", feature = "comctl"))]

mod structs;

pub mod messages_tb;

pub mod decl {
	pub use super::structs::*;
}
