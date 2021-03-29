use std::any::Any;
use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::{AtomStr, IdMenu};
use crate::funcs::PostQuitMessage;
use crate::gui::base::Base;
use crate::gui::events::{ProcessResult, WindowEvents};
use crate::gui::immut::Immut;
use crate::gui::traits::Child;
use crate::handles::HWND;
use crate::msg::WndMsg;
use crate::structs::{POINT, SIZE};
use crate::WString;

static mut BASE_SUBCLASS_ID: usize = 0;

/// Variant field for child controls: creation options or just a control ID.
pub enum OptsId<Op> {
	/// The control will be created with
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	Wnd(Op),
	/// The control belongs to a dialog and will be attached with
	/// [`GetDlgItem`](crate::HWND::GetDlgItem).
	Dlg(u16),
}

//------------------------------------------------------------------------------

/// Base to all native child controls.
pub struct NativeControlBase(Immut<Obj>);

struct Obj { // actual fields of NativeControlBase
	hwnd: HWND,
	ptr_parent: NonNull<Base>,
	subclass_events: WindowEvents, // for control subclassing
}

impl Child for NativeControlBase {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl NativeControlBase {
	pub fn new(parent_ref: &Base) -> NativeControlBase {
		Self(
			Immut::new(
				Obj {
					hwnd: unsafe { HWND::null_handle() },
					ptr_parent: NonNull::from(parent_ref), // ref implicitly converted to pointer
					subclass_events: WindowEvents::new(),
				},
			),
		)
	}

	pub fn hwnd_ref(&self) -> &HWND {
		&self.0.hwnd
	}

	pub fn parent_ref(&self) -> &Base {
		unsafe { self.0.ptr_parent.as_ref() }
	}

	pub fn on_subclass(&self) -> &WindowEvents {
		if !self.0.hwnd.is_null() {
			panic!("Cannot add subclass events after the control is created.");
		} else if !self.parent_ref().hwnd_ref().is_null() {
			panic!("Cannot add subclass events after the parent window is created.");
		}
		&self.0.subclass_events
	}

	pub fn create_window(
		&self,
		class_name: &str,
		title: Option<&str>,
		pos: POINT, sz: SIZE,
		ctrl_id: u16,
		ex_styles: co::WS_EX,
		styles: co::WS) -> WinResult<HWND>
	{
		let hparent = *self.parent_ref().hwnd_ref();

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

	pub fn create_dlg(&self, ctrl_id: u16) -> WinResult<HWND> {
		if self.parent_ref().create_wm() != co::WM::INITDIALOG {
			panic!("Parent window is not a dialog, cannot create control.");
		}

		let hparent = *self.parent_ref().hwnd_ref();

		if !self.0.hwnd.is_null() {
			panic!("Cannot create control twice.");
		} else if hparent.is_null() {
			panic!("Cannot create control before parent window is created.");
		}

		self.0.as_mut().hwnd = hparent.GetDlgItem(ctrl_id as i32)?;
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
				self as *const Self as usize, // pass pointer to self
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
			let mut maybe_processed = ProcessResult::NotHandled;

			if !ptr_self.is_null() {
				let ref_self = unsafe { &mut *ptr_self };
				if !ref_self.0.hwnd.is_null() {
					maybe_processed = ref_self.0.subclass_events.process_effective_message(wm_any);
				}
			}

			if msg == co::WM::NCDESTROY { // always check
				hwnd.RemoveWindowSubclass(Self::subclass_proc, subclass_id)?;
			}

			Ok(match maybe_processed {
				ProcessResult::HandledWithRet(res) => res.into(),
				ProcessResult::HandledWithoutRet => 0,
				ProcessResult::NotHandled => hwnd.DefSubclassProc(wm_any).into(),
			})
		}
		(hwnd, msg, wparam, lparam)
			.unwrap_or_else(|err| { PostQuitMessage(err); 0 })
	}
}
