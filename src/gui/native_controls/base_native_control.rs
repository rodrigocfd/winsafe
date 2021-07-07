use std::any::Any;
use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::{AtomStr, IdMenu};
use crate::funcs::PostQuitMessage;
use crate::gui::base::Base;
use crate::gui::events::{ProcessResult, WindowEvents};
use crate::gui::traits::Child;
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::handles::HWND;
use crate::msg::WndMsg;
use crate::structs::{POINT, SIZE};
use crate::various::WString;

static mut BASE_SUBCLASS_ID: usize = 0;

/// Variant field for child controls: creation options or just a control ID.
pub enum OptsId<Op> {
	/// The control will be created with
	/// [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	Wnd(Op),
	/// The control belongs to a dialog and will be attached with
	/// [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	Dlg(u16),
}

//------------------------------------------------------------------------------

/// Base to all native child controls.
pub(in crate::gui) struct BaseNativeControl(VeryUnsafeCell<Obj>);

struct Obj { // actual fields of BaseNativeControl
	hwnd: HWND,
	ptr_parent: NonNull<Base>,
	subclass_events: WindowEvents, // for control subclassing
}

impl Child for BaseNativeControl {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl BaseNativeControl {
	pub(in crate::gui) fn new(parent_base_ref: &Base) -> BaseNativeControl {
		Self(
			VeryUnsafeCell::new(
				Obj {
					hwnd: HWND::NULL,
					ptr_parent: NonNull::from(parent_base_ref),
					subclass_events: WindowEvents::new(),
				},
			),
		)
	}

	pub(in crate::gui) fn hwnd_ref(&self) -> &HWND {
		&self.0.hwnd
	}

	pub(in crate::gui) fn parent_base_ref(&self) -> &Base {
		unsafe { self.0.ptr_parent.as_ref() }
	}

	pub(in crate::gui) fn on_subclass(&self) -> &WindowEvents {
		if !self.0.hwnd.is_null() {
			panic!("Cannot add subclass events after the control is created.");
		} else if !self.parent_base_ref().hwnd_ref().is_null() {
			panic!("Cannot add subclass events after the parent window is created.");
		}
		&self.0.subclass_events
	}

	pub(in crate::gui) fn create_window(
		&self,
		class_name: &str,
		title: Option<&str>,
		pos: POINT,
		sz: SIZE,
		ctrl_id: u16,
		ex_styles: co::WS_EX,
		styles: co::WS) -> WinResult<HWND>
	{
		let hparent = *self.parent_base_ref().hwnd_ref();

		if !self.0.hwnd.is_null() {
			panic!("Cannot create control twice.");
		} else if hparent.is_null() {
			panic!("Cannot create control before parent window is created.");
		}

		self.0.as_mut().hwnd = HWND::CreateWindowEx(
			ex_styles,
			AtomStr::Str(WString::from_str(class_name)),
			title, styles,
			pos.x, pos.y, sz.cx, sz.cy,
			Some(hparent),
			IdMenu::Id(ctrl_id),
			hparent.hinstance(),
			None,
		)?;

		self.install_subclass_if_needed()?;
		Ok(self.0.hwnd)
	}

	pub(in crate::gui) fn create_dlg(&self, ctrl_id: u16) -> WinResult<HWND> {
		if self.parent_base_ref().creation_wm() != co::WM::INITDIALOG {
			panic!("Parent window is not a dialog, cannot create control.");
		}

		let hparent = *self.parent_base_ref().hwnd_ref();

		if !self.0.hwnd.is_null() {
			panic!("Cannot create control twice.");
		} else if hparent.is_null() {
			panic!("Cannot create control before parent window is created.");
		}

		self.0.as_mut().hwnd = hparent.GetDlgItem(ctrl_id)?;
		self.install_subclass_if_needed()?;
		Ok(self.0.hwnd)
	}

	fn install_subclass_if_needed(&self) -> WinResult<()> {
		if !self.0.subclass_events.is_empty() {
			let subclass_id = unsafe {
				BASE_SUBCLASS_ID += 1;
				BASE_SUBCLASS_ID
			};

			self.0.hwnd.SetWindowSubclass(
				Self::subclass_proc, subclass_id,
				self as *const _ as _, // pass pointer to self
			)
		} else {
			Ok(())
		}
	}

	extern "system" fn subclass_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize,
		subclass_id: usize, ref_data: usize) -> isize
	{
		|hwnd: HWND, msg, wparam, lparam| -> WinResult<isize>
		{
			let ptr_self = ref_data as *mut Self; // retrieve
			let wm_any = WndMsg { msg_id: msg, wparam, lparam };
			let mut process_result = ProcessResult::NotHandled;

			if !ptr_self.is_null() {
				let ref_self = unsafe { &mut *ptr_self };
				if !ref_self.0.hwnd.is_null() {
					process_result = ref_self.0.subclass_events.process_effective_message(wm_any);
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
		(hwnd, msg, wparam, lparam)
			.unwrap_or_else(|err| { PostQuitMessage(err); 0 })
	}
}
