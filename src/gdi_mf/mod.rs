#![cfg_attr(docsrs, doc(cfg(all(feature = "gdi", feature = "mf"))))]

mod com_interfaces;

pub mod traits {
	pub use super::com_interfaces::traits::*;
}
