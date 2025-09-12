use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::msg::*;
use crate::prelude::*;

/// Base to all dialog windows.
///
/// Owns the window procedure for all dialog windows.
pub(in crate::gui) struct DlgBase {
	base: BaseWnd,
	dlg_id: u16,
}

impl Drop for DlgBase {
	fn drop(&mut self) {
		if *self.base.hwnd() != HWND::NULL {
			unsafe {
				self.base.hwnd().SetWindowLongPtr(co::GWLP::DWLP_USER, 0); // clear passed pointer
			}
		}
	}
}

impl DlgBase {
	#[must_use]
	pub(in crate::gui) fn new(dlg_id: u16) -> Self {
		Self { base: BaseWnd::new(WndTy::Dlg), dlg_id }
	}

	#[must_use]
	pub(in crate::gui) const fn base(&self) -> &BaseWnd {
		&self.base
	}

	pub(in crate::gui) fn create_dialog_param(&self, hinst: &HINSTANCE, hparent: Option<&HWND>) {
		if *self.base.hwnd() != HWND::NULL {
			panic!("Cannot create dialog twice.");
		}

		unsafe {
			// The hwnd member is saved in WM_INITDIALOG processing in dlg_proc.
			hinst.CreateDialogParam(
				IdStr::Id(self.dlg_id),
				hparent,
				Self::dlg_proc,
				Some(self as *const _ as _), // pointer to object itself
			)
		}
		.expect(DONTFAIL);
	}
	pub(in crate::gui) fn dialog_box_param(&self, hinst: &HINSTANCE, hparent: Option<&HWND>) {
		if *self.base.hwnd() != HWND::NULL {
			panic!("Cannot create dialog twice.");
		}

		unsafe {
			// The hwnd member is saved in WM_INITDIALOG processing in dlg_proc.
			hinst.DialogBoxParam(
				IdStr::Id(self.dlg_id),
				hparent,
				Self::dlg_proc,
				Some(self as *const _ as _), // pointer to object itself
			)
		}
		.expect(DONTFAIL);
	}

	pub(in crate::gui) fn set_icon(&self, hinst: &HINSTANCE, icon_id: u16) -> SysResult<()> {
		// If an icon ID was specified, load it from the resources.
		// Resource icons are automatically released by the system.
		unsafe {
			self.base.hwnd().SendMessage(wm::SetIcon {
				hicon: hinst
					.LoadImageIcon(IdOicStr::Id(icon_id), SIZE::with(16, 16), co::LR::DEFAULTCOLOR)?
					.leak(),
				size: co::ICON_SZ::SMALL,
			});

			self.base.hwnd().SendMessage(wm::SetIcon {
				hicon: hinst
					.LoadImageIcon(IdOicStr::Id(icon_id), SIZE::with(32, 32), co::LR::DEFAULTCOLOR)?
					.leak(),
				size: co::ICON_SZ::BIG,
			});
		}
		Ok(())
	}

	extern "system" fn dlg_proc(hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize) -> isize {
		let wm_any = WndMsg::new(msg, wparam, lparam);
		Self::dlg_proc_proc(hwnd, wm_any).unwrap_or_else(|err| {
			quit_error::post_quit_error(wm_any, err);
			true as _
		})
	}

	fn dlg_proc_proc(hwnd: HWND, p: WndMsg) -> AnyResult<isize> {
		let ptr_self = match p.msg_id {
			co::WM::INITDIALOG => {
				// first message being handled
				let msg = unsafe { wm::InitDialog::from_generic_wm(p) };
				let ptr_self = msg.additional_data as *const Self;
				unsafe {
					hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, ptr_self as _); // store
				}
				let ref_self = unsafe { &*ptr_self };
				ref_self.base.set_hwnd(unsafe { hwnd.raw_copy() }); // store HWND in struct field
				ptr_self
			},
			_ => hwnd.GetWindowLongPtr(co::GWLP::DWLP_USER) as *const Self, // retrieve
		};

		// If no pointer stored, then no processing is done.
		// Prevents processing before WM_INITDIALOG and after WM_NCDESTROY.
		if ptr_self.is_null() {
			return Ok(0); // FALSE
		}
		let ref_self = unsafe { &*ptr_self };

		// Execute before-user closures, keep track if at least one was executed.
		// Execute user closure, if any.
		// Execute post-user closures, keep track if at least one was executed.
		let (at_least_one_before, user_ret, at_least_one_after) = ref_self.base.process_msgs(p)?;

		// Always check.
		if p.msg_id == co::WM::NCDESTROY {
			unsafe {
				hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, 0); // clear passed pointer
			}
			ref_self.base.set_hwnd(HWND::NULL); // clear stored HWND
			ref_self.base.clear_messages(); // prevents circular references
		}

		if let Some(user_ret) = user_ret {
			match p.msg_id {
				co::WM::GETDLGCODE | co::WM::SETCURSOR => unsafe {
					hwnd.SetWindowLongPtr(co::GWLP::DWLP_MSGRESULT, user_ret); // special case
					Ok(1) // TRUE
				},
				_ => Ok(user_ret),
			}
		} else if at_least_one_before || at_least_one_after {
			Ok(1) // TRUE
		} else {
			Ok(0) // FALSE
		}
	}
}
