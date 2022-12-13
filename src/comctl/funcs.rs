#![allow(non_snake_case)]

use crate::comctl;
use crate::comctl::decl::INITCOMMONCONTROLSEX;
use crate::kernel::decl::{LANGID, SysResult};
use crate::kernel::privs::bool_to_sysresult;

/// [`InitCommonControls`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrols)
/// function.
pub fn InitCommonControls() {
	unsafe { comctl::ffi::InitCommonControls() }
}

/// [`InitCommonControlsEx`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrolsex)
/// function.
pub fn InitCommonControlsEx(icce: &INITCOMMONCONTROLSEX) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { comctl::ffi::InitCommonControlsEx(icce as *const _ as  _) }
	)
}

/// [`InitMUILanguage`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initmuilanguage)
/// function.
pub fn InitMUILanguage(ui_lang: LANGID) {
	unsafe { comctl::ffi::InitMUILanguage(ui_lang.0) }
}
