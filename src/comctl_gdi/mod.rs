#![cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "gdi"))))]

pub mod messages;

mod structs;

pub mod decl {
	pub use super::structs::*;
}
