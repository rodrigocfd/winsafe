#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IDXGIAdapter2`](crate::IDXGIAdapter2) virtual table.
#[repr(C)]
pub struct IDXGIAdapter2VT {
	pub IDXGIAdapter1VT: IDXGIAdapter1VT,
	pub GetDesc2: fn(COMPTR, PVOID) -> HRES,
}

impl dxgi_IDXGIObject for IDXGIAdapter2 {}
impl dxgi_IDXGIAdapter for IDXGIAdapter2 {}
impl dxgi_IDXGIAdapter1 for IDXGIAdapter2 {}
impl dxgi_IDXGIAdapter2 for IDXGIAdapter2 {}

com_interface! { IDXGIAdapter2: "0aa1ae0a-fa0e-4b84-8644-e05ff8e5acb5";
	/// [`IDXGIAdapter2`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nn-dxgi1_2-idxgiadapter2)
	/// COM interface over [`IDXGIAdapter2VT`](crate::vt::IDXGIAdapter2VT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIAdapter2`](crate::IDXGIAdapter2).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIAdapter2: dxgi_IDXGIAdapter1 {
	/// [`IDXGIAdapter::GetDesc2`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgiadapter2-getdesc2)
	/// method.
	fn GetDesc2(&self) -> HrResult<DXGI_ADAPTER_DESC2> {
		let mut desc = DXGI_ADAPTER_DESC2::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIAdapter2VT>(self).GetDesc2)(
					self.ptr(),
					&mut desc as *mut _ as _,
				)
			},
		).map(|_| desc)
	}
}
