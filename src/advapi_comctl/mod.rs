#![cfg_attr(docsrs, doc(cfg(all(feature = "advapi", feature = "comctl"))))]

pub mod messages;

mod structs;

pub mod decl {
	pub use super::structs::*;
}
