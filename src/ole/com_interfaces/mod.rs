mod ibindctx;
mod ipersist;
mod ipicture;
mod isequentialstream;
mod istream;
mod iunknown;

pub mod decl {
	pub use super::ibindctx::IBindCtx;
	pub use super::ipersist::IPersist;
	pub use super::ipicture::IPicture;
	pub use super::isequentialstream::ISequentialStream;
	pub use super::istream::IStream;
	pub use super::iunknown::IUnknown;
}

pub mod traits {
	pub use super::ibindctx::ole_IBindCtx;
	pub use super::ipersist::ole_IPersist;
	pub use super::ipicture::ole_IPicture;
	pub use super::isequentialstream::ole_ISequentialStream;
	pub use super::istream::ole_IStream;
	pub use super::iunknown::ole_IUnknown;
}

pub mod vt {
	pub use super::ibindctx::IBindCtxVT;
	pub use super::ipersist::IPersistVT;
	pub use super::ipicture::IPictureVT;
	pub use super::isequentialstream::ISequentialStreamVT;
	pub use super::istream::IStreamVT;
	pub use super::iunknown::IUnknownVT;
}
