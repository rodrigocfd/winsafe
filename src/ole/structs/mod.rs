mod com_ptr;
mod guid;
mod others;

pub mod decl {
	pub use super::com_ptr::ComPtr;
	pub use super::guid::GUID;
	pub use super::others::*;
}
