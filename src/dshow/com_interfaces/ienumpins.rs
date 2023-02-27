#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;
use std::mem::ManuallyDrop;

use crate::co;
use crate::dshow::decl::IPin;
use crate::kernel::ffi_types::HRES;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IEnumPins`](crate::IEnumPins) virtual table.
#[repr(C)]
pub struct IEnumPinsVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(ComPtr, u32, *mut ComPtr, *mut u32) -> HRES,
	pub Skip: fn(ComPtr, u32) -> HRES,
	pub Reset: fn(ComPtr) -> HRES,
	pub Clone: fn(ComPtr, *mut ComPtr) -> HRES,
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
	/// # let pins = IEnumPins::from(unsafe { winsafe::ComPtr::null() });
	///
	/// for pin in pins.iter() {
	///     let pin = pin?;
	///     // ...
	/// }
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter(&self) -> Box<dyn Iterator<Item = HrResult<IPin>> + '_> {
		Box::new(EnumPinsIter::new(unsafe { self.ptr() }))
	}

	/// [`IEnumPins::Next`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumpins-next)
	/// method.
	///
	/// Prefer using
	/// [`IEnumPins::iter`](crate::prelude::dshow_IEnumPins::iter), which
	/// is simpler.
	#[must_use]
	fn Next(&self) -> HrResult<Option<IPin>> {
		let mut fetched = u32::default();
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IEnumPinsVT>();
			match ok_to_hrresult(
				(vt.Next)(self.ptr(), 1, &mut ppv_queried, &mut fetched), // retrieve only 1
			) {
				Ok(_) => Ok(Some(IPin::from(ppv_queried))),
				Err(hr) => match hr {
					co::HRESULT::S_FALSE => Ok(None), // no pin found
					hr => Err(hr), // actual error
				},
			}
		}
	}

	/// [`IEnumPins::Reset`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumpins-reset)
	/// method.
	fn Reset(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IEnumPinsVT>();
			ok_to_hrresult((vt.Reset)(self.ptr()))
		}
	}

	/// [`IEnumPins::Skip`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumpins-skip)
	/// method.
	fn Skip(&self, count: u32) -> HrResult<bool> {
		unsafe {
			let vt = self.vt_ref::<IEnumPinsVT>();
			okfalse_to_hrresult((vt.Skip)(self.ptr(), count))
		}
	}
}

//------------------------------------------------------------------------------

struct EnumPinsIter<'a> {
	array: ManuallyDrop<IEnumPins>,
	_owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for EnumPinsIter<'a> {
	type Item = HrResult<IPin>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.array.Next() {
			Err(err) => Some(Err(err)),
			Ok(maybe_item) => maybe_item.map(|item| Ok(item)),
		}
	}
}

impl<'a> EnumPinsIter<'a> {
	fn new(com_ptr: ComPtr) -> Self {
		Self {
			array: ManuallyDrop::new(IEnumPins(com_ptr)),
			_owner: PhantomData,
		}
	}
}
