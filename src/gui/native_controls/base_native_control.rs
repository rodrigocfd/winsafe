use std::ptr::NonNull;

use crate::co;
use crate::gui::base::Base;
use crate::gui::events::{ProcessResult, WindowEvents};
use crate::gui::privs::post_quit_error;
use crate::kernel::decl::{AnyResult, SysResult};
use crate::kernel::privs::as_mut;
use crate::msg::WndMsg;
use crate::prelude::{comctl_Hwnd, Handle, user_Hwnd};
use crate::user::decl::{AtomStr, HWND, IdMenu, POINT, SIZE};

static mut BASE_SUBCLASS_ID: usize = 0;

/// Variant field for child controls: creation options or just a control ID.
pub enum OptsId<P> {
	/// The control will be created with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx). We
	/// keep a lot of options for manual creation.
	Wnd(P),
	/// The control belongs to a dialog and will be attached with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem). We keep
	/// just the control ID from the dialog resource.
	Dlg(u16),
}

//------------------------------------------------------------------------------

/// Base to all native child controls.
///
/// Owns the window procedure for all subclassed native child controls.
pub(in crate::gui) struct BaseNativeControl {
	hwnd: HWND,
	parent_ptr: NonNull<Base>, // base of WindowControl, WindowMain or WindowModal
	subclass_events: WindowEvents, // for control subclassing
}

impl BaseNativeControl {
	pub(in crate::gui) fn new(parent: &Base) -> Self {
		if *parent.hwnd() != HWND::NULL {
			panic!("Cannot create a child control after the parent window is created.");
		}

		Self {
			hwnd: HWND::NULL,
			parent_ptr: NonNull::from(parent),
			subclass_events: WindowEvents::new(),
		}
	}

	pub(in crate::gui) fn hwnd(&self) -> &HWND {
		&self.hwnd
	}

	pub(in crate::gui) fn parent(&self) -> &Base {
		unsafe { self.parent_ptr.as_ref() }
	}

	pub(in crate::gui) fn on_subclass(&self) -> &WindowEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add subclass events after control creation.");
		} else if *self.parent().hwnd() != HWND::NULL {
			panic!("Cannot add subclass events after parent window creation.");
		}
		&self.subclass_events
	}

	pub(in crate::gui) fn create_window(&self,
		class_name: &str,
		title: Option<&str>,
		pos: POINT,
		sz: SIZE,
		ctrl_id: u16,
		ex_styles: co::WS_EX,
		styles: co::WS) -> SysResult<()>
	{
		let hparent = self.parent().hwnd();

		if *self.hwnd() != HWND::NULL {
			panic!("Cannot create control twice.");
		} else if *hparent == HWND::NULL {
			panic!("Cannot create control before parent window creation.");
		}

		unsafe {
			*as_mut(&self.hwnd) = HWND::CreateWindowEx(
				ex_styles,
				AtomStr::from_str(class_name),
				title, styles,
				pos, sz,
				Some(hparent),
				IdMenu::Id(ctrl_id),
				&hparent.hinstance(),
				None,
			)?;
		}

		self.install_subclass_if_needed()?;
		Ok(())
	}

	pub(in crate::gui) fn create_dlg(&self, ctrl_id: u16) -> SysResult<()> {
		if !self.parent().is_dialog() {
			panic!("Parent window is not a dialog, cannot create control.");
		}

		let hparent = self.parent().hwnd();

		if *self.hwnd() != HWND::NULL {
			panic!("Cannot create control twice.");
		} else if *hparent == HWND::NULL {
			panic!("Cannot create control before parent window creation.");
		}

		*unsafe { as_mut(&self.hwnd) } = hparent.GetDlgItem(ctrl_id)?;
		self.install_subclass_if_needed()?;
		Ok(())
	}

	fn install_subclass_if_needed(&self) -> SysResult<()> {
		if !self.subclass_events.is_empty() {
			let subclass_id = unsafe {
				BASE_SUBCLASS_ID += 1;
				BASE_SUBCLASS_ID
			};

			unsafe {
				self.hwnd().SetWindowSubclass(
					Self::subclass_proc, subclass_id,
					self as *const _ as _, // pass pointer to self
				)?;
			}
		}
		Ok(())
	}

	extern "system" fn subclass_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize,
		subclass_id: usize, ref_data: usize) -> isize
	{
		let wm_any = WndMsg::new(msg, wparam, lparam);
		Self::subclass_proc_proc(hwnd, wm_any, subclass_id, ref_data)
			.unwrap_or_else(|err| { post_quit_error(wm_any, err); 0 })
	}

	fn subclass_proc_proc(
		hwnd: HWND, wm_any: WndMsg,
		subclass_id: usize, ref_data: usize) -> AnyResult<isize>
	{
		let ptr_self = ref_data as *mut Self; // retrieve
		let mut process_result = ProcessResult::NotHandled;

		if !ptr_self.is_null() {
			let ref_self = unsafe { &mut *ptr_self };
			if *ref_self.hwnd() != HWND::NULL {
				process_result = ref_self.subclass_events.process_one_message(wm_any)?;
			}
		}

		if wm_any.msg_id == co::WM::NCDESTROY { // always check
			hwnd.RemoveWindowSubclass(Self::subclass_proc, subclass_id)?;
			if !ptr_self.is_null() {
				let ref_self = unsafe { &mut *ptr_self };
				ref_self.subclass_events.clear(); // prevents circular references
			}
		}

		Ok(match process_result {
			ProcessResult::HandledWithRet(res) => res,
			ProcessResult::HandledWithoutRet => 0,
			ProcessResult::NotHandled => hwnd.DefSubclassProc(wm_any).into(),
		})
	}
}
