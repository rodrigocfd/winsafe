use std::cell::UnsafeCell;
use std::rc::Rc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, privs::*};
use crate::msg::*;
use crate::prelude::*;

/// Exposes window
/// [messages](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
///
/// You cannot directly instantiate this object, it is created internally by the
/// window.
pub struct WindowEvents {
	is_dialog: bool,
	msgs: UnsafeCell<
		FuncStore< // ordinary WM messages
			co::WM,
			Box<dyn Fn(WndMsg) -> AnyResult<WmRet>>,
		>,
	>,
	inis: UnsafeCell<
		FuncStore< // WM_CREATE and WM_INITDIALOG messages
			co::WM,
			Box<dyn Fn(&HWND, WndMsg) -> AnyResult<WmRet>>,
		>,
	>,
	cmds: UnsafeCell<
		FuncStore< // WM_COMMAND notifications
			(u16, co::CMD), // control ID, notif code
			Box<dyn Fn() -> AnyResult<WmRet>>,
		>,
	>,
	nfys: UnsafeCell<
		FuncStore< // WM_NOTIFY notifications
			(u16, NmhdrCode), // idFrom, code
			Box<dyn Fn(wm::Notify) -> AnyResult<WmRet>>,
		>,
	>,
	tmrs: UnsafeCell<
		FuncStore< // WM_TIMER messages
			usize, // timer ID
			Box<dyn Fn() -> AnyResult<()>>, // return value is never meaningful
		>,
	>,
}

impl WindowEvents {
	#[must_use]
	pub(in crate::gui) const fn new(is_dialog: bool) -> Self {
		Self {
			is_dialog,
			msgs: UnsafeCell::new(FuncStore::new()),
			inis: UnsafeCell::new(FuncStore::new()),
			cmds: UnsafeCell::new(FuncStore::new()),
			nfys: UnsafeCell::new(FuncStore::new()),
			tmrs: UnsafeCell::new(FuncStore::new()),
		}
	}

	pub(in crate::gui) fn is_empty(&self) -> bool {
		unsafe {
			{ &*self.msgs.get() }.is_empty()
				&& { &*self.inis.get() }.is_empty()
				&& { &*self.cmds.get() }.is_empty()
				&& { &*self.nfys.get() }.is_empty()
				&& { &*self.tmrs.get() }.is_empty()
		}
	}

	pub(in crate::gui) fn clear_events(&self) {
		unsafe {
			{ &mut *self.tmrs.get() }.clear();
			{ &mut *self.nfys.get() }.clear();
			{ &mut *self.cmds.get() }.clear();
			{ &mut *self.inis.get() }.clear();
			{ &mut *self.msgs.get() }.clear();
		}
	}

	/// Searches for all functions for the given message, and runs all of them,
	/// discarding the results.
	///
	/// Returns `true` if at least one message was processed.
	pub(in crate::gui) fn process_all_messages(&self,
		hwnd: &HWND,
		wm_any: WndMsg,
	) -> AnyResult<bool>
	{
		let mut at_least_one = false;

		if wm_any.msg_id == co::WM::CREATE || wm_any.msg_id == co::WM::INITDIALOG {
			let inis = unsafe { &*self.inis.get() };
			for func in inis.filter(wm_any.msg_id) {
				match func(hwnd, wm_any)? {
					WmRet::HandledWithRet(_)
						| WmRet::HandledOk => { at_least_one = true; }
					_ => {},
				}
			}
		} else if wm_any.msg_id == co::WM::COMMAND {
			let wm_cmd = unsafe { wm::Command::from_generic_wm(wm_any) };
			let key_cmd = (wm_cmd.event.ctrl_id(), wm_cmd.event.code());
			let cmds = unsafe { &*self.cmds.get() };
			for func in cmds.filter(key_cmd) {
				match func()? {
					WmRet::HandledWithRet(_)
						| WmRet::HandledOk => { at_least_one = true; }
					_ => {},
				}
			}
		} else if wm_any.msg_id == co::WM::NOTIFY {
			let wm_nfy = unsafe { wm::Notify::from_generic_wm(wm_any) };
			let key_nfy = (wm_nfy.nmhdr.idFrom(), wm_nfy.nmhdr.code);
			let nfys = unsafe { &*self.nfys.get() };
			for func in nfys.filter(key_nfy) {
				match func(unsafe { wm::Notify::from_generic_wm(wm_any) })? { // wm::Notify cannot be Copy
					WmRet::HandledWithRet(_)
						| WmRet::HandledOk => { at_least_one = true; }
					_ => {},
				}
			}
		} else if wm_any.msg_id == co::WM::TIMER {
			let wm_tmr = unsafe { wm::Timer::from_generic_wm(wm_any) };
			let tmrs = unsafe { &*self.tmrs.get() };
			for func in tmrs.filter(wm_tmr.timer_id) {
				func()?;
				at_least_one = true;
			}
		}

		let msgs = unsafe { &*self.msgs.get() };
		for func in msgs.filter(wm_any.msg_id) {
			match func(wm_any)? {
				WmRet::HandledWithRet(_)
					| WmRet::HandledOk => { at_least_one = true; }
				_ => {},
			}
		}
		Ok(at_least_one)
	}

	/// Searches for the last added user function for the given message, and
	/// runs if it exists, returning the result.
	pub(in crate::gui) fn process_last_message(&self,
		hwnd: &HWND,
		wm_any: WndMsg,
	) -> AnyResult<WmRet>
	{
		if wm_any.msg_id == co::WM::CREATE || wm_any.msg_id == co::WM::INITDIALOG {
			let inis = unsafe { &*self.inis.get() };
			for func in inis.filter_rev(wm_any.msg_id) {
				match func(hwnd, wm_any)? {
					WmRet::NotHandled => {},
					r => return Ok(r), // handled: stop here
				}
			}
		} else if wm_any.msg_id == co::WM::COMMAND {
			let wm_cmd = unsafe { wm::Command::from_generic_wm(wm_any) };
			let key_cmd = (wm_cmd.event.ctrl_id(), wm_cmd.event.code());
			let cmds = unsafe { &*self.cmds.get() };
			for func in cmds.filter_rev(key_cmd) {
				match func()? {
					WmRet::NotHandled => {},
					r => return Ok(r), // handled: stop here
				}
			}
		} else if wm_any.msg_id == co::WM::NOTIFY {
			let wm_nfy = unsafe { wm::Notify::from_generic_wm(wm_any) };
			let key_nfy = (wm_nfy.nmhdr.idFrom(), wm_nfy.nmhdr.code);
			let nfys = unsafe { &*self.nfys.get() };
			for func in nfys.filter_rev(key_nfy) {
				match unsafe { func(wm::Notify::from_generic_wm(wm_any))? } { // wm::Notify cannot be Copy
					WmRet::NotHandled => {},
					r => return Ok(r), // handled: stop here
				}
			}
		} else if wm_any.msg_id == co::WM::TIMER {
			let wm_tmr = unsafe { wm::Timer::from_generic_wm(wm_any) };
			let tmrs = unsafe { &*self.tmrs.get() };
			if let Some(func) = tmrs.filter_rev(wm_tmr.timer_id).next() { // just execute the last, if any
				func()?;
				return Ok(WmRet::HandledOk); // handled: stop here
			}
		}

		let msgs = unsafe { &*self.msgs.get() };
		for func in msgs.filter_rev(wm_any.msg_id) {
			match func(wm_any)? {
				WmRet::NotHandled => {},
				r => return Ok(r), // handled: stop here
			}
		}
		Ok(WmRet::NotHandled)
	}

//------------------------------------------------------------------------------

	/// Event to any [window message](crate::co::WM).
	///
	/// Instead of using this event, you should always prefer the specific
	/// events, which will give you the correct message parameters. This generic
	/// method should be used only when you have a custom, non-standard window
	/// message – which should be pretty rare.
	///
	/// # Examples
	///
	/// Handling a custom, user-defined message:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co, gui, msg};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// let CUSTOM_MSG = unsafe { co::WM::from_raw(0x1234) };
	///
	/// wnd.on().wm(
	///     CUSTOM_MSG,
	///     move |p: msg::WndMsg| -> w::AnyResult<gui::WmRet> {
	///         println!("Msg ID: {}", p.msg_id);
	///         Ok(gui::WmRet::HandledOk)
	///     },
	/// );
	/// ```
	pub fn wm<F>(&self, ident: co::WM, func: F)
		where F: Fn(WndMsg) -> AnyResult<WmRet> + 'static,
	{
		unsafe { &mut *self.msgs.get() }.push(ident, Box::new(func));
	}

	/// If a dialog window, will handle `co::WM::INITDIALOG`, otherwise will
	/// handle `co::WM::CREATE`.
	pub(in crate::gui) fn wm_create_or_initdialog<F>(&self, func: F)
		where F: Fn(&HWND, WndMsg) -> AnyResult<WmRet> + 'static,
	{
		unsafe { &mut *self.inis.get() }.push(
			if self.is_dialog { co::WM::INITDIALOG } else { co::WM::CREATE },
			Box::new(func),
		);
	}

	/// [`WM_COMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-command)
	/// message, for specific code and control ID.
	///
	/// A command notification must be narrowed by the
	/// [command code](crate::co::CMD) and the control ID, so the closure will
	/// be fired for that specific control at that specific event.
	///
	/// Instead of using this event, you should always prefer the specific
	/// command notifications, which will give you the correct message
	/// parameters. This generic method should be used only when you have a
	/// custom, non-standard window notification.
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// const CTRL_ID: u16 = 1010;
	///
	/// wnd.on().wm_command(
	///     CTRL_ID,
	///     co::BN::CLICKED,
	///     move || -> w::AnyResult<gui::WmRet> {
	///         println!("Button clicked!");
	///         Ok(gui::WmRet::HandledOk)
	///     },
	/// );
	/// ```
	pub fn wm_command<F>(&self,
		ctrl_id: impl Into<u16>,
		code: impl Into<co::CMD>,
		func: F,
	)
		where F: Fn() -> AnyResult<WmRet> + 'static,
	{
		let code: co::CMD = code.into();
		unsafe { &mut *self.cmds.get() }.push(
			(ctrl_id.into(), code),
			Box::new(func),
		);
	}

	/// [`WM_NOTIFY`](crate::msg::wm::Notify) message, for specific ID and
	/// notification code.
	///
	/// Instead of using this event, you should always prefer the specific
	/// notifications, which will give you the correct notification struct. This
	/// generic method should be used only when you have a custom, non-standard
	/// window notification.
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// const CTRL_ID: u16 = 1010;
	///
	/// wnd.on().wm_notify(
	///     CTRL_ID,
	///     co::NM::DBLCLK,
	///     move |_| -> w::AnyResult<gui::WmRet> {
	///         println!("Status bar double clicked!");
	///         Ok(gui::WmRet::HandledOk)
	///     },
	/// );
	/// ```
	pub fn wm_notify<F>(&self,
		id_from: impl Into<u16>,
		code: impl Into<NmhdrCode>,
		func: F,
	)
		where F: Fn(wm::Notify) -> AnyResult<WmRet> + 'static,
	{
		unsafe { &mut *self.nfys.get() }.push(
			(id_from.into(), code.into()),
			Box::new(func),
		);
	}

	/// [`WM_TIMER`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-timer)
	/// message, narrowed to a specific timer ID.
	pub fn wm_timer<F>(&self, timer_id: usize, func: F)
		where F: Fn() -> AnyResult<()> + 'static,
	{
		unsafe { &mut *self.tmrs.get() }.push(timer_id, Box::new(func));
	}

//------------------------------------------------------------------------------

	/// [`WM_CREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create)
	/// message, sent only to non-dialog windows. Dialog windows must handle
	/// [`wm_init_dialog`](crate::gui::events::WindowEvents::wm_init_dialog)
	/// instead.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui, msg};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// wnd.on().wm_create(
	///     move |p: msg::wm::Create| -> w::AnyResult<i32> {
	///         println!("Client area: {}x{}",
	///             p.createstruct.cx,
	///             p.createstruct.cy,
	///         );
	///         Ok(0)
	///     },
	/// );
	/// ```
	pub fn wm_create<F>(&self, func: F)
		where F: Fn(wm::Create) -> AnyResult<i32> + 'static,
	{
		unsafe { &mut *self.inis.get() }.push(
			co::WM::CREATE,
			Box::new(move |_, p| {
				let ret_val = func(unsafe { wm::Create::from_generic_wm(p) })? as isize;
				Ok(WmRet::HandledWithRet(ret_val))
			}),
		);
	}

	/// [`WM_INITDIALOG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
	/// message, sent only to dialog windows. Non-dialog windows must handle
	/// [`wm_create`](crate::gui::events::WindowEvents::wm_create) instead.
	///
	/// Return `true` to set the focus to the first control in the dialog.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui, msg};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// wnd.on().wm_init_dialog(
	///     move |p: msg::wm::InitDialog| -> w::AnyResult<bool> {
	///         println!("Focused HWND: {}", p.hwnd_focus);
	///         Ok(true)
	///     },
	/// );
	/// ```
	pub fn wm_init_dialog<F>(&self, func: F)
		where F: Fn(wm::InitDialog) -> AnyResult<bool> + 'static,
	{
		unsafe { &mut *self.inis.get() }.push(
			co::WM::INITDIALOG,
			Box::new(move |_, p| {
				let ret_val = func(unsafe { wm::InitDialog::from_generic_wm(p) })? as isize;
				Ok(WmRet::HandledWithRet(ret_val))
			}),
		);
	}

	/// [`WM_COMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-command)
	/// message, handling both `CMD::Accelerator` and `CMD::Menu`, for a
	/// specific command ID.
	///
	/// Ideal to be used with menu commands whose IDs are shared with
	/// accelerators, like menu items.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// const CTRL_ID: u16 = 1010;
	///
	/// wnd.on().wm_command_accel_menu(
	///     CTRL_ID,
	///     move || -> w::AnyResult<()> {
	///         println!("Hello!");
	///         Ok(())
	///     },
	/// );
	/// ```
	pub fn wm_command_accel_menu<F>(&self, ctrl_id: impl Into<u16> + Copy, func: F)
		where F: Fn() -> AnyResult<()> + 'static,
	{
		let shared_func = Rc::new(func);

		self.wm_command(ctrl_id, co::CMD::Menu, {
			let shared_func = shared_func.clone();
			move || {
				shared_func()?;
				Ok(WmRet::HandledOk)
			}
		});

		self.wm_command(ctrl_id, co::CMD::Accelerator, {
			let shared_func = shared_func.clone();
			move || {
				shared_func()?;
				Ok(WmRet::HandledOk)
			}
		});
	}

//------------------------------------------------------------------------------

	pub_fn_wm_withparm_noret! { wm_activate, co::WM::ACTIVATE, wm::Activate;
		/// [`WM_ACTIVATE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-activate)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_activate_app, co::WM::ACTIVATEAPP, wm::ActivateApp;
		/// [`WM_ACTIVATEAPP`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-activateapp)
		/// message.
	}

	/// [`WM_APPCOMMAND`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-appcommand)
	/// message.
	pub fn wm_app_command<F>(&self, func: F)
		where F: Fn(wm::AppCommand) -> AnyResult<()> + 'static,
	{
		self.wm(co::WM::APPCOMMAND, move |p| {
			func(unsafe { wm::AppCommand::from_generic_wm(p) })?;
			Ok(WmRet::HandledWithRet(1)) // TRUE
		});
	}

	pub_fn_wm_noparm_noret! { wm_cancel_mode, co::WM::CANCELMODE;
		/// [`WM_CANCELMODE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-cancelmode)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_capture_changed, co::WM::CAPTURECHANGED, wm::CaptureChanged;
		/// [`WM_CAPTURECHANGED`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-capturechanged)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_char, co::WM::CHAR, wm::Char;
		/// [`WM_CHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-char)
		/// message.
	}

	pub_fn_wm_noparm_noret! { wm_child_activate, co::WM::CHILDACTIVATE;
		/// [`WM_CHILDACTIVATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-childactivate)
		/// message.
	}

	pub_fn_wm_noparm_noret! { wm_close, co::WM::CLOSE;
		/// [`WM_CLOSE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-close)
		/// message.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * dialog [`WindowMain`](crate::gui::WindowMain) – calls [`DestroyWindow`](crate::prelude::user_Hwnd::DestroyWindow);
		/// * dialog [`WindowModal`](crate::gui::WindowModal) – calls [`EndDialog`](crate::prelude::user_Hwnd::EndDialog);
		/// * non-dialog [`WindowModal`](crate::gui::WindowModal) – re-enables parent and calls [`DestroyWindow`](crate::prelude::user_Hwnd::DestroyWindow).
	}

	pub_fn_wm_noparm_noret! { wm_context_menu, co::WM::CONTEXTMENU;
		/// [`WM_CONTEXTMENU`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-contextmenu)
		/// message.
	}

	pub_fn_wm_ctlcolor! { wm_ctl_color_btn, co::WM::CTLCOLORBTN, wm::CtlColorBtn;
		/// [`WM_CTLCOLORBTN`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorbtn)
		/// message.
	}

	pub_fn_wm_ctlcolor! { wm_ctl_color_dlg, co::WM::CTLCOLORDLG, wm::CtlColorDlg;
		/// [`WM_CTLCOLORDLG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-ctlcolordlg)
		/// message.
	}

	pub_fn_wm_ctlcolor! { wm_ctl_color_edit, co::WM::CTLCOLOREDIT, wm::CtlColorEdit;
		/// [`WM_CTLCOLOREDIT`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcoloredit)
		/// message.
	}

	pub_fn_wm_ctlcolor! { wm_ctl_color_list_box, co::WM::CTLCOLORLISTBOX, wm::CtlColorListBox;
		/// [`WM_CTLCOLORLISTBOX`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorlistbox)
		/// message.
	}

	pub_fn_wm_ctlcolor! { wm_ctl_color_scroll_bar, co::WM::CTLCOLORSCROLLBAR, wm::CtlColorScrollBar;
		/// [`WM_CTLCOLORSCROLLBAR`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorscrollbar)
		/// message.
	}

	pub_fn_wm_ctlcolor! { wm_ctl_color_static, co::WM::CTLCOLORSTATIC, wm::CtlColorStatic;
		/// [`WM_CTLCOLORSTATIC`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorstatic)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_dead_char, co::WM::DEADCHAR, wm::DeadChar;
		/// [`WM_DEADCHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-deadchar)
		/// message.
	}

	pub_fn_wm_withparm_boolret! { wm_delete_item, co::WM::DELETEITEM, wm::DeleteItem;
		/// [`WM_DELETEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-deleteitem)
		/// message.
	}

	pub_fn_wm_noparm_noret! { wm_destroy, co::WM::DESTROY;
		/// [`WM_DESTROY`](crate::msg::wm::Destroy) message.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// wnd.on().wm_destroy(
		///     move || -> w::AnyResult<()> {
		///         println!("Window is gone, goodbye!");
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	pub_fn_wm_withparm_noret! { wm_device_change, co::WM::DEVICECHANGE, wm::DeviceChange;
		/// [`WM_DEVICECHANGE`](https://learn.microsoft.com/en-us/windows/win32/devio/wm-devicechange)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_display_change, co::WM::DISPLAYCHANGE, wm::DisplayChange;
		/// [`WM_DISPLAYCHANGE`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-displaychange)
		/// message.
	}

	#[cfg(feature = "shell")]
	pub_fn_wm_withparm_noret! { wm_drop_files, co::WM::DROPFILES, wm::DropFiles;
		/// [`WM_DROPFILES`](https://learn.microsoft.com/en-us/windows/win32/shell/wm-dropfiles)
		/// message.
		///
		/// **Note:** To use this method, enable the `shell`
		/// [Cargo feature](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section).
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui, msg};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// wnd.on().wm_drop_files(
		///     move |p: msg::wm::DropFiles| -> w::AnyResult<()> {
		///         for dropped_file in p.hdrop.DragQueryFile()? {
		///             let dropped_file = dropped_file?;
		///             println!("Dropped: {}", dropped_file);
		///         }
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	pub_fn_wm_withparm_noret! { wm_enable, co::WM::ENABLE, wm::Enable;
		/// [`WM_ENABLE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-enable)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_end_session, co::WM::ENDSESSION, wm::EndSession;
		/// [`WM_ENDSESSION`](https://learn.microsoft.com/en-us/windows/win32/shutdown/wm-endsession)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_enter_idle, co::WM::ENTERIDLE, wm::EnterIdle;
		/// [`WM_ENTERIDLE`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-enteridle)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_enter_menu_loop, co::WM::ENTERMENULOOP, wm::EnterMenuLoop;
		/// [`WM_ENTERMENULOOP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-entermenuloop)
		/// message.
	}

	pub_fn_wm_noparm_noret! { wm_enter_size_move, co::WM::ENTERSIZEMOVE;
		/// [`WM_ENTERSIZEMOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-entersizemove)
		/// message.
	}

	/// [`WM_ERASEBKGND`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-erasebkgnd)
	/// message.
	pub fn wm_erase_bkgnd<F>(&self, func: F)
		where F: Fn(wm::EraseBkgnd) -> AnyResult<i32> + 'static,
	{
		self.wm(co::WM::ERASEBKGND, move |p| {
			let ret_val = func(unsafe { wm::EraseBkgnd::from_generic_wm(p) })? as isize;
			Ok(WmRet::HandledWithRet(ret_val))
		});
	}

	pub_fn_wm_withparm_noret! { wm_exit_menu_loop, co::WM::EXITMENULOOP, wm::ExitMenuLoop;
		/// [`WM_EXITMENULOOP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-exitmenuloop)
		/// message.
	}

	pub_fn_wm_noparm_noret! { wm_exit_size_move, co::WM::EXITSIZEMOVE;
		/// [`WM_EXITSIZEMOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-exitsizemove)
		/// message.
	}

	pub_fn_wm_withparm_coret! { wm_get_dlg_code, co::WM::GETDLGCODE, wm::GetDlgCode, co::DLGC;
		/// [`WM_GETDLGCODE`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-getdlgcode)
		/// message.
	}

	/// [`WM_GETFONT`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-getfont)
	/// message.
	pub fn wm_get_font<F>(&self, func: F)
		where F: Fn() -> AnyResult<Option<HFONT>> + 'static,
	{
		self.wm(co::WM::GETFONT, move |_| {
			let ret_val = func()?.map_or(0, |h| h.ptr() as isize);
			Ok(WmRet::HandledWithRet(ret_val))
		});
	}

	/// [`WM_GETHMENU`](https://learn.microsoft.com/en-us/windows/win32/winmsg/mn-gethmenu)
	/// message. Originally has `MN` prefix.
	pub fn wm_get_hmenu<F>(&self, func: F)
		where F: Fn() -> AnyResult<Option<HMENU>> + 'static
	{
		self.wm(co::WM::MN_GETHMENU, move |_| {
			let ret_val = func()?.map_or(0, |h| h.ptr() as isize);
			Ok(WmRet::HandledWithRet(ret_val))
		});
	}

	pub_fn_wm_withparm_noret! { wm_get_min_max_info, co::WM::GETMINMAXINFO, wm::GetMinMaxInfo;
		/// [`WM_GETMINMAXINFO`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-getminmaxinfo)
		/// message.
	}

	/// [`WM_GETTEXT`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-gettext)
	/// message.
	pub fn wm_get_text<F>(&self, func: F)
		where F: Fn(wm::GetText) -> AnyResult<u32> + 'static,
	{
		self.wm(co::WM::GETTEXT, move |p| {
			let ret_val = func(unsafe { wm::GetText::from_generic_wm(p) })? as isize;
			Ok(WmRet::HandledWithRet(ret_val))
		});
	}

	/// [`WM_GETTEXTLENGTH`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-gettextlength)
	/// message.
	pub fn wm_get_text_length<F>(&self, func: F)
		where F: Fn() -> AnyResult<u32> + 'static,
	{
		self.wm(co::WM::GETTEXTLENGTH, move |_| {
			let ret_val = func()? as isize;
			Ok(WmRet::HandledWithRet(ret_val))
		});
	}

	pub_fn_wm_withparm_noret! { wm_get_title_bar_info_ex, co::WM::GETTITLEBARINFOEX, wm::GetTitleBarInfoEx;
		/// [`WM_GETTITLEBARINFOEX`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-gettitlebarinfoex)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_h_scroll, co::WM::HSCROLL, wm::HScroll;
		/// [`WM_HSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-hscroll)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_help, co::WM::HELP, wm::Help;
		/// [`WM_HELP`](https://learn.microsoft.com/en-us/windows/win32/shell/wm-help)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_init_menu_popup, co::WM::INITMENUPOPUP, wm::InitMenuPopup;
		/// [`WM_INITMENUPOPUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-initmenupopup)
		/// message.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui, msg};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// wnd.on().wm_init_menu_popup(
		///     move |p: msg::wm::InitMenuPopup| -> w::AnyResult<()> {
		///         if p.hmenu.GetMenuItemID(0).unwrap() == 3001 { // check ID of 1st item
		///             p.hmenu.EnableMenuItem(w::IdPos::Id(3001), false)?;
		///         }
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	pub_fn_wm_withparm_noret! { wm_key_down, co::WM::KEYDOWN, wm::KeyDown;
		/// [`WM_KEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keydown)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_key_up, co::WM::KEYUP, wm::KeyUp;
		/// [`WM_KEYUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keyup)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_kill_focus, co::WM::KILLFOCUS, wm::KillFocus;
		/// [`WM_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-killfocus)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_l_button_dbl_clk, co::WM::LBUTTONDBLCLK, wm::LButtonDblClk;
		/// [`WM_LBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondblclk)
		/// message.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui, msg};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// wnd.on().wm_l_button_dbl_clk(
		///     move |p: msg::wm::LButtonDblClk| -> w::AnyResult<()> {
		///         println!("Point: {}x{}", p.coords.x, p.coords.y);
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	pub_fn_wm_withparm_noret! { wm_l_button_down, co::WM::LBUTTONDOWN, wm::LButtonDown;
		/// [`WM_LBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown)
		/// message.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui, msg};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// wnd.on().wm_l_button_down(
		///     move |p: msg::wm::LButtonDown| -> w::AnyResult<()> {
		///         println!("Point: {}x{}", p.coords.x, p.coords.y);
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	pub_fn_wm_withparm_noret! { wm_l_button_up, co::WM::LBUTTONUP, wm::LButtonUp;
		/// [`WM_LBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttonup)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_m_button_dbl_clk, co::WM::MBUTTONDBLCLK, wm::MButtonDblClk;
		/// [`WM_MBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondblclk)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_m_button_down, co::WM::MBUTTONDOWN, wm::MButtonDown;
		/// [`WM_MBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondown)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_m_button_up, co::WM::MBUTTONUP, wm::MButtonUp;
		/// [`WM_MBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttonup)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_menu_command, co::WM::MENUCOMMAND, wm::MenuCommand;
		/// [`WM_MENUCOMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menucommand)
		/// message.
	}

	pub_fn_wm_withparm_coret! { wm_menu_drag, co::WM::MENUDRAG, wm::MenuDrag, co::MND;
		/// [`WM_MENUDRAG`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menudrag)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_menu_r_button_up, co::WM::MENURBUTTONUP, wm::MenuRButtonUp;
		/// [`WM_MENURBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menurbuttonup)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_mouse_h_wheel, co::WM::MOUSEHWHEEL, wm::MouseHWheel;
		/// [`WM_MOUSEHWHEEL`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousehwheel)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_mouse_hover, co::WM::MOUSEHOVER, wm::MouseHover;
		/// [`WM_MOUSEHOVER`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousehover)
		/// message.
	}

	pub_fn_wm_noparm_noret! { wm_mouse_leave, co::WM::MOUSELEAVE;
		/// [`WM_MOUSELEAVE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mouseleave)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_mouse_move, co::WM::MOUSEMOVE, wm::MouseMove;
		/// [`WM_MOUSEMOVE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_move, co::WM::MOVE, wm::Move;
		/// [`WM_MOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-move)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_mouse_wheel, co::WM::MOUSEWHEEL, wm::MouseWheel;
		/// [`WM_MOUSEWHEEL`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_moving, co::WM::MOVING, wm::Moving;
		/// [`WM_MOVING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-moving)
		/// message.
	}

	pub_fn_wm_withparm_coret! { wm_nc_calc_size, co::WM::NCCALCSIZE, wm::NcCalcSize, co::WVR;
		/// [`WM_NCCALCSIZE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize)
		/// message.
	}

	pub_fn_wm_withparm_boolret! { wm_nc_create, co::WM::NCCREATE, wm::NcCreate;
		/// [`WM_NCCREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccreate)
		/// message.
	}

	pub_fn_wm_noparm_noret! { wm_nc_destroy, co::WM::NCDESTROY;
		/// [`WM_NCDESTROY`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-ncdestroy)
		/// message.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * non-dialog [`WindowMain`](crate::gui::WindowMain);
		/// * dialog [`WindowMain`](crate::gui::WindowMain).
		///
		/// In both cases, [`PostQuitMessage`](crate::PostQuitMessage) is called.
	}

	pub_fn_wm_withparm_coret! { wm_nc_hit_test, co::WM::NCHITTEST, wm::NcHitTest, co::HT;
		/// [`WM_NCHITTEST`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-nchittest)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_nc_paint, co::WM::NCPAINT, wm::NcPaint;
		/// [`WM_NCPAINT`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-ncpaint)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_next_dlg_ctl, co::WM::NEXTDLGCTL, wm::NextDlgCtl;
		/// [`WM_NEXTDLGCTL`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-nextdlgctl)
		/// message.
	}

	pub_fn_wm_noparm_noret! { wm_null, co::WM::NULL;
		/// [`WM_NULL`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-null)
		/// message.
	}

	pub_fn_wm_noparm_noret! { wm_paint, co::WM::PAINT;
		/// [`WM_PAINT`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-paint)
		/// message.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// let wnd2 = wnd.clone(); // to pass into the closure
		///
		/// wnd.on().wm_paint(
		///     move || -> w::AnyResult<()> {
		///         let hdc = wnd2.hwnd().BeginPaint()?;
		///
		///         // hdc painting...
		///
		///         Ok(())
		///
		///         // EndPaint() automatically called
		///     },
		/// );
		/// ```
	}

	pub_fn_wm_withparm_noret! { wm_parent_notify, co::WM::PARENTNOTIFY, wm::ParentNotify;
		/// [`WM_PARENTNOTIFY`](https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-parentnotify)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_power_broadcast, co::WM::POWERBROADCAST, wm::PowerBroadcast;
		/// [`WM_POWERBROADCAST`](https://learn.microsoft.com/en-us/windows/win32/power/wm-powerbroadcast)
		/// message.
	}

	pub_fn_wm_noparm_boolret! { wm_query_open, co::WM::QUERYOPEN;
		/// [`WM_QUERYOPEN`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-queryopen)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_r_button_dbl_clk, co::WM::RBUTTONDBLCLK, wm::RButtonDblClk;
		/// [`WM_RBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondblclk)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_r_button_down, co::WM::RBUTTONDOWN, wm::RButtonDown;
		/// [`WM_RBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondown)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_r_button_up, co::WM::RBUTTONUP, wm::RButtonUp;
		/// [`WM_RBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttonup)
	}

	pub_fn_wm_withparm_boolret! { wm_set_cursor, co::WM::SETCURSOR, wm::SetCursor;
		/// [`WM_SETCURSOR`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-setcursor)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_set_focus, co::WM::SETFOCUS, wm::SetFocus;
		/// [`WM_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-setfocus)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_set_font, co::WM::SETFONT, wm::SetFont;
		/// [`WM_SETFONT`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-setfont)
		/// message.
	}

	/// [`WM_SETICON`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-seticon)
	/// message.
	pub fn wm_set_icon<F>(&self, func: F)
		where F: Fn(wm::SetIcon) -> AnyResult<Option<HICON>> + 'static,
	{
		self.wm(co::WM::SETICON, move |p| {
			let ret_val = func(unsafe { wm::SetIcon::from_generic_wm(p) })?
				.map_or(0, |h| h.ptr() as isize);
			Ok(WmRet::HandledWithRet(ret_val))
		});
	}

	pub_fn_wm_withparm_noret! { wm_set_redraw, co::WM::SETREDRAW, wm::SetRedraw;
		/// [`WM_SETREDRAW`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-setredraw)
		/// message.
	}

	pub_fn_wm_withparm_boolret! { wm_set_text, co::WM::SETTEXT, wm::SetText;
		/// [`WM_SETTEXT`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-settext)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_show_window, co::WM::SHOWWINDOW, wm::ShowWindow;
		/// [`WM_SHOWWINDOW`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_size, co::WM::SIZE, wm::Size;
		/// [`WM_SIZE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-size)
		/// message.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui, msg};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// wnd.on().wm_size(
		///     move |p: msg::wm::Size| -> w::AnyResult<()> {
		///         println!("Client area: {}x{}",
		///             p.client_area.cx,
		///             p.client_area.cy,
		///         );
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	pub_fn_wm_withparm_noret! { wm_sizing, co::WM::SIZING, wm::Sizing;
		/// [`WM_SIZING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-sizing)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_style_changed, co::WM::STYLECHANGED, wm::StyleChanged;
		/// [`WM_STYLECHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanged)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_style_changing, co::WM::STYLECHANGING, wm::StyleChanging;
		/// [`WM_STYLECHANGING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanging)
		/// message.
	}

	pub_fn_wm_noparm_noret! { wm_sync_paint, co::WM::SYNCPAINT;
		/// [`WM_SYNCPAINT`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-syncpaint)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_sys_char, co::WM::SYSCHAR, wm::SysChar;
		/// [`WM_SYSCHAR`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syschar)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_sys_command, co::WM::SYSCOMMAND, wm::SysCommand;
		/// [`WM_SYSCOMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syscommand)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_sys_dead_char, co::WM::SYSDEADCHAR, wm::SysDeadChar;
		/// [`WM_SYSDEADCHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-sysdeadchar)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_sys_key_down, co::WM::SYSKEYDOWN, wm::SysKeyDown;
		/// [`WM_SYSKEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-syskeydown)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_sys_key_up, co::WM::SYSKEYUP, wm::SysKeyUp;
		/// [`WM_SYSKEYUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-syskeyup)
		/// message.
	}

	pub_fn_wm_noparm_noret! { wm_theme_changed, co::WM::THEMECHANGED;
		/// [`WM_THEMECHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-themechanged)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_uninit_menu_popup, co::WM::UNINITMENUPOPUP, wm::UninitMenuPopup;
		/// [`WM_UNINITMENUPOPUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-uninitmenupopup)
		/// message.
	}

	pub_fn_wm_noparm_boolret! { wm_undo, co::WM::UNDO;
		/// [`WM_UNDO`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-undo)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_v_scroll, co::WM::VSCROLL, wm::VScroll;
		/// [`WM_VSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-vscroll)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_window_pos_changed, co::WM::WINDOWPOSCHANGED, wm::WindowPosChanged;
		/// [`WM_WINDOWPOSCHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanged)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_window_pos_changing, co::WM::WINDOWPOSCHANGING, wm::WindowPosChanging;
		/// [`WM_WINDOWPOSCHANGING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanging)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_wts_session_change, co::WM::WTSSESSION_CHANGE, wm::WtsSessionChange;
		/// [`WM_WTSSESSION_CHANGE`](https://learn.microsoft.com/en-us/windows/win32/termserv/wm-wtssession-change)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_x_button_dbl_clk, co::WM::XBUTTONDBLCLK, wm::XButtonDblClk;
		/// [`WM_XBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondblclk)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_x_button_down, co::WM::XBUTTONDOWN, wm::XButtonDown;
		/// [`WM_XBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondown)
		/// message.
	}

	pub_fn_wm_withparm_noret! { wm_x_button_up, co::WM::XBUTTONUP, wm::XButtonUp;
		/// [`WM_XBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttonup)
		/// message.
	}
}
