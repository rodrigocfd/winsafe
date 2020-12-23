use std::error::Error;
use std::ffi::c_void;

use crate::co;
use crate::enums::{AtomStr, IdMenu};
use crate::funcs::{RegisterClassEx, SetLastError};
use crate::gui::events::{Events, ProcessResult};
use crate::handles::{HINSTANCE, HWND};
use crate::internal_defs::str_dyn_error;
use crate::msg::{Wm, WmAny};
use crate::structs::{ATOM, POINT, SIZE, WNDCLASSEX};
use crate::Utf16;

/// Base to all ordinary windows.
#[derive(Clone)]
pub struct WindowBase {
	hwnd: HWND,
	events: Events,
}

impl WindowBase {
	pub fn new() -> WindowBase {
		Self {
			hwnd: HWND::default(),
			events: Events::new(),
		}
	}

	pub fn hwnd(&self) -> HWND {
		self.hwnd
	}

	pub fn on(&self) -> Events {
		self.events.clone()
	}

	pub fn register_class(
		&self, wcx: &mut WNDCLASSEX) -> Result<ATOM, co::ERROR>
	{
		wcx.lpfnWndProc = Some(Self::window_proc);
		SetLastError(co::ERROR::SUCCESS);

		match RegisterClassEx(&wcx) {
			Ok(atom) => Ok(atom), // window class registered successfully
			Err(err) => match err {
				co::ERROR::CLASS_ALREADY_EXISTS => {
					// https://devblogs.microsoft.com/oldnewthing/20150429-00/?p=44984
					// https://devblogs.microsoft.com/oldnewthing/20041011-00/?p=37603
					// Retrieve ATOM of existing window class.
					Ok(wcx.hInstance.GetClassInfoEx(
						&Utf16::from_utf16_nullt(wcx.lpszClassName).to_string(), wcx)?)
				},
				err => Err(err),
			}
		}
	}

	pub fn create_window(
		&self,
		hinst: HINSTANCE,
		parent: Option<HWND>,
		class_name: &str,
		title: Option<&str>,
		hmenu: IdMenu,
		pos: POINT, sz: SIZE,
		ex_styles: co::WS_EX, styles: co::WS) -> Result<HWND, Box<dyn Error>>
	{
		if self.hwnd.is_null() {
			return Err(str_dyn_error("Cannot create a window twice."));
		}

		match HWND::CreateWindowEx(ex_styles,
			AtomStr::Str(String::from(class_name)), title,
			styles, pos.x, pos.y, sz.cx, sz.cy, parent, hmenu, hinst,
			Some(self as *const WindowBase as *const c_void)) // pass pointer to self
		{
			Ok(hwnd) => Ok(hwnd), // our hwnd member is set during WM_NCCREATE processing, already set at this point
			Err(err) => Err(Box::new(err)),
		}
	}

	unsafe extern "system" fn window_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize) -> isize
	{
		let wm_any = WmAny { msg, wparam, lparam };

		let ptr_self = match wm_any.message() {
			Wm::NcCreate(wm) => { // first message being handled
				let ptr_self = wm.createstruct.lpCreateParams as *mut Self;
				let ref_self = ptr_self.as_mut().unwrap();
				ref_self.hwnd.SetWindowLongPtr(co::GWLP::USERDATA, ptr_self as isize); // store
				ref_self.hwnd = hwnd; // store HWND in struct field
				ptr_self
			},
			_ => {
				hwnd.GetWindowLongPtr(co::GWLP::USERDATA) as *mut Self // retrieve
			},
		};

		// If no pointer stored, then no processing is done.
		// Prevents processing before WM_NCCREATE and after WM_NCDESTROY.
		if ptr_self.is_null() {
			return hwnd.DefWindowProc(wm_any);
		}

		// Execute user handler, if any.
		let ref_self = ptr_self.as_mut().unwrap();
		let maybe_processed = ref_self.events.process_message(wm_any);

		if let Wm::NcDestroy(_) = wm_any.message() { // always check
			hwnd.SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
			ref_self.hwnd = HWND::default(); // clear stored HWND
		}

		match maybe_processed {
			ProcessResult::HandledWithRet(ret) => ret,
			ProcessResult::HandledWithoutRet => 0,
			ProcessResult::NotHandled => hwnd.DefWindowProc(wm_any),
		}
	}
}