mod com_ptr;
mod guid;

pub mod decl {
	pub use super::com_ptr::ComPtr;
	pub use super::guid::GUID;
}
