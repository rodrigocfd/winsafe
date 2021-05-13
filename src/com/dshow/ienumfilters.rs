#![allow(non_snake_case)]

macro_rules! pub_struct_IEnumFilters {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::dshow::vt::IEnumFiltersVT;
		use crate::privs::hr_to_winresult_bool;

		pub_struct_IUnknown! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn ienumfilters_vt(&self) -> &IEnumFiltersVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IEnumFilters::Reset`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-reset)
			/// method.
			pub fn Reset(&self) -> WinResult<()> {
				hr_to_winresult((self.ienumfilters_vt().Reset)(self.ppvt))
			}

			/// [`IEnumFilters::Skip`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-skip)
			/// method.
			pub fn Skip(&self, cFilters: u32) -> WinResult<bool> {
				hr_to_winresult_bool(
					(self.ienumfilters_vt().Skip)(self.ppvt, cFilters),
				)
			}
		}
	};
}

pub_struct_IEnumFilters! {
	/// [`IEnumFilters`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ienumfilters)
	/// COM interface over [`IEnumFiltersVT`](crate::dshow::vt::IEnumFiltersVT).
	/// Inherits from [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IEnumFilters, crate::com::dshow::vt::IEnumFiltersVT
}
