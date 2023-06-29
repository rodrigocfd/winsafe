#![allow(non_camel_case_types, non_snake_case)]

use crate::gdi;
use crate::gdi::decl::LOGPALETTE;
use crate::gdi::guard::DeleteObjectGuard;
use crate::kernel::decl::SysResult;
use crate::kernel::privs::ptr_to_sysresult_handle;
use crate::prelude::{GdiObject, Handle};

impl_handle! { HPALETTE;
	/// Handle to a
	/// [palette](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hpalette).
}

impl GdiObject for HPALETTE {}
impl gdi_Hpalette for HPALETTE {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HFONT`](crate::HFONT).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_Hpalette: Handle {
	/// [`CreatePalette`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpalette)
	/// function.
	#[must_use]
	fn CreatePalette(
		pal: &LOGPALETTE) -> SysResult<DeleteObjectGuard<HPALETTE>>
	{
		unsafe {
			ptr_to_sysresult_handle(gdi::ffi::CreatePalette(pal as *const _ as _))
				.map(|h| DeleteObjectGuard::new(h))
		}
	}
}
