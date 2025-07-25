#![allow(non_snake_case)]

use crate::co;
use crate::comctl::ffi;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;

/// [`InitCommonControls`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrols)
/// function.
pub fn InitCommonControls() {
	unsafe { ffi::InitCommonControls() }
}

/// [`InitCommonControlsEx`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrolsex)
/// function.
pub fn InitCommonControlsEx(icce: &INITCOMMONCONTROLSEX) -> SysResult<()> {
	BoolRet(unsafe { ffi::InitCommonControlsEx(pcvoid(icce)) }).to_sysresult()
}

/// [`InitMUILanguage`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initmuilanguage)
/// function.
pub fn InitMUILanguage(ui_lang: LANGID) {
	unsafe { ffi::InitMUILanguage(ui_lang.into()) }
}

/// [`PropertySheet`](https://learn.microsoft.com/en-us/windows/win32/api/prsht/nf-prsht-propertysheetw)
/// function.
///
/// # Safety
///
/// This function will use pointers declared in
/// [`PROPSHEETHEADER`](crate::PROPSHEETHEADER), make sure these pointers are
/// correct.
pub unsafe fn PropertySheet(header: &PROPSHEETHEADER) -> SysResult<isize> {
	let ret = unsafe { ffi::PropertySheetW(pcvoid(header)) };
	match GetLastError() {
		co::ERROR::SUCCESS => Ok(ret),
		err => Err(err),
	}
}

/// [`TaskDialogIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-taskdialogindirect)
/// function.
///
/// Fill the [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG) fields you need, and
/// leave the others as default. The needed flags will be automatically set.
///
/// Returns:
/// 1. the ID of the button clicked by the user. If you passed custom
/// buttons in the `buttons` field, the corresponding ID will be wrapped in a
/// [`co::DLGID`](crate::co::DLGID) constant, whose `u16` can be retrieved by
/// calling [`co::DLGID::raw`](crate::co::DLGID::raw) method;
/// 2. the ID of the radio button selected by the user, if you used the
/// `radio_buttons` field;
/// 3. `true` if the custom check box is checked, if you used the
/// `verification_text` field.
///
/// [`HWND::TaskDialog`](crate::HWND::TaskDialog) is a simpler version of this
/// function.
///
/// # Examples
///
/// Simple information dialog:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// w::TaskDialogIndirect(&w::TASKDIALOGCONFIG {
///     common_buttons: co::TDCBF::OK,
///     main_icon:      w::IconIdTd::Td(co::TD_ICON::INFORMATION),
///     flags:          co::TDF::ALLOW_DIALOG_CANCELLATION,
///     window_title:   Some("Title"),
///     content:        Some("Content"),
///     ..Default::default()
/// })?;
/// # w::HrResult::Ok(())
/// ```
///
/// OK/Cancel confirmation:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let (ret, _, _) = w::TaskDialogIndirect(&w::TASKDIALOGCONFIG {
///     common_buttons: co::TDCBF::OK | co::TDCBF::CANCEL,
///     main_icon:      w::IconIdTd::Td(co::TD_ICON::WARNING),
///     flags:          co::TDF::ALLOW_DIALOG_CANCELLATION,
///     window_title:   Some("Title"),
///     content:        Some("Do you want?"),
///     ..Default::default()
/// })?;
///
/// if ret == co::DLGID::OK {
///    println!("Yes.");
/// }
/// # w::HrResult::Ok(())
/// ```
///
/// Callback handling:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// w::TaskDialogIndirect(&w::TASKDIALOGCONFIG {
///     common_buttons: co::TDCBF::OK,
///     window_title:   Some("Title"),
///     content:        Some("Content"),
///     callback:       Some(Box::new(
///         |hwnd: &w::HWND, tdn: w::Tdn| -> co::HRESULT {
///             match tdn {
///                 w::Tdn::Created => println!("created"),
///                 _ => {},
///             }
///             co::HRESULT::S_OK
///         },
///     )),
///     ..Default::default()
/// })?;
/// # w::HrResult::Ok(())
///
/// ```
pub fn TaskDialogIndirect(config: &TASKDIALOGCONFIG) -> HrResult<(co::DLGID, u16, bool)> {
	let tdc_buf = config.to_raw();
	let mut pn_button = 0i32;
	let mut pn_radio_button = 0i32;
	let mut pf_bool = 0;

	HrRet(unsafe {
		ffi::TaskDialogIndirect(
			pcvoid(&tdc_buf.raw),
			&mut pn_button,
			&mut pn_radio_button,
			&mut pf_bool,
		)
	})
	.to_hrresult()?;

	Ok((unsafe { co::DLGID::from_raw(pn_button as _) }, pn_radio_button as _, pf_bool != 0))
}
