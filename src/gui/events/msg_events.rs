use std::cell::UnsafeCell;
use std::rc::Rc;

use crate::co;
use crate::gui::events::func_store::FuncStore;
use crate::handles::{HDC, HICON};
use crate::msg;
use crate::msg::Message;

/// The result of processing a message.
pub enum ProcessResult {
	NotHandled,            // message was not handler because no such handler is stored
	HandledWithRet(isize), // return value is meaningful
	HandledWithoutRet,     // return value is not meaningful, whatever default value
}

//------------------------------------------------------------------------------

/// Exposes window
/// [messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
pub struct MsgEvents {
	obj: UnsafeCell<Obj>,
}

struct Obj { // actual fields of MsgEvents
	msgs: FuncStore< // ordinary WM messages
		co::WM,
		Box<dyn FnMut(msg::Wm) -> Option<isize> + 'static>, // return value may be meaningful
	>,
	tmrs: FuncStore< // WM_TIMER messages
		u32,
		Box<dyn FnMut() + 'static>, // return value is never meaningful
	>,
	cmds: FuncStore< // WM_COMMAND notifications
		(co::CMD, u16), // code, ctrl_id
		Box<dyn FnMut() + 'static>, // return value is never meaningful
	>,
	nfys: FuncStore< // WM_NOTIFY notifications
		(u16, co::NM), // idFrom, code
		Box<dyn FnMut(msg::WmNotify) -> Option<isize> + 'static>, // return value may be meaningful
	>,
}

cref_mref!(MsgEvents);

/// A message which has no parameters and returns zero.
macro_rules! wm_empty {
	(
		$name:ident, $wmconst:expr,
		$(#[$attr:meta])*
	) => {
		$(#[$attr])*
		pub fn $name<F>(&self, func: F)
			where F: FnMut() + 'static,
		{
			self.add_msg($wmconst, {
				let mut func = func;
				move |_| { func(); None } // return value is never meaningful
			});
		}
	};
}

/// A message with parameters which returns zero.
macro_rules! wm_ret_none {
	(
		$name:ident, $wmconst:expr, $parm:ty,
		$(#[$attr:meta])*
	) => {
		$(#[$attr])*
		pub fn $name<F>(&self, func: F)
			where F: FnMut($parm) + 'static,
		{
			self.add_msg($wmconst, {
				let mut func = func;
				move |p| { func(<$parm>::from_generic_wm(p)); None } // return value is never meaningful
			});
		}
	};
}

impl MsgEvents {
	pub(crate) fn new() -> MsgEvents {
		Self {
			obj: UnsafeCell::new(
				Obj {
					msgs: FuncStore::new(),
					tmrs: FuncStore::new(),
					cmds: FuncStore::new(),
					nfys: FuncStore::new(),
				},
			),
		}
	}

	/// Tells whether no functions have been added.
	pub(crate) fn is_empty(&self) -> bool {
		self.cref().msgs.is_empty()
			&& self.cref().tmrs.is_empty()
			&& self.cref().cmds.is_empty()
			&& self.cref().nfys.is_empty()
	}

	/// Searches for an user function for the given message, and runs it if found.
	pub(crate) fn process_message(&self, wm_any: msg::Wm) -> ProcessResult {
		match wm_any.msg_id {
			co::WM::NOTIFY => {
				let wm_nfy = msg::WmNotify::from_generic_wm(wm_any);
				let key = (wm_nfy.nmhdr.idFrom as u16, wm_nfy.nmhdr.code);
				match self.mref().nfys.find(key) {
					Some(func) => { // we have a stored function to handle this WM_NOTIFY notification
						match func(wm_nfy) { // execute user function
							Some(res) => ProcessResult::HandledWithRet(res), // meaningful return value
							None => ProcessResult::HandledWithoutRet,
						}
					},
					None => ProcessResult::NotHandled, // no stored WM_NOTIFY notification
				}
			},
			co::WM::COMMAND => {
				let wm_cmd = msg::WmCommand::from_generic_wm(wm_any);
				let key = (wm_cmd.code, wm_cmd.ctrl_id);
				match self.mref().cmds.find(key) {
					Some(func) => { // we have a stored function to handle this WM_COMMAND notification
						func(); // execute user function
						ProcessResult::HandledWithoutRet
					},
					None => ProcessResult::NotHandled, // no stored WM_COMMAND notification
				}
			},
			co::WM::TIMER => {
				let wm_tmr = msg::WmTimer::from_generic_wm(wm_any);
				match self.mref().tmrs.find(wm_tmr.timer_id) {
					Some(func) => { // we have a stored function to handle this WM_TIMER message
						func(); // execute user function
						ProcessResult::HandledWithoutRet
					},
					None => ProcessResult::NotHandled, // no stored WM_TIMER message
				}
			}
			_ => { // any other message
				match self.mref().msgs.find(wm_any.msg_id) {
					Some(func) => { // we have a stored function to handle this message
						match func(wm_any) { // execute user function
							Some(res) => ProcessResult::HandledWithRet(res), // meaningful return value
							None => ProcessResult::HandledWithoutRet,
						}
					},
					None => ProcessResult::NotHandled, // no stored function
				}
			}
		}
	}

	/// Raw add message.
	pub(crate) fn add_msg<F>(&self, ident: co::WM, func: F)
		where F: FnMut(msg::Wm) -> Option<isize> + 'static,
	{
		self.mref().msgs.insert(ident, Box::new(func));
	}

	/// Raw add notification.
	pub(crate) fn add_nfy<F>(&self, id_from: u16, code: co::NM, func: F)
		where F: FnMut(msg::WmNotify) -> Option<isize> + 'static,
	{
		self.mref().nfys.insert((id_from, code), Box::new(func));
	}

	/// Event to any [window message](crate::co::WM).
	///
	/// Instead of using this event, you should always prefer the specific
	/// events, which will give you the correct message parameters. This generic
	/// method should be used when you have a custom, non-standard window message.
	///
	/// # Examples
	///
	/// Handling a custom, user-defined message:
	///
	/// ```rust,ignore
	/// use winsafe::co::WM;
	/// use winsafe::gui::WindowMain;
	///
	/// let wnd: WindowMain; // initialize it somewhere...
	///
	/// let CUSTOM_MSG = WM::from(0x1234);
	///
	/// wnd.on().wm(CUSTOM_MSG, {
	///   let wnd = wnd.clone(); // pass into the closure
	///   move |parms| {
	///     println!("HWND: {}, msg ID: {}", wnd.hwnd(), parms.msg_id);
	///     0
	///   }
	/// });
	/// ```
	pub fn wm<F>(&self, ident: co::WM, func: F)
		where F: FnMut(msg::Wm) -> isize + 'static,
	{
		self.add_msg(ident, {
			let mut func = func;
			move |p| Some(func(p)) // return value is meaningful
		});
	}

	/// [`WM_TIMER`](crate::msg::WmTimer) message, narrowed to a specific timer
	/// ID.
	pub fn wm_timer<F>(&self, timer_id: u32, func: F)
		where F: FnMut() + 'static,
	{
		self.mref().tmrs.insert(timer_id, Box::new(func));
	}

	/// [`WM_COMMAND`](crate::msg::WmCommand) message, for specific code and
	/// control ID.
	///
	/// A command notification must be narrowed by the
	/// [command code](crate::co::CMD) and the control ID, so the closure will
	/// be fired for that specific control at that specific event.
	///
	/// Instead of using this event, you should always prefer the specific
	/// command notifications, which will give you the correct message
	/// parameters. This generic method should be used when you have a custom,
	/// non-standard window notification.
	pub fn wm_command<F>(&self, code: co::CMD, ctrl_id: u16, func: F)
		where F: FnMut() + 'static,
	{
		self.mref().cmds.insert((code, ctrl_id), Box::new(func));
	}

	/// [`WM_COMMAND`](crate::msg::WmCommand) message, handling both
	/// `co::CMD::Accelerator` and `co::CMD::Menu`, for a specific command ID.
	///
	/// Ideal to be used with menu commands whose IDs are shared with
	/// accelerators.
	///
	/// # Examples
	///
	/// Closing the window on ESC key:
	///
	/// ```rust,ignore
	/// use winsafe::co;
	/// use winsafe::gui::WindowMain;
	/// use winsafe::msg::WmClose;
	///
	/// let wnd: WindowMain; // initialize it somewhere...
	///
	/// wnd.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), {
	///   let wnd = wnd.clone(); // pass into the closure
	///   move || {
	///     wnd.hwnd().PostMessage(WmClose {}).unwrap();
	///   }
	/// });
	/// ```
	pub fn wm_command_accel_menu<F>(&self, ctrl_id: u16, func: F)
		where F: FnMut() + 'static,
	{
		let shared_func = Rc::new(UnsafeCell::new(func));

		self.wm_command(co::CMD::Menu, ctrl_id, {
			let shared_func = shared_func.clone();
			move || (unsafe { &mut *shared_func.get() })()
		});

		self.wm_command(co::CMD::Accelerator, ctrl_id, {
			let shared_func = shared_func.clone();
			move || (unsafe { &mut *shared_func.get() })()
		});
	}

	/// [`WM_NOTIFY`](crate::msg::WmNotify) message, for specific ID and
	/// notification code.
	///
	/// A notification must be narrowed by the [notification code](crate::co::NM)
	/// and the control ID, so the closure will be fired for that specific
	/// control at the specific event.
	///
	/// Instead of using this event, you should always prefer the specific
	/// notifications, which will give you the correct notification struct. This
	/// generic method should be used when you have a custom, non-standard window
	/// notification.
	pub fn wm_notify<F>(&self, id_from: u16, code: co::NM, func: F)
		where F: FnMut(msg::WmNotify) -> isize + 'static,
	{
		self.add_nfy(id_from, code, {
			let mut func = func;
			move |p| Some(func(p)) // return value is meaningful
		});
	}

	wm_ret_none! { wm_activate, co::WM::ACTIVATE, msg::WmActivate,
		/// [`WM_ACTIVATEAPP`](crate::msg::WmActivateApp) message.
		///
		/// Warning: default handled in [`WindowMain`](crate::gui::WindowMain).
	}

	wm_ret_none! { wm_activate_app, co::WM::ACTIVATEAPP, msg::WmActivateApp,
		/// [`WM_ACTIVATEAPP`](crate::msg::WmActivateApp) message.
	}

	/// [`WM_APPCOMMAND`](crate::msg::WmAppCommand) message.
	pub fn wm_app_command<F>(&self, func: F)
		where F: FnMut(msg::WmAppCommand) + 'static,
	{
		self.add_msg(co::WM::APPCOMMAND, {
			let mut func = func;
			move |p| { func(msg::WmAppCommand::from_generic_wm(p)); Some(true as isize) }
		});
	}

	wm_empty! { wm_close, co::WM::CLOSE,
		/// [`WM_CLOSE`](crate::msg::WmClose) message.
		///
		/// Warning: default handled in [`DialogMain`](crate::gui::DialogMain).
	}

	/// [`WM_CREATE`](crate::msg::WmCreate) message.
	///
	/// This is where you physically create the child controls, by calling their
	/// `create` method.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui::WindowMain;
	///
	/// let wnd: WindowMain; // initialize it somewhere...
	///
	/// wnd.on().wm_create({
	///   let wnd = wnd.clone(); // pass into the closure
	///   move |parms| {
	///     println!("HWND: {}, client area: {}x{}",
	///       wnd.hwnd(),
	///       parms.createstruct.cx, parms.createstruct.cy);
	///     0
	///   }
	/// });
	/// ```
	pub fn wm_create<F>(&self, func: F)
		where F: FnMut(msg::WmCreate) -> i32 + 'static,
	{
		self.add_msg(co::WM::CREATE, {
			let mut func = func;
			move |p| Some(func(msg::WmCreate::from_generic_wm(p)) as isize)
		});
	}

	/// [`WM_CTLCOLORBTN`](crate::msg::WmCtlColorBtn) message.
	pub fn wm_ctl_color_btn<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorBtn) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLORBTN, {
			let mut func = func;
			move |p| Some(func(msg::WmCtlColorBtn::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLORDLG`](crate::msg::WmCtlColorDlg) message.
	pub fn wm_ctl_color_dlg<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorDlg) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLORDLG, {
			let mut func = func;
			move |p| Some(func(msg::WmCtlColorDlg::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLOREDIT`](crate::msg::WmCtlColorEdit) message.
	pub fn wm_ctl_color_edit<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorEdit) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLOREDIT, {
			let mut func = func;
			move |p| Some(func(msg::WmCtlColorEdit::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLORLISTBOX`](crate::msg::WmCtlColorListBox) message.
	pub fn wm_ctl_color_list_box<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorListBox) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLORLISTBOX, {
			let mut func = func;
			move |p| Some(func(msg::WmCtlColorListBox::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLORSCROLLBAR`](crate::msg::WmCtlColorScrollBar) message.
	pub fn wm_ctl_color_scroll_bar<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorScrollBar) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLORSCROLLBAR, {
			let mut func = func;
			move |p| Some(func(msg::WmCtlColorScrollBar::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLORSTATIC`](crate::msg::WmCtlColorStatic) message.
	pub fn wm_ctl_color_static<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorStatic) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLORSTATIC, {
			let mut func = func;
			move |p| Some(func(msg::WmCtlColorStatic::from_generic_wm(p)).ptr as isize)
		});
	}

	wm_empty! { wm_destroy, co::WM::DESTROY,
		/// [`WM_DESTROY`](crate::msg::WmDestroy) message.
	}

	wm_ret_none! { wm_drop_files, co::WM::DROPFILES, msg::WmDropFiles,
		/// [`WM_DROPFILES`](crate::msg::WmDropFiles) message.
	}

	wm_ret_none! { wm_enable, co::WM::ENABLE, msg::WmEnable,
		/// [`WM_ENABLE`](crate::msg::WmEnable) message.
	}

	wm_ret_none! { wm_end_session, co::WM::ENDSESSION, msg::WmEndSession,
		/// [`WM_ENDSESSION`](crate::msg::WmEndSession) message.
	}

	wm_ret_none! { wm_enter_idle, co::WM::ENTERIDLE, msg::WmEnterIdle,
		/// [`WM_ENTERIDLE`](crate::msg::WmEnterIdle) message.
	}

	wm_ret_none! { wm_enter_size_move, co::WM::ENTERSIZEMOVE, msg::WmEnterSizeMove,
		/// [`WM_ENTERSIZEMOVE`](crate::msg::WmEnterSizeMove) message.
	}

	/// [`WM_ERASEBKGND`](crate::msg::WmEraseBkgnd) message.
	pub fn wm_erase_bkgnd<F>(&self, func: F)
		where F: FnMut(msg::WmEraseBkgnd) -> i32 + 'static,
	{
		self.add_msg(co::WM::ERASEBKGND, {
			let mut func = func;
			move |p| Some(func(msg::WmEraseBkgnd::from_generic_wm(p)) as isize)
		});
	}

	wm_ret_none! { wm_exit_size_move, co::WM::EXITSIZEMOVE, msg::WmExitSizeMove,
		/// [`WM_EXITSIZEMOVE`](crate::msg::WmExitSizeMove) message.
	}

	/// [`WM_INITDIALOG`](crate::msg::WmInitDialog) message.
	pub fn wm_init_dialog<F>(&self, func: F)
		where F: FnMut(msg::WmInitDialog) -> bool + 'static,
	{
		self.add_msg(co::WM::INITDIALOG, {
			let mut func = func;
			move |p| Some(func(msg::WmInitDialog::from_generic_wm(p)) as isize)
		});
	}

	wm_ret_none! { wm_get_min_max_info, co::WM::GETMINMAXINFO, msg::WmGetMinMaxInfo,
		/// [`WM_GETMINMAXINFO`](crate::msg::WmGetMinMaxInfo) message.
	}

	wm_ret_none! { wm_init_menu_popup, co::WM::INITMENUPOPUP, msg::WmInitMenuPopup,
		/// [`WM_INITMENUPOPUP`](crate::msg::WmInitMenuPopup) message.
	}

	wm_ret_none! { wm_l_button_dbl_clk, co::WM::LBUTTONDBLCLK, msg::WmLButtonDblClk,
		/// [`WM_LBUTTONDBLCLK`](crate::msg::WmLButtonDblClk) message.
	}

	wm_ret_none! { wm_l_button_down, co::WM::LBUTTONDOWN, msg::WmLButtonDown,
		/// [`WM_LBUTTONDOWN`](crate::msg::WmLButtonDown) message.
	}

	wm_ret_none! { wm_l_button_up, co::WM::LBUTTONUP, msg::WmLButtonUp,
		/// [`WM_LBUTTONUP`](crate::msg::WmLButtonUp) message.
	}

	wm_ret_none! { wm_m_button_dbl_clk, co::WM::MBUTTONDBLCLK, msg::WmMButtonDblClk,
		/// [`WM_MBUTTONDBLCLK`](crate::msg::WmMButtonDblClk) message.
	}

	wm_ret_none! { wm_m_button_down, co::WM::MBUTTONDOWN, msg::WmMButtonDown,
		/// [`WM_MBUTTONDOWN`](crate::msg::WmMButtonDown) message.
	}

	wm_ret_none! { wm_m_button_up, co::WM::MBUTTONUP, msg::WmMButtonUp,
		/// [`WM_MBUTTONUP`](crate::msg::WmMButtonUp) message.
	}

	wm_ret_none! { wm_mouse_hover, co::WM::MOUSEHOVER, msg::WmMouseHover,
		/// [`WM_MOUSEHOVER`](crate::msg::WmMouseHover) message.
	}

	wm_ret_none! { wm_mouse_move, co::WM::MOUSEMOVE, msg::WmMouseMove,
		/// [`WM_MOUSEMOVE`](crate::msg::WmMouseMove) message.
	}

	wm_ret_none! { wm_move, co::WM::MOVE, msg::WmMove,
		/// [`WM_MOVE`](crate::msg::WmMove) message.
	}

	wm_ret_none! { wm_moving, co::WM::MOVING, msg::WmMoving,
		/// [`WM_MOVING`](crate::msg::WmMoving) message.
	}

	/// [`WM_NCCREATE`](crate::msg::WmNcCreate) message.
	pub fn wm_nc_create<F>(&self, func: F)
		where F: FnMut(msg::WmNcCreate) -> bool + 'static,
	{
		self.add_msg(co::WM::NCCREATE, {
			let mut func = func;
			move |p| Some(func(msg::WmNcCreate::from_generic_wm(p)) as isize)
		});
	}

	wm_empty! { wm_nc_destroy, co::WM::NCDESTROY,
		/// [`WM_NCDESTROY`](crate::msg::WmNcDestroy) message.
		///
		/// Warning: default handled in [`WindowMain`](crate::gui::WindowMain) and
		/// [`DialogMain`](crate::gui::DialogMain).
	}

	wm_ret_none! { wm_nc_paint, co::WM::NCPAINT, msg::WmNcPaint,
		/// [`WM_NCPAINT`](crate::msg::WmNcPaint) message.
		///
		/// Warning: default handled in [`WindowControl`](crate::gui::WindowControl).
	}

	wm_empty! { wm_null, co::WM::NULL,
		/// [`WM_NULL`](crate::msg::WmNull) message.
		///
		/// Usually this message is not handled.
	}

	wm_empty! { wm_paint, co::WM::PAINT,
		/// [`WM_PAINT`](crate::msg::WmPaint) message.
	}

	/// [`WM_QUERYOPEN`](crate::msg::WmQueryOpen) message.
	pub fn wm_query_open<F>(&self, func: F)
		where F: FnMut(msg::WmQueryOpen) -> bool + 'static,
	{
		self.add_msg(co::WM::QUERYOPEN, {
			let mut func = func;
			move |p| Some(func(msg::WmQueryOpen::from_generic_wm(p)) as isize)
		});
	}

	wm_ret_none! { wm_r_button_dbl_clk, co::WM::RBUTTONDBLCLK, msg::WmRButtonDblClk,
		/// [`WM_RBUTTONDBLCLK`](crate::msg::WmRButtonDblClk) message.
	}

	wm_ret_none! { wm_r_button_down, co::WM::RBUTTONDOWN, msg::WmRButtonDown,
		/// [`WM_RBUTTONDOWN`](crate::msg::WmRButtonDown) message.
	}

	wm_ret_none! { wm_r_button_up, co::WM::RBUTTONUP, msg::WmRButtonUp,
		/// [`WM_RBUTTONUP`](crate::msg::WmRButtonUp) message.
	}

	wm_ret_none! { wm_set_focus, co::WM::SETFOCUS, msg::WmSetFocus,
		/// [`WM_SETFOCUS`](crate::msg::WmSetFocus) message.
		///
		/// Warning: default handled in [`WindowMain`](crate::gui::WindowMain).
	}

	wm_ret_none! { wm_set_font, co::WM::SETFONT, msg::WmSetFont,
		/// [`WM_SETFONT`](crate::msg::WmSetFont) message.
	}

	/// [`WM_SETICON`](crate::msg::WmSetIcon) message.
	pub fn wm_set_icon<F>(&self, func: F)
		where F: FnMut(msg::WmSetIcon) -> Option<HICON> + 'static,
	{
		self.add_msg(co::WM::SETICON, {
			let mut func = func;
			move |p| Some(
				match func(msg::WmSetIcon::from_generic_wm(p)) {
					Some(hicon) => hicon.ptr as isize,
					None => 0,
				},
			)
		});
	}

	wm_ret_none! { wm_show_window, co::WM::SHOWWINDOW, msg::WmShowWindow,
		/// [`WM_SHOWWINDOW`](crate::msg::WmShowWindow) message.
	}

	wm_ret_none! { wm_size, co::WM::SIZE, msg::WmSize,
		/// [`WM_SIZE`](crate::msg::WmSize) message.
		///
		/// # Examples
		///
		/// ```rust,ignore
		/// use winsafe::gui::WindowMain;
		///
		/// let wnd: WindowMain; // initialize it somewhere...
		///
		/// wnd.on().wm_size({
		///   let wnd = wnd.clone(); // pass into the closure
		///   move |parms| {
		///     println!("HWND: {}, client area: {}x{}",
		///       wnd.hwnd(),
		///       parms.width, parms.height);
		///   }
		/// });
		/// ```
	}

	wm_ret_none! { wm_sizing, co::WM::SIZING, msg::WmSizing,
		/// [`WM_SIZING`](crate::msg::WmSizing) message.
	}

	wm_ret_none! { wm_style_changed, co::WM::STYLECHANGED, msg::WmStyleChanged,
		/// [`WM_STYLECHANGED`](crate::msg::WmStyleChanged) message.
	}

	wm_ret_none! { wm_style_changing, co::WM::STYLECHANGING, msg::WmStyleChanging,
		/// [`WM_STYLECHANGING`](crate::msg::WmStyleChanging) message.
	}

	wm_ret_none! { wm_theme_changed, co::WM::THEMECHANGED, msg::WmThemeChanged,
		/// [`WM_THEMECHANGED`](crate::msg::WmThemeChanged) message.
	}

	wm_ret_none! { wm_window_pos_changed, co::WM::WINDOWPOSCHANGED, msg::WmWindowPosChanged,
		/// [`WM_WINDOWPOSCHANGED`](crate::msg::WmWindowPosChanged) message.
	}

	wm_ret_none! { wm_window_pos_changing, co::WM::WINDOWPOSCHANGING, msg::WmWindowPosChanging,
		/// [`WM_WINDOWPOSCHANGING`](crate::msg::WmWindowPosChanging) message.
	}

	wm_ret_none! { wm_x_button_dbl_clk, co::WM::XBUTTONDBLCLK, msg::WmXButtonDblClk,
		/// [`WM_XBUTTONDBLCLK`](crate::msg::WmXButtonDblClk) message.
	}

	wm_ret_none! { wm_x_button_down, co::WM::XBUTTONDOWN, msg::WmXButtonDown,
		/// [`WM_XBUTTONDOWN`](crate::msg::WmXButtonDown) message.
	}

	wm_ret_none! { wm_x_button_up, co::WM::XBUTTONUP, msg::WmXButtonUp,
		/// [`WM_XBUTTONUP`](crate::msg::WmXButtonUp) message.
	}
}
