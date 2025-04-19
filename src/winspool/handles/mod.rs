mod hprinter;
mod hwnd;

pub mod decl {
	pub use super::hprinter::HPRINTER;
}

pub mod traits {
	pub use super::hprinter::winspool_Hprinter;
	pub use super::hwnd::winspool_Hwnd;
}
