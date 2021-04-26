#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IUnknown, IUnknownVT, PPComVT};
use crate::com::dshow::vt::IEnumFiltersVT;
use crate::privs::{hr_to_winresult, hr_to_winresult_bool};

/// [`IEnumFilters`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersist)
/// COM interface.
///
/// Virtual table: [`IEnumFiltersVT`](crate::dshow::vt::IEnumFiltersVT).
///
/// Inherits from:
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[derive(Clone)]
pub struct IEnumFilters {
	/// Methods of base interface [`IUnknown`](crate::IUnknown).
	pub IUnknown: IUnknown,
}

impl From<PPComVT<IEnumFiltersVT>> for IEnumFilters {
	fn from(ppv: PPComVT<IEnumFiltersVT>) -> Self {
		Self {
			IUnknown: IUnknown::from(ppv as PPComVT<IUnknownVT>)
		}
	}
}

impl IEnumFilters {
	unsafe fn ppv(&self) -> PPComVT<IEnumFiltersVT> {
		self.IUnknown.ppv::<IEnumFiltersVT>()
	}

	/// [`IEnumFilters::Reset`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-reset)
	/// method.
	pub fn Reset(&self) -> WinResult<()> {
		hr_to_winresult(
			unsafe { ((**self.ppv()).Reset)(self.ppv()) },
		)
	}

	/// [`IEnumFilters::Skip`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-skip)
	/// method.
	pub fn Skip(&self, cFilters: u32) -> WinResult<bool> {
		hr_to_winresult_bool(
			unsafe { ((**self.ppv()).Skip)(self.ppv(), cFilters) },
		)
	}
}
