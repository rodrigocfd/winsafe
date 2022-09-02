#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;
use std::mem::ManuallyDrop;

use crate::co;
use crate::dshow::decl::IBaseFilter;
use crate::kernel::ffi_types::HRES;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::prelude::ole_IUnknown;
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

com_interface! { IEnumFilters: "dshow";
	"56a86893-0ad4-11ce-b03a-0020af0ba770";
	/// [`IEnumFilters`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ienumfilters)
	/// COM interface over [`IEnumFiltersVT`](crate::vt::IEnumFiltersVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IEnumFilters for IEnumFilters {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IEnumFilters`](crate::IEnumFilters).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "dshow")))]
pub trait dshow_IEnumFilters: ole_IUnknown {
	/// Returns an iterator over the [`IBaseFilter`](crate::IBaseFilter)
	/// elements which successively calls
	/// [`IEnumFilters::Next`](crate::prelude::dshow_IEnumFilters::Next).
	///
	/// # Examples
	///
	/// Enumerating the [`IBaseFilter`](crate::IBaseFilter) objects:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::IEnumFilters;
	///
	/// let filters: IEnumFilters; // initialized somewhere
	/// # let filters = IEnumFilters::from(unsafe { winsafe::ComPtr::null() });
	///
	/// for filter in filters.iter() {
	///     let filter = filter?;
	///     // ...
	/// }
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = HrResult<IBaseFilter>> + 'a> {
		Box::new(EnumFiltersIter::new(unsafe { self.ptr() }))
	}

	/// [`IEnumFilters::Next`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-next)
	/// method.
	///
	/// Prefer using
	/// [`IEnumFilters::iter`](crate::prelude::dshow_IEnumFilters::iter), which
	/// is simpler.
	#[must_use]
	fn Next(&self) -> HrResult<Option<IBaseFilter>> {
		let mut fetched = u32::default();
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IEnumFiltersVT>();
			match ok_to_hrresult(
				(vt.Next)(self.ptr(), 1, &mut ppv_queried, &mut fetched), // retrieve only 1
			) {
				Ok(_) => Ok(Some(IBaseFilter::from(ppv_queried))),
				Err(hr) => match hr {
					co::HRESULT::S_FALSE => Ok(None), // no filter found
					hr => Err(hr), // actual error
				},
			}
		}
	}

	/// [`IEnumFilters::Reset`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-reset)
	/// method.
	fn Reset(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IEnumFiltersVT>();
			ok_to_hrresult((vt.Reset)(self.ptr()))
		}
	}

	/// [`IEnumFilters::Skip`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-skip)
	/// method.
	fn Skip(&self, count: u32) -> HrResult<bool> {
		unsafe {
			let vt = self.vt_ref::<IEnumFiltersVT>();
			okfalse_to_hrresult((vt.Skip)(self.ptr(), count))
		}
	}
}

//------------------------------------------------------------------------------

struct EnumFiltersIter<'a> {
	array: ManuallyDrop<IEnumFilters>,
	_owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for EnumFiltersIter<'a> {
	type Item = HrResult<IBaseFilter>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.array.Next() {
			Err(err) => Some(Err(err)),
			Ok(maybe_item) => maybe_item.map(|item| Ok(item)),
		}
	}
}

impl<'a> EnumFiltersIter<'a> {
	fn new(com_ptr: ComPtr) -> Self {
		Self {
			array: ManuallyDrop::new(IEnumFilters(com_ptr)),
			_owner: PhantomData,
		}
	}
}
