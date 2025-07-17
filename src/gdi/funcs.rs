#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::gdi::ffi;
use crate::kernel::privs::*;

/// [`GdiFlush`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gdiflush)
/// function.
pub fn GdiFlush() -> SysResult<()> {
	BoolRet(unsafe { ffi::GdiFlush() }).to_sysresult()
}

/// [`GdiGetBatchLimit`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gdigetbatchlimit)
/// function.
///
/// # Related functions
///
/// * [`GdiSetBatchLimit`](crate::GdiSetBatchLimit)
#[must_use]
pub fn GdiGetBatchLimit() -> u32 {
	unsafe { ffi::GdiGetBatchLimit() }
}

/// [`GdiSetBatchLimit`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gdisetbatchlimit)
/// function.
///
/// # Related functions
///
/// * [`GdiGetBatchLimit`](crate::GdiGetBatchLimit)
pub fn GdiSetBatchLimit(limit: u32) -> SysResult<u32> {
	match unsafe { ffi::GdiSetBatchLimit(limit) } {
		0 => Err(co::ERROR::INVALID_PARAMETER),
		n => Ok(n),
	}
}
