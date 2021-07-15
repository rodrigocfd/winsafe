use crate::aliases::WinResult;
use crate::co;
use crate::enums::{AtomStr, IdMenu};
use crate::funcs::{PostQuitMessage, RegisterClassEx, SetLastError};
use crate::gui::base::Base;
use crate::gui::events::ProcessResult;
use crate::handles::HWND;
use crate::msg::{MsgSendRecv, wm, WndMsg};
use crate::structs::{ATOM, POINT, SIZE, WNDCLASSEX};
use crate::various::WString;

/// Base to all ordinary windows.
pub(in crate::gui) struct RawBase {
	base: Base,
}

impl Drop for RawBase {
	fn drop(&mut self) {
		if !self.base.hwnd_ref().is_null() {
			self.base.hwnd_ref().SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
		}
	}
}

impl RawBase {
	pub(in crate::gui) fn new(parent_base_ref: Option<&Base>) -> RawBase {
		Self {
			base: Base::new(parent_base_ref, false),
		}
	}

	pub(in crate::gui) fn base_ref(&self) -> &Base {
		&self.base
	}

	pub(in crate::gui) fn focus_first_child(&self) {
		// https://stackoverflow.com/a/2835220/6923555
		if let Ok(hchild) = self.base.hwnd_ref().GetWindow(co::GW::CHILD) {
			hchild.SetFocus();
		}
	}

	pub(in crate::gui) fn register_class(&self,
		wcx: &mut WNDCLASSEX) -> WinResult<ATOM>
	{
		wcx.lpfnWndProc = Some(Self::window_proc);
		SetLastError(co::ERROR::SUCCESS);

		RegisterClassEx(&wcx)
			.or_else(|err| {
				match err {
					co::ERROR::CLASS_ALREADY_EXISTS => {
						// https://devblogs.microsoft.com/oldnewthing/20150429-00/?p=44984
						// https://devblogs.microsoft.com/oldnewthing/20041011-00/?p=37603
						// Retrieve ATOM of existing window class.
						let hinst = wcx.hInstance;
						hinst.GetClassInfoEx(&wcx.lpszClassName().unwrap(), wcx)
					},
					_ => Err(err), // any other error will bubble up
				}
			})
	}

	pub(in crate::gui) fn create_window(
		&self,
		class_name: &str,
		title: Option<&str>,
		hmenu: IdMenu,
		pos: POINT,
		sz: SIZE,
		ex_styles: co::WS_EX,
		styles: co::WS) -> WinResult<()>
	{
		if !self.base.hwnd_ref().is_null() {
			panic!("Cannot create window twice.");
		}

		// Our hwnd member is set during WM_NCCREATE processing, already set when
		// CreateWindowEx returns.
		HWND::CreateWindowEx(
			ex_styles,
			AtomStr::Str(class_name.to_owned()),
			title, styles,
			pos.x, pos.y, sz.cx, sz.cy,
			self.base.parent_base_ref().map(|parent| *parent.hwnd_ref()),
			hmenu,
			self.base.parent_hinstance()?,
			Some(self as *const _ as _), // pass pointer to self
		).map(|_| ())
	}

	/// Generates a hash string from current fields, so it must called after all
	/// the fields are set.
	pub(in crate::gui) fn generate_wcx_class_name_hash(
		wcx: &WNDCLASSEX) -> WString
	{
		WString::from_str(
			&format!(
				"WNDCLASS.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}",
				wcx.style,
				wcx.lpfnWndProc.map_or(0, |p| p as usize),
				wcx.cbClsExtra, wcx.cbWndExtra,
				wcx.hInstance, wcx.hIcon, wcx.hCursor, wcx.hbrBackground,
				wcx.lpszMenuName(),
				wcx.hIconSm,
			),
		)
	}

	pub(in crate::gui) fn ui_thread_message_handler(&self) {
		self.base.ui_thread_message_handler();
	}

	pub(in crate::gui) fn run_ui_thread<F: FnOnce()>(&self, func: F) {
		self.base.run_ui_thread(func);
	}

	extern "system" fn window_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize) -> isize
	{
		|hwnd: HWND, msg, wparam, lparam| -> WinResult<isize>
		{
			let wm_any = WndMsg { msg_id: msg, wparam, lparam };

			let ptr_self = match msg {
				co::WM::NCCREATE => { // first message being handled
					let wm_ncc = wm::NcCreate::from_generic_wm(wm_any);
					let ptr_self = wm_ncc.createstruct.lpCreateParams as *mut Self;
					hwnd.SetWindowLongPtr(co::GWLP::USERDATA, ptr_self as _); // store
					let ref_self = unsafe { &mut *ptr_self };
					ref_self.base.set_hwnd(hwnd); // store HWND in struct field
					ptr_self
				},
				_ => hwnd.GetWindowLongPtr(co::GWLP::USERDATA) as *mut Self, // retrieve
			};

			// If no pointer stored, then no processing is done.
			// Prevents processing before WM_NCCREATE and after WM_NCDESTROY.
			if ptr_self.is_null() {
				return Ok(hwnd.DefWindowProc(wm_any));
			}

			// Execute privileged closures.
			let ref_self = unsafe { &mut *ptr_self };
			ref_self.base.process_privileged_messages(wm_any);

			// Execute user closure, if any.
			let process_result = ref_self.base.process_effective_message(wm_any);

			if msg == co::WM::NCDESTROY { // always check
				hwnd.SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
				ref_self.base.set_hwnd(HWND::NULL); // clear stored HWND
			}

			Ok(match process_result {
				ProcessResult::HandledWithRet(res) => res,
				ProcessResult::HandledWithoutRet => 0,
				ProcessResult::NotHandled => hwnd.DefWindowProc(wm_any).into(),
			})
		}
		(hwnd, msg, wparam, lparam)
			.unwrap_or_else(|err| { PostQuitMessage(err); 0 })
	}
}
