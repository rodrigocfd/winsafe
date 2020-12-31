use std::error::Error;
use std::ffi::c_void;

use crate::co;
use crate::enums::{AtomStr, IdMenu};
use crate::gui::events::{MsgEvents, ProcessResult};
use crate::handles::{HINSTANCE, HWND};
use crate::msg::Wm;
use crate::priv_funcs::str_dyn_error;
use crate::structs::{POINT, SIZE};
use crate::WString;

static mut BASE_CTRL_ID: u16 = 20_000; // in-between Visual Studio Resource Editor values
static mut BASE_SUBCLASS_ID: usize = 0;

/// Base to all native child controls.
pub struct NativeControlBase {
	hwnd: HWND,
	ctrl_id: u16, // cannot be changed
	subclass_events: MsgEvents,
	ptr_parent_hwnd: *const HWND, // used only in control creation
}

impl NativeControlBase {
	pub fn auto_ctrl_id() -> u16 {
		unsafe {
			let new_id = BASE_CTRL_ID;
			BASE_CTRL_ID += 1;
			new_id
		}
	}

	pub fn new_with_id(ctrl_id: u16, parent_hwnd: &HWND) -> NativeControlBase {
		Self {
			hwnd: unsafe { HWND::null_handle() },
			ctrl_id,
			subclass_events: MsgEvents::new(),
			ptr_parent_hwnd: parent_hwnd, // convert reference to pointer
		}
	}

	pub fn is_parent_created(&self) -> bool {
		let parent_hwnd = unsafe { *self.ptr_parent_hwnd };
		!parent_hwnd.is_null()
	}

	pub fn hwnd(&self) -> &HWND {
		&self.hwnd
	}

	pub fn ctrl_id(&self) -> u16 {
		self.ctrl_id
	}

	pub fn on_subclass(&self) -> &MsgEvents {
		if !self.hwnd.is_null() {
			panic!("Cannot add subclass events after the control is created.");
		} else if self.is_parent_created() {
			panic!("Cannot add subclass events after the parent window is created.");
		}
		&self.subclass_events
	}

	pub fn create_window(
		&mut self,
		class_name: &str,
		title: Option<&str>,
		pos: POINT, sz: SIZE,
		ex_styles: co::WS_EX,
		styles: co::WS) -> Result<HWND, Box<dyn Error>>
	{
		if !self.hwnd.is_null() {
			panic!("Cannot create control twice.");
		} else if !self.is_parent_created() {
			panic!("Cannot create control before parent window is created.");
		}

		let parent_hwnd = unsafe { *self.ptr_parent_hwnd };

		self.hwnd = HWND::CreateWindowEx(
			ex_styles,
			AtomStr::Str(WString::from_str(class_name)),
			title, styles,
			pos.x, pos.y, sz.cx, sz.cy,
			Some(parent_hwnd),
			IdMenu::Id(self.ctrl_id),
			unsafe {
				HINSTANCE::from_ptr(
					parent_hwnd.GetWindowLongPtr(co::GWLP::HINSTANCE) as *mut c_void
				)
			},
			None,
		)?;

		self.install_subclass_if_needed()?;
		Ok(self.hwnd)
	}

	fn install_subclass_if_needed(&self) -> Result<(), Box<dyn Error>> {
		if !self.subclass_events.is_empty() {
			let subclass_id = unsafe {
				BASE_SUBCLASS_ID += 1;
				BASE_SUBCLASS_ID
			};

			self.hwnd.SetWindowSubclass(
				Self::subclass_proc, subclass_id,
				self as *const Self as usize, // pass pointer to self
			)
			.map_err(|_| str_dyn_error("SetWindowSubclass failed."))
		} else {
			Ok(())
		}
	}

	extern "system" fn subclass_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize,
		subclass_id: usize, ref_data: usize) -> isize
	{
		let ptr_self = ref_data as *mut Self; // retrieve
		let wm_any = Wm { msg_id: msg, wparam, lparam };
		let mut maybe_processed = ProcessResult::NotHandled;

		if !ptr_self.is_null() {
			let ref_self = unsafe { &mut *ptr_self };
			if !ref_self.hwnd.is_null() {
				maybe_processed = ref_self.subclass_events.process_message(wm_any);
			}
		}

		if msg == co::WM::NCDESTROY { // always check
			hwnd.RemoveWindowSubclass(Self::subclass_proc, subclass_id).ok();
		}

		match maybe_processed {
			ProcessResult::HandledWithRet(res) => res.into(),
			ProcessResult::HandledWithoutRet => 0,
			ProcessResult::NotHandled => hwnd.DefSubclassProc(wm_any).into(),
		}
	}
}
