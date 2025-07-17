#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::dwm::ffi;
use crate::ole::privs::*;

/// [`DwmEnableMMCSS`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmenablemmcss)
/// function.
pub fn DwmEnableMMCSS(enable: bool) -> HrResult<()> {
	HrRet(unsafe { ffi::DwmEnableMMCSS(enable as _) }).to_hrresult()
}

/// [`DwmFlush`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmflush)
/// function.
pub fn DwmFlush() -> HrResult<()> {
	HrRet(unsafe { ffi::DwmFlush() }).to_hrresult()
}

/// [`DwmGetColorizationColor`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmgetcolorizationcolor)
/// function.
///
/// Returns the color in the `0xAARRGGBB` format, and whether the color is an
/// opaque blend.
#[must_use]
pub fn DwmGetColorizationColor() -> HrResult<(u32, bool)> {
	let mut colorization = 0u32;
	let mut opaque_blend = 0;

	HrRet(unsafe { ffi::DwmGetColorizationColor(&mut colorization, &mut opaque_blend) })
		.to_hrresult()
		.map(|_| (colorization, opaque_blend != 0))
}

/// [`DwmIsCompositionEnabled`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmiscompositionenabled)
/// function.
#[must_use]
pub fn DwmIsCompositionEnabled() -> HrResult<bool> {
	let mut pf_enabled = 0;
	HrRet(unsafe { ffi::DwmIsCompositionEnabled(&mut pf_enabled) })
		.to_hrresult()
		.map(|_| pf_enabled != 0)
}

/// [`DwmShowContact`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmshowcontact)
/// function.
pub fn DwmShowContact(pointer_id: u32, show_contact: co::DWMSC) -> HrResult<()> {
	HrRet(unsafe { ffi::DwmShowContact(pointer_id, show_contact.raw()) }).to_hrresult()
}
