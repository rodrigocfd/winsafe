#![allow(non_snake_case)]

use crate::comctl;

/// [`InitCommonControls`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrols)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub fn InitCommonControls() {
	unsafe { comctl::ffi::InitCommonControls() }
}
