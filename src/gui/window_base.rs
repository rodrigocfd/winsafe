use crate::aliases::WinResult;
use crate::co;
use crate::enums::{AtomStr, IdMenu};
use crate::funcs::{RegisterClassEx, SetLastError};
use crate::gui::base::Base;
use crate::gui::events::{MsgEvents, ProcessResult};
use crate::gui::parent::Parent;
use crate::handles::{HINSTANCE, HWND};
use crate::msg::{MessageHandleable, Wm, WmNcCreate, WmWinsafeError};
use crate::structs::{ATOM, POINT, SIZE, WNDCLASSEX};
use crate::WString;

/// Base to all ordinary windows.
pub struct WindowBase {
	base: Base,
}

impl Drop for WindowBase {
	fn drop(&mut self) {
		if !self.hwnd_ref().is_null() {
			self.hwnd_ref().SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
		}
	}
}

impl Parent for WindowBase {
	fn hwnd_ref(&self) -> &HWND {
		self.base.hwnd_ref()
	}

	fn events_ref(&self) -> &MsgEvents {
		&self.base.events_ref()
	}

	fn add_child_to_be_created(&self,
		func: Box<dyn Fn() -> WinResult<()> + 'static>)
	{
		self.base.add_child_to_be_created(func);
	}
}

impl WindowBase {
	pub fn new(parent: Option<&dyn Parent>) -> WindowBase {
		Self {
			base: Base::new(parent),
		}
	}

	pub fn parent_hwnd(&self) -> Option<HWND> {
		self.base.parent_hwnd()
	}

	pub fn parent_hinstance(&self) -> WinResult<HINSTANCE> {
		self.base.parent_hinstance()
	}

	pub fn focus_first_child(&self) {
		// https://stackoverflow.com/a/2835220/6923555
		if let Ok(hchild) = self.hwnd_ref().GetWindow(co::GW::CHILD) {
			hchild.SetFocus();
		}
	}

	pub fn register_class(&self, wcx: &mut WNDCLASSEX) -> WinResult<ATOM> {
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
		class_name: &str,
		title: Option<&str>,
		hmenu: IdMenu,
		pos: POINT,
		sz: SIZE,
		ex_styles: co::WS_EX,
		styles: co::WS) -> WinResult<()>
	{
		if !self.hwnd_ref().is_null() {
			panic!("Cannot create window twice.");
		}

		// Our hwnd member is set during WM_NCCREATE processing, already set when
		// CreateWindowEx returns.
		HWND::CreateWindowEx(
			ex_styles,
			AtomStr::Str(WString::from_str(class_name)),
			title, styles,
			pos.x, pos.y, sz.cx, sz.cy,
			self.parent_hwnd(),
			hmenu, self.base.parent_hinstance()?,
			Some(self as *const Self as isize), // pass pointer to self
		).map(|_| ())
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
		|hwnd: HWND, msg, wparam, lparam| -> WinResult<isize>
		{
			let wm_any = Wm { msg_id: msg, wparam, lparam };

			let ptr_self = match msg {
				co::WM::NCCREATE => { // first message being handled
					let wm_ncc = WmNcCreate::from_generic_wm(wm_any);
					let ptr_self = wm_ncc.createstruct.lpCreateParams as *mut Self;
					hwnd.SetWindowLongPtr(co::GWLP::USERDATA, ptr_self as isize); // store
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

			// Create children.
			let ref_self = unsafe { &mut *ptr_self };
			if wm_any.msg_id == co::WM::CREATE {
				ref_self.base.create_children()?;
			}

			// Execute user closure, if any.
			let maybe_processed = ref_self.base.process_message(wm_any);

			if msg == co::WM::NCDESTROY { // always check
				hwnd.SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
				ref_self.base.set_hwnd(unsafe { HWND::null_handle() }); // clear stored HWND
			}

			Ok(match maybe_processed {
				ProcessResult::HandledWithRet(res) => res.into(),
				ProcessResult::HandledWithoutRet => 0,
				ProcessResult::NotHandled => hwnd.DefWindowProc(wm_any).into(),
			})
		}
		(hwnd, msg, wparam, lparam).unwrap_or_else(|err| {
			hwnd.PostMessage(WmWinsafeError { code: err }).ok();
			0
		})
	}
}
