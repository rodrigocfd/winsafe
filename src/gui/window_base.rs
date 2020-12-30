use crate::co;
use crate::enums::{AtomStr, IdMenu};
use crate::funcs::{RegisterClassEx, SetLastError};
use crate::gui::events::{MsgEvents, ProcessResult};
use crate::handles::{HINSTANCE, HWND};
use crate::msg::{Wm, WmNcCreate};
use crate::structs::{ATOM, POINT, SIZE, WNDCLASSEX};
use crate::WString;

/// Base to all ordinary windows.
pub struct WindowBase {
	hwnd: HWND,
	events: MsgEvents,
}

impl Drop for WindowBase {
	fn drop(&mut self) {
		if !self.hwnd.is_null() {
			self.hwnd.SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
		}
	}
}

impl WindowBase {
	pub fn new() -> WindowBase {
		Self {
			hwnd: unsafe { HWND::null_handle() },
			events: MsgEvents::new(),
		}
	}

	pub fn hwnd(&self) -> &HWND {
		&self.hwnd
	}

	pub fn on(&self) -> MsgEvents {
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
					let hinst = wcx.hInstance;
					Ok(hinst.GetClassInfoEx(&wcx.lpszClassName(), wcx)?)
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
		pos: POINT,
		sz: SIZE,
		ex_styles: co::WS_EX,
		styles: co::WS) -> Result<HWND, co::ERROR>
	{
		// Our hwnd member is set during WM_NCCREATE processing, already set when
		// CreateWindowEx returns.
		HWND::CreateWindowEx(
			ex_styles,
			AtomStr::Str(WString::from_str(class_name)),
			title, styles,
			pos.x, pos.y, sz.cx, sz.cy,
			parent, hmenu, hinst,
			Some(self as *const WindowBase as isize), // pass pointer to self
		)
	}

	/// Generates a hash string from current fields, so it must called after all
	/// the fields are set.
	pub fn generate_wcx_class_name_hash(wcx: &WNDCLASSEX) -> WString {
		WString::from_str(
			&format!("WNDCLASS.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}",
				wcx.style,
				match wcx.lpfnWndProc {
					Some(p) => p as usize,
					None => 0,
				},
				wcx.cbClsExtra, wcx.cbWndExtra,
				wcx.hInstance, wcx.hIcon, wcx.hCursor, wcx.hbrBackground,
				wcx.lpszMenuName().as_ptr() as usize, wcx.hIconSm,
			),
		)
	}

	extern "system" fn window_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize) -> isize
	{
		let wm_any = Wm { msg_id: msg, wparam, lparam };

		let ptr_self = match msg {
			co::WM::NCCREATE => { // first message being handled
				let wm_ncc: WmNcCreate = wm_any.into();
				let ptr_self = wm_ncc.createstruct.lpCreateParams as *mut Self;
				hwnd.SetWindowLongPtr(co::GWLP::USERDATA, ptr_self as isize); // store
				let ref_self = unsafe { &mut *ptr_self };
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
			return hwnd.DefWindowProc(wm_any).into();
		}

		// Execute user closure, if any.
		let ref_self = unsafe { &mut *ptr_self };
		let maybe_processed = ref_self.events.process_message(wm_any);

		if msg == co::WM::NCDESTROY { // always check
			hwnd.SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
			ref_self.hwnd = unsafe { HWND::null_handle() }; // clear stored HWND
		}

		match maybe_processed {
			ProcessResult::HandledWithRet(res) => res.into(),
			ProcessResult::HandledWithoutRet => 0,
			ProcessResult::NotHandled => hwnd.DefWindowProc(wm_any).into(),
		}
	}
}
