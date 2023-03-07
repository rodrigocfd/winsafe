#![allow(non_camel_case_types, non_snake_case)]

use crate::dshow::decl::AM_MEDIA_TYPE;
use crate::kernel::ffi_types::{HRES, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IEnumMediaTypes`](crate::IEnumMediaTypes) virtual table.
#[repr(C)]
pub struct IEnumMediaTypesVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(ComPtr, u32, *mut PVOID, *mut u32) -> HRES,
	pub Skip: fn(ComPtr, u32) -> HRES,
	pub Reset: fn(ComPtr) -> HRES,
	pub Clone: fn(ComPtr, *mut ComPtr) -> HRES,
}

com_interface! { IEnumMediaTypes: "89c31040-846b-11ce-97d3-00aa0055595a";
	/// [`IEnumMediaTypes`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ienummediatypes)
	/// COM interface over [`IEnumMediaTypesVT`](crate::vt::IEnumMediaTypesVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IEnumMediaTypes for IEnumMediaTypes {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IEnumMediaTypes`](crate::IEnumMediaTypes).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IEnumMediaTypes: ole_IUnknown {
	/// Returns an iterator over the [`AM_MEDIA_TYPE`](crate::AM_MEDIA_TYPE)
	/// elements which calls
	/// [`IEnumMediaTypes::next`](crate::prelude::dshow_IEnumMediaTypes::Next)
	/// internally.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::IEnumMediaTypes;
	///
	/// let types: IEnumMediaTypes; // initialized somewhere
	/// # let types = IEnumMediaTypes::from(unsafe { winsafe::ComPtr::null() });
	///
	/// for amt in types.iter() {
	///     let amt = amt?;
	///     println!("{} {}",
	///         amt.majortype.to_string(), amt.lSampleSize);
	/// }
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter(&self) -> Box<dyn Iterator<Item = HrResult<&'_ AM_MEDIA_TYPE<'_>>> + '_> {
		Box::new(EnumMediaTypesIter::new(self))
	}

	/// [`IEnumMediaTypes::Next`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienummediatypes-next)
	/// method.
	///
	/// Prefer using
	/// [`IEnumMediaTypes::iter`](crate::prelude::dshow_IEnumMediaTypes::iter),
	/// which is simpler.
	#[must_use]
	fn Next(&self, amt: &mut AM_MEDIA_TYPE) -> HrResult<bool> {
		unsafe {
			let vt = self.vt_ref::<IEnumMediaTypesVT>();
			okfalse_to_hrresult(
				(vt.Next)(self.ptr(), 1, amt as *mut _ as _, std::ptr::null_mut()), // retrieve only 1
			)
		}
	}

	/// [`IEnumMediaTypes::Reset`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienummediatypes-reset)
	/// method.
	fn Reset(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IEnumMediaTypesVT>();
			ok_to_hrresult((vt.Reset)(self.ptr()))
		}
	}

	/// [`IEnumMediaTypes::Skip`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienummediatypes-skip)
	/// method.
	fn Skip(&self, count: u32) -> HrResult<bool> {
		unsafe {
			let vt = self.vt_ref::<IEnumMediaTypesVT>();
			okfalse_to_hrresult((vt.Skip)(self.ptr(), count))
		}
	}
}

//------------------------------------------------------------------------------

struct EnumMediaTypesIter<'a, I>
	where I: dshow_IEnumMediaTypes,
{
	enum_mt: &'a I,
	amt: AM_MEDIA_TYPE<'a>,
}

impl<'a, I> Iterator for EnumMediaTypesIter<'a, I>
	where I: dshow_IEnumMediaTypes,
{
	type Item = HrResult<&'a AM_MEDIA_TYPE<'a>>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.enum_mt.Next(&mut self.amt) {
			Err(err) => Some(Err(err)),
			Ok(success) => if success {
				// Returning a reference cannot be done until GATs
				// stabilization, so we simply cheat the borrow checker.
				let ptr = &self.amt as *const AM_MEDIA_TYPE;
				Some(Ok(unsafe { &*ptr }))
			} else {
				None
			},
		}
	}
}

impl<'a, I> EnumMediaTypesIter<'a, I>
	where I: dshow_IEnumMediaTypes,
{
	fn new(enum_mt: &'a I) -> Self {
		Self { enum_mt, amt: AM_MEDIA_TYPE::default() }
	}
}
