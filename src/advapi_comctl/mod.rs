#![cfg_attr(docsrs, doc(cfg(all(feature = "advapi", feature = "comctl"))))]

mod structs;

pub mod messages;

pub mod decl {
	pub use super::structs::*;
}
