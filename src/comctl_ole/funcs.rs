#![allow(non_snake_case)]

use crate::{co, comctl_ole};
use crate::comctl_ole::decl::TASKDIALOGCONFIG;
use crate::kernel::ffi_types::BOOL;
use crate::ole::decl::HrResult;
use crate::ole::privs::ok_to_hrresult;

/// [`TaskDialogIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-taskdialogindirect)
/// function.
///
/// Returns:
/// * the selected `co::DLGID` button;
/// * if `pRadioButtons` of [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG) struct
/// was set, the `u16` control ID of one of the specified radio buttons;
/// otherwise zero.
///
/// Unless you need something specific, consider using the
/// [`task_dlg`](crate::task_dlg) high-level abstractions.
///
/// If you don't need all customizations, consider the
/// [`TaskDialog`](crate::prelude::comctl_ole_Hwnd::TaskDialog) method.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{
///     co, HWND, IconIdTdicon, TASKDIALOG_BUTTON, TASKDIALOGCONFIG,
///     TaskDialogIndirect, WString,
/// };
///
/// let hwnd: HWND; // initialized somewhere
/// # let hwnd = HWND::NULL;
///
/// let mut tdc = TASKDIALOGCONFIG::default();
/// tdc.hwndParent = hwnd;
/// tdc.dwCommonButtons = co::TDCBF::YES | co::TDCBF::NO;
/// tdc.set_pszMainIcon(IconIdTdicon::Tdicon(co::TD_ICON::INFORMATION));
///
/// let mut title = WString::from_str("Title");
/// tdc.set_pszWindowTitle(Some(&mut title));
///
/// let mut header = WString::from_str("Header");
/// tdc.set_pszMainInstruction(Some(&mut header));
///
/// let mut body = WString::from_str("Body");
/// tdc.set_pszContent(Some(&mut body));
///
/// // A custom button to appear before Yes and No.
/// let mut btn1 = TASKDIALOG_BUTTON::default();
/// let mut btn1_text = WString::from_str("Hello");
/// btn1.set_pszButtonText(Some(&mut btn1_text));
/// btn1.set_nButtonID(333); // this ID is returned if user clicks this button
/// let btns_slice = &mut [btn1];
/// tdc.set_pButtons(Some(btns_slice));
///
/// TaskDialogIndirect(&tdc, None)?;
/// # Ok::<_, co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
pub fn TaskDialogIndirect(
	task_config: &TASKDIALOGCONFIG,
	verification_flag_checked: Option<&mut bool>) -> HrResult<(co::DLGID, u16)>
{
	let mut pn_button = i32::default();
	let mut pn_radio_button = i32::default();
	let mut pf_bool: BOOL = 0;

	ok_to_hrresult(
		unsafe {
			comctl_ole::ffi::TaskDialogIndirect(
				task_config as *const _ as _,
				&mut pn_button,
				&mut pn_radio_button,
				verification_flag_checked.as_ref()
					.map_or(std::ptr::null_mut(), |_| &mut pf_bool),
			)
		},
	)?;

	if let Some(pf) = verification_flag_checked {
		*pf = pf_bool != 0;
	}
	Ok((co::DLGID(pn_button as _), pn_radio_button as _))
}
