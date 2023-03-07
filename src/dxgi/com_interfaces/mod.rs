mod idxgiadapter;
mod idxgifactory;
mod idxgiobject;

pub mod decl {
	pub use super::idxgiadapter::IDXGIAdapter;
	pub use super::idxgifactory::IDXGIFactory;
	pub use super::idxgiobject::IDXGIObject;
}

pub mod traits {
	pub use super::idxgiadapter::dxgi_IDXGIAdapter;
	pub use super::idxgifactory::dxgi_IDXGIFactory;
	pub use super::idxgiobject::dxgi_IDXGIObject;
}

pub mod vt {
	pub use super::idxgiadapter::IDXGIAdapterVT;
	pub use super::idxgifactory::IDXGIFactoryVT;
	pub use super::idxgiobject::IDXGIObjectVT;
}
