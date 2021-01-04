use std::ffi::c_void;

use crate::co;
use crate::enums::{HwndPlace, IdStr};
use crate::gui::events::{MsgEvents, ProcessResult};
use crate::gui::globals::ui_font;
use crate::handles::{HFONT, HINSTANCE, HWND};
use crate::msg::{Wm, WmInitDialog, WmSetFont};

/// Base to all dialog windows.
pub struct DialogBase {
	hwnd: HWND,
	events: MsgEvents,
	is_modal: bool, // will center on parent
}

impl Drop for DialogBase {
	fn drop(&mut self) {
		if !self.hwnd.is_null() {
			self.hwnd.SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
		}
	}
}

impl DialogBase {
	pub fn new(is_modal: bool) -> DialogBase {
		Self {
			hwnd: unsafe { HWND::null_handle() },
			events: MsgEvents::new(),
			is_modal,
		}
	}

	pub fn hwnd(&self) -> &HWND {
		&self.hwnd
	}

	pub fn on(&self) -> &MsgEvents {
		if !self.hwnd.is_null() {
			panic!("Cannot add event after dialog is created.");
		}
		&self.events
	}

	pub fn create_dialog_param(&self, hinst: HINSTANCE,
		parent: Option<HWND>, dialog_id: i32) -> Result<HWND, co::ERROR>
	{
		if !self.hwnd.is_null() {
			panic!("Cannot create dialog twice.");
		}

		// Our hwnd member is set during WM_INITDIALOG processing, already set
		// when CreateDialogParam returns.
		hinst.CreateDialogParam(
			IdStr::Id(dialog_id),
			parent,
			Self::dialog_proc,
			Some(self as *const Self as isize), // pass pointer to self
		)
	}

	extern "system" fn dialog_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize) -> isize
	{
		let wm_any = Wm { msg_id: msg, wparam, lparam };

		let ptr_self = match msg {
			co::WM::INITDIALOG => {
				let wm_idlg: WmInitDialog = wm_any.into();
				let ptr_self = wm_idlg.additional_data as *mut Self;
				hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, ptr_self as isize); // store
				let ref_self = unsafe { &mut *ptr_self };
				ref_self.hwnd = hwnd; // store HWND in struct field
				ref_self.center_on_parent_if_modal().expect("Failed to center modal dialog on parent.");
				ref_self.set_ui_font_on_children();
				ptr_self
			},
			_ => hwnd.GetWindowLongPtr(co::GWLP::DWLP_USER) as *mut Self, // retrieve
		};

		// If no pointer stored, then no processing is done.
		// Prevents processing before WM_INITDIALOG and after WM_NCDESTROY.
		if ptr_self.is_null() {
			return hwnd.DefWindowProc(wm_any).into();
		}

		// Execute user closure, if any.
		let ref_self = unsafe { &mut *ptr_self };
		let maybe_processed = ref_self.events.process_message(wm_any);

		if msg == co::WM::NCDESTROY { // always check
			hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, 0); // clear passed pointer
			ref_self.hwnd = unsafe { HWND::null_handle() }; // clear stored HWND
		}

		match maybe_processed {
			ProcessResult::HandledWithRet(res) => res.into(),
			ProcessResult::HandledWithoutRet => true as isize,
			ProcessResult::NotHandled => false as isize,
		}
	}

	fn center_on_parent_if_modal(&self) -> Result<(), co::ERROR> {
		if self.is_modal {
			let rc = self.hwnd.GetWindowRect()?;
			let rc_parent = self.hwnd.GetParent()?.unwrap().GetWindowRect()?;
			self.hwnd.SetWindowPos(HwndPlace::None,
				rc_parent.left + ((rc_parent.right - rc_parent.left) / 2) - (rc.right - rc.left) / 2,
				rc_parent.top + ((rc_parent.bottom - rc_parent.top) / 2) - (rc.bottom - rc.top) / 2,
				0, 0, co::SWP::NOSIZE | co::SWP::NOZORDER)?;
		}
		Ok(())
	}

	fn set_ui_font_on_children(&self) {
		self.hwnd.SendMessage(WmSetFont{ hfont: ui_font(), redraw: false });
		self.hwnd.EnumChildWindows(
			Self::enum_proc, unsafe { ui_font().as_ptr() } as isize);
	}

	extern "system" fn enum_proc(hchild: HWND, lparam: isize) -> i32 {
		let hfont = unsafe { HFONT::from_ptr(lparam as *mut c_void) };
		hchild.SendMessage(WmSetFont { hfont, redraw: false });
		true as i32
	}
}
