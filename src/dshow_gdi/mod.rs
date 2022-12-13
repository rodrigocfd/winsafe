#![cfg_attr(docsrs, doc(cfg(all(feature = "dshow", feature = "gdi"))))]

mod com_interfaces;

pub mod traits {
	pub use super::com_interfaces::traits::*;
}
