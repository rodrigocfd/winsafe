pub(crate) mod privs;

mod hdc;
mod ipicture;

pub mod traits {
	pub use super::hdc::GdiOleautHdc;
	pub use super::ipicture::GdiOleautIPicture;
}
