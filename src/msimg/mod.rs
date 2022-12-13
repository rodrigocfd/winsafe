#![cfg_attr(docsrs, doc(cfg(feature = "msimg")))]

pub(in crate::msimg) mod ffi;

mod handles;

pub mod traits {
	pub use super::handles::traits::*;
}
