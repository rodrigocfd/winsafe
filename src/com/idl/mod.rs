//! [IDL](https://docs.microsoft.com/en-us/windows/win32/api/_com/) COM
//! interfaces, structs and constants, which are shared among other COM modules.
//!
//! To enable the IDL COM module, use:
//!
//! ```toml
//! [dependencies]
//! winsafe = { version = "0.0.8", features = ["idl"] }
//! ```

pub mod co;

pub(in crate::com) mod ipersist;
pub(in crate::com) mod ipicture;
pub(in crate::com) mod isequentialstream;
pub(in crate::com) mod istream;

pub use ipersist::IPersist;
pub use ipicture::IPicture;
pub use isequentialstream::ISequentialStream;
pub use istream::IStream;

pub(crate) mod prelude {
	pub use super::ipersist::IPersistT;
	pub use super::ipicture::IPictureT;
	pub use super::isequentialstream::ISequentialStreamT;
	pub use super::istream::IStreamT;
}

/// [IDL](https://docs.microsoft.com/en-us/windows/win32/api/_com/) COM
/// virtual tables.
pub mod vt {
	pub use super::ipersist::IPersistVT;
	pub use super::ipicture::IPictureVT;
	pub use super::isequentialstream::ISequentialStreamVT;
	pub use super::istream::IStreamVT;
}
