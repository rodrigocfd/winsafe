use std::ptr::NonNull;

use crate::co;
use crate::gui::base::Base;
use crate::gui::events::{ProcessResult, WindowEvents};
use crate::gui::privs::post_quit_error;
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::kernel::decl::ErrResult;
use crate::msg::WndMsg;
use crate::prelude::{ComctlHwnd, Handle, UserHwnd};
use crate::user::decl::{AtomStr, HWND, IdMenu, POINT, SIZE};

static mut BASE_SUBCLASS_ID: usize = 0;

/// Variant field for child controls: creation options or just a control ID.
pub enum OptsId<P> {
	/// The control will be created with
	/// [`HWND::CreateWindowEx`](crate::prelude::UserHwnd::CreateWindowEx). We
	/// keep a lot of options for manual creation.
	Wnd(P),
	/// The control belongs to a dialog and will be attached with
	/// [`HWND::GetDlgItem`](crate::prelude::UserHwnd::GetDlgItem). We keep just
	/// the control ID from the dialog resource.
	Dlg(u16),
}

//------------------------------------------------------------------------------

/// Base to all native child controls.
pub(in crate::gui) struct BaseNativeControl {
	hwnd: VeryUnsafeCell<HWND>,
	parent_ptr: NonNull<Base>, // base of WindowControl, WindowMain or WindowModal
	subclass_events: WindowEvents, // for control subclassing
}

impl BaseNativeControl {
	pub(in crate::gui) fn new(parent: &Base) -> Self {
		Self {
			hwnd: VeryUnsafeCell::new(HWND::NULL),
			parent_ptr: NonNull::from(parent),
			subclass_events: WindowEvents::new(),
		}
	}

	pub(in crate::gui) fn hwnd(&self) -> HWND {
		*self.hwnd
	}

	pub(in crate::gui) fn parent(&self) -> &Base {
		unsafe { self.parent_ptr.as_ref() }
	}

	pub(in crate::gui) fn on_subclass(&self) -> &WindowEvents {
		if !self.hwnd().is_null() {
			panic!("Cannot add subclass events after control creation.");
		} else if !self.parent().hwnd().is_null() {
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
		styles: co::WS)
	{
		let hparent = self.parent().hwnd();

		if !self.hwnd().is_null() {
			panic!("Cannot create control twice.");
		} else if hparent.is_null() {
			panic!("Cannot create control before parent window creation.");
		}

		*self.hwnd.as_mut() = unsafe {
			HWND::CreateWindowEx(
				ex_styles,
				AtomStr::from_str(class_name),
				title, styles,
				pos, sz,
				Some(hparent),
				IdMenu::Id(ctrl_id),
				hparent.hinstance(),
				None,
			).unwrap()
		};

		self.install_subclass_if_needed();
	}

	pub(in crate::gui) fn create_dlg(&self, ctrl_id: u16) {
		if !self.parent().is_dialog() {
			panic!("Parent window is not a dialog, cannot create control.");
		}

		let hparent = self.parent().hwnd();

		if !self.hwnd().is_null() {
			panic!("Cannot create control twice.");
		} else if hparent.is_null() {
			panic!("Cannot create control before parent window creation.");
		}

		*self.hwnd.as_mut() = hparent.GetDlgItem(ctrl_id).unwrap();
		self.install_subclass_if_needed();
	}

	fn install_subclass_if_needed(&self) {
		if !self.subclass_events.is_empty() {
			let subclass_id = unsafe {
				BASE_SUBCLASS_ID += 1;
				BASE_SUBCLASS_ID
			};

			self.hwnd().SetWindowSubclass(
				Self::subclass_proc, subclass_id,
				self as *const _ as _, // pass pointer to self
			).unwrap()
		}
	}

	extern "system" fn subclass_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize,
		subclass_id: usize, ref_data: usize) -> isize
	{
		Self::subclass_proc_proc(hwnd, msg, wparam, lparam, subclass_id, ref_data)
			.unwrap_or_else(|err| { post_quit_error(err); 0 })
	}

	fn subclass_proc_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize,
		subclass_id: usize, ref_data: usize) -> ErrResult<isize>
	{
		let ptr_self = ref_data as *mut Self; // retrieve
		let wm_any = WndMsg { msg_id: msg, wparam, lparam };
		let mut process_result = ProcessResult::NotHandled;

		if !ptr_self.is_null() {
			let ref_self = unsafe { &mut *ptr_self };
			if !ref_self.hwnd().is_null() {
				process_result = ref_self.subclass_events.process_one_message(wm_any)?;
			}
		}

		if msg == co::WM::NCDESTROY { // always check
			hwnd.RemoveWindowSubclass(Self::subclass_proc, subclass_id)?;
		}

		Ok(match process_result {
			ProcessResult::HandledWithRet(res) => res,
			ProcessResult::HandledWithoutRet => 0,
			ProcessResult::NotHandled => hwnd.DefSubclassProc(wm_any).into(),
		})
	}
}
