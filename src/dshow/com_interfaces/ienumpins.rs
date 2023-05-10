#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::dshow::decl::IPin;
use crate::kernel::ffi_types::{COMPTR, HRES};
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult, vt};
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IEnumPins`](crate::IEnumPins) virtual table.
#[repr(C)]
pub struct IEnumPinsVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(COMPTR, u32, *mut COMPTR, *mut u32) -> HRES,
	pub Skip: fn(COMPTR, u32) -> HRES,
	pub Reset: fn(COMPTR) -> HRES,
	pub Clone: fn(COMPTR, *mut COMPTR) -> HRES,
}

com_interface! { IEnumPins: "56a86893-0ad4-11ce-b03a-0020af0ba770";
	/// [`IEnumPins`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ienumpins)
	/// COM interface over [`IEnumPinsVT`](crate::vt::IEnumPinsVT).
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
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IEnumPins: ole_IUnknown {
	/// Returns an iterator over the [`IPin`](crate::IPin) elements which calls
	/// [`IEnumPins::Next`](crate::prelude::dshow_IEnumPins::Next) internally.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::IEnumPins;
	///
	/// let pins: IEnumPins; // initialized somewhere
	/// # let pins = unsafe { IEnumPins::null() };
	///
	/// for pin in pins.iter() {
	///     let pin = pin?;
	///     // ...
	/// }
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter(&self) -> Box<dyn Iterator<Item = HrResult<IPin>> + '_> {
		Box::new(EnumPinsIter::new(self))
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

		match ok_to_hrresult(
			unsafe {
				(vt::<IEnumPinsVT>(self).Next)(
					self.ptr(),
					1, // retrieve only 1
					queried.as_mut(),
					&mut fetched,
				)
			},
		) {
			Ok(_) => Ok(Some(queried)),
			Err(hr) => match hr {
				co::HRESULT::S_FALSE => Ok(None), // no pin found
				hr => Err(hr), // actual error
			},
		}
	}

	/// [`IEnumPins::Reset`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumpins-reset)
	/// method.
	fn Reset(&self) -> HrResult<()> {
		ok_to_hrresult(unsafe { (vt::<IEnumPinsVT>(self).Reset)(self.ptr()) })
	}

	/// [`IEnumPins::Skip`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumpins-skip)
	/// method.
	fn Skip(&self, count: u32) -> HrResult<bool> {
		okfalse_to_hrresult(
			unsafe { (vt::<IEnumPinsVT>(self).Skip)(self.ptr(), count) },
		)
	}
}

//------------------------------------------------------------------------------

struct EnumPinsIter<'a, I>
	where I: dshow_IEnumPins,
{
	enum_pins: &'a I,
}

impl<'a, I> Iterator for EnumPinsIter<'a, I>
	where I: dshow_IEnumPins,
{
	type Item = HrResult<IPin>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.enum_pins.Next() {
			Err(err) => Some(Err(err)),
			Ok(maybe_item) => maybe_item.map(|item| Ok(item)),
		}
	}
}

impl<'a, I> EnumPinsIter<'a, I>
	where I: dshow_IEnumPins,
{
	fn new(enum_pins: &'a I) -> Self {
		Self { enum_pins }
	}
}
