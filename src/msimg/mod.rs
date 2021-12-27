pub(in crate::msimg) mod ffi;

mod hdc;

pub mod traits {
	pub use super::hdc::MsimgHdc;
}
