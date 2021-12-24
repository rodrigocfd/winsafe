use crate::kernel::decl::{ErrResult, HINSTANCE, SetLastError, WinResult,
	WString};
use crate::co;
use crate::user::decl::{ATOM, AtomStr, HBRUSH, HCURSOR, HICON, HWND, IdIdcStr,
	IdMenu, POINT, RegisterClassEx, SIZE, WNDCLASSEX};
use crate::gui::base::Base;
use crate::gui::events::ProcessResult;
use crate::gui::privs::post_quit_error;
use crate::prelude::{Handle, KernelHinstance, MsgSendRecv, UserHinstance,
	UserHwnd};
use crate::msg::{wm, WndMsg};

/// Base to all ordinary windows.
pub(in crate::gui) struct RawBase {
	pub(in crate::gui) base: Base,
}

impl Drop for RawBase {
	fn drop(&mut self) {
		if !self.base.hwnd().is_null() {
			self.base.hwnd().SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
		}
	}
}

impl RawBase {
	pub(in crate::gui) fn new(parent_base: Option<&Base>) -> Self {
		Self {
			base: Base::new(false, parent_base),
		}
	}

	pub(in crate::gui) fn delegate_focus_to_first_child(&self) -> ErrResult<()> {
		if let Some(hwnd_cur_focus) = HWND::GetFocus() {
			if self.base.hwnd() == hwnd_cur_focus {
				// https://stackoverflow.com/a/2835220/6923555
				if let Ok(hchild_first) = self.base.hwnd().GetWindow(co::GW::CHILD) {
					hchild_first.SetFocus(); // if window receives focus, delegate to first child
				}
			}
		}
		Ok(())
	}

	/// Fills `WNDCLASSEX` with the given values, and generates the class name.
	pub(in crate::gui) fn fill_wndclassex<'a>(
		hinst: HINSTANCE,
		class_style: co::CS,
		class_icon: HICON, class_icon_sm: HICON,
		class_bg_brush: HBRUSH,
		class_cursor: HCURSOR,
		wcx: &mut WNDCLASSEX<'a>,
		class_name_buf: &'a mut WString) -> WinResult<()>
	{
		wcx.lpfnWndProc = Some(Self::window_proc);
		wcx.hInstance = hinst;
		wcx.style = class_style;
		wcx.hIcon = class_icon;
		wcx.hIconSm = class_icon_sm;
		wcx.hbrBackground = class_bg_brush;

		wcx.hCursor = match class_cursor.as_opt() {
			Some(h) => h,
			None => HINSTANCE::NULL.LoadCursor(IdIdcStr::Idc(co::IDC::ARROW))?,
		};

		if wcx.lpszClassName().is_none() {
			*class_name_buf = WString::from_str(
				&format!(
					"WNDCLASS.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}",
					wcx.style,
					wcx.lpfnWndProc.map_or(0, |p| p as usize),
					wcx.cbClsExtra, wcx.cbWndExtra,
					wcx.hInstance, wcx.hIcon, wcx.hCursor, wcx.hbrBackground,
					wcx.lpszMenuName(),
					wcx.hIconSm,
				),
			);
			wcx.set_lpszClassName(Some(class_name_buf));
		}

		Ok(())
	}

	pub(in crate::gui) fn register_class(&self,
		wcx: &mut WNDCLASSEX) -> WinResult<ATOM>
	{
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
		class_name: ATOM,
		title: Option<&str>,
		hmenu: IdMenu,
		pos: POINT,
		sz: SIZE,
		ex_styles: co::WS_EX,
		styles: co::WS) -> WinResult<()>
	{
		if !self.base.hwnd().is_null() {
			panic!("Cannot create window twice.");
		}

		// Our hwnd member is set during WM_NCCREATE processing, already set when
		// CreateWindowEx returns.
		HWND::CreateWindowEx(
			ex_styles,
			AtomStr::Atom(class_name),
			title, styles,
			pos, sz,
			self.base.parent_base().map(|parent| parent.hwnd()),
			hmenu,
			self.base.parent_base().map_or_else(
				|| HINSTANCE::GetModuleHandle(None),
				|parent| Ok(parent.hwnd().hinstance()),
			)?,
			Some(self as *const _ as _), // pass pointer to self
		).map(|_| ())
	}

	extern "system" fn window_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize) -> isize
	{
		Self::window_proc_proc(hwnd, msg, wparam, lparam)
			.unwrap_or_else(|err| { post_quit_error(err); 0 })
	}

	fn window_proc_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize) -> ErrResult<isize>
	{
		let wm_any = WndMsg { msg_id: msg, wparam, lparam };

		let ptr_self = match msg {
			co::WM::NCCREATE => { // first message being handled
				let wm_ncc = wm::NcCreate::from_generic_wm(wm_any);
				let ptr_self = wm_ncc.createstruct.lpCreateParams as *mut Self;
				hwnd.SetWindowLongPtr(co::GWLP::USERDATA, ptr_self as _); // store
				let ref_self = unsafe { &mut *ptr_self };
				*ref_self.base.hwnd_mut() = hwnd; // store HWND in struct field
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
		ref_self.base.process_privileged_messages(wm_any)?;

		// Execute user closure, if any.
		let process_result = ref_self.base.process_user_message(wm_any)?;

		if msg == co::WM::NCDESTROY { // always check
			hwnd.SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
			*ref_self.base.hwnd_mut() = HWND::NULL; // clear stored HWND
		}

		Ok(match process_result {
			ProcessResult::HandledWithRet(res) => res,
			ProcessResult::HandledWithoutRet => 0,
			ProcessResult::NotHandled => hwnd.DefWindowProc(wm_any).into(),
		})
	}
}
