#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::comctl::ffi;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

handle! { HPROPSHEETPAGE;
	/// Handle to a
	/// [property sheet](https://learn.microsoft.com/en-us/windows/win32/controls/property-sheets)
	/// page.
}

/// This trait is enabled with the `comctl` feature, and provides methods for
/// [`HPROPSHEETPAGE`](crate::HPROPSHEETPAGE).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait comctl_Hpropsheetpage: Handle {
	/// [`CreatePropertySheetPage`](https://learn.microsoft.com/en-us/windows/win32/api/prsht/nf-prsht-createpropertysheetpagew)
	/// function.
	#[must_use]
	unsafe fn CreatePropertySheetPage(
		page: &PROPSHEETPAGE,
	) -> HrResult<DestroyPropertySheetPageGuard>
	{
		match ptr_to_option_handle(
			ffi::CreatePropertySheetPageW(page as *const _ as _),
		) {
			None => Err(co::HRESULT::E_FAIL),
			Some(h) => Ok(DestroyPropertySheetPageGuard::new(h)),
		}
	}
}
