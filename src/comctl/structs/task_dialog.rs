#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::comctl::proc;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;

/// [`TASKDIALOGCONFIG`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-taskdialog_button)
/// struct.
///
/// Used with [`TaskDialogIndirect`](crate::TaskDialogIndirect) function.
///
/// Not all `flags` constants are available, some of them are automatically set
/// as you fill other parameters.
#[derive(Default)]
pub struct TASKDIALOGCONFIG<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm, 'n, 'o, 'p> {
	pub hwnd_parent: Option<&'a HWND>,
	pub hinstance: Option<&'b HINSTANCE>,
	pub flags: co::TDF,
	/// Predefined buttons. Will be placed after the custom `buttons`.
	pub common_buttons: co::TDCBF,
	/// Window caption. If not specified, the .exe name is used.
	pub window_title: Option<&'c str>,
	pub main_icon: IconIdTd<'d>,
	/// Text shown before the main content, in a larger font.
	pub main_instruction: Option<&'e str>,
	/// The main text of the dialog.
	pub content: Option<&'f str>,
	/// Command ID and button text. Will be placed before the predefined
	/// `common_buttons`.
	pub buttons: &'g [(u16, &'h str)],
	/// Any ID from `common_buttons` or `buttons`.
	pub default_button_id: u16,
	/// Command ID and radio button text.
	pub radio_buttons: &'i [(u16, &'j str)],
	/// Any ID from `radio_buttons`.
	pub default_radio_button_id: u16,
	/// Text of the label of the verification check box.
	pub verification_text: Option<&'k str>,
	/// Text of the collapsible section.
	pub more_info: Option<&'l str>,
	/// Text of the button that expands/collapses `more_info`, when the section
	/// is expanded.
	pub more_info_btn_expanded: Option<&'m str>,
	/// Text of the button that expands/collapses `more_info`, when the section
	/// is collapsed.
	pub more_info_btn_collapsed: Option<&'n str>,
	pub footer_icon: IconId<'o>,
	pub footer_text: Option<&'p str>,
	pub callback: Option<Box<dyn Fn(&HWND, Tdn) -> co::HRESULT>>,
	pub width: u32,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm, 'n, 'o, 'p>
	TASKDIALOGCONFIG<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm, 'n, 'o, 'p>
{
	pub(in crate::comctl) fn to_raw(&self) -> TASKDIALOGCONFIG_buf {
		let mut raw = TASKDIALOGCONFIG_raw::default();
		raw.hwndParent = unsafe { self.hwnd_parent.unwrap_or(&HWND::NULL).raw_copy() };
		raw.hInstance = unsafe { self.hinstance.unwrap_or(&HINSTANCE::NULL).raw_copy() };
		raw.dwFlags = self.flags;
		raw.dwCommonButtons = self.common_buttons;

		let w_title = self
			.window_title // must force heap because variable will be moved
			.map_or(WString::new(), |s| WString::from_str_force_heap(s));
		raw.pszWindowTitle = w_title.as_ptr();

		match &self.main_icon {
			IconIdTd::None => {
				let new_flags = raw.dwFlags | co::TDF::USE_HICON_MAIN;
				raw.dwFlags = new_flags;
			},
			IconIdTd::Icon(h) => {
				raw.hMainIcon = h.ptr();
				let new_flags = raw.dwFlags | co::TDF::USE_HICON_MAIN;
				raw.dwFlags = new_flags;
			},
			IconIdTd::Id(id) => {
				raw.hMainIcon = MAKEINTRESOURCE(*id as _) as _;
			},
			IconIdTd::Td(td) => {
				raw.hMainIcon = td.raw() as _;
			},
		}

		let w_instruc = self
			.main_instruction
			.map_or(WString::new(), |s| WString::from_str_force_heap(s));
		raw.pszMainInstruction = w_instruc.as_ptr();

		let w_content = self
			.content
			.map_or(WString::new(), |s| WString::from_str_force_heap(s));
		raw.pszContent = w_content.as_ptr();

		let btns_buf: (Vec<_>, Vec<_>) = self
			.buttons
			.iter()
			.map(|(id, txt)| {
				let txt_buf = WString::from_str_force_heap(*txt);
				let btn_buf = TASKDIALOG_BUTTON {
					nButtonID: *id as _,
					pszButtonText: txt_buf.as_ptr(),
				};
				(txt_buf, btn_buf)
			})
			.unzip();
		raw.cButtons = btns_buf.1.len() as _;
		raw.pButtons = btns_buf.1.as_ptr() as _;
		raw.nDefaultButton = self.default_button_id as _;

		let radios_buf: (Vec<_>, Vec<_>) = self
			.radio_buttons
			.iter()
			.map(|(id, txt)| {
				let txt_buf = WString::from_str_force_heap(*txt);
				let btn_buf = TASKDIALOG_BUTTON {
					nButtonID: *id as _,
					pszButtonText: txt_buf.as_ptr(),
				};
				(txt_buf, btn_buf)
			})
			.unzip();
		raw.cRadioButtons = radios_buf.1.len() as _;
		raw.pRadioButtons = radios_buf.1.as_ptr() as _;
		raw.nDefaultRadioButton = self.default_radio_button_id as _;

		let w_verif = self
			.verification_text
			.map_or(WString::new(), |s| WString::from_str_force_heap(s));
		raw.pszVerificationText = w_verif.as_ptr();

		let w_more_info = self
			.more_info
			.map_or(WString::new(), |s| WString::from_str_force_heap(s));
		raw.pszExpandedInformation = w_more_info.as_ptr();

		let w_expanded_info = self
			.more_info_btn_expanded
			.map_or(WString::new(), |s| WString::from_str_force_heap(s));
		raw.pszExpandedControlText = w_expanded_info.as_ptr();

		let w_collapsed_info = self
			.more_info_btn_collapsed
			.map_or(WString::new(), |s| WString::from_str_force_heap(s));
		raw.pszCollapsedControlText = w_collapsed_info.as_ptr();

		match &self.footer_icon {
			IconId::None => {
				let new_flags = raw.dwFlags | co::TDF::USE_HICON_FOOTER;
				raw.dwFlags = new_flags;
			},
			IconId::Icon(h) => {
				raw.hFooterIcon = h.ptr();
				let new_flags = raw.dwFlags | co::TDF::USE_HICON_FOOTER;
				raw.dwFlags = new_flags;
			},
			IconId::Id(id) => {
				raw.hFooterIcon = MAKEINTRESOURCE(*id as _) as _;
			},
		}

		let w_footer = self
			.footer_text
			.map_or(WString::new(), |s| WString::from_str_force_heap(s));
		raw.pszFooter = w_footer.as_ptr();

		raw.pfCallback = Some(proc::func_task_dialog_callback);
		raw.lpCallbackData = self as *const _ as _; // object will exist until TaskDialogIndirect() returns
		raw.cxWidth = self.width;

		TASKDIALOGCONFIG_buf {
			raw,
			w_title,
			w_instruc,
			w_content,
			btns_buf,
			radios_buf,
			w_verif,
			w_more_info,
			w_expanded_info,
			w_collapsed_info,
			w_footer,
		}
	}
}

#[allow(unused)]
pub(in crate::comctl) struct TASKDIALOGCONFIG_buf {
	pub raw: TASKDIALOGCONFIG_raw,
	w_title: WString,
	w_instruc: WString,
	w_content: WString,
	btns_buf: (Vec<WString>, Vec<TASKDIALOG_BUTTON>),
	radios_buf: (Vec<WString>, Vec<TASKDIALOG_BUTTON>),
	w_verif: WString,
	w_more_info: WString,
	w_expanded_info: WString,
	w_collapsed_info: WString,
	w_footer: WString,
}

#[repr(C, packed)]
pub(in crate::comctl) struct TASKDIALOGCONFIG_raw {
	cbSize: u32,
	pub hwndParent: HWND,
	pub hInstance: HINSTANCE,
	pub dwFlags: co::TDF,
	pub dwCommonButtons: co::TDCBF,
	pub pszWindowTitle: *const u16,
	pub hMainIcon: *const std::ffi::c_void, // union with pszMainIcon
	pub pszMainInstruction: *const u16,
	pub pszContent: *const u16,
	pub cButtons: u32,
	pub pButtons: *const TASKDIALOG_BUTTON,
	pub nDefaultButton: i32, // actually co::DLGID, which is u16
	pub cRadioButtons: u32,
	pub pRadioButtons: *mut TASKDIALOG_BUTTON,
	pub nDefaultRadioButton: i32,
	pub pszVerificationText: *const u16,
	pub pszExpandedInformation: *const u16,
	pub pszExpandedControlText: *const u16,
	pub pszCollapsedControlText: *const u16,
	pub hFooterIcon: *const std::ffi::c_void, // union with pszFooterIcon
	pub pszFooter: *const u16,
	pub pfCallback: Option<PFTASKDIALOGCALLBACK>,
	pub lpCallbackData: usize,
	pub cxWidth: u32,
}

impl_default_with_size!(TASKDIALOGCONFIG_raw, cbSize);

#[repr(C, packed)]
pub(in crate::comctl) struct TASKDIALOG_BUTTON {
	pub nButtonID: i32,
	pub pszButtonText: *const u16,
}

impl_default!(TASKDIALOG_BUTTON);
