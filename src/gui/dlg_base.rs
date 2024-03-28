use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::msg::*;
use crate::prelude::*;

/// Base to all dialog windows.
///
/// Owns the window procedure for all dialog windows.
pub(in crate::gui) struct DlgBase {
	base: Base,
	dialog_id: u16,
}

impl Drop for DlgBase {
	fn drop(&mut self) {
		if *self.base.hwnd() != HWND::NULL {
			unsafe { self.base.hwnd().SetWindowLongPtr(co::GWLP::DWLP_USER, 0); } // clear passed pointer
		}
	}
}

impl DlgBase {
	pub(in crate::gui) fn new(parent: Option<&Base>, dialog_id: u16) -> Self {
		Self {
			base: Base::new(true, parent),
			dialog_id,
		}
	}

	pub(in crate::gui) const fn base(&self) -> &Base {
		&self.base
	}

	pub(in crate::gui) fn create_dialog_param(&self) -> SysResult<()> {
		if *self.base.hwnd() != HWND::NULL {
			panic!("Cannot create dialog twice.");
		}

		// Our hwnd member is set during WM_INITDIALOG processing; already set
		// when CreateDialogParam returns.
		unsafe {
			self.base.parent_hinstance()?.CreateDialogParam(
				IdStr::Id(self.dialog_id),
				self.base.parent().map(|parent| parent.hwnd()),
				Self::dialog_proc,
				// Pass pointer to Self.
				// At this moment, the parent struct is already created and pinned.
				Some(self as *const _ as _),
			)?;
		}

		Ok(())
	}

	pub(in crate::gui) fn dialog_box_param(&self) -> SysResult<i32> {
		if *self.base.hwnd() != HWND::NULL {
			panic!("Cannot create dialog twice.");
		}

		// Our hwnd member is set during WM_INITDIALOG processing; already set
		// when DialogBoxParam returns.
		let ret = unsafe {
			self.base.parent_hinstance()?.DialogBoxParam(
				IdStr::Id(self.dialog_id),
				self.base.parent().map(|parent| parent.hwnd()),
				Self::dialog_proc,
				// Pass pointer to Self.
				// At this moment, the parent struct is already created and pinned.
				Some(self as *const _ as _),
			)?
		};

		Ok(ret as _)
	}

	extern "system" fn dialog_proc(
		hwnd: HWND,
		msg: co::WM,
		wparam: usize,
		lparam: isize,
	) -> isize
	{
		let wm_any = WndMsg::new(msg, wparam, lparam);
		Self::dialog_proc_proc(hwnd, wm_any)
			.unwrap_or_else(|err| { post_quit_error(wm_any, err); true as _ })
	}

	fn dialog_proc_proc(hwnd: HWND, wm_any: WndMsg) -> AnyResult<isize> {
		let ptr_self = match wm_any.msg_id {
			co::WM::INITDIALOG => { // first message being handled
				let wm_idlg = wm::InitDialog::from_generic_wm(wm_any);
				let ptr_self = wm_idlg.additional_data as *mut Self;
				unsafe { hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, ptr_self as _); } // store
				let ref_self = unsafe { &mut *ptr_self };
				ref_self.base.set_hwnd(unsafe { hwnd.raw_copy() }); // store HWND in struct field
				ptr_self
			},
			_ => hwnd.GetWindowLongPtr(co::GWLP::DWLP_USER) as *mut Self, // retrieve
		};

		// If no pointer stored, then no processing is done.
		// Prevents processing before WM_INITDIALOG and after WM_NCDESTROY.
		if ptr_self.is_null() {
			return Ok(hwnd.DefWindowProc(wm_any));
		}

		// Execute privileged closures, keep track if at least one was executed.
		let ref_self = unsafe { &mut *ptr_self };
		let at_least_one_privileged = ref_self.base.process_privileged_messages(&hwnd, wm_any)?;

		if wm_any.msg_id == co::WM::INITDIALOG {
			// Child controls are created in privileged closures, so we set the
			// system font only now.
			unsafe {
				ref_self.base.hwnd().SendMessage(wm::SetFont { // on the window itself
					hfont: ui_font(),
					redraw: false,
				});
			}
			ref_self.base.hwnd().EnumChildWindows(|hchild| {
				unsafe {
					hchild.SendMessage(wm::SetFont { // on each child control
						hfont: ui_font(),
						redraw: false,
					});
				}
				true
			});
		}

		// Execute user closure, if any.
		let process_result = ref_self.base.process_user_message(wm_any)?;

		// Execute post-user privileged closures, keep track if at least one was executed.
		let at_least_one_privileged_post = ref_self.base.process_privileged_post_messages(&hwnd, wm_any)?;

		if wm_any.msg_id == co::WM::NCDESTROY { // always check
			unsafe { hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, 0); } // clear passed pointer
			ref_self.base.set_hwnd(HWND::NULL); // clear stored HWND
			ref_self.base.clear_events(); // prevents circular references
		}

		Ok(match process_result {
			ProcessResult::HandledWithRet(res) => res,
			ProcessResult::HandledWithoutRet => 1, // TRUE
			ProcessResult::NotHandled => if at_least_one_privileged || at_least_one_privileged_post {
				1 // TRUE
			} else {
				0 // FALSE
			},
		})
	}
}
