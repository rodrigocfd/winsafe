//! Provides high-level abstractions to
//! [`TaskDialogIndirect`](crate::TaskDialogIndirect) and
//! [`HWND::TaskDialog`](crate::prelude::comctl_ole_Hwnd::TaskDialog) functions.

use crate::co;
use crate::comctl_ole::decl::{
	IconIdTdicon, TASKDIALOG_BUTTON, TASKDIALOGCONFIG, TaskDialogIndirect,
};
use crate::kernel::decl::WString;
use crate::ole::decl::HrResult;
use crate::user::decl::HWND;

/// Displays an error modal window with an OK button.
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
pub fn error(
	hparent: HWND,
	title: &str,
	header: Option<&str>,
	body: &str) -> HrResult<()>
{
	generate(hparent, title, header, body, None,
		co::TDCBF::OK, co::TD_ICON::ERROR)
		.map(|_| ())
}

/// Displays an information modal window with an OK button.
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
pub fn info(
	hparent: HWND,
	title: &str,
	header: Option<&str>,
	body: &str) -> HrResult<()>
{
	generate(hparent, title, header, body, None,
		co::TDCBF::OK, co::TD_ICON::INFORMATION)
		.map(|_| ())
}

/// Displays a question modal window with OK and Cancel buttons. The text of the
/// OK button can be customized.
///
/// Returns `true` if the user clicked OK.
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
#[must_use]
pub fn ok_cancel(
	hparent: HWND,
	title: &str,
	header: Option<&str>,
	body: &str,
	ok_text: Option<&str>) -> HrResult<bool>
{
	let mut btns = co::TDCBF::CANCEL;
	if ok_text.is_none() {
		btns |= co::TDCBF::OK;
	}

	generate(hparent, title, header, body, ok_text,
		btns, co::TD_ICON::WARNING)
		.map(|dlg_id| dlg_id == co::DLGID::OK)
}

/// Displays a question modal window with Yes and No buttons.
///
/// Returns `true` if the user clicked Yes.
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
#[must_use]
pub fn yes_no(
	hparent: HWND,
	title: &str,
	header: Option<&str>,
	body: &str) -> HrResult<bool>
{
	generate(hparent, title, header, body, None,
		co::TDCBF::YES | co::TDCBF::NO, co::TD_ICON::WARNING)
		.map(|dlg_id| dlg_id == co::DLGID::YES)
}

/// Displays a question modal window with Yes, No and Cancel buttons.
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
#[must_use]
pub fn yes_no_cancel(
	hparent: HWND,
	title: &str,
	header: Option<&str>,
	body: &str) -> HrResult<co::DLGID>
{
	generate(hparent, title, header, body, None,
		co::TDCBF::YES | co::TDCBF::NO | co::TDCBF::CANCEL, co::TD_ICON::WARNING)
}

fn generate(
	hparent: HWND,
	title: &str,
	header: Option<&str>,
	body: &str,
	ok_text: Option<&str>,
	btns: co::TDCBF,
	ico: co::TD_ICON) -> HrResult<co::DLGID>
{
	let mut ok_text_buf = WString::from_opt_str(ok_text);
	let mut custom_btns = if ok_text.is_some() {
		let mut td_btn = TASKDIALOG_BUTTON::default();
		td_btn.set_nButtonID(co::DLGID::OK.into());
		td_btn.set_pszButtonText(Some(&mut ok_text_buf));

		let mut custom_btns = Vec::with_capacity(1);
		custom_btns.push(td_btn);
		custom_btns
	} else {
		Vec::default()
	};

	let mut tdc = TASKDIALOGCONFIG::default();
	tdc.hwndParent = hparent;
	tdc.dwFlags = co::TDF::ALLOW_DIALOG_CANCELLATION | co::TDF::POSITION_RELATIVE_TO_WINDOW;
	tdc.dwCommonButtons = btns;
	tdc.set_pszMainIcon(IconIdTdicon::Tdicon(ico));

	if ok_text.is_some() {
		tdc.set_pButtons(Some(&mut custom_btns));
	}

	let mut title_buf = WString::from_str(title);
	tdc.set_pszWindowTitle(Some(&mut title_buf));

	let mut header_buf = WString::from_opt_str(header);
	if header.is_some() {
		tdc.set_pszMainInstruction(Some(&mut header_buf));
	}

	let mut body_buf = WString::from_str(body);
	tdc.set_pszContent(Some(&mut body_buf));

	TaskDialogIndirect(&tdc, None)
		.map(|(dlg_id, _)| dlg_id)
}
