use std::rc::Rc;

use crate::co;
use crate::gui::events::func_store::FuncStore;
use crate::gui::immut::Immut;
use crate::handles::{HDC, HICON};
use crate::msg;
use crate::msg::MessageHandleable;

/// The result of processing a message.
pub enum ProcessResult {
	NotHandled,            // message was not handler because no such handler is stored
	HandledWithRet(isize), // return value is meaningful
	HandledWithoutRet,     // return value is not meaningful, whatever default value
}

//------------------------------------------------------------------------------

/// Exposes window
/// [messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
///
/// You cannot directly instantiate this object, it is created internally by the
/// window.
pub struct MsgEvents(Immut<Obj>);

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
		Self(
			Immut::new(
				Obj {
					msgs: FuncStore::new(),
					tmrs: FuncStore::new(),
					cmds: FuncStore::new(),
					nfys: FuncStore::new(),
				},
			),
		)
	}

	/// Tells whether no functions have been added.
	pub(crate) fn is_empty(&self) -> bool {
		self.0.msgs.is_empty()
			&& self.0.tmrs.is_empty()
			&& self.0.cmds.is_empty()
			&& self.0.nfys.is_empty()
	}

	/// Searches for the last added user function for the given message, and runs
	/// if it exists, returning the result.
	pub(crate) fn process_effective_message(&self, wm_any: msg::Wm) -> ProcessResult {
		match wm_any.msg_id {
			co::WM::NOTIFY => {
				let wm_nfy = msg::WmNotify::from_generic_wm(wm_any);
				let key = (wm_nfy.nmhdr.idFrom as u16, wm_nfy.nmhdr.code);
				match self.0.as_mut().nfys.find(key) {
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
				match self.0.as_mut().cmds.find(key) {
					Some(func) => { // we have a stored function to handle this WM_COMMAND notification
						func(); // execute user function
						ProcessResult::HandledWithoutRet
					},
					None => ProcessResult::NotHandled, // no stored WM_COMMAND notification
				}
			},
			co::WM::TIMER => {
				let wm_tmr = msg::WmTimer::from_generic_wm(wm_any);
				match self.0.as_mut().tmrs.find(wm_tmr.timer_id) {
					Some(func) => { // we have a stored function to handle this WM_TIMER message
						func(); // execute user function
						ProcessResult::HandledWithoutRet
					},
					None => ProcessResult::NotHandled, // no stored WM_TIMER message
				}
			}
			_ => { // any other message
				match self.0.as_mut().msgs.find(wm_any.msg_id) {
					Some(func) => { // we have a stored function to handle this message
						match func(wm_any) { // execute user function
							Some(res) => ProcessResult::HandledWithRet(res), // meaningful return value
							None => ProcessResult::HandledWithoutRet,
						}
					},
					None => ProcessResult::NotHandled, // no stored function
				}
			},
		}
	}

	/// Searches for all user functions for the given message, and runs all of
	/// them, discarding the results.
	pub(crate) fn process_all_messages(&self, wm_any: msg::Wm) {
		match wm_any.msg_id {
			co::WM::NOTIFY => {
				let wm_nfy = msg::WmNotify::from_generic_wm(wm_any);
				let key = (wm_nfy.nmhdr.idFrom as u16, wm_nfy.nmhdr.code);
				self.0.as_mut().nfys.find_all(key, |func| {
					func(wm_nfy);
				});
			},
			co::WM::COMMAND => {
				let wm_cmd = msg::WmCommand::from_generic_wm(wm_any);
				let key = (wm_cmd.code, wm_cmd.ctrl_id);
				self.0.as_mut().cmds.find_all(key, |func| {
					func();
				});
			},
			co::WM::TIMER => {
				let wm_tmr = msg::WmTimer::from_generic_wm(wm_any);
				self.0.as_mut().tmrs.find_all(wm_tmr.timer_id, |func| {
					func();
				});
			},
			_ => { // any other message
				self.0.as_mut().msgs.find_all(wm_any.msg_id, |func| {
					func(wm_any);
				});
			},
		}
	}

	/// Raw add message.
	pub(crate) fn add_msg<F>(&self, ident: co::WM, func: F)
		where F: FnMut(msg::Wm) -> Option<isize> + 'static,
	{
		self.0.as_mut().msgs.insert(ident, Box::new(func));
	}

	/// Raw add notification.
	pub(crate) fn add_nfy<F>(&self, id_from: u16, code: co::NM, func: F)
		where F: FnMut(msg::WmNotify) -> Option<isize> + 'static,
	{
		self.0.as_mut().nfys.insert((id_from, code), Box::new(func));
	}

	/// Event to any [window message](crate::co::WM).
	///
	/// **Note:** Instead of using this event, you should always prefer the
	/// specific events, which will give you the correct message parameters. This
	/// generic method should be used when you have a custom, non-standard window
	/// message.
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
	///
	/// Posted to the installing thread's message queue when a timer expires.
	pub fn wm_timer<F>(&self, timer_id: u32, func: F)
		where F: FnMut() + 'static,
	{
		self.0.as_mut().tmrs.insert(timer_id, Box::new(func));
	}

	/// [`WM_COMMAND`](crate::msg::WmCommand) message, for specific code and
	/// control ID.
	///
	/// A command notification must be narrowed by the
	/// [command code](crate::co::CMD) and the control ID, so the closure will
	/// be fired for that specific control at that specific event.
	///
	/// **Note:** Instead of using this event, you should always prefer the
	/// specific command notifications, which will give you the correct message
	/// parameters. This generic method should be used when you have a custom,
	/// non-standard window notification.
	pub fn wm_command<F>(&self, code: co::CMD, ctrl_id: u16, func: F)
		where F: FnMut() + 'static,
	{
		self.0.as_mut().cmds.insert((code, ctrl_id), Box::new(func));
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
		let shared_func = Rc::new(Immut::new(func));

		self.wm_command(co::CMD::Menu, ctrl_id, {
			let shared_func = shared_func.clone();
			move || shared_func.as_mut()()
		});

		self.wm_command(co::CMD::Accelerator, ctrl_id, {
			let shared_func = shared_func.clone();
			move || shared_func.as_mut()()
		});
	}

	/// [`WM_NOTIFY`](crate::msg::WmNotify) message, for specific ID and
	/// notification code.
	///
	/// A notification must be narrowed by the [notification code](crate::co::NM)
	/// and the control ID, so the closure will be fired for that specific
	/// control at the specific event.
	///
	/// **Note:** Instead of using this event, you should always prefer the
	/// specific notifications, which will give you the correct notification
	/// struct. This generic method should be used when you have a custom,
	/// non-standard window notification.
	pub fn wm_notify<F>(&self, id_from: u16, code: co::NM, func: F)
		where F: FnMut(msg::WmNotify) -> isize + 'static,
	{
		self.add_nfy(id_from, code, {
			let mut func = func;
			move |p| Some(func(p)) // return value is meaningful
		});
	}

	wm_ret_none! { wm_activate, co::WM::ACTIVATE, msg::WmActivate,
		/// [`WM_ACTIVATE`](crate::msg::WmActivate) message.
		///
		/// Sent to both the window being activated and the window being
		/// deactivated. If the windows use the same input queue, the message is
		/// sent synchronously, first to the window procedure of the top-level
		/// window being deactivated, then to the window procedure of the
		/// top-level window being activated. If the windows use different input
		/// queues, the message is sent asynchronously, so the window is activated
		/// immediately.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * non-dialog [`CustomMain`](crate::gui::CustomMain).
	}

	wm_ret_none! { wm_activate_app, co::WM::ACTIVATEAPP, msg::WmActivateApp,
		/// [`WM_ACTIVATEAPP`](crate::msg::WmActivateApp) message.
		///
		/// Sent when a window belonging to a different application than the
		/// active window is about to be activated. The message is sent to the
		/// application whose window is being activated and to the application
		/// whose window is being deactivated.
	}

	/// [`WM_APPCOMMAND`](crate::msg::WmAppCommand) message.
	///
	/// Notifies a window that the user generated an application command event,
	/// for example, by clicking an application command button using the mouse or
	/// typing an application command key on the keyboard.
	pub fn wm_app_command<F>(&self, func: F)
		where F: FnMut(msg::WmAppCommand) + 'static,
	{
		self.add_msg(co::WM::APPCOMMAND, {
			let mut func = func;
			move |p| { func(msg::WmAppCommand::from_generic_wm(p)); Some(true as isize) }
		});
	}

	wm_empty! { wm_cancel_mode, co::WM::CANCELMODE,
		/// [`WM_CANCELMODE`](crate::msg::WmCancelMode) message.
		///
		/// Sent to cancel certain modes, such as mouse capture. For example, the
		/// system sends this message to the active window when a dialog box or
		/// message box is displayed. Certain functions also send this message
		/// explicitly to the specified window regardless of whether it is the
		/// active window. For example, the
		/// [`EnableWindow`](crate::HWND::EnableWindow) function sends this
		/// message when disabling the specified window.
	}

	wm_empty! { wm_child_activate, co::WM::CHILDACTIVATE,
		/// [`WM_CHILDACTIVATE`](crate::msg::WmChildActivate) message.
		///
		/// Sent to a child window when the user clicks the window's title bar or
		/// when the window is activated, moved, or sized.
	}

	wm_empty! { wm_close, co::WM::CLOSE,
		/// [`WM_CLOSE`](crate::msg::WmClose) message.
		///
		/// Sent as a signal that a window or an application should terminate.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * dialog [`CustomMain`](crate::gui::CustomMain);
		/// * dialog [`CustomModal`](crate::gui::CustomModal);
		/// * non-dialog [`CustomModal`](crate::gui::CustomModal).
	}

	wm_empty! { wm_context_menu, co::WM::CONTEXTMENU,
		/// [`WM_CONTEXTMENU`](crate::msg::WmContextMenu) message.
		///
		/// Notifies a window that the user desires a context menu to appear. The
		/// user may have clicked the right mouse button (right-clicked) in the
		/// window, pressed Shift+F10 or pressed the applications key (context
		/// menu key) available on some keyboards.
	}

	/// [`WM_CREATE`](crate::msg::WmCreate) message, sent only to non-dialog
	/// windows. Dialog windows receive
	/// [`WM_INITDIALOG`](crate::gui::events::MsgEvents::wm_init_dialog) instead.
	///
	/// Sent when an application requests that a window be created by calling the
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx) function. The message is
	/// sent before the function returns. The window procedure of the new window
	/// receives this message after the window is created, but before the window
	/// becomes visible.
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
	///
	/// Sent to the parent window of a button before drawing the button. The
	/// parent window can change the button's text and background colors.
	/// However, only owner-drawn buttons respond to the parent window processing
	/// this message.
	pub fn wm_ctl_color_btn<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorBtn) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLORBTN, {
			let mut func = func;
			move |p| Some(func(msg::WmCtlColorBtn::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLORDLG`](crate::msg::WmCtlColorDlg) message.
	///
	/// Sent to a dialog box before the system draws the dialog box. By
	/// responding to this message, the dialog box can set its text and
	/// background colors using the specified display device context handle.
	pub fn wm_ctl_color_dlg<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorDlg) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLORDLG, {
			let mut func = func;
			move |p| Some(func(msg::WmCtlColorDlg::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLOREDIT`](crate::msg::WmCtlColorEdit) message.
	///
	/// An edit control that is not read-only or disabled sends the message to
	/// its parent window when the control is about to be drawn. By responding to
	/// this message, the parent window can use the specified device context
	/// handle to set the text and background colors of the edit control.
	pub fn wm_ctl_color_edit<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorEdit) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLOREDIT, {
			let mut func = func;
			move |p| Some(func(msg::WmCtlColorEdit::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLORLISTBOX`](crate::msg::WmCtlColorListBox) message.
	///
	/// Sent to the parent window of a list box before the system draws the list
	/// box. By responding to this message, the parent window can set the text
	/// and background colors of the list box by using the specified display
	/// device context handle.
	pub fn wm_ctl_color_list_box<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorListBox) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLORLISTBOX, {
			let mut func = func;
			move |p| Some(func(msg::WmCtlColorListBox::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLORSCROLLBAR`](crate::msg::WmCtlColorScrollBar) message.
	///
	/// Sent to the parent window of a scroll bar control when the control is
	/// about to be drawn. By responding to this message, the parent window can
	/// use the display context handle to set the background color of the scroll
	/// bar control.
	pub fn wm_ctl_color_scroll_bar<F>(&self, func: F)
		where F: FnMut(msg::WmCtlColorScrollBar) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLORSCROLLBAR, {
			let mut func = func;
			move |p| Some(func(msg::WmCtlColorScrollBar::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLORSTATIC`](crate::msg::WmCtlColorStatic) message.
	///
	/// A static control, or an edit control that is read-only or disabled, sends
	/// the message to its parent window when the control is about to be drawn.
	/// By responding to this message, the parent window can use the specified
	/// device context handle to set the text foreground and background colors of
	/// the static control.
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
		///
		/// Sent when a window is being destroyed. It is sent to the window
		/// procedure of the window being destroyed after the window is removed
		/// from the screen.
		///
		/// This message is sent first to the window being destroyed and then to
		/// the child windows (if any) as they are destroyed. During the
		/// processing of the message, it can be assumed that all child windows
		/// still exist.
	}

	wm_ret_none! { wm_drop_files, co::WM::DROPFILES, msg::WmDropFiles,
		/// [`WM_DROPFILES`](crate::msg::WmDropFiles) message.
		///
		/// Sent when the user drops a file on the window of an application that
		/// has registered itself as a recipient of dropped files.
	}

	wm_ret_none! { wm_enable, co::WM::ENABLE, msg::WmEnable,
		/// [`WM_ENABLE`](crate::msg::WmEnable) message.
		///
		/// Sent when an application changes the enabled state of a window. It is
		/// sent to the window whose enabled state is changing. This message is
		/// sent before the [`EnableWindow`](crate::HWND::EnableWindow) function
		/// returns, but after the enabled state
		/// ([`WS_DISABLED`](crate::co::WS::DISABLED) style bit) of the window has
		/// changed.
	}

	wm_ret_none! { wm_end_session, co::WM::ENDSESSION, msg::WmEndSession,
		/// [`WM_ENDSESSION`](crate::msg::WmEndSession) message.
		///
		/// Sent to an application after the system processes the results of the
		/// [`WM_QUERYENDSESSION`](crate::gui::events::MsgEvents) message. The
		/// `WM_ENDSESSION` message informs the application whether the session is ending.
	}

	wm_ret_none! { wm_enter_idle, co::WM::ENTERIDLE, msg::WmEnterIdle,
		/// [`WM_ENTERIDLE`](crate::msg::WmEnterIdle) message.
		///
		/// Sent to the owner window of a modal dialog box or menu that is
		/// entering an idle state. A modal dialog box or menu enters an idle
		/// state when no messages are waiting in its queue after it has processed
		/// one or more previous messages.
	}

	wm_ret_none! { wm_enter_size_move, co::WM::ENTERSIZEMOVE, msg::WmEnterSizeMove,
		/// [`WM_ENTERSIZEMOVE`](crate::msg::WmEnterSizeMove) message.
		///
		/// Sent one time to a window after it enters the moving or sizing modal
		/// loop. The window enters the moving or sizing modal loop when the user
		/// clicks the window's title bar or sizing border, or when the window
		/// passes the
		/// [`WM_SYSCOMMAND`](crate::gui::events::MsgEvents::wm_sys_command)
		/// message to the `DefWindowProc` function and the `wParam` parameter of
		/// the message specifies the [`SC_MOVE`](crate::co::SC::MOVE) or
		/// [`SC_SIZE`](crate::co::SC::SIZE) value. The operation is complete when
		/// `DefWindowProc` returns.
		///
		/// The system sends the message regardless of whether the dragging of
		/// full windows is enabled.
	}

	/// [`WM_ERASEBKGND`](crate::msg::WmEraseBkgnd) message.
	///
	/// Sent when the window background must be erased (for example, when a
	/// window is resized). The message is sent to prepare an invalidated portion
	/// of a window for painting.
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
		///
		/// Sent one time to a window, after it has exited the moving or sizing
		/// modal loop. The window enters the moving or sizing modal loop when the
		/// user clicks the window's title bar or sizing border, or when the
		/// window passes the
		/// [`WM_SYSCOMMAND`](crate::gui::events::MsgEvents::wm_sys_command)
		/// message to the `DefWindowProc` function and the `wParam` parameter of
		/// the message specifies the [`SC_MOVE`](crate::co::SC::MOVE) or
		/// [`SC_SIZE`](crate::co::SC::SIZE) value. The operation is complete when
		/// `DefWindowProc` returns.
	}

	wm_ret_none! { wm_get_min_max_info, co::WM::GETMINMAXINFO, msg::WmGetMinMaxInfo,
		/// [`WM_GETMINMAXINFO`](crate::msg::WmGetMinMaxInfo) message.
		///
		/// Sent to a window when the size or position of the window is about to
		/// change. An application can use this message to override the window's
		/// default maximized size and position, or its default minimum or maximum
		/// tracking size.
	}

	/// [`WM_INITDIALOG`](crate::msg::WmInitDialog) message, sent only to dialog
	/// windows. Non-dialog windows receive
	/// [`WM_CREATE`](crate::gui::events::MsgEvents::wm_create) instead.
	///
	/// Sent to the dialog box procedure immediately before a dialog box is
	/// displayed. Dialog box procedures typically use this message to initialize
	/// controls and carry out any other initialization tasks that affect the
	/// appearance of the dialog box.
	pub fn wm_init_dialog<F>(&self, func: F)
		where F: FnMut(msg::WmInitDialog) -> bool + 'static,
	{
		self.add_msg(co::WM::INITDIALOG, {
			let mut func = func;
			move |p| Some(func(msg::WmInitDialog::from_generic_wm(p)) as isize)
		});
	}

	wm_ret_none! { wm_init_menu_popup, co::WM::INITMENUPOPUP, msg::WmInitMenuPopup,
		/// [`WM_INITMENUPOPUP`](crate::msg::WmInitMenuPopup) message.
		///
		/// Sent when a drop-down menu or submenu is about to become active. This
		/// allows an application to modify the menu before it is displayed,
		/// without changing the entire menu.
	}

	wm_ret_none! { wm_l_button_dbl_clk, co::WM::LBUTTONDBLCLK, msg::WmLButtonDblClk,
		/// [`WM_LBUTTONDBLCLK`](crate::msg::WmLButtonDblClk) message.
		///
		/// Posted when the user double-clicks the left mouse button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	wm_ret_none! { wm_l_button_down, co::WM::LBUTTONDOWN, msg::WmLButtonDown,
		/// [`WM_LBUTTONDOWN`](crate::msg::WmLButtonDown) message.
		///
		/// Posted when the user presses the left mouse button while the cursor is
		/// in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
	}

	wm_ret_none! { wm_l_button_up, co::WM::LBUTTONUP, msg::WmLButtonUp,
		/// [`WM_LBUTTONUP`](crate::msg::WmLButtonUp) message.
		///
		/// Posted when the user releases the left mouse button while the cursor
		/// is in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
	}

	wm_ret_none! { wm_m_button_dbl_clk, co::WM::MBUTTONDBLCLK, msg::WmMButtonDblClk,
		/// [`WM_MBUTTONDBLCLK`](crate::msg::WmMButtonDblClk) message.
		///
		/// Posted when the user double-clicks the middle mouse button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	wm_ret_none! { wm_m_button_down, co::WM::MBUTTONDOWN, msg::WmMButtonDown,
		/// [`WM_MBUTTONDOWN`](crate::msg::WmMButtonDown) message.
		///
		/// Posted when the user presses the middle mouse button while the cursor
		/// is in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
	}

	wm_ret_none! { wm_m_button_up, co::WM::MBUTTONUP, msg::WmMButtonUp,
		/// [`WM_MBUTTONUP`](crate::msg::WmMButtonUp) message.
		///
		/// Posted when the user releases the middle mouse button while the cursor
		/// is in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
	}

	wm_ret_none! { wm_mouse_hover, co::WM::MOUSEHOVER, msg::WmMouseHover,
		/// [`WM_MOUSEHOVER`](crate::msg::WmMouseHover) message.
		///
		/// Posted to a window when the cursor hovers over the client area of the
		/// window for the period of time specified in a prior call to
		/// [`TrackMouseEvent`](crate::TrackMouseEvent).
	}

	wm_ret_none! { wm_mouse_move, co::WM::MOUSEMOVE, msg::WmMouseMove,
		/// [`WM_MOUSEMOVE`](crate::msg::WmMouseMove) message.
		///
		/// Posted to a window when the cursor moves. If the mouse is not
		/// captured, the message is posted to the window that contains the
		/// cursor. Otherwise, the message is posted to the window that has
		/// captured the mouse.
	}

	wm_ret_none! { wm_move, co::WM::MOVE, msg::WmMove,
		/// [`WM_MOVE`](crate::msg::WmMove) message.
		///
		/// Sent after a window has been moved.
	}

	wm_ret_none! { wm_moving, co::WM::MOVING, msg::WmMoving,
		/// [`WM_MOVING`](crate::msg::WmMoving) message.
		///
		/// Sent to a window that the user is moving. By processing this message,
		/// an application can monitor the position of the drag rectangle and, if
		/// needed, change its position.
	}

	/// [`WM_NCCALCSIZE`](crate::msg::WmNcCalcSize) message.
	///
	/// Sent when the size and position of a window's client area must be
	/// calculated. By processing this message, an application can control the
	/// content of the window's client area when the size or position of the
	/// window changes.
	pub fn wm_nc_calc_size<F>(&self, func: F)
		where F: FnMut(msg::WmNcCalcSize) -> co::WVR + 'static
	{
		self.add_msg(co::WM::NCCALCSIZE, {
			let mut func = func;
			move |p| Some(func(msg::WmNcCalcSize::from_generic_wm(p)).0 as isize)
		});
	}

	/// [`WM_NCCREATE`](crate::msg::WmNcCreate) message.
	///
	/// Sent prior to the [`WM_CREATE`](crate::gui::events::MsgEvents::wm_create)
	/// message when a window is first created.
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
		/// Notifies a window that its nonclient area is being destroyed. The
		/// [`DestroyWindow`](crate::HWND::DestroyWindow) function sends the
		/// message to the window following the
		/// [`WM_DESTROY`](crate::gui::events::MsgEvents::wm_destroy) message.
		/// `WM_DESTROY` is used to free the allocated memory object associated
		/// with the window.
		///
		/// The `WM_NCDESTROY` message is sent after the child windows have been
		/// destroyed. In contrast, `WM_DESTROY` is sent before the child windows
		/// are destroyed.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * non-dialog [`CustomMain`](crate::gui::CustomMain);
		/// * dialog [`CustomMain`](crate::gui::CustomMain).
	}

	wm_ret_none! { wm_nc_paint, co::WM::NCPAINT, msg::WmNcPaint,
		/// [`WM_NCPAINT`](crate::msg::WmNcPaint) message.
		///
		/// Sent to a window when its frame must be painted.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * non-dialog [`CustomControl`](crate::gui::CustomControl);
		/// * dialog [`CustomControl`](crate::gui::CustomControl).
	}

	wm_empty! { wm_null, co::WM::NULL,
		/// [`WM_NULL`](crate::msg::WmNull) message.
		///
		/// Performs no operation. An application sends the message if it wants to
		/// post a message that the recipient window will ignore.
	}

	wm_empty! { wm_paint, co::WM::PAINT,
		/// [`WM_PAINT`](crate::msg::WmPaint) message.
		///
		/// Sent when the system or another application makes a request to paint a
		/// portion of an application's window. The message is sent when the
		/// [`UpdateWindow`](crate::HWND::UpdateWindow) or
		/// [`RedrawWindow`](crate::HWND::RedrawWindow) function is called, or by
		/// the [`DispatchMessage`](crate::DispatchMessage) function when the
		/// application obtains a `WM_PAINT` message by using the
		/// [`GetMessage`](crate::GetMessage) or
		/// [`PeekMessage`](crate::PeekMessage) function.
	}

	/// [`WM_QUERYOPEN`](crate::msg::WmQueryOpen) message.
	///
	/// Sent to an icon when the user requests that the window be restored to its
	/// previous size and position.
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
		///
		/// Posted when the user double-clicks the right mouse button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	wm_ret_none! { wm_r_button_down, co::WM::RBUTTONDOWN, msg::WmRButtonDown,
		/// [`WM_RBUTTONDOWN`](crate::msg::WmRButtonDown) message.
		///
		/// Posted when the user presses the right mouse button while the cursor
		/// is in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
	}

	wm_ret_none! { wm_r_button_up, co::WM::RBUTTONUP, msg::WmRButtonUp,
		/// [`WM_RBUTTONUP`](crate::msg::WmRButtonUp) message.
		///
		/// Posted when the user releases the right mouse button while the cursor
		/// is in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
	}

	wm_ret_none! { wm_set_focus, co::WM::SETFOCUS, msg::WmSetFocus,
		/// [`WM_SETFOCUS`](crate::msg::WmSetFocus) message.
		///
		/// Sent to a window after it has gained the keyboard focus.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * non-dialog [`CustomMain`](crate::gui::CustomMain);
		/// * non-dialog [`CustomModal`](crate::gui::CustomModal).
	}

	wm_ret_none! { wm_set_font, co::WM::SETFONT, msg::WmSetFont,
		/// [`WM_SETFONT`](crate::msg::WmSetFont) message.
		///
		/// Sets the font that a control is to use when drawing text.
	}

	/// [`WM_SETICON`](crate::msg::WmSetIcon) message.
	///
	/// Associates a new large or small icon with a window. The system displays
	/// the large icon in the Alt+TAB dialog box, and the small icon in the
	/// window caption.
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
		///
		/// Sent to a window when the window is about to be hidden or shown.
	}

	wm_ret_none! { wm_size, co::WM::SIZE, msg::WmSize,
		/// [`WM_SIZE`](crate::msg::WmSize) message.
		///
		/// Sent to a window after its size has changed.
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
		///
		/// Sent to a window that the user is resizing. By processing this
		/// message, an application can monitor the size and position of the drag
		/// rectangle and, if needed, change its size or position.
	}

	wm_ret_none! { wm_style_changed, co::WM::STYLECHANGED, msg::WmStyleChanged,
		/// [`WM_STYLECHANGED`](crate::msg::WmStyleChanged) message.
		///
		/// Sent to a window after the
		/// [`SetWindowLongPtr`](crate::HWND::SetWindowLongPtr) function has
		/// changed one or more of the window's styles.
	}

	wm_ret_none! { wm_style_changing, co::WM::STYLECHANGING, msg::WmStyleChanging,
		/// [`WM_STYLECHANGING`](crate::msg::WmStyleChanging) message.
		///
		/// Sent to a window when the
		/// [`SetWindowLongPtr`](crate::HWND::SetWindowLongPtr) function is about
		/// to change one or more of the window's styles.
	}

	wm_ret_none! { wm_theme_changed, co::WM::THEMECHANGED, msg::WmThemeChanged,
		/// [`WM_THEMECHANGED`](crate::msg::WmThemeChanged) message.
		///
		/// Broadcast to every window following a theme change event. Examples of
		/// theme change events are the activation of a theme, the deactivation of
		/// a theme, or a transition from one theme to another.
	}

	wm_ret_none! { wm_window_pos_changed, co::WM::WINDOWPOSCHANGED, msg::WmWindowPosChanged,
		/// [`WM_WINDOWPOSCHANGED`](crate::msg::WmWindowPosChanged) message.
		///
		/// Sent to a window whose size, position, or place in the Z order has
		/// changed as a result of a call to the
		/// [`SetWindowPos`](crate::HWND::SetWindowPos) function or another
		/// window-management function.
	}

	wm_ret_none! { wm_window_pos_changing, co::WM::WINDOWPOSCHANGING, msg::WmWindowPosChanging,
		/// [`WM_WINDOWPOSCHANGING`](crate::msg::WmWindowPosChanging) message.
		///
		/// Sent to a window whose size, position, or place in the Z order is
		/// about to change as a result of a call to the
		/// [`SetWindowPos`](crate::HWND::SetWindowPos) function or another
		/// window-management function.
	}

	wm_ret_none! { wm_x_button_dbl_clk, co::WM::XBUTTONDBLCLK, msg::WmXButtonDblClk,
		/// [`WM_XBUTTONDBLCLK`](crate::msg::WmXButtonDblClk) message.
		///
		/// Posted when the user double-clicks the first or second X button while
		/// the cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	wm_ret_none! { wm_x_button_down, co::WM::XBUTTONDOWN, msg::WmXButtonDown,
		/// [`WM_XBUTTONDOWN`](crate::msg::WmXButtonDown) message.
		///
		/// Posted when the user presses the first or second X button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	wm_ret_none! { wm_x_button_up, co::WM::XBUTTONUP, msg::WmXButtonUp,
		/// [`WM_XBUTTONUP`](crate::msg::WmXButtonUp) message.
		///
		/// Posted when the user releases the first or second X button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}
}
