use winsafe::{self as w, co};

/// Displays the system error message box.
pub fn err(hwnd: &w::HWND, title: &str, text: &str) {
	w::TaskDialogIndirect(&w::TASKDIALOGCONFIG {
		hwnd_parent: Some(hwnd),
		window_title: Some(title),
		content: Some(text),
		main_icon: w::IconIdTd::Td(co::TD_ICON::ERROR),
		common_buttons: co::TDCBF::OK,
		flags: co::TDF::ALLOW_DIALOG_CANCELLATION | co::TDF::POSITION_RELATIVE_TO_WINDOW,
		..Default::default()
	})
	.expect("TaskDialog failed.");
}

/// Displays the system OK/Cancel message box.
#[must_use]
pub fn ok_cancel(hwnd: &w::HWND, title: &str, text: &str, ok_text: &str) -> bool {
	let (ret, _, _) = w::TaskDialogIndirect(&w::TASKDIALOGCONFIG {
		hwnd_parent: Some(hwnd),
		common_buttons: co::TDCBF::CANCEL,
		main_icon: w::IconIdTd::Td(co::TD_ICON::WARNING),
		flags: co::TDF::ALLOW_DIALOG_CANCELLATION,
		window_title: Some(title),
		content: Some(text),
		buttons: &[(co::DLGID::OK.into(), ok_text)],
		..Default::default()
	})
	.expect("TaskDialog failed.");

	ret == co::DLGID::OK
}
