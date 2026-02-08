mod idxgiadapter;
mod idxgiadapter1;
mod idxgiadapter2;
mod idxgidevice;
mod idxgidevicesubobject;
mod idxgifactory;
mod idxgifactory1;
mod idxgifactory2;
mod idxgikeyedmutex;
mod idxgiobject;
mod idxgioutput;
mod idxgiresource;
mod idxgisurface;
mod idxgiswapchain;

pub mod decl {
	pub use super::idxgiadapter::IDXGIAdapter;
	pub use super::idxgiadapter1::IDXGIAdapter1;
	pub use super::idxgiadapter2::IDXGIAdapter2;
	pub use super::idxgidevice::IDXGIDevice;
	pub use super::idxgidevicesubobject::IDXGIDeviceSubObject;
	pub use super::idxgifactory::IDXGIFactory;
	pub use super::idxgifactory1::IDXGIFactory1;
	pub use super::idxgifactory2::IDXGIFactory2;
	pub use super::idxgikeyedmutex::IDXGIKeyedMutex;
	pub use super::idxgiobject::IDXGIObject;
	pub use super::idxgioutput::IDXGIOutput;
	pub use super::idxgiresource::IDXGIResource;
	pub use super::idxgisurface::IDXGISurface;
	pub use super::idxgiswapchain::IDXGISwapChain;
}

pub mod traits {
	pub use super::idxgiadapter::dxgi_IDXGIAdapter;
	pub use super::idxgiadapter1::dxgi_IDXGIAdapter1;
	pub use super::idxgiadapter2::dxgi_IDXGIAdapter2;
	pub use super::idxgidevice::dxgi_IDXGIDevice;
	pub use super::idxgidevicesubobject::dxgi_IDXGIDeviceSubObject;
	pub use super::idxgifactory::dxgi_IDXGIFactory;
	pub use super::idxgifactory1::dxgi_IDXGIFactory1;
	pub use super::idxgifactory2::dxgi_IDXGIFactory2;
	pub use super::idxgikeyedmutex::dxgi_IDXGIKeyedMutex;
	pub use super::idxgiobject::dxgi_IDXGIObject;
	pub use super::idxgioutput::dxgi_IDXGIOutput;
	pub use super::idxgiresource::dxgi_IDXGIResource;
	pub use super::idxgisurface::dxgi_IDXGISurface;
	pub use super::idxgiswapchain::dxgi_IDXGISwapChain;
}
