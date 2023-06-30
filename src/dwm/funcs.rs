#![allow(non_snake_case)]

use crate::dwm;
use crate::kernel::ffi_types::BOOL;
use crate::ole::decl::HrResult;
use crate::ole::privs::ok_to_hrresult;

/// [`DwmIsCompositionEnabled`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmiscompositionenabled)
/// function.
#[must_use]
pub fn DwmIsCompositionEnabled() -> HrResult<bool> {
	let mut pf_enabled: BOOL = 0;
	ok_to_hrresult(unsafe { dwm::ffi::DwmIsCompositionEnabled(&mut pf_enabled) })
		.map(|_| pf_enabled != 0)
}

/// [`DwmFlush`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmflush)
/// function.
pub fn DwmFlush() -> HrResult<()> {
	ok_to_hrresult(unsafe { dwm::ffi::DwmFlush() })
}
