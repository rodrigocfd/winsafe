#![cfg_attr(docsrs, doc(cfg(all(feature = "advapi", feature = "ktm"))))]

pub(in crate::advapi_ktm) mod ffi;

mod handles;

pub mod traits {
	pub use super::handles::traits::*;
}
