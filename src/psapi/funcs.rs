#![allow(non_snake_case)]

use crate::decl::*;
use crate::kernel::privs::*;
use crate::psapi::ffi;

/// [`GetPerformanceInfo`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getperformanceinfo)
/// function.
#[must_use]
pub fn GetPerformanceInfo() -> SysResult<PERFORMANCE_INFORMATION> {
	let mut pi = PERFORMANCE_INFORMATION::default();
	BoolRet(unsafe {
		ffi::GetPerformanceInfo(pvoid(&mut pi), std::mem::size_of::<PERFORMANCE_INFORMATION>() as _)
	})
	.to_sysresult()
	.map(|_| pi)
}
