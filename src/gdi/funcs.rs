#![allow(non_snake_case)]

use crate::gdi;
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::privs::bool_to_sysresult;

/// [`GdiFlush`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gdiflush)
/// function.
pub fn GdiFlush() -> SysResult<()> {
	bool_to_sysresult(unsafe { gdi::ffi::GdiFlush() })
}

/// [`GdiGetBatchLimit`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gdigetbatchlimit)
/// function.
#[must_use]
pub fn GdiGetBatchLimit() -> u32 {
	unsafe { gdi::ffi::GdiGetBatchLimit() }
}

/// [`GdiSetBatchLimit`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gdisetbatchlimit)
/// function.
pub fn GdiSetBatchLimit(limit: u32) -> SysResult<u32> {
	match unsafe { gdi::ffi::GdiSetBatchLimit(limit) } {
		0 => Err(GetLastError()),
		n => Ok(n),
	}
}
