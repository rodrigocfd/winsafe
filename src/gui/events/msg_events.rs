use std::cell::UnsafeCell;

use crate::co;
use crate::gui::events::func_store::FuncStore;
use crate::handles::HDC;
use crate::msg;

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
		Box<dyn FnMut(msg::Wm) -> Option<isize> + Send + Sync + 'static>, // return value may be meaningful
	>,
	tmrs: FuncStore< // WM_TIMER messages
		u32,
		Box<dyn FnMut() + Send + Sync + 'static>, // return value is never meaningful
	>,
	cmds: FuncStore< // WM_COMMAND notifications
		(co::CMD, u16), // code, ctrl_id
		Box<dyn FnMut() + Send + Sync + 'static>, // return value is never meaningful
	>,
	nfys: FuncStore< // WM_NOTIFY notifications
		(u16, co::NM), // idFrom, code
		Box<dyn FnMut(msg::WmNotify) -> Option<isize> + Send + Sync + 'static>, // return value may be meaningful
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
			where F: FnMut() + Send + Sync + 'static,
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
			where F: FnMut($parm) + Send + Sync + 'static,
		{
			self.add_msg($wmconst, {
				let mut func = func;
				move |p| { func(p.into()); None } // return value is never meaningful
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

	/// Searches for an user function for the given message, and runs it if found.
	pub(crate) fn process_message(&self, wm_any: msg::Wm) -> ProcessResult {
		match wm_any.msg_id {
			co::WM::NOTIFY => {
				let wm_nfy: msg::WmNotify = wm_any.into();
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
				let wm_cmd: msg::WmCommand = wm_any.into();
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
				let wm_tmr: msg::WmTimer = wm_any.into();
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
		where F: FnMut(msg::Wm) -> Option<isize> + Send + Sync + 'static,
	{
		self.mref().msgs.insert(ident, Box::new(func));
	}

	/// Raw add notification.
	pub(crate) fn add_nfy<F>(&self, id_from: u16, code: co::NM, func: F)
		where F: FnMut(msg::WmNotify) -> Option<isize> + Send + Sync + 'static,
	{
		self.mref().nfys.insert((id_from, code), Box::new(func));
	}

	/// Event to any [window message](crate::co::WM).
	///
	/// You should always prefer the specific events, which will give you the
	/// correct message parameters. This generic method should be used when you
	/// have a custom, non-standard window message.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::co::WM;
	/// use winsafe::gui::{WindowMain, WindowMainOpts};
	/// use winsafe::gui::Parent; // necessary to access the trait methods
	///
	/// let wnd = WindowMain::new(
	///   WindowMainOpts::default(),
	/// );
	///
	/// let CUSTOM_MSG = WM::from(0x1234);
	///
	/// wnd.on().wm(CUSTOM_MSG, {
	///   let wnd = wnd.clone();
	///   move |parms| {
	///     println!("HWND: {}, Message ID: {}", wnd.hwnd(), parms.msg_id);
	///     0
	///   }
	/// });
	/// ```
	pub fn wm<F>(&self, ident: co::WM, func: F)
		where F: FnMut(msg::Wm) -> isize + Send + Sync + 'static,
	{
		self.add_msg(ident, {
			let mut func = func;
			move |p| Some(func(p)) // return value is meaningful
		});
	}

	/// [`WM_TIMER`](crate::msg::WmTimer) message, narrowed to a specific timer
	/// ID.
	pub fn wm_timer<F>(&self, timer_id: u32, func: F)
		where F: FnMut() + Send + Sync + 'static,
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
	/// You should always prefer the specific command notifications, which will
	/// give you the correct message parameters.
	pub fn wm_command<F>(&self, code: co::CMD, ctrl_id: u16, func: F)
		where F: FnMut() + Send + Sync + 'static,
	{
		self.mref().cmds.insert((code, ctrl_id), Box::new(func));
	}

	/// [`WM_NOTIFY`](crate::msg::WmNotify) message, for specific ID and
	/// notification code.
	///
	/// A notification must be narrowed by the [notification code](crate::co::NM)
	/// and the control ID, so the closure will be fired for that specific
	/// control at the specific event.
	///
	/// You should always prefer the specific notifications, which will give you
	/// the correct notification struct.
	pub fn wm_notify<F>(&self, id_from: u16, code: co::NM, func: F)
		where F: FnMut(msg::WmNotify) -> isize + Send + Sync + 'static,
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
		where F: FnMut(msg::WmAppCommand) + Send + Sync + 'static,
	{
		self.add_msg(co::WM::APPCOMMAND, {
			let mut func = func;
			move |p| { func(p.into()); Some(1) } // TRUE
		});
	}

	wm_empty! { wm_close, co::WM::CLOSE,
		/// [`WM_CLOSE`](crate::msg::WmClose) message.
	}

	/// [`WM_CREATE`](crate::msg::WmCreate) message.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui::{WindowMain, WindowMainOpts};
	/// use winsafe::gui::Parent; // necessary to access the trait methods
	///
	/// let wnd = WindowMain::new(
	///   WindowMainOpts::default(),
	/// );
	///
	/// wnd.on().wm_create({
	///   let wnd = wnd.clone();
	///   move |parms| {
	///     println!("HWND: {}, client area: {}x{}",
	///       wnd.hwnd(),
	///       parms.createstruct.cx, parms.createstruct.cy);
	///     0
	///   }
	/// });
	/// ```
	pub fn wm_create<F>(&self, func: F)
		where F: FnMut(msg::WmCreate) -> i32 + Send + Sync + 'static,
	{
		self.add_msg(co::WM::CREATE, {
			let mut func = func;
			move |p| Some(func(p.into()) as isize)
		});
	}

	/// [`WM_CTLCOLORBTN`](crate::msg::WmCtlColorBtn) message.
	pub fn wm_ctl_color_btn<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorBtn) -> HDC + Send + Sync + 'static,
	{
		self.add_msg(co::WM::CTLCOLORBTN, {
			let mut func = func;
			move |p| Some(unsafe { func(p.into()).as_ptr() } as isize)
		});
	}

	/// [`WM_CTLCOLORDLG`](crate::msg::WmCtlColorDlg) message.
	pub fn wm_ctl_color_dlg<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorDlg) -> HDC + Send + Sync + 'static,
	{
		self.add_msg(co::WM::CTLCOLORDLG, {
			let mut func = func;
			move |p| Some(unsafe { func(p.into()).as_ptr() } as isize)
		});
	}

	/// [`WM_CTLCOLOREDIT`](crate::msg::WmCtlColorEdit) message.
	pub fn wm_ctl_color_edit<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorEdit) -> HDC + Send + Sync + 'static,
	{
		self.add_msg(co::WM::CTLCOLOREDIT, {
			let mut func = func;
			move |p| Some(unsafe { func(p.into()).as_ptr() } as isize)
		});
	}

	/// [`WM_CTLCOLORLISTBOX`](crate::msg::WmCtlColorListBox) message.
	pub fn wm_ctl_color_list_box<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorListBox) -> HDC + Send + Sync + 'static,
	{
		self.add_msg(co::WM::CTLCOLORLISTBOX, {
			let mut func = func;
			move |p| Some(unsafe { func(p.into()).as_ptr() } as isize)
		});
	}

	/// [`WM_CTLCOLORSCROLLBAR`](crate::msg::WmCtlColorScrollBar) message.
	pub fn wm_ctl_color_scroll_bar<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorScrollBar) -> HDC + Send + Sync + 'static,
	{
		self.add_msg(co::WM::CTLCOLORSCROLLBAR, {
			let mut func = func;
			move |p| Some(unsafe { func(p.into()).as_ptr() } as isize)
		});
	}

	/// [`WM_CTLCOLORSTATIC`](crate::msg::WmCtlColorStatic) message.
	pub fn wm_ctl_color_static<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorStatic) -> HDC + Send + Sync + 'static,
	{
		self.add_msg(co::WM::CTLCOLORSTATIC, {
			let mut func = func;
			move |p| Some(unsafe { func(p.into()).as_ptr() } as isize)
		});
	}

	wm_empty! { wm_destroy, co::WM::DESTROY,
		/// [`WM_DESTROY`](crate::msg::WmDestroy) message.
	}

	wm_ret_none! { wm_drop_files, co::WM::DROPFILES, msg::WmDropFiles,
		/// [`WM_DROPFILES`](crate::msg::WmDropFiles) message.
	}

	wm_ret_none! { wm_end_session, co::WM::ENDSESSION, msg::WmEndSession,
		/// [`WM_ENDSESSION`](crate::msg::WmEndSession) message.
	}

	/// [`WM_INITDIALOG`](crate::msg::WmInitDialog) message.
	pub fn wm_init_dialog<F>(&mut self, func: F)
		where F: FnMut(msg::WmInitDialog) -> bool + Send + Sync + 'static,
	{
		self.add_msg(co::WM::INITDIALOG, {
			let mut func = func;
			move |p| Some(func(p.into()) as isize)
		});
	}

	wm_ret_none! { wm_init_menu_popup, co::WM::INITMENUPOPUP, msg::WmInitMenuPopup,
		/// [`WM_INITMENUPOPUP`](crate::msg::WmInitMenuPopup) message.
	}

	/// [`WM_NCCREATE`](crate::msg::WmNcCreate) message.
	pub fn wm_nc_create<F>(&self, func: F)
		where F: FnMut(msg::WmNcCreate) -> bool + Send + Sync + 'static,
	{
		self.add_msg(co::WM::NCCREATE, {
			let mut func = func;
			move |p| Some(func(p.into()) as isize)
		});
	}

	wm_empty! { wm_nc_destroy, co::WM::NCDESTROY,
		/// [`WM_NCDESTROY`](crate::msg::WmNcDestroy) message.
		///
		/// Warning: default handled in [`WindowMain`](crate::gui::WindowMain).
	}

	wm_empty! { wm_nc_paint, co::WM::NCPAINT,
		/// [`WM_NCPAINT`](crate::msg::WmNcPaint) message.
	}

	wm_empty! { wm_null, co::WM::NULL,
		/// [`WM_NULL`](crate::msg::WmNull) message.
		///
		/// Usually this message is not handled.
	}

	wm_empty! { wm_paint, co::WM::PAINT,
		/// [`WM_PAINT`](crate::msg::WmPaint) message.
	}

	wm_ret_none! { wm_set_focus, co::WM::SETFOCUS, msg::WmSetFocus,
		/// [`WM_SETFOCUS`](crate::msg::WmSetFocus) message.
		///
		/// Warning: default handled in [`WindowMain`](crate::gui::WindowMain).
	}

	wm_ret_none! { wm_size, co::WM::SIZE, msg::WmSize,
		/// [`WM_SIZE`](crate::msg::WmSize) message.
		///
		/// # Examples
		///
		/// ```rust,ignore
		/// use winsafe::gui::{WindowMain, WindowMainOpts};
		///
		/// let wnd = WindowMain::new(
		///   WindowMainOpts::default(),
		/// );
		///
		/// wnd.on().wm_size({
		///   let wnd = wnd.clone();
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
}
