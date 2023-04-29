#![allow(non_camel_case_types, non_snake_case)]

use crate::dxgi::decl::IDXGIAdapter;
use crate::kernel::ffi_types::{HRES, PCVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::dxgi_IDXGIObject;
use crate::vt::IDXGIObjectVT;

/// [`IDXGIDevice`](crate::IDXGIDevice) virtual table.
#[repr(C)]
pub struct IDXGIDeviceVT {
	pub IDXGIObjectVT: IDXGIObjectVT,
	pub GetAdapter: fn(ComPtr, *mut ComPtr) -> HRES,
	pub CreateSurface: fn(ComPtr, *const u32, u32, u32, PCVOID, *mut ComPtr) -> HRES,
	pub QueryResourceResidency: fn(ComPtr, ComPtr, *mut u32, u32) -> HRES,
	pub SetGPUThreadPriority: fn(ComPtr, i32) -> HRES,
	pub GetGPUThreadPriority: fn(ComPtr, *mut i32) -> HRES,
}

com_interface! { IDXGIDevice: "54ec77fa-1377-44e6-8c32-88fd5f44c84c";
	/// [`IDXGIDevice`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgidevice)
	/// COM interface over [`IDXGIDeviceVT`](crate::vt::IDXGIDeviceVT).
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
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIDevice: dxgi_IDXGIObject {
	fn_com_get! { GetAdapter: IDXGIDeviceVT, IDXGIAdapter;
		/// [`IDXGIDevice::GetAdapter`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice-getadapter)
		/// method.
	}

	/// [`IDXGIDevice::GetGPUThreadPriority`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice-getgputhreadpriority)
	/// method.
	#[must_use]
	fn GetGPUThreadPriority(&self) -> HrResult<i8> {
		let mut priority = i32::default();
		unsafe {
			let vt = self.vt_ref::<IDXGIDeviceVT>();
			ok_to_hrresult(
				(vt.GetGPUThreadPriority)(self.ptr(), &mut priority),
			).map(|_| priority as _)
		}
	}

	/// [`IDXGIDevice::SetGPUThreadPriority`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice-setgputhreadpriority)
	/// method.
	fn SetGPUThreadPriority(&self, priority: i8) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IDXGIDeviceVT>();
			ok_to_hrresult((vt.SetGPUThreadPriority)(self.ptr(), priority as _))
		}
	}
}
