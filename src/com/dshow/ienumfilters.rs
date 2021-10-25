#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::HRESULT;
use crate::privs::{hr_to_winresult, hr_to_winresult_bool};

/// [`IEnumFilters`](crate::dshow::IEnumFilters) virtual table.
pub struct IEnumFiltersVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(ComPtr, u32, *mut ComPtr, *mut u32) -> HRESULT,
	pub Skip: fn(ComPtr, u32) -> HRESULT,
	pub Reset: fn(ComPtr) -> HRESULT,
	pub Clone: fn(ComPtr, *mut ComPtr) -> HRESULT,
}

/// [`IEnumFilters`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ienumfilters)
/// COM interface over [`IEnumFiltersVT`](crate::dshow::vt::IEnumFiltersVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IEnumFilters(ComPtr);

impl_iunknown!(IEnumFilters, 0x56a86893, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
impl IEnumFiltersT for IEnumFilters {}

/// Exposes the [`IEnumFilters`](crate::dshow::IEnumFilters) methods.
pub trait IEnumFiltersT: IUnknownT {
	/// [`IEnumFilters::Reset`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-reset)
	/// method.
	fn Reset(&self) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IEnumFiltersVT);
			hr_to_winresult((vt.Reset)(self.ptr()))
		}
	}

	/// [`IEnumFilters::Skip`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-skip)
	/// method.
	fn Skip(&self, count: u32) -> WinResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IEnumFiltersVT);
			hr_to_winresult_bool((vt.Skip)(self.ptr(), count))
		}
	}
}
