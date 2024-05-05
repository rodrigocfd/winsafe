#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IDXGIAdapter1`](crate::IDXGIAdapter1) virtual table.
#[repr(C)]
pub struct IDXGIAdapter1VT {
	pub IDXGIAdapterVT: IDXGIAdapterVT,
	pub GetDesc1: fn(COMPTR, PVOID) -> HRES,
}

com_interface! { IDXGIAdapter1: "29038f61-3839-4626-91fd-086879011a05";
	/// [`IDXGIAdapter1`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgiadapter1)
	/// COM interface over [`IDXGIAdapter1VT`](crate::vt::IDXGIAdapter1VT).
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
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let adapter: w::IDXGIAdapter1; // initialized somewhere
	/// # let adapter = unsafe { w::IDXGIAdapter1::null() };
	/// let mut desc = w::DXGI_ADAPTER_DESC1::default();
	///
	/// adapter.GetDesc1(&mut desc)?;
	/// # w::HrResult::Ok(())
	/// ```
	fn GetDesc1(&self, desc: &mut DXGI_ADAPTER_DESC1) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIAdapter1VT>(self).GetDesc1)(
					self.ptr(),
					desc as *mut _ as _,
				)
			},
		)
	}
}
