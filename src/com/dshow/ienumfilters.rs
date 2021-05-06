#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IUnknownVT, PPComVT};
use crate::com::dshow::vt::IEnumFiltersVT;
use crate::privs::{hr_to_winresult, hr_to_winresult_bool};

macro_rules! IEnumFilters_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IEnumFilters::Reset`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-reset)
			/// method.
			pub fn Reset(&self) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IEnumFiltersVT>() };
				hr_to_winresult(unsafe { ((**ppvt).Reset)(ppvt) })
			}

			/// [`IEnumFilters::Skip`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-skip)
			/// method.
			pub fn Skip(&self, cFilters: u32) -> WinResult<bool> {
				let ppvt = unsafe { self.ppvt::<IEnumFiltersVT>() };
				hr_to_winresult_bool(unsafe { ((**ppvt).Skip)(ppvt, cFilters) })
			}
		}
	};
}

IEnumFilters_impl! {
	/// [`IEnumFilters`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ienumfilters)
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
	IEnumFilters, IEnumFiltersVT
}
