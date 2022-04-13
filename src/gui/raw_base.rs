use crate::co;
use crate::gui::base::Base;
use crate::gui::events::{ProcessResult, WindowEventsAll};
use crate::gui::privs::post_quit_error;
use crate::kernel::decl::{ErrResult, HINSTANCE, SetLastError, WString};
use crate::msg::{wm, WndMsg};
use crate::prelude::{GdiHbrush, Handle, MsgSendRecv, UserHinstance, UserHwnd};
use crate::user::decl::{
	ATOM, AtomStr, HBRUSH, HCURSOR, HICON, HWND, IdIdcStr, IdIdiStr, IdMenu,
	POINT, RegisterClassEx, SIZE, WNDCLASSEX,
};

/// The class background brush to be loaded for
/// [`WindowMainOpts`](crate::gui::WindowMainOpts),
/// [`WindowModalOpts`](crate::gui::WindowModalOpts) or
/// [`WindowControlOpts`](crate::gui::WindowControlOpts).
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub enum Brush {
	/// A solid [system color](co::COLOR).
	Color(co::COLOR),
	/// A brush handle, previously created by you.
	Handle(HBRUSH),
}

impl Brush {
	pub fn as_hbrush(&self) -> HBRUSH {
		match self {
			Brush::Color(c) => HBRUSH::from_sys_color(*c),
			Brush::Handle(h) => *h,
		}
	}
}

/// The class cursor to be loaded for
/// [`WindowMainOpts`](crate::gui::WindowMainOpts),
/// [`WindowModalOpts`](crate::gui::WindowModalOpts) or
/// [`WindowControlOpts`](crate::gui::WindowControlOpts).
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub enum Cursor {
	/// A cursor handle, previously loaded by you.
	Handle(HCURSOR),
	/// A resource ID.
	Id(u16),
	/// A [`co::IDC`](crate::co::IDC) constant for a stock system cursor.
	Idc(co::IDC),
	/// A resource string identifier.
	Str(WString),
}

impl Cursor {
	pub fn as_hcursor(&self, hinst: HINSTANCE) -> HCURSOR {
		match self {
			Cursor::Handle(h) => *h,
			Cursor::Id(id) => hinst.LoadCursor(IdIdcStr::Id(*id)).unwrap(),
			Cursor::Idc(idc) => HINSTANCE::NULL.LoadCursor(IdIdcStr::Idc(*idc)).unwrap(),
			Cursor::Str(s) => hinst.LoadCursor(IdIdcStr::Str(s.clone())).unwrap(),
		}
	}
}

/// The class icon to be loaded for
/// [`WindowMainOpts`](crate::gui::WindowMainOpts),
/// [`WindowModalOpts`](crate::gui::WindowModalOpts) or
/// [`WindowControlOpts`](crate::gui::WindowControlOpts).
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub enum Icon {
	/// An icon handle, previously loaded by you.
	Handle(HICON),
	/// A resource ID.
	Id(u16),
	/// A [`co::IDC`](crate::co::IDC) constant for a stock system icon.
	Idi(co::IDI),
	/// No icon.
	None,
	/// A resource string identifier.
	Str(WString),
}

impl Icon {
	pub fn as_hicon(&self, hinst: HINSTANCE) -> HICON {
		match self {
			Icon::Handle(h) => *h,
			Icon::Id(id) => hinst.LoadIcon(IdIdiStr::Id(*id)).unwrap(),
			Icon::Idi(idi) => HINSTANCE::NULL.LoadIcon(IdIdiStr::Idi(*idi)).unwrap(),
			Icon::None => HICON::NULL,
			Icon::Str(s) => hinst.LoadIcon(IdIdiStr::Str(s.clone())).unwrap(),
		}
	}
}

//------------------------------------------------------------------------------

/// Base to all ordinary windows.
pub(in crate::gui) struct RawBase {
	base: Base,
}

impl Drop for RawBase {
	fn drop(&mut self) {
		if !self.base.hwnd().is_null() {
			self.base.hwnd().SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
		}
	}
}

impl RawBase {
	pub(in crate::gui) fn new(parent: Option<&Base>) -> Self {
		Self { base: Base::new(false, parent) }
	}

	pub(in crate::gui) unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		// At this moment, the parent struct is already created and pinned.
		&self.base as *const _ as _
	}

	pub(in crate::gui) const fn hwnd(&self) -> HWND {
		self.base.hwnd()
	}

	pub(in crate::gui) fn on(&self) -> &WindowEventsAll {
		self.base.on()
	}

	// pub(in crate::gui) fn privileged_on(&self) -> &WindowEventsAll {
	// 	self.base.privileged_on()
	// }

	pub(in crate::gui) fn parent(&self) -> Option<&Base> {
		self.base.parent()
	}

	pub(in crate::gui) fn parent_hinstance(&self) -> HINSTANCE {
		self.base.parent_hinstance()
	}

	pub(in crate::gui) fn delegate_focus_to_first_child(&self) {
		if let Some(hwnd_cur_focus) = HWND::GetFocus() {
			if self.hwnd() == hwnd_cur_focus {
				// https://stackoverflow.com/a/2835220/6923555
				if let Ok(hchild_first) = self.hwnd().GetWindow(co::GW::CHILD) {
					hchild_first.SetFocus(); // if window receives focus, delegate to first child
				}
			}
		}
	}

	/// Fills `WNDCLASSEX` with the given values, and generates a class name as
	/// a hash of all fields.
	pub(in crate::gui) fn fill_wndclassex<'a>(
		hinst: HINSTANCE,
		class_style: co::CS,
		class_icon: &Icon, class_icon_sm: &Icon,
		class_bg_brush: &Brush,
		class_cursor: &Cursor,
		wcx: &mut WNDCLASSEX<'a>,
		class_name_buf: &'a mut WString)
	{
		wcx.lpfnWndProc = Some(Self::window_proc);
		wcx.hInstance = hinst;
		wcx.style = class_style;
		wcx.hIcon = class_icon.as_hicon(hinst);
		wcx.hIconSm = class_icon_sm.as_hicon(hinst);
		wcx.hbrBackground = class_bg_brush.as_hbrush();
		wcx.hCursor = class_cursor.as_hcursor(hinst);

		if wcx.lpszClassName().is_none() { // an actual class name was not provided?
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
	}

	pub(in crate::gui) fn register_class(&self, wcx: &mut WNDCLASSEX) -> ATOM {
		SetLastError(co::ERROR::SUCCESS);
		match RegisterClassEx(&wcx) {
			Ok(atom) => atom,
			Err(err) => match err {
				co::ERROR::CLASS_ALREADY_EXISTS => {
					// https://devblogs.microsoft.com/oldnewthing/20150429-00/?p=44984
					// https://devblogs.microsoft.com/oldnewthing/20041011-00/?p=37603
					// Retrieve ATOM of existing window class.
					let hinst = wcx.hInstance;
					hinst.GetClassInfoEx(&wcx.lpszClassName().unwrap(), wcx).unwrap()
				},
				_ => panic!(),
			},
		}
	}

	pub(in crate::gui) fn create_window(
		&self,
		class_name: ATOM,
		title: Option<&str>,
		hmenu: IdMenu,
		pos: POINT,
		sz: SIZE,
		ex_styles: co::WS_EX,
		styles: co::WS)
	{
		if !self.hwnd().is_null() {
			panic!("Cannot create window twice.");
		}

		// Our hwnd member is set during WM_NCCREATE processing; already set when
		// CreateWindowEx returns.
		unsafe {
			HWND::CreateWindowEx(
				ex_styles,
				AtomStr::Atom(class_name),
				title, styles,
				pos, sz,
				self.base.parent().map(|parent| parent.hwnd()),
				hmenu,
				self.base.parent_hinstance(),
				// Pass pointer to Self.
				// At this moment, the parent struct is already created and pinned.
				Some(self as *const _ as _),
			).unwrap();
		}
	}

	pub(in crate::gui) fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()> + Send + 'static,
	{
		self.base.spawn_new_thread(func);
	}

	pub(in crate::gui) fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()> + Send + 'static
	{
		self.base.run_ui_thread(func);
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
				unsafe { ref_self.base.set_hwnd(hwnd); } // store HWND in struct field
				ptr_self
			},
			_ => hwnd.GetWindowLongPtr(co::GWLP::USERDATA) as *mut Self, // retrieve
		};

		// If no pointer stored, then no processing is done.
		// Prevents processing before WM_NCCREATE and after WM_NCDESTROY.
		if ptr_self.is_null() {
			return Ok(hwnd.DefWindowProc(wm_any));
		}

		// Execute privileged closures, discard results.
		let ref_self = unsafe { &mut *ptr_self };
		ref_self.base.process_privileged_messages(wm_any)?;

		// Execute user closure, if any.
		let process_result = ref_self.base.process_user_message(wm_any)?;

		if msg == co::WM::NCDESTROY { // always check
			hwnd.SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
			unsafe { ref_self.base.set_hwnd(HWND::NULL); } // clear stored HWND
		}

		Ok(match process_result {
			ProcessResult::HandledWithRet(res) => res,
			ProcessResult::HandledWithoutRet => 0,
			ProcessResult::NotHandled => hwnd.DefWindowProc(wm_any).into(),
		})
	}
}
