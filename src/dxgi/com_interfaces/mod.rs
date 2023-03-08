mod idxgiadapter;
mod idxgifactory;
mod idxgiobject;
mod idxgioutput;

pub mod decl {
	pub use super::idxgiadapter::IDXGIAdapter;
	pub use super::idxgifactory::IDXGIFactory;
	pub use super::idxgiobject::IDXGIObject;
	pub use super::idxgioutput::IDXGIOutput;
}

pub mod traits {
	pub use super::idxgiadapter::dxgi_IDXGIAdapter;
	pub use super::idxgifactory::dxgi_IDXGIFactory;
	pub use super::idxgiobject::dxgi_IDXGIObject;
	pub use super::idxgioutput::dxgi_IDXGIOutput;
}

pub mod vt {
	pub use super::idxgiadapter::IDXGIAdapterVT;
	pub use super::idxgifactory::IDXGIFactoryVT;
	pub use super::idxgiobject::IDXGIObjectVT;
	pub use super::idxgioutput::IDXGIOutputVT;
}
