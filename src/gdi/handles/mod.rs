mod handle_gdi;
mod hbitmap;
mod hbrush;
mod hdc;
mod hfont;
mod hpen;
mod hrgn;

pub mod decl {
	pub use super::hfont::HFONT;
	pub use super::hpen::HPEN;
}

pub mod traits {
	pub use super::handle_gdi::HandleGdi;
	pub use super::hbitmap::GdiHbitmap;
	pub use super::hbrush::GdiHbrush;
	pub use super::hdc::GdiHdc;
	pub use super::hfont::GdiHfont;
	pub use super::hpen::GdiHpen;
	pub use super::hrgn::GdiHrgn;
}
