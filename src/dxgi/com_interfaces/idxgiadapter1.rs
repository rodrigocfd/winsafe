#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::dxgi::vts::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IDXGIAdapter1: "29038f61-3839-4626-91fd-086879011a05";
	/// [`IDXGIAdapter1`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgiadapter1)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGIAdapter1 {}
impl dxgi_IDXGIAdapter for IDXGIAdapter1 {}
impl dxgi_IDXGIAdapter1 for IDXGIAdapter1 {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIAdapter1`](crate::IDXGIAdapter1).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIAdapter1: dxgi_IDXGIAdapter {
	/// [`IDXGIAdapter::GetDesc1`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ns-dxgi-dxgi_adapter_desc1)
	/// method.
	#[must_use]
	fn GetDesc1(&self) -> HrResult<DXGI_ADAPTER_DESC1> {
		let mut desc = DXGI_ADAPTER_DESC1::default();
		HrRet(unsafe {
			(vt::<IDXGIAdapter1VT>(self).GetDesc1)(self.ptr(), pvoid(&mut desc))
		})
		.to_hrresult()
		.map(|_| desc)
	}
}
