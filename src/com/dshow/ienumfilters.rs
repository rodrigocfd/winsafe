#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::co;
use crate::com::dshow::ibasefilter::IBaseFilter;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::HRES;
use crate::privs::{ok_to_hrresult, okfalse_to_hrresult};

/// [`IEnumFilters`](crate::dshow::IEnumFilters) virtual table.
#[repr(C)]
pub struct IEnumFiltersVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(ComPtr, u32, *mut ComPtr, *mut u32) -> HRES,
	pub Skip: fn(ComPtr, u32) -> HRES,
	pub Reset: fn(ComPtr) -> HRES,
	pub Clone: fn(ComPtr, *mut ComPtr) -> HRES,
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
	/// [`IEnumFilters::Next`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-next)
	/// method.
	fn Next(&self) -> HrResult<Option<IBaseFilter>> {
		let mut ppv_queried = ComPtr::null();
		let mut fetched = u32::default();

		match unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IEnumFiltersVT);
			ok_to_hrresult(
				(vt.Next)(self.ptr(), 1, &mut ppv_queried, &mut fetched),
			)
		}.map(|_| IBaseFilter::from(ppv_queried)) {
			Ok(filter) => Ok(Some(filter)),
			Err(hr) => match hr {
				co::HRESULT::S_FALSE => Ok(None), // no filter found
				hr => Err(hr), // actual error
			},
		}
	}

	/// [`IEnumFilters::Reset`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-reset)
	/// method.
	fn Reset(&self) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IEnumFiltersVT);
			ok_to_hrresult((vt.Reset)(self.ptr()))
		}
	}

	/// [`IEnumFilters::Skip`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-skip)
	/// method.
	fn Skip(&self, count: u32) -> HrResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IEnumFiltersVT);
			okfalse_to_hrresult((vt.Skip)(self.ptr(), count))
		}
	}
}
