#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPVT};
use crate::ffi::HRESULT;
use crate::structs::IID;

/// [`IEnumFilters`](crate::dshow::IEnumFilters) virtual table.
pub struct IEnumFiltersVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(PPVT, u32, *mut PPVT, *mut u32) -> HRESULT,
	pub Skip: fn(PPVT, u32) -> HRESULT,
	pub Reset: fn(PPVT) -> HRESULT,
	pub Clone: fn(PPVT, *mut PPVT) -> HRESULT,
}

/// [`IEnumFilters`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ienumfilters)
/// COM interface over [`IEnumFiltersVT`](crate::dshow::vt::IEnumFiltersVT).
/// Inherits from [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IEnumFilters {
	pub(crate) ppvt: PPVT,
}

impl ComInterface for IEnumFilters {
	const IID: IID = IID::new(0x56a86893, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770);
}

macro_rules! impl_IEnumFilters {
	($name:ty, $vt:ty) => {
		use crate::privs::hr_to_winresult_bool;

		impl $name {
			fn ienumfilters_vt(&self) -> &IEnumFiltersVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
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

impl_IUnknown!(IEnumFilters, IEnumFiltersVT);
impl_IEnumFilters!(IEnumFilters, IEnumFiltersVT);
