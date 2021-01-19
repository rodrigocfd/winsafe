use crate::aliases::WinResult;
use crate::co;
use crate::enums::{HwndPlace, IdStr};
use crate::gui::base::Base;
use crate::gui::events::{MsgEvents, ProcessResult};
use crate::gui::globals::ui_font;
use crate::gui::traits::Parent;
use crate::handles::{HFONT, HINSTANCE, HWND};
use crate::msg::{MessageHandleable, Wm, WmInitDialog, WmSetFont, WmWinsafeError};
use crate::structs::POINT;

pub enum AfterCreate {
	Nothing, // main
	CenterOnParent, // modal
	ReposSetid(POINT, u16), // control
}

//------------------------------------------------------------------------------

/// Base to all dialog windows.
pub struct DialogBase {
	base: Base,
	dialog_id: i32,
	after_create: AfterCreate, // action to be done before WM_INITDIALOG is dispatched to user
}

impl Drop for DialogBase {
	fn drop(&mut self) {
		if !self.hwnd_ref().is_null() {
			self.hwnd_ref().SetWindowLongPtr(co::GWLP::DWLP_USER, 0); // clear passed pointer
		}
	}
}

impl Parent for DialogBase {
	fn hwnd_ref(&self) -> &HWND {
		&self.base.hwnd_ref()
	}

	fn events_ref(&self) -> &MsgEvents {
		self.base.events_ref()
	}

	fn add_child_to_be_created(&self,
		func: Box<dyn Fn() -> WinResult<()> + 'static>)
	{
		self.base.add_child_to_be_created(func);
	}
}

impl DialogBase {
	pub fn new(
		parent: Option<&dyn Parent>,
		dialog_id: i32,
		after_create: AfterCreate) -> DialogBase
	{
		Self {
			base: Base::new(parent),
			dialog_id,
			after_create,
		}
	}

	pub fn parent_hinstance(&self) -> WinResult<HINSTANCE> {
		self.base.parent_hinstance()
	}

	pub fn create_dialog_param(&self) -> WinResult<()> {
		if !self.hwnd_ref().is_null() {
			panic!("Cannot create dialog twice.");
		}

		// Our hwnd member is set during WM_INITDIALOG processing, already set
		// when CreateDialogParam returns.
		self.base.parent_hinstance()?.CreateDialogParam(
			IdStr::Id(self.dialog_id),
			self.base.parent_hwnd(),
			Self::dialog_proc,
			Some(self as *const Self as isize), // pass pointer to self
		).map(|_| ())
	}

	pub fn dialog_box_param(&self) -> WinResult<i32> {
		if !self.hwnd_ref().is_null() {
			panic!("Cannot create dialog twice.");
		}

		// Our hwnd member is set during WM_INITDIALOG processing, already set
		// when DialogBoxParam returns.
		self.base.parent_hinstance()?.DialogBoxParam(
			IdStr::Id(self.dialog_id),
			self.base.parent_hwnd(),
			Self::dialog_proc, Some(self as *const Self as isize), // pass pointer to self
		).map(|res| res as i32)
	}

	extern "system" fn dialog_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize) -> isize
	{
		|hwnd: HWND, msg, wparam, lparam| -> WinResult<isize>
		{
			let wm_any = Wm { msg_id: msg, wparam, lparam };

			let ptr_self = match msg {
				co::WM::INITDIALOG => { // first message being handled
					let wm_idlg = WmInitDialog::from_generic_wm(wm_any);
					let ptr_self = wm_idlg.additional_data as *mut Self;
					hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, ptr_self as isize); // store
					let ref_self = unsafe { &mut *ptr_self };
					ref_self.base.set_hwnd(hwnd); // store HWND in struct field

					ref_self.base.create_children()?;
					ref_self.after_create_action()?;
					ref_self.set_ui_font_on_children();
					ptr_self
				},
				_ => hwnd.GetWindowLongPtr(co::GWLP::DWLP_USER) as *mut Self, // retrieve
			};

			// If no pointer stored, then no processing is done.
			// Prevents processing before WM_INITDIALOG and after WM_NCDESTROY.
			if ptr_self.is_null() {
				return Ok(hwnd.DefWindowProc(wm_any));
			}

			// Execute user closure, if any.
			let ref_self = unsafe { &mut *ptr_self };
			let maybe_processed = ref_self.base.process_message(wm_any);

			if msg == co::WM::NCDESTROY { // always check
				hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, 0); // clear passed pointer
				ref_self.base.set_hwnd(unsafe { HWND::null_handle() }); // clear stored HWND
			}

			Ok(match maybe_processed {
				ProcessResult::HandledWithRet(res) => res.into(),
				ProcessResult::HandledWithoutRet => true as isize,
				ProcessResult::NotHandled => false as isize,
			})
		}
		(hwnd, msg, wparam, lparam).unwrap_or_else(|err| {
			hwnd.PostMessage(WmWinsafeError { code: err }).ok();
			true as isize
		})
	}

	fn after_create_action(&self) -> WinResult<()> {
		match self.after_create {
			AfterCreate::Nothing => Ok(()),
			AfterCreate::CenterOnParent => {
				let rc = self.hwnd_ref().GetWindowRect()?;
				let rc_parent = self.hwnd_ref().GetParent()?.GetWindowRect()?;
				self.hwnd_ref().SetWindowPos(HwndPlace::None,
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
		self.hwnd_ref().SendMessage(WmSetFont { hfont: ui_font(), redraw: false });
		self.hwnd_ref().EnumChildWindows(Self::enum_proc, ui_font().ptr as isize);
	}
	extern "system" fn enum_proc(hchild: HWND, lparam: isize) -> i32 {
		let hfont = HFONT { ptr: lparam as *mut _ };
		hchild.SendMessage(WmSetFont { hfont, redraw: false });
		true as i32
	}
}
