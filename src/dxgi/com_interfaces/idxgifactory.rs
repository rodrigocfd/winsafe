#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::ffi_types::{HANDLE, HRES, PCVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{dxgi_IDXGIObject, Handle};
use crate::user::decl::HWND;
use crate::vt::IUnknownVT;

/// [`IBindCtx`](crate::IBindCtx) virtual table.
#[repr(C)]
pub struct IDXGIFactoryVT {
	pub IUnknownVT: IUnknownVT,
	pub EnumAdapters: fn(ComPtr, u32, *const ComPtr) -> HRES,
	pub MakeWindowAssociation: fn(ComPtr, HANDLE, u32) -> HRES,
	pub GetWindowAssociation: fn(ComPtr, *mut HANDLE) -> HRES,
	pub CreateSwapChain: fn(ComPtr, *const ComPtr, PCVOID, *mut ComPtr) -> HRES,
	pub CreateSoftwareAdapter: fn(ComPtr, HANDLE, *mut ComPtr) -> HRES,
}

com_interface! { IDXGIFactory: "7b7166ec-21c7-44ae-b21a-c9ae321ae369";
	/// [`IDXGIFactory`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgifactory)
	/// COM inteface over [`IDXGIFactoryVT`](crate::IDXGIFactoryVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::CreateDXGIFactory;
	///
	/// let factory = CreateDXGIFactory()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl dxgi_IDXGIObject for IDXGIFactory {}
impl dxgi_IDXGIFactory for IDXGIFactory {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIFactory`](crate::IDXGIFactory).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIFactory: dxgi_IDXGIObject {
	/// [`IDXGIFactory::GetWindowAssociation`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-getwindowassociation)
	/// method.
	#[must_use]
	fn GetWindowAssociation(&self) -> HrResult<HWND> {
		let mut hwnd = HWND::NULL;
		unsafe {
			let vt = self.vt_ref::<IDXGIFactoryVT>();
			ok_to_hrresult((vt.GetWindowAssociation)(self.ptr(), &mut hwnd.0))
		}.map(|_| hwnd)
	}

	/// [`IDXGIFactory::MakeWindowAssociation`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-makewindowassociation)
	/// method.
	fn MakeWindowAssociation(&self,
		hwnd: &HWND, flags: co::DXGI_MWA) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<IDXGIFactoryVT>();
			ok_to_hrresult((vt.MakeWindowAssociation)(self.ptr(), hwnd.0, flags.0))
		}
	}
}
