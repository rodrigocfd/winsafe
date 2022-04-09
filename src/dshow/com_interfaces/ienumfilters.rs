#![allow(non_snake_case)]

use crate::co;
use crate::dshow::decl::IBaseFilter;
use crate::ffi_types::HRES;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::prelude::OleIUnknown;
use crate::vt::IUnknownVT;

/// [`IEnumFilters`](crate::IEnumFilters) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
#[repr(C)]
pub struct IEnumFiltersVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(ComPtr, u32, *mut ComPtr, *mut u32) -> HRES,
	pub Skip: fn(ComPtr, u32) -> HRES,
	pub Reset: fn(ComPtr) -> HRES,
	pub Clone: fn(ComPtr, *mut ComPtr) -> HRES,
}

/// [`IEnumFilters`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ienumfilters)
/// COM interface over [`IEnumFiltersVT`](crate::vt::IEnumFiltersVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub struct IEnumFilters(ComPtr);

impl_iunknown!(IEnumFilters, "56a86893-0ad4-11ce-b03a-0020af0ba770");
impl DshowIEnumFilters for IEnumFilters {}

/// [`IEnumFilters`](crate::IEnumFilters) methods from `dshow` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub trait DshowIEnumFilters: OleIUnknown {
	/// [`IEnumFilters::Next`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-next)
	/// method.
	#[must_use]
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
