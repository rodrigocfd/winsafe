#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::gdi::ffi;
use crate::guard::*;
use crate::kernel::privs::*;

impl HPALETTE {
	/// [`CreatePalette`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpalette)
	/// function.
	#[must_use]
	pub fn CreatePalette(pal: &LOGPALETTE) -> SysResult<DeleteObjectPaletteGuard> {
		unsafe {
			PtrRet(ffi::CreatePalette(pcvoid(pal)))
				.to_invalidparm_handle()
				.map(|h| DeleteObjectPaletteGuard::new(h))
		}
	}
}
