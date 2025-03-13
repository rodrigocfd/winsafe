#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::dshow::{iterators::*, vts::*};
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IEnumPins: "56a86893-0ad4-11ce-b03a-0020af0ba770";
	/// [`IEnumPins`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ienumpins)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IEnumPins for IEnumPins {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IEnumPins`](crate::IEnumPins).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IEnumPins: ole_IUnknown {
	/// Returns an iterator over the [`IPin`](crate::IPin) elements which calls
	/// [`IEnumPins::Next`](crate::prelude::dshow_IEnumPins::Next) internally.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let pins: w::IEnumPins; // initialized somewhere
	/// # let pins = unsafe { w::IEnumPins::null() };
	///
	/// for pin in pins.iter() {
	///     let pin = pin?;
	///     // ...
	/// }
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	fn iter(&self) -> impl Iterator<Item = HrResult<IPin>> + '_ {
		IenumpinsIter::new(self)
	}

	/// [`IEnumPins::Next`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumpins-next)
	/// method.
	///
	/// Prefer using
	/// [`IEnumPins::iter`](crate::prelude::dshow_IEnumPins::iter), which
	/// is simpler.
	#[must_use]
	fn Next(&self) -> HrResult<Option<IPin>> {
		let mut queried = unsafe { IPin::null() };
		let mut fetched = u32::default();

		match ok_to_hrresult(unsafe {
			(vt::<IEnumPinsVT>(self).Next)(
				self.ptr(),
				1, // retrieve only 1
				queried.as_mut(),
				&mut fetched,
			)
		}) {
			Ok(_) => Ok(Some(queried)),
			Err(hr) => match hr {
				co::HRESULT::S_FALSE => Ok(None), // no pin found
				hr => Err(hr),                    // actual error
			},
		}
	}

	fn_com_noparm! { Reset: IEnumPinsVT;
		/// [`IEnumPins::Reset`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumpins-reset)
		/// method.
	}

	/// [`IEnumPins::Skip`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumpins-skip)
	/// method.
	fn Skip(&self, count: u32) -> HrResult<bool> {
		okfalse_to_hrresult(unsafe { (vt::<IEnumPinsVT>(self).Skip)(self.ptr(), count) })
	}
}
