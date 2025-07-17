#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::dshow::{iterators::*, vts::*};
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IEnumFilters: "56a86893-0ad4-11ce-b03a-0020af0ba770";
	/// [`IEnumFilters`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ienumfilters)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IEnumFilters for IEnumFilters {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IEnumFilters`](crate::IEnumFilters).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IEnumFilters: ole_IUnknown {
	/// Returns an iterator over the [`IBaseFilter`](crate::IBaseFilter)
	/// elements which calls
	/// [`IEnumFilters::Next`](crate::prelude::dshow_IEnumFilters::Next)
	/// internally.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let filters: w::IEnumFilters; // initialized somewhere
	/// # let filters = unsafe { w::IEnumFilters::null() };
	///
	/// for filter in filters.iter() {
	///     let filter = filter?;
	///     // ...
	/// }
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	fn iter(&self) -> impl Iterator<Item = HrResult<IBaseFilter>> + '_ {
		IenumfiltersIter::new(self)
	}

	/// [`IEnumFilters::Next`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-next)
	/// method.
	///
	/// Prefer using
	/// [`IEnumFilters::iter`](crate::prelude::dshow_IEnumFilters::iter), which
	/// is simpler.
	#[must_use]
	fn Next(&self) -> HrResult<Option<IBaseFilter>> {
		let mut queried = unsafe { IBaseFilter::null() };
		let mut fetched = 0u32;

		match HrRet(unsafe {
			(vt::<IEnumFiltersVT>(self).Next)(
				self.ptr(),
				1, // retrieve only 1
				queried.as_mut(),
				&mut fetched,
			)
		})
		.to_hrresult()
		{
			Ok(_) => Ok(Some(queried)),
			Err(hr) => match hr {
				co::HRESULT::S_FALSE => Ok(None), // no filter found
				hr => Err(hr),                    // actual error
			},
		}
	}

	fn_com_noparm! { Reset: IEnumFiltersVT;
		/// [`IEnumFilters::Reset`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-reset)
		/// method.
	}

	/// [`IEnumFilters::Skip`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-skip)
	/// method.
	fn Skip(&self, count: u32) -> HrResult<bool> {
		HrRet(unsafe { (vt::<IEnumFiltersVT>(self).Skip)(self.ptr(), count) }).to_bool_hrresult()
	}
}
