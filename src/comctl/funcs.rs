#![allow(non_snake_case)]

use crate::comctl;
use crate::comctl::decl::INITCOMMONCONTROLSEX;
use crate::kernel::decl::{LANGID, WinResult};
use crate::kernel::privs::bool_to_winresult;

/// [`InitCommonControls`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrols)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub fn InitCommonControls() {
	unsafe { comctl::ffi::InitCommonControls() }
}

/// [`InitCommonControlsEx`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrolsex)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub fn InitCommonControlsEx(icce: &INITCOMMONCONTROLSEX) -> WinResult<()> {
	bool_to_winresult(
		unsafe { comctl::ffi::InitCommonControlsEx(icce as *const _ as  _) }
	)
}

/// [`InitMUILanguage`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initmuilanguage)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub fn InitMUILanguage(ui_lang: LANGID) {
	unsafe { comctl::ffi::InitMUILanguage(ui_lang.0) }
}
