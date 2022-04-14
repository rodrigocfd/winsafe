use crate::co;
use crate::gui::base::Base;
use crate::gui::events::{ProcessResult, WindowEventsAll};
use crate::gui::privs::{post_quit_error, ui_font};
use crate::kernel::decl::{ErrResult, IdStr};
use crate::msg::{wm, WndMsg};
use crate::prelude::{Handle, MsgSendRecv, UserHinstance, UserHwnd};
use crate::user::decl::HWND;

/// Base to all dialog windows.
pub(in crate::gui) struct DlgBase {
	base: Base,
	dialog_id: u16,
}

impl Drop for DlgBase {
	fn drop(&mut self) {
		if !self.base.hwnd().is_null() {
			self.base.hwnd().SetWindowLongPtr(co::GWLP::DWLP_USER, 0); // clear passed pointer
		}
	}
}

impl DlgBase {
	pub(in crate::gui) fn new(parent: Option<&Base>, dialog_id: u16) -> DlgBase {
		Self {
			base: Base::new(true, parent),
			dialog_id,
		}
	}

	pub(in crate::gui) unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		// At this moment, the parent struct is already created and pinned.
		&self.base as *const _ as _
	}

	pub(in crate::gui) const fn hwnd(&self) -> HWND {
		self.base.hwnd()
	}

	pub(in crate::gui) fn on(&self) -> &WindowEventsAll {
		self.base.on()
	}

	pub(in crate::gui) fn privileged_on(&self) -> &WindowEventsAll {
		self.base.privileged_on()
	}

	pub(in crate::gui) fn parent(&self) -> Option<&Base> {
		self.base.parent()
	}

	pub(in crate::gui) fn create_dialog_param(&self) {
		if !self.base.hwnd().is_null() {
			panic!("Cannot create dialog twice.");
		}

		// Our hwnd member is set during WM_INITDIALOG processing; already set
		// when CreateDialogParam returns.
		self.base.parent_hinstance().CreateDialogParam(
			IdStr::Id(self.dialog_id),
			self.base.parent().map(|parent| parent.hwnd()),
			Self::dialog_proc,
			// Pass pointer to Self.
			// At this moment, the parent struct is already created and pinned.
			Some(self as *const _ as _),
		).unwrap();
	}

	pub(in crate::gui) fn dialog_box_param(&self) -> i32 {
		if !self.base.hwnd().is_null() {
			panic!("Cannot create dialog twice.");
		}

		// Our hwnd member is set during WM_INITDIALOG processing; already set
		// when DialogBoxParam returns.
		let ret = self.base.parent_hinstance().DialogBoxParam(
			IdStr::Id(self.dialog_id),
			self.base.parent().map(|parent| parent.hwnd()),
			Self::dialog_proc,
			// Pass pointer to Self.
			// At this moment, the parent struct is already created and pinned.
			Some(self as *const _ as _),
		).unwrap();

		ret as _
	}

	pub(in crate::gui) fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()> + Send + 'static,
	{
		self.base.spawn_new_thread(func);
	}

	pub(in crate::gui) fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()> + Send + 'static
	{
		self.base.run_ui_thread(func);
	}

	extern "system" fn dialog_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize) -> isize
	{
		let wm_any = WndMsg::new(msg, wparam, lparam);
		Self::dialog_proc_proc(hwnd, wm_any)
			.unwrap_or_else(|err| { post_quit_error(wm_any, err); true as _ })
	}

	fn dialog_proc_proc(hwnd: HWND, wm_any: WndMsg) -> ErrResult<isize> {
		let ptr_self = match wm_any.msg_id {
			co::WM::INITDIALOG => { // first message being handled
				let wm_idlg = wm::InitDialog::from_generic_wm(wm_any);
				let ptr_self = wm_idlg.additional_data as *mut Self;
				hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, ptr_self as _); // store
				let ref_self = unsafe { &mut *ptr_self };
				unsafe { ref_self.base.set_hwnd(hwnd); } // store HWND in struct field
				ptr_self
			},
			_ => hwnd.GetWindowLongPtr(co::GWLP::DWLP_USER) as *mut Self, // retrieve
		};

		// If no pointer stored, then no processing is done.
		// Prevents processing before WM_INITDIALOG and after WM_NCDESTROY.
		if ptr_self.is_null() {
			return Ok(hwnd.DefWindowProc(wm_any));
		}

		// Execute privileged closures.
		let ref_self = unsafe { &mut *ptr_self };
		ref_self.base.process_privileged_messages(wm_any)?;

		if wm_any.msg_id == co::WM::INITDIALOG {
			// Child controls are created in privileged closures, so we set the
			// system font only now.
			ref_self.base.hwnd().SendMessage(wm::SetFont { // on the window itself
				hfont: ui_font(),
				redraw: false,
			});
			ref_self.base.hwnd().EnumChildWindows(|hchild| {
				hchild.SendMessage(wm::SetFont { // on each child control
					hfont: ui_font(),
					redraw: false,
				});
				true
			});
		}

		// Execute user closure, if any.
		let process_result = ref_self.base.process_user_message(wm_any)?;

		if wm_any.msg_id == co::WM::NCDESTROY { // always check
			hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, 0); // clear passed pointer
			unsafe { ref_self.base.set_hwnd(HWND::NULL); } // clear stored HWND
		}

		Ok(match process_result {
			ProcessResult::HandledWithRet(res) => res,
			ProcessResult::HandledWithoutRet => 1, // TRUE
			ProcessResult::NotHandled => 0, // FALSE
		})
	}
}
