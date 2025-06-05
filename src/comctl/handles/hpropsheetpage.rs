#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::comctl::ffi;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;

handle! { HPROPSHEETPAGE;
	/// Handle to a
	/// [property sheet](https://learn.microsoft.com/en-us/windows/win32/controls/property-sheets)
	/// page.
}

impl HPROPSHEETPAGE {
	/// [`CreatePropertySheetPage`](https://learn.microsoft.com/en-us/windows/win32/api/prsht/nf-prsht-createpropertysheetpagew)
	/// function.
	#[must_use]
	pub unsafe fn CreatePropertySheetPage(
		page: &PROPSHEETPAGE,
	) -> HrResult<DestroyPropertySheetPageGuard> {
		unsafe {
			match ptr_to_option_handle(ffi::CreatePropertySheetPageW(pcvoid(page))) {
				None => Err(co::HRESULT::E_FAIL),
				Some(h) => Ok(DestroyPropertySheetPageGuard::new(h)),
			}
		}
	}
}
