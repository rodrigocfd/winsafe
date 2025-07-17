#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::dxgi::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IDXGIResource: "035f3ab4-482e-4e50-b41f-8a7f8bd8960b";
	/// [`IDXGIResource`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgiresource)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGIResource {}
impl dxgi_IDXGIDeviceSubObject for IDXGIResource {}
impl dxgi_IDXGIResource for IDXGIResource {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIResource`](crate::IDXGIResource).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIResource: dxgi_IDXGIDeviceSubObject {
	/// [`IDXGIResource::GetEvictionPriority`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiresource-getevictionpriority)
	/// method.
	#[must_use]
	fn GetEvictionPriority(&self) -> HrResult<co::DXGI_RESOURCE_PRIORITY> {
		let mut eviction_priority = co::DXGI_RESOURCE_PRIORITY::default();
		HrRet(unsafe {
			(vt::<IDXGIResourceVT>(self).GetEvictionPriority)(
				self.ptr(),
				eviction_priority.as_mut(),
			)
		}).to_hrresult()
		.map(|_| eviction_priority)
	}

	/// [`IDXGIResource::GetSharedHandle`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiresource-getsharedhandle)
	/// method.
	#[must_use]
	fn GetSharedHandle(&self) -> HrResult<*mut std::ffi::c_void> {
		let mut handle: *mut std::ffi::c_void = std::ptr::null_mut();
		HrRet(unsafe {
			(vt::<IDXGIResourceVT>(self).GetSharedHandle)(self.ptr(), &mut handle)
		}).to_hrresult()
		.map(|_| handle)
	}

	/// [`IDXGIResource::GetUsage`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiresource-getusage)
	/// method.
	#[must_use]
	fn GetUsage(&self) -> HrResult<co::DXGI_USAGE> {
		let mut usage = co::DXGI_USAGE::default();
		HrRet(unsafe {
			(vt::<IDXGIResourceVT>(self).GetUsage)(self.ptr(), usage.as_mut())
		}).to_hrresult()
		.map(|_| usage)
	}

	/// [`IDXGIResource::SetEvictionPriority`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiresource-setevictionpriority)
	/// method.
	fn SetEvictionPriority(&self, eviction_priority: co::DXGI_RESOURCE_PRIORITY) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IDXGIResourceVT>(self).SetEvictionPriority)(self.ptr(), eviction_priority.raw())
		}).to_hrresult()
	}
}
