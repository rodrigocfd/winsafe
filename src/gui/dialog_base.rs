use std::ptr::NonNull;

use crate::co;
use crate::enums::{HwndPlace, IdStr};
use crate::gui::events::{MsgEvents, ProcessResult};
use crate::gui::globals::ui_font;
use crate::gui::traits::Parent;
use crate::handles::{HFONT, HINSTANCE, HWND};
use crate::msg::{Message, Wm, WmInitDialog, WmSetFont};
use crate::structs::POINT;

pub enum AfterCreate {
	Nothing, // main
	CenterOnParent, // modal
	ReposSetid(POINT, u16), // control
}

//------------------------------------------------------------------------------

/// Base to all dialog windows.
pub struct DialogBase {
	hwnd: HWND,
	dialog_id: i32,
	events: MsgEvents,
	after_create: AfterCreate, // action to be done before WM_INITDIALOG is dispatched to user
	ptr_parent_hwnd: Option<NonNull<HWND>>, // used only in control creation
}

impl Drop for DialogBase {
	fn drop(&mut self) {
		if !self.hwnd.is_null() {
			self.hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, 0); // clear passed pointer
		}
	}
}

impl Parent for DialogBase {
	fn hwnd_ref(&self) -> &HWND {
		&self.hwnd
	}

	fn events_ref(&self) -> &MsgEvents {
		if !self.hwnd.is_null() {
			panic!("Cannot add event after dialog is created.");
		}
		&self.events
	}
}

impl DialogBase {
	pub fn new(
		parent: Option<&dyn Parent>,
		dialog_id: i32,
		after_create: AfterCreate) -> DialogBase
	{
		Self {
			hwnd: unsafe { HWND::null_handle() },
			dialog_id,
			events: MsgEvents::new(),
			after_create,
			ptr_parent_hwnd: parent.map(|parent| NonNull::from(parent.hwnd_ref())), // ref implicitly converted to pointer
		}
	}

	pub fn parent_hwnd(&self) -> Option<HWND> {
		self.ptr_parent_hwnd.map(|ptr| unsafe { *ptr.as_ref() })
	}

	pub fn create_dialog_param(&self, hinst: HINSTANCE) -> Result<HWND, co::ERROR> {
		if !self.hwnd.is_null() {
			panic!("Cannot create dialog twice.");
		}

		// Our hwnd member is set during WM_INITDIALOG processing, already set
		// when CreateDialogParam returns.
		hinst.CreateDialogParam(
			IdStr::Id(self.dialog_id),
			self.parent_hwnd(),
			Self::dialog_proc,
			Some(self as *const Self as isize), // pass pointer to self
		)
	}

	pub fn dialog_box_param(&self, hinst: HINSTANCE) -> Result<(), co::ERROR> {
		if !self.hwnd.is_null() {
			panic!("Cannot create dialog twice.");
		}

		// Our hwnd member is set during WM_INITDIALOG processing, already set
		// when DialogBoxParam returns.
		hinst.DialogBoxParam(
			IdStr::Id(self.dialog_id),
			self.parent_hwnd(),
			Self::dialog_proc, Some(self as *const Self as isize), // pass pointer to self
		).map(|_| ())
	}

	extern "system" fn dialog_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize) -> isize
	{
		let wm_any = Wm { msg_id: msg, wparam, lparam };

		let ptr_self = match msg {
			co::WM::INITDIALOG => { // first message being handled
				let wm_idlg = WmInitDialog::from_generic_wm(wm_any);
				let ptr_self = wm_idlg.additional_data as *mut Self;
				hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, ptr_self as isize); // store
				let ref_self = unsafe { &mut *ptr_self };
				ref_self.hwnd = hwnd; // store HWND in struct field
				ref_self.after_create_action().expect("Pre-WM_INITDIALOG actions failed.");
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

	fn after_create_action(&self) -> Result<(), co::ERROR> {
		match self.after_create {
			AfterCreate::Nothing => Ok(()),
			AfterCreate::CenterOnParent => {
				let rc = self.hwnd.GetWindowRect()?;
				let rc_parent = self.hwnd.GetParent()?.GetWindowRect()?;
				self.hwnd.SetWindowPos(HwndPlace::None,
					rc_parent.left + ((rc_parent.right - rc_parent.left) / 2) - (rc.right - rc.left) / 2,
					rc_parent.top + ((rc_parent.bottom - rc_parent.top) / 2) - (rc.bottom - rc.top) / 2,
					0, 0, co::SWP::NOSIZE | co::SWP::NOZORDER)?;
				Ok(())
			},
			AfterCreate::ReposSetid(pos, ctrl_id) => {
				self.hwnd_ref().SetWindowPos(HwndPlace::None,
					pos.x, pos.y, 0, 0, co::SWP::NOZORDER | co::SWP::NOSIZE)?;
				self.hwnd_ref().SetWindowLongPtr(co::GWLP::ID, ctrl_id as isize); // so the custom control has an ID
				Ok(())
			},
		}
	}

	fn set_ui_font_on_children(&self) {
		self.hwnd.SendMessage(WmSetFont { hfont: ui_font(), redraw: false });
		self.hwnd.EnumChildWindows(Self::enum_proc, ui_font().ptr as isize);
	}
	extern "system" fn enum_proc(hchild: HWND, lparam: isize) -> i32 {
		let hfont = HFONT { ptr: lparam as *mut _ };
		hchild.SendMessage(WmSetFont { hfont, redraw: false });
		true as i32
	}
}
