#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::dxgi::vts::*;
use crate::macros::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IDXGIDevice1: "77db970f-6276-48ba-ba28-070143b4392c";
	/// [`IDXGIDevice1`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgidevice1)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGIDevice1 {}
impl dxgi_IDXGIDevice for IDXGIDevice1 {}
impl dxgi_IDXGIDevice1 for IDXGIDevice1 {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIDevice1`](crate::IDXGIDevice1).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIDevice1: dxgi_IDXGIDevice {
	/// [`IDXGIDevice1::GetMaximumFrameLatency`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice1-getmaximumframelatency)
	/// method.
	#[must_use]
	fn GetMaximumFrameLatency(&self) -> HrResult<u32> {
		let mut max_latency = 0u32;
		HrRet(unsafe {
			(vt::<IDXGIDevice1VT>(self).GetMaximumFrameLatency)(self.ptr(), &mut max_latency)
		})
		.to_hrresult()
		.map(|_| max_latency)
	}

	/// [`IDXGIDevice1::SetMaximumFrameLatency`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice1-setmaximumframelatency)
	/// method.
	fn SetMaximumFrameLatency(&self, max_latency: u32) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IDXGIDevice1VT>(self).SetMaximumFrameLatency)(self.ptr(), max_latency as _)
		})
		.to_hrresult()
	}
}
