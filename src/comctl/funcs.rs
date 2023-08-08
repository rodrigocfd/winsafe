#![allow(non_snake_case)]

use crate::comctl::ffi;
use crate::decl::*;
use crate::kernel::privs::*;

/// [`InitCommonControls`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrols)
/// function.
pub fn InitCommonControls() {
	unsafe { ffi::InitCommonControls() }
}

/// [`InitCommonControlsEx`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrolsex)
/// function.
pub fn InitCommonControlsEx(icce: &INITCOMMONCONTROLSEX) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { ffi::InitCommonControlsEx(icce as *const _ as  _) }
	)
}

/// [`InitMUILanguage`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initmuilanguage)
/// function.
pub fn InitMUILanguage(ui_lang: LANGID) {
	unsafe { ffi::InitMUILanguage(ui_lang.into()) }
}
