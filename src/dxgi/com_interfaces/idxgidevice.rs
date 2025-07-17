#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::dxgi::vts::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IDXGIDevice: "54ec77fa-1377-44e6-8c32-88fd5f44c84c";
	/// [`IDXGIDevice`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgidevice)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGIDevice {}
impl dxgi_IDXGIDevice for IDXGIDevice {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIDevice`](crate::IDXGIDevice).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIDevice: dxgi_IDXGIObject {
	/// [`IDXGIDevice::CreateSurface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice-createsurface)
	/// method.
	#[must_use]
	fn CreateSurface(
		&self,
		desc: &DXGI_SURFACE_DESC,
		num_surfaces: u32,
		usage: co::DXGI_USAGE,
		shared_resource: Option<&DXGI_SHARED_RESOURCE>,
	) -> HrResult<IDXGISurface> {
		let mut queried = unsafe { IDXGISurface::null() };
		HrRet(unsafe {
			(vt::<IDXGIDeviceVT>(self).CreateSurface)(
				self.ptr(),
				pcvoid(desc),
				num_surfaces,
				usage.raw(),
				pcvoid_or_null(shared_resource),
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	fn_com_interface_get! { GetAdapter: IDXGIDeviceVT => IDXGIAdapter;
		/// [`IDXGIDevice::GetAdapter`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice-getadapter)
		/// method.
	}

	/// [`IDXGIDevice::GetGPUThreadPriority`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice-getgputhreadpriority)
	/// method.
	#[must_use]
	fn GetGPUThreadPriority(&self) -> HrResult<i8> {
		let mut priority = 0i32;
		HrRet(unsafe {
			(vt::<IDXGIDeviceVT>(self).GetGPUThreadPriority)(self.ptr(), &mut priority)
		}).to_hrresult()
		.map(|_| priority as _)
	}

	/// [`IDXGIDevice::QueryResourceResidency`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice-queryresourceresidency)
	/// method.
	#[must_use]
	fn QueryResourceResidency(
		&self,
		resources: &[&impl dxgi_IDXGIResource],
	) -> HrResult<Vec<co::DXGI_RESIDENCY>> {
		let mut status = vec![co::DXGI_RESIDENCY::default(); resources.len()];
		HrRet(unsafe {
			(vt::<IDXGIDeviceVT>(self).QueryResourceResidency)(
				self.ptr(),
				resources.as_ptr() as _,
				status.as_mut_ptr() as _,
				resources.len() as _,
			)
		}).to_hrresult()
		.map(|_| status)
	}

	/// [`IDXGIDevice::SetGPUThreadPriority`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice-setgputhreadpriority)
	/// method.
	fn SetGPUThreadPriority(&self, priority: i8) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IDXGIDeviceVT>(self).SetGPUThreadPriority)(self.ptr(), priority as _)
		}).to_hrresult()
	}
}
