#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::decl::*;
use crate::kernel::{ffi_types::*, privs::*};
use crate::prelude::*;

/// [`NMOBJECTNOTIFY`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmobjectnotify)
/// struct.
#[repr(C)]
pub struct NMOBJECTNOTIFY<'a> {
	pub hdr: NMHDR,
	pub iItem: i32,
	piid: *mut co::IID,
	Object: COMPTR,
	pub hrResult: co::HRESULT,
	pub dwFlags: u32,

	_piid: PhantomData<&'a mut co::IID>,
}

impl_default!(NMOBJECTNOTIFY, 'a);
impl_drop_comptr!(Object, NMOBJECTNOTIFY, 'a);

impl<'a> NMOBJECTNOTIFY<'a> {
	pub_fn_ptr_get_set!('a, piid, set_piid, co::IID);
	pub_fn_comptr_get_set!(Object, set_Object, ole_IUnknown);
}

/// [`TASKDIALOG_BUTTON`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-taskdialog_button)
/// struct.
#[repr(C, packed)]
pub struct TASKDIALOG_BUTTON<'a> {
	nButtonID: i32,
	pszButtonText: *mut u16,

	_pszButtonText: PhantomData<&'a mut u16>,
}

impl_default!(TASKDIALOG_BUTTON, 'a);

impl<'a> TASKDIALOG_BUTTON<'a> {
	pub_fn_resource_id_get_set!(nButtonID, set_nButtonID);
	pub_fn_string_ptr_get_set!('a, pszButtonText, set_pszButtonText);
}

/// [`TASKDIALOGCONFIG`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-taskdialogconfig)
/// struct.
#[repr(C, packed)]
pub struct TASKDIALOGCONFIG<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j> {
	cbSize: u32,
	pub hwndParent: HWND,
	pub hInstance: HINSTANCE,
	pub dwFlags: co::TDF,
	pub dwCommonButtons: co::TDCBF,
	pszWindowTitle: *mut u16,
	pszMainIcon: *const u16, // union with HICON
	pszMainInstruction: *mut u16,
	pszContent: *mut u16,
	cButtons: u32,
	pButtons: *mut TASKDIALOG_BUTTON<'d>,
	pub nDefaultButton: i32, // actually co::DLGID, which is u16
	cRadioButtons: u32,
	pRadioButtons: *mut TASKDIALOG_BUTTON<'e>,
	pub nDefaultRadioButton: i32,
	pszVerificationText: *mut u16,
	pszExpandedInformation: *mut u16,
	pszExpandedControlText: *mut u16,
	pszCollapsedControlText: *mut u16,
	pszFooterIcon: *const u16, // union with HICON
	pszFooter: *mut u16,
	pub pfCallback: Option<PFTASKDIALOGCALLBACK>,
	pub lpCallbackData: isize,
	pub cxWidth: u32,

	_pszWindowTitle: PhantomData<&'a mut u16>,
	_pszMainInstruction: PhantomData<&'b mut u16>,
	_pszContent: PhantomData<&'c mut u16>,
	_pButtons: PhantomData<&'d mut TASKDIALOG_BUTTON<'d>>,
	_pRadioButtons: PhantomData<&'e mut TASKDIALOG_BUTTON<'e>>,
	_pszVerificationText: PhantomData<&'f mut u16>,
	_pszExpandedInformation: PhantomData<&'g mut u16>,
	_pszExpandedControlText: PhantomData<&'h mut u16>,
	_pszCollapsedControlText: PhantomData<&'i mut u16>,
	_pszFooter: PhantomData<&'j mut u16>,
}

impl_default_with_size!(TASKDIALOGCONFIG, cbSize, 'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j);

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j>
	TASKDIALOGCONFIG<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j>
{
	pub_fn_string_ptr_get_set!('a, pszWindowTitle, set_pszWindowTitle);

	/// Returns the `pszMainIcon` field.
	#[must_use]
	pub fn pszMainIcon(&self) -> IconIdTdicon {
		if IS_INTRESOURCE(self.pszMainIcon) {
			if self.pszMainIcon as u16 >= 0xfffc {
				IconIdTdicon::Tdicon(unsafe { co::TD_ICON::from_raw(self.pszMainIcon as _) })
			} else {
				IconIdTdicon::Id(self.pszMainIcon as _)
			}
		} else {
			IconIdTdicon::Icon(unsafe { HICON::from_ptr(self.pszMainIcon as _) })
		}
	}

	/// Sets the `pszMainIcon` field.
	pub fn set_pszMainIcon(&mut self, val: IconIdTdicon) {
		match val {
			IconIdTdicon::None => self.pszMainIcon = std::ptr::null_mut(),
			IconIdTdicon::Icon(hicon) => self.pszMainIcon = hicon.ptr() as _,
			IconIdTdicon::Id(id) => self.pszMainIcon = MAKEINTRESOURCE(id as _),
			IconIdTdicon::Tdicon(tdi) => self.pszMainIcon = MAKEINTRESOURCE(tdi.raw() as _),
		}
	}

	pub_fn_string_ptr_get_set!('b, pszMainInstruction, set_pszMainInstruction);
	pub_fn_string_ptr_get_set!('c, pszContent, set_pszContent);
	pub_fn_array_buf_get_set!('d, pButtons, set_pButtons, cButtons, TASKDIALOG_BUTTON);
	pub_fn_array_buf_get_set!('e, pRadioButtons, set_pRadioButtons, cRadioButtons, TASKDIALOG_BUTTON);
	pub_fn_string_ptr_get_set!('f, pszVerificationText, set_pszVerificationText);
	pub_fn_string_ptr_get_set!('g, pszExpandedInformation, set_pszExpandedInformation);
	pub_fn_string_ptr_get_set!('h, pszExpandedControlText, set_pszExpandedControlText);
	pub_fn_string_ptr_get_set!('i, pszCollapsedControlText, set_pszCollapsedControlText);

	/// Returns the `pszFooterIcon` field.
	#[must_use]
	pub fn pszFooterIcon(&self) -> IconId {
		if IS_INTRESOURCE(self.pszFooterIcon) {
			IconId::Id(self.pszFooterIcon as _)
		} else {
			IconId::Icon(unsafe { HICON::from_ptr(self.pszFooterIcon as _) })
		}
	}

	/// Sets the `pszFooterIcon` field.
	pub fn set_pszFooterIcon(&mut self, val: IconId) {
		match val {
			IconId::None => self.pszFooterIcon = std::ptr::null_mut(),
			IconId::Icon(hicon) => self.pszFooterIcon = hicon.ptr() as _,
			IconId::Id(id) => self.pszFooterIcon = MAKEINTRESOURCE(id as _),
		}
	}

	pub_fn_string_ptr_get_set!('j, pszFooter, set_pszFooter);
}
