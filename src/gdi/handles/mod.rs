mod hbitmap;
mod hbrush;
mod hdc;
mod hfont;
mod hinstance;
mod hpalette;
mod hpen;
mod hrgn;

pub mod decl {
	pub use super::hfont::HFONT;
	pub use super::hpen::HPEN;
}
