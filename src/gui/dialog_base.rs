use crate::co;
use crate::enums::IdStr;
use crate::gui::events::{MsgEvents, ProcessResult};
use crate::handles::{HINSTANCE, HWND};
use crate::msg::{Wm, WmInitDialog};

/// Base to all dialog windows.
pub struct DialogBase {
	hwnd: HWND,
	events: MsgEvents,
}

impl Drop for DialogBase {
	fn drop(&mut self) {
		if !self.hwnd.is_null() {
			self.hwnd.SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
		}
	}
}

impl DialogBase {
	pub fn new() -> DialogBase {
		Self {
			hwnd: unsafe { HWND::null_handle() },
			events: MsgEvents::new(),
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

		hinst.CreateDialogParam(
			IdStr::Id(dialog_id),
			parent,
			Self::dialog_proc,
			Some(self as *const Self as isize), // pass pointer to self)
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
			ProcessResult::HandledWithoutRet => 1, // TRUE
			ProcessResult::NotHandled => hwnd.DefWindowProc(wm_any).into(),
		}
	}

	fn center_on_parent(hdlg: HWND) {

	}
}
