#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::gdi::ffi;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl gdi_Hpalette for HPALETTE {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HFONT`](crate::HFONT).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_Hpalette: Handle {
	/// [`CreatePalette`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpalette)
	/// function.
	#[must_use]
	fn CreatePalette(pal: &LOGPALETTE) -> SysResult<DeleteObjectPaletteGuard> {
		unsafe {
			ptr_to_invalidparm_handle(ffi::CreatePalette(pcvoid(pal)))
				.map(|h| DeleteObjectPaletteGuard::new(h))
		}
	}
}
