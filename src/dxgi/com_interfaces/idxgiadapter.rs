#![allow(non_camel_case_types, non_snake_case)]

use crate::dxgi::decl::DXGI_ADAPTER_DESC;
use crate::kernel::decl::GUID;
use crate::kernel::ffi_types::{HRES, PCVOID, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::dxgi_IDXGIObject;
use crate::vt::IDXGIObjectVT;

/// [`IDXGIAdapter`](crate::IDXGIAdapter) virtual table.
#[repr(C)]
pub struct IDXGIAdapterVT {
	pub IUnknownVT: IDXGIObjectVT,
	pub EnumOutputs: fn(ComPtr, u32, *mut ComPtr) -> HRES,
	pub GetDesc: fn(ComPtr, PVOID) -> HRES,
	pub CheckInterfaceSupport: fn(ComPtr, PCVOID, *mut i64) -> HRES,
}

com_interface! { IDXGIAdapter: "2411e7e1-12ac-4ccf-bd14-9798e8534dc0";
	/// [`IDXGIAdapter`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgiadapter)
	/// COM interface over [`IDXGIAdapterVT`](crate::vt::IDXGIAdapterVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGIAdapter {}
impl dxgi_IDXGIAdapter for IDXGIAdapter {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIAdapter`](crate::IDXGIAdapter).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIAdapter: dxgi_IDXGIObject {
	/// [`IDXGIAdapter::CheckInterfaceSupport`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiadapter-checkinterfacesupport)
	/// method.
	#[must_use]
	fn CheckInterfaceSupport(&self, interface_name: &GUID) -> HrResult<i64> {
		let mut umd_ver = i64::default();
		unsafe {
			let vt = self.vt_ref::<IDXGIAdapterVT>();
			ok_to_hrresult(
				(vt.CheckInterfaceSupport)(
					self.ptr(),
					interface_name as *const _ as _,
					&mut umd_ver,
				),
			)
		}.map(|_| umd_ver)
	}

	/// [`IDXGIAdapter::GetDesc`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiadapter-getdesc)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{IDXGIAdapter, DXGI_ADAPTER_DESC};
	///
	/// let adapter: IDXGIAdapter; // initialized somewhere
	/// # let adapter = IDXGIAdapter::from(unsafe { winsafe::ComPtr::null() });
	/// let mut desc = DXGI_ADAPTER_DESC::default();
	///
	/// adapter.GetDesc(&mut desc)?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn GetDesc(&self, desc: &mut DXGI_ADAPTER_DESC) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IDXGIAdapterVT>();
			ok_to_hrresult((vt.GetDesc)(self.ptr(), desc as *mut _ as _))
		}
	}
}
