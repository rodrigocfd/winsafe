#![allow(non_snake_case)]

use crate::{co, comdlg};
use crate::comdlg::decl::CHOOSECOLOR;

/// [`ChooseColor`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms646912(v=vs.85))
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, ChooseColor, CHOOSECOLOR, COLORREF, HWND};
///
/// let parent_hwnd: HWND; // initialized somewhere
/// # let parent_hwnd = HWND::NULL;
///
/// let mut cc = CHOOSECOLOR::default();
/// let mut custom_colors = [COLORREF::new(255, 255, 255); 16];
///
/// cc.hwndOwner = parent_hwnd;
/// cc.Flags = co::CC::ANYCOLOR | co::CC::FULLOPEN | co::CC::RGBINIT;
/// cc.rgbResult = COLORREF::new(255, 0, 0); // color initially chosen
/// cc.set_lpCustColors(Some(&mut custom_colors));
///
/// if ChooseColor(&mut cc)? {
///     println!("The color: {} {} {}",
///         cc.rgbResult.GetRValue(),
///         cc.rgbResult.GetGValue(),
///         cc.rgbResult.GetBValue(),
///     );
/// }
/// # Ok::<_, co::CDERR>(())
/// ```
pub fn ChooseColor(cc: &mut CHOOSECOLOR) -> Result<bool, co::CDERR> {
	match unsafe { comdlg::ffi::ChooseColorW(cc as *mut _ as _) } {
		0 => match CommDlgExtendedError() {
			co::CDERR::NoValue => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`CommDlgExtendedError`](https://learn.microsoft.com/en-us/windows/win32/api/commdlg/nf-commdlg-commdlgextendederror)
/// function.
pub fn CommDlgExtendedError() -> co::CDERR {
	unsafe { co::CDERR::from_raw(comdlg::ffi::CommDlgExtendedError()) }
}
