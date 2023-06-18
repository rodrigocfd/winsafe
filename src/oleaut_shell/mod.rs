#![cfg_attr(docsrs, doc(cfg(all(feature = "oleaut", feature = "shell"))))]

pub mod co;

mod com_interfaces;

pub mod traits {
	pub use super::com_interfaces::traits::*;
}
