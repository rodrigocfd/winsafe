use std::rc::Rc;

use crate::co;
use crate::gui::events::func_store::FuncStore;
use crate::gui::immut::Immut;
use crate::handles::{HDC, HICON};
use crate::msg::{MsgSendRecv, wm, WndMsg};

/// The result of processing a message.
pub enum ProcessResult {
	/// Message was not handled because no function was found.
	NotHandled,
	/// Message handled, and return value is meaningful.
	HandledWithRet(isize),
	/// Message handled, but you should return the default value (0 or FALSE).
	HandledWithoutRet,
}

//------------------------------------------------------------------------------

/// Exposes window
/// [messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
///
/// You cannot directly instantiate this object, it is created internally by the
/// window.
pub struct WindowEvents(Immut<Obj>);

struct Obj { // actual fields of WindowEvents
	msgs: FuncStore< // ordinary WM messages
		co::WM,
		Box<dyn FnMut(WndMsg) -> Option<isize> + 'static>, // return value may be meaningful
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
		Box<dyn FnMut(wm::Notify) -> Option<isize> + 'static>, // return value may be meaningful
	>,
}

impl WindowEvents {
	pub(crate) fn new() -> WindowEvents {
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
	pub(crate) fn process_effective_message(&self, wm_any: WndMsg) -> ProcessResult {
		match wm_any.msg_id {
			co::WM::NOTIFY => {
				let wm_nfy = wm::Notify::from_generic_wm(wm_any);
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
				let wm_cmd = wm::Command::from_generic_wm(wm_any);
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
				let wm_tmr = wm::Timer::from_generic_wm(wm_any);
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
	pub(crate) fn process_all_messages(&self, wm_any: WndMsg) {
		match wm_any.msg_id {
			co::WM::NOTIFY => {
				let wm_nfy = wm::Notify::from_generic_wm(wm_any);
				let key = (wm_nfy.nmhdr.idFrom as u16, wm_nfy.nmhdr.code);
				self.0.as_mut().nfys.find_all(key, |func| {
					func(wm_nfy);
				});
			},
			co::WM::COMMAND => {
				let wm_cmd = wm::Command::from_generic_wm(wm_any);
				let key = (wm_cmd.code, wm_cmd.ctrl_id);
				self.0.as_mut().cmds.find_all(key, |func| {
					func();
				});
			},
			co::WM::TIMER => {
				let wm_tmr = wm::Timer::from_generic_wm(wm_any);
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
		where F: FnMut(WndMsg) -> Option<isize> + 'static,
	{
		self.0.as_mut().msgs.insert(ident, Box::new(func));
	}

	/// Raw add notification.
	pub(crate) fn add_nfy<F>(&self, id_from: u16, code: co::NM, func: F)
		where F: FnMut(wm::Notify) -> Option<isize> + 'static,
	{
		self.0.as_mut().nfys.insert((id_from, code), Box::new(func));
	}
}

/// A message which has no parameters and returns zero.
macro_rules! wm_empty {
	(
		$name:ident, $wmconst:expr,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
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
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
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

impl WindowEvents {
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
	/// use winsafe::{co, gui::WindowMain};
	///
	/// let wnd: WindowMain; // initialize it somewhere...
	///
	/// let CUSTOM_MSG = co::WM::from(0x1234);
	///
	/// wnd.on().wm(CUSTOM_MSG, {
	///     let wnd = wnd.clone(); // pass into the closure
	///     move |parms| {
	///         println!("HWND: {}, msg ID: {}", wnd.hwnd(), parms.msg_id);
	///         0
	///     }
	/// });
	/// ```
	pub fn wm<F>(&self, ident: co::WM, func: F)
		where F: FnMut(WndMsg) -> isize + 'static,
	{
		self.add_msg(ident, {
			let mut func = func;
			move |p| Some(func(p)) // return value is meaningful
		});
	}

	/// [`WM_TIMER`](crate::msg::wm::Timer) message, narrowed to a specific timer
	/// ID.
	///
	/// Posted to the installing thread's message queue when a timer expires.
	pub fn wm_timer<F>(&self, timer_id: u32, func: F)
		where F: FnMut() + 'static,
	{
		self.0.as_mut().tmrs.insert(timer_id, Box::new(func));
	}

	/// [`WM_COMMAND`](crate::msg::wm::Command) message, for specific code and
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

	/// [`WM_COMMAND`](crate::msg::wm::Command) message, handling both
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
	/// use winsafe::{co, gui::WindowMain, msg::wm};
	///
	/// let wnd: WindowMain; // initialize it somewhere...
	///
	/// wnd.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), {
	///     let wnd = wnd.clone(); // pass into the closure
	///     move || {
	///         wnd.hwnd().PostMessage(wm::Close {}).unwrap();
	///     }
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

	/// [`WM_NOTIFY`](crate::msg::wm::Notify) message, for specific ID and
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
		where F: FnMut(wm::Notify) -> isize + 'static,
	{
		self.add_nfy(id_from, code, {
			let mut func = func;
			move |p| Some(func(p)) // return value is meaningful
		});
	}

	wm_ret_none! { wm_activate, co::WM::ACTIVATE, wm::Activate,
		/// [`WM_ACTIVATE`](crate::msg::wm::Activate) message.
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
		/// * non-dialog [`WindowMain`](crate::gui::WindowMain).
	}

	wm_ret_none! { wm_activate_app, co::WM::ACTIVATEAPP, wm::ActivateApp,
		/// [`WM_ACTIVATEAPP`](crate::msg::wm::ActivateApp) message.
		///
		/// Sent when a window belonging to a different application than the
		/// active window is about to be activated. The message is sent to the
		/// application whose window is being activated and to the application
		/// whose window is being deactivated.
	}

	/// [`WM_APPCOMMAND`](crate::msg::wm::AppCommand) message.
	///
	/// Notifies a window that the user generated an application command event,
	/// for example, by clicking an application command button using the mouse or
	/// typing an application command key on the keyboard.
	pub fn wm_app_command<F>(&self, func: F)
		where F: FnMut(wm::AppCommand) + 'static,
	{
		self.add_msg(co::WM::APPCOMMAND, {
			let mut func = func;
			move |p| { func(wm::AppCommand::from_generic_wm(p)); Some(true as isize) }
		});
	}

	wm_empty! { wm_cancel_mode, co::WM::CANCELMODE,
		/// [`WM_CANCELMODE`](crate::msg::wm::CancelMode) message.
		///
		/// Sent to cancel certain modes, such as mouse capture. For example, the
		/// system sends this message to the active window when a dialog box or
		/// message box is displayed. Certain functions also send this message
		/// explicitly to the specified window regardless of whether it is the
		/// active window. For example, the
		/// [`EnableWindow`](crate::HWND::EnableWindow) function sends this
		/// message when disabling the specified window.
	}

	wm_ret_none! { wm_char, co::WM::CHAR, wm::Char,
		/// [`WM_CHAR`](crate::msg::wm::Char) message.
		///
		/// Posted to the window with the keyboard focus when a
		/// [`WM_KEYDOWN`](crate::msg::wm::KeyDown) message is translated by the
		/// [`TranslateMessage`](crate::TranslateMessage) function. The `WM_CHAR`
		/// message contains the character code of the key that was pressed.
	}

	wm_empty! { wm_child_activate, co::WM::CHILDACTIVATE,
		/// [`WM_CHILDACTIVATE`](crate::msg::wm::ChildActivate) message.
		///
		/// Sent to a child window when the user clicks the window's title bar or
		/// when the window is activated, moved, or sized.
	}

	wm_empty! { wm_close, co::WM::CLOSE,
		/// [`WM_CLOSE`](crate::msg::wm::Close) message.
		///
		/// Sent as a signal that a window or an application should terminate.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * dialog [`WindowMain`](crate::gui::WindowMain);
		/// * dialog [`WindowModal`](crate::gui::WindowModal);
		/// * non-dialog [`WindowModal`](crate::gui::WindowModal).
	}

	wm_empty! { wm_context_menu, co::WM::CONTEXTMENU,
		/// [`WM_CONTEXTMENU`](crate::msg::wm::ContextMenu) message.
		///
		/// Notifies a window that the user desires a context menu to appear. The
		/// user may have clicked the right mouse button (right-clicked) in the
		/// window, pressed Shift+F10 or pressed the applications key (context
		/// menu key) available on some keyboards.
	}

	/// [`WM_CREATE`](crate::msg::wm::Create) message, sent only to non-dialog
	/// windows. Dialog windows receive
	/// [`WM_INITDIALOG`](crate::gui::events::WindowEvents::wm_init_dialog)
	/// instead.
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
	///     let wnd = wnd.clone(); // pass into the closure
	///     move |parms| {
	///         println!("HWND: {}, client area: {}x{}",
	///             wnd.hwnd(),
	///             parms.createstruct.cx,
	///             parms.createstruct.cy,
	///         );
	///         0
	///     }
	/// });
	/// ```
	pub fn wm_create<F>(&self, func: F)
		where F: FnMut(wm::Create) -> i32 + 'static,
	{
		self.add_msg(co::WM::CREATE, {
			let mut func = func;
			move |p| Some(func(wm::Create::from_generic_wm(p)) as isize)
		});
	}

	/// [`WM_CTLCOLORBTN`](crate::msg::wm::CtlColorBtn) message.
	///
	/// Sent to the parent window of a button before drawing the button. The
	/// parent window can change the button's text and background colors.
	/// However, only owner-drawn buttons respond to the parent window processing
	/// this message.
	pub fn wm_ctl_color_btn<F>(&self, func: F)
		where F: FnMut(wm::CtlColorBtn) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLORBTN, {
			let mut func = func;
			move |p| Some(func(wm::CtlColorBtn::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLORDLG`](crate::msg::wm::CtlColorDlg) message.
	///
	/// Sent to a dialog box before the system draws the dialog box. By
	/// responding to this message, the dialog box can set its text and
	/// background colors using the specified display device context handle.
	pub fn wm_ctl_color_dlg<F>(&self, func: F)
		where F: FnMut(wm::CtlColorDlg) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLORDLG, {
			let mut func = func;
			move |p| Some(func(wm::CtlColorDlg::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLOREDIT`](crate::msg::wm::CtlColorEdit) message.
	///
	/// An edit control that is not read-only or disabled sends the message to
	/// its parent window when the control is about to be drawn. By responding to
	/// this message, the parent window can use the specified device context
	/// handle to set the text and background colors of the edit control.
	pub fn wm_ctl_color_edit<F>(&self, func: F)
		where F: FnMut(wm::CtlColorEdit) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLOREDIT, {
			let mut func = func;
			move |p| Some(func(wm::CtlColorEdit::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLORLISTBOX`](crate::msg::wm::CtlColorListBox) message.
	///
	/// Sent to the parent window of a list box before the system draws the list
	/// box. By responding to this message, the parent window can set the text
	/// and background colors of the list box by using the specified display
	/// device context handle.
	pub fn wm_ctl_color_list_box<F>(&self, func: F)
		where F: FnMut(wm::CtlColorListBox) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLORLISTBOX, {
			let mut func = func;
			move |p| Some(func(wm::CtlColorListBox::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLORSCROLLBAR`](crate::msg::wm::CtlColorScrollBar) message.
	///
	/// Sent to the parent window of a scroll bar control when the control is
	/// about to be drawn. By responding to this message, the parent window can
	/// use the display context handle to set the background color of the scroll
	/// bar control.
	pub fn wm_ctl_color_scroll_bar<F>(&self, func: F)
		where F: FnMut(wm::CtlColorScrollBar) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLORSCROLLBAR, {
			let mut func = func;
			move |p| Some(func(wm::CtlColorScrollBar::from_generic_wm(p)).ptr as isize)
		});
	}

	/// [`WM_CTLCOLORSTATIC`](crate::msg::wm::CtlColorStatic) message.
	///
	/// A static control, or an edit control that is read-only or disabled, sends
	/// the message to its parent window when the control is about to be drawn.
	/// By responding to this message, the parent window can use the specified
	/// device context handle to set the text foreground and background colors of
	/// the static control.
	pub fn wm_ctl_color_static<F>(&self, func: F)
		where F: FnMut(wm::CtlColorStatic) -> HDC + 'static,
	{
		self.add_msg(co::WM::CTLCOLORSTATIC, {
			let mut func = func;
			move |p| Some(func(wm::CtlColorStatic::from_generic_wm(p)).ptr as isize)
		});
	}

	wm_ret_none! { wm_dead_char, co::WM::DEADCHAR, wm::DeadChar,
		/// [`WM_DEADCHAR`](crate::msg::wm::DeadChar) message.
		///
		/// Posted to the window with the keyboard focus when a
		/// [`WM_KEYUP`](crate::msg::wm::KeyUp) message is translated by the
		/// [`TranslateMessage`](crate::TranslateMessage) function. `WM_DEADCHAR`
		/// specifies a character code generated by a dead key. A dead key is a
		/// key that generates a character, such as the umlaut (double-dot), that
		/// is combined with another character to form a composite character. For
		/// example, the umlaut-O character (Ö) is generated by typing the dead
		/// key for the umlaut character, and then typing the O key.
	}

	wm_empty! { wm_destroy, co::WM::DESTROY,
		/// [`WM_DESTROY`](crate::msg::wm::Destroy) message.
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

	wm_ret_none! { wm_drop_files, co::WM::DROPFILES, wm::DropFiles,
		/// [`WM_DROPFILES`](crate::msg::wm::DropFiles) message.
		///
		/// Sent when the user drops a file on the window of an application that
		/// has registered itself as a recipient of dropped files.
	}

	wm_ret_none! { wm_enable, co::WM::ENABLE, wm::Enable,
		/// [`WM_ENABLE`](crate::msg::wm::Enable) message.
		///
		/// Sent when an application changes the enabled state of a window. It is
		/// sent to the window whose enabled state is changing. This message is
		/// sent before the [`EnableWindow`](crate::HWND::EnableWindow) function
		/// returns, but after the enabled state
		/// ([`WS_DISABLED`](crate::co::WS::DISABLED) style bit) of the window has
		/// changed.
	}

	wm_ret_none! { wm_end_session, co::WM::ENDSESSION, wm::EndSession,
		/// [`WM_ENDSESSION`](crate::msg::wm::EndSession) message.
		///
		/// Sent to an application after the system processes the results of the
		/// [`WM_QUERYENDSESSION`](crate::gui::events::WindowEvents) message. The
		/// `WM_ENDSESSION` message informs the application whether the session is ending.
	}

	wm_ret_none! { wm_enter_idle, co::WM::ENTERIDLE, wm::EnterIdle,
		/// [`WM_ENTERIDLE`](crate::msg::wm::EnterIdle) message.
		///
		/// Sent to the owner window of a modal dialog box or menu that is
		/// entering an idle state. A modal dialog box or menu enters an idle
		/// state when no messages are waiting in its queue after it has processed
		/// one or more previous messages.
	}

	wm_ret_none! { wm_enter_size_move, co::WM::ENTERSIZEMOVE, wm::EnterSizeMove,
		/// [`WM_ENTERSIZEMOVE`](crate::msg::wm::EnterSizeMove) message.
		///
		/// Sent one time to a window after it enters the moving or sizing modal
		/// loop. The window enters the moving or sizing modal loop when the user
		/// clicks the window's title bar or sizing border, or when the window
		/// passes the
		/// [`WM_SYSCOMMAND`](crate::gui::events::WindowEvents::wm_sys_command)
		/// message to the `DefWindowProc` function and the `wParam` parameter of
		/// the message specifies the [`SC_MOVE`](crate::co::SC::MOVE) or
		/// [`SC_SIZE`](crate::co::SC::SIZE) value. The operation is complete when
		/// `DefWindowProc` returns.
		///
		/// The system sends the message regardless of whether the dragging of
		/// full windows is enabled.
	}

	/// [`WM_ERASEBKGND`](crate::msg::wm::EraseBkgnd) message.
	///
	/// Sent when the window background must be erased (for example, when a
	/// window is resized). The message is sent to prepare an invalidated portion
	/// of a window for painting.
	pub fn wm_erase_bkgnd<F>(&self, func: F)
		where F: FnMut(wm::EraseBkgnd) -> i32 + 'static,
	{
		self.add_msg(co::WM::ERASEBKGND, {
			let mut func = func;
			move |p| Some(func(wm::EraseBkgnd::from_generic_wm(p)) as isize)
		});
	}

	wm_ret_none! { wm_exit_size_move, co::WM::EXITSIZEMOVE, wm::ExitSizeMove,
		/// [`WM_EXITSIZEMOVE`](crate::msg::wm::ExitSizeMove) message.
		///
		/// Sent one time to a window, after it has exited the moving or sizing
		/// modal loop. The window enters the moving or sizing modal loop when the
		/// user clicks the window's title bar or sizing border, or when the
		/// window passes the
		/// [`WM_SYSCOMMAND`](crate::gui::events::WindowEvents::wm_sys_command)
		/// message to the `DefWindowProc` function and the `wParam` parameter of
		/// the message specifies the [`SC_MOVE`](crate::co::SC::MOVE) or
		/// [`SC_SIZE`](crate::co::SC::SIZE) value. The operation is complete when
		/// `DefWindowProc` returns.
	}

	wm_ret_none! { wm_get_min_max_info, co::WM::GETMINMAXINFO, wm::GetMinMaxInfo,
		/// [`WM_GETMINMAXINFO`](crate::msg::wm::GetMinMaxInfo) message.
		///
		/// Sent to a window when the size or position of the window is about to
		/// change. An application can use this message to override the window's
		/// default maximized size and position, or its default minimum or maximum
		/// tracking size.
	}

	wm_ret_none! { wm_help, co::WM::HELP, wm::Help,
		/// [`WM_HELP`](crate::msg::wm::Help) message.
		///
		/// Indicates that the user pressed the F1 key.
	}

	/// [`WM_INITDIALOG`](crate::msg::wm::InitDialog) message, sent only to dialog
	/// windows. Non-dialog windows receive
	/// [`WM_CREATE`](crate::gui::events::WindowEvents::wm_create) instead.
	///
	/// Sent to the dialog box procedure immediately before a dialog box is
	/// displayed. Dialog box procedures typically use this message to initialize
	/// controls and carry out any other initialization tasks that affect the
	/// appearance of the dialog box.
	pub fn wm_init_dialog<F>(&self, func: F)
		where F: FnMut(wm::InitDialog) -> bool + 'static,
	{
		self.add_msg(co::WM::INITDIALOG, {
			let mut func = func;
			move |p| Some(func(wm::InitDialog::from_generic_wm(p)) as isize)
		});
	}

	wm_ret_none! { wm_init_menu_popup, co::WM::INITMENUPOPUP, wm::InitMenuPopup,
		/// [`WM_INITMENUPOPUP`](crate::msg::wm::InitMenuPopup) message.
		///
		/// Sent when a drop-down menu or submenu is about to become active. This
		/// allows an application to modify the menu before it is displayed,
		/// without changing the entire menu.
	}

	wm_ret_none! { wm_key_down, co::WM::KEYDOWN, wm::KeyDown,
		/// [`WM_KEYDOWN`](crate::msg::wm::KeyDown) message.
		///
		/// Posted to the window with the keyboard focus when a nonsystem key is
		/// pressed. A nonsystem key is a key that is pressed when the ALT key is
		/// not pressed.
	}

	wm_ret_none! { wm_key_up, co::WM::KEYUP, wm::KeyUp,
		/// [`WM_KEYUP`](crate::msg::wm::KeyUp) message.
		///
		/// Posted to the window with the keyboard focus when a nonsystem key is
		/// released. A nonsystem key is a key that is pressed when the ALT key is
		/// not pressed, or a keyboard key that is pressed when a window has the
		/// keyboard focus.
	}

	wm_ret_none! { wm_kill_focus, co::WM::KILLFOCUS, wm::KillFocus,
		/// [`WM_KILLFOCUS`](crate::msg::wm::KillFocus) message.
		///
		/// Sent to a window immediately before it loses the keyboard focus.
	}

	wm_ret_none! { wm_l_button_dbl_clk, co::WM::LBUTTONDBLCLK, wm::LButtonDblClk,
		/// [`WM_LBUTTONDBLCLK`](crate::msg::wm::LButtonDblClk) message.
		///
		/// Posted when the user double-clicks the left mouse button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	wm_ret_none! { wm_l_button_down, co::WM::LBUTTONDOWN, wm::LButtonDown,
		/// [`WM_LBUTTONDOWN`](crate::msg::wm::LButtonDown) message.
		///
		/// Posted when the user presses the left mouse button while the cursor is
		/// in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
	}

	wm_ret_none! { wm_l_button_up, co::WM::LBUTTONUP, wm::LButtonUp,
		/// [`WM_LBUTTONUP`](crate::msg::wm::LButtonUp) message.
		///
		/// Posted when the user releases the left mouse button while the cursor
		/// is in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
	}

	wm_ret_none! { wm_m_button_dbl_clk, co::WM::MBUTTONDBLCLK, wm::MButtonDblClk,
		/// [`WM_MBUTTONDBLCLK`](crate::msg::wm::MButtonDblClk) message.
		///
		/// Posted when the user double-clicks the middle mouse button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	wm_ret_none! { wm_m_button_down, co::WM::MBUTTONDOWN, wm::MButtonDown,
		/// [`WM_MBUTTONDOWN`](crate::msg::wm::MButtonDown) message.
		///
		/// Posted when the user presses the middle mouse button while the cursor
		/// is in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
	}

	wm_ret_none! { wm_m_button_up, co::WM::MBUTTONUP, wm::MButtonUp,
		/// [`WM_MBUTTONUP`](crate::msg::wm::MButtonUp) message.
		///
		/// Posted when the user releases the middle mouse button while the cursor
		/// is in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
	}

	wm_ret_none! { wm_mouse_hover, co::WM::MOUSEHOVER, wm::MouseHover,
		/// [`WM_MOUSEHOVER`](crate::msg::wm::MouseHover) message.
		///
		/// Posted to a window when the cursor hovers over the client area of the
		/// window for the period of time specified in a prior call to
		/// [`TrackMouseEvent`](crate::TrackMouseEvent).
	}

	wm_ret_none! { wm_mouse_move, co::WM::MOUSEMOVE, wm::MouseMove,
		/// [`WM_MOUSEMOVE`](crate::msg::wm::MouseMove) message.
		///
		/// Posted to a window when the cursor moves. If the mouse is not
		/// captured, the message is posted to the window that contains the
		/// cursor. Otherwise, the message is posted to the window that has
		/// captured the mouse.
	}

	wm_ret_none! { wm_move, co::WM::MOVE, wm::Move,
		/// [`WM_MOVE`](crate::msg::wm::Move) message.
		///
		/// Sent after a window has been moved.
	}

	wm_ret_none! { wm_moving, co::WM::MOVING, wm::Moving,
		/// [`WM_MOVING`](crate::msg::wm::Moving) message.
		///
		/// Sent to a window that the user is moving. By processing this message,
		/// an application can monitor the position of the drag rectangle and, if
		/// needed, change its position.
	}

	/// [`WM_NCCALCSIZE`](crate::msg::wm::NcCalcSize) message.
	///
	/// Sent when the size and position of a window's client area must be
	/// calculated. By processing this message, an application can control the
	/// content of the window's client area when the size or position of the
	/// window changes.
	pub fn wm_nc_calc_size<F>(&self, func: F)
		where F: FnMut(wm::NcCalcSize) -> co::WVR + 'static
	{
		self.add_msg(co::WM::NCCALCSIZE, {
			let mut func = func;
			move |p| Some(func(wm::NcCalcSize::from_generic_wm(p)).0 as isize)
		});
	}

	/// [`WM_NCCREATE`](crate::msg::wm::NcCreate) message.
	///
	/// Sent prior to the
	/// [`WM_CREATE`](crate::gui::events::WindowEvents::wm_create) message when a
	/// window is first created.
	pub fn wm_nc_create<F>(&self, func: F)
		where F: FnMut(wm::NcCreate) -> bool + 'static,
	{
		self.add_msg(co::WM::NCCREATE, {
			let mut func = func;
			move |p| Some(func(wm::NcCreate::from_generic_wm(p)) as isize)
		});
	}

	wm_empty! { wm_nc_destroy, co::WM::NCDESTROY,
		/// [`WM_NCDESTROY`](crate::msg::wm::NcDestroy) message.
		///
		/// Notifies a window that its nonclient area is being destroyed. The
		/// [`DestroyWindow`](crate::HWND::DestroyWindow) function sends the
		/// message to the window following the
		/// [`WM_DESTROY`](crate::gui::events::WindowEvents::wm_destroy) message.
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
		/// * non-dialog [`WindowMain`](crate::gui::WindowMain);
		/// * dialog [`WindowMain`](crate::gui::WindowMain).
	}

	wm_ret_none! { wm_nc_paint, co::WM::NCPAINT, wm::NcPaint,
		/// [`WM_NCPAINT`](crate::msg::wm::NcPaint) message.
		///
		/// Sent to a window when its frame must be painted.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * non-dialog [`WindowControl`](crate::gui::WindowControl);
		/// * dialog [`WindowControl`](crate::gui::WindowControl).
	}

	wm_empty! { wm_null, co::WM::NULL,
		/// [`WM_NULL`](crate::msg::wm::Null) message.
		///
		/// Performs no operation. An application sends the message if it wants to
		/// post a message that the recipient window will ignore.
	}

	wm_empty! { wm_paint, co::WM::PAINT,
		/// [`WM_PAINT`](crate::msg::wm::Paint) message.
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

	wm_ret_none! { wm_parent_notify, co::WM::PARENTNOTIFY, wm::ParentNotify,
		/// [`WM_PARENTNOTIFY`](crate::msg::wm::ParentNotify) message.
		///
		/// Sent to a window when a significant action occurs on a descendant
		/// window.
	}

	/// [`WM_QUERYOPEN`](crate::msg::wm::QueryOpen) message.
	///
	/// Sent to an icon when the user requests that the window be restored to its
	/// previous size and position.
	pub fn wm_query_open<F>(&self, func: F)
		where F: FnMut(wm::QueryOpen) -> bool + 'static,
	{
		self.add_msg(co::WM::QUERYOPEN, {
			let mut func = func;
			move |p| Some(func(wm::QueryOpen::from_generic_wm(p)) as isize)
		});
	}

	wm_ret_none! { wm_r_button_dbl_clk, co::WM::RBUTTONDBLCLK, wm::RButtonDblClk,
		/// [`WM_RBUTTONDBLCLK`](crate::msg::wm::RButtonDblClk) message.
		///
		/// Posted when the user double-clicks the right mouse button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	wm_ret_none! { wm_r_button_down, co::WM::RBUTTONDOWN, wm::RButtonDown,
		/// [`WM_RBUTTONDOWN`](crate::msg::wm::RButtonDown) message.
		///
		/// Posted when the user presses the right mouse button while the cursor
		/// is in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
	}

	wm_ret_none! { wm_r_button_up, co::WM::RBUTTONUP, wm::RButtonUp,
		/// [`WM_RBUTTONUP`](crate::msg::wm::RButtonUp) message.
		///
		/// Posted when the user releases the right mouse button while the cursor
		/// is in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
	}

	wm_ret_none! { wm_set_focus, co::WM::SETFOCUS, wm::SetFocus,
		/// [`WM_SETFOCUS`](crate::msg::wm::SetFocus) message.
		///
		/// Sent to a window after it has gained the keyboard focus.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * non-dialog [`WindowMain`](crate::gui::WindowMain);
		/// * non-dialog [`WindowModal`](crate::gui::WindowModal).
	}

	wm_ret_none! { wm_set_font, co::WM::SETFONT, wm::SetFont,
		/// [`WM_SETFONT`](crate::msg::wm::SetFont) message.
		///
		/// Sets the font that a control is to use when drawing text.
	}

	/// [`WM_SETICON`](crate::msg::wm::SetIcon) message.
	///
	/// Associates a new large or small icon with a window. The system displays
	/// the large icon in the Alt+TAB dialog box, and the small icon in the
	/// window caption.
	pub fn wm_set_icon<F>(&self, func: F)
		where F: FnMut(wm::SetIcon) -> Option<HICON> + 'static,
	{
		self.add_msg(co::WM::SETICON, {
			let mut func = func;
			move |p| Some(
				match func(wm::SetIcon::from_generic_wm(p)) {
					Some(hicon) => hicon.ptr as isize,
					None => 0,
				},
			)
		});
	}

	wm_ret_none! { wm_show_window, co::WM::SHOWWINDOW, wm::ShowWindow,
		/// [`WM_SHOWWINDOW`](crate::msg::wm::ShowWindow) message.
		///
		/// Sent to a window when the window is about to be hidden or shown.
	}

	wm_ret_none! { wm_size, co::WM::SIZE, wm::Size,
		/// [`WM_SIZE`](crate::msg::wm::Size) message.
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
		///     let wnd = wnd.clone(); // pass into the closure
		///     move |parms| {
		///         println!("HWND: {}, client area: {}x{}",
		///             wnd.hwnd(),
		///             parms.width,
		///             parms.height,
		///         );
		///     }
		/// });
		/// ```
	}

	wm_ret_none! { wm_sizing, co::WM::SIZING, wm::Sizing,
		/// [`WM_SIZING`](crate::msg::wm::Sizing) message.
		///
		/// Sent to a window that the user is resizing. By processing this
		/// message, an application can monitor the size and position of the drag
		/// rectangle and, if needed, change its size or position.
	}

	wm_ret_none! { wm_style_changed, co::WM::STYLECHANGED, wm::StyleChanged,
		/// [`WM_STYLECHANGED`](crate::msg::wm::StyleChanged) message.
		///
		/// Sent to a window after the
		/// [`SetWindowLongPtr`](crate::HWND::SetWindowLongPtr) function has
		/// changed one or more of the window's styles.
	}

	wm_ret_none! { wm_style_changing, co::WM::STYLECHANGING, wm::StyleChanging,
		/// [`WM_STYLECHANGING`](crate::msg::wm::StyleChanging) message.
		///
		/// Sent to a window when the
		/// [`SetWindowLongPtr`](crate::HWND::SetWindowLongPtr) function is about
		/// to change one or more of the window's styles.
	}

	wm_ret_none! { wm_sys_char, co::WM::SYSCHAR, wm::SysChar,
		/// [`WM_SYSCHAR`](crate::msg::wm::SysChar) message.
		///
		/// Posted to the window with the keyboard focus when a
		/// [`WM_SYSKEYDOWN`](crate::msg::wm::SysKeyDown) message is translated by
		/// the [`TranslateMessage`](crate::TranslateMessage) function. It
		/// specifies the character code of a system character key that is, a
		/// character key that is pressed while the ALT key is down.
	}

	wm_ret_none! { wm_sys_command, co::WM::SYSCOMMAND, wm::SysCommand,
		/// [`WM_SYSCOMMAND`](crate::msg::wm::SysCommand) message.
		///
		/// A window receives this message when the user chooses a command from
		/// the Window menu (formerly known as the system or control menu) or when
		/// the user chooses the maximize button, minimize button, restore button,
		/// or close button.
	}

	wm_ret_none! { wm_sys_dead_char, co::WM::SYSDEADCHAR, wm::SysDeadChar,
		/// [`WM_SYSDEADCHAR`](crate::msg::wm::SysDeadChar) message.
		///
		/// Sent to the window with the keyboard focus when a
		/// [`WM_SYSKEYDOWN`](crate::msg::wm::SysKeyDown) message is translated by
		/// the [`TranslateMessage`](crate::TranslateMessage) function.
		/// `WM_SYSDEADCHAR` specifies the character code of a system dead key
		/// that is, a dead key that is pressed while holding down the ALT key.
	}

	wm_ret_none! { wm_sys_key_down, co::WM::SYSKEYDOWN, wm::SysKeyDown,
		/// [`WM_SYSKEYDOWN`](crate::msg::wm::SysKeyDown) message.
		///
		/// Posted to the window with the keyboard focus when the user presses the
		/// F10 key (which activates the menu bar) or holds down the ALT key and
		/// then presses another key. It also occurs when no window currently has
		/// the keyboard focus; in this case, the `WM_SYSKEYDOWN` message is sent
		/// to the active window. The window that receives the message can
		/// distinguish between these two contexts by checking the context code in
		/// the lParam parameter.
	}

	wm_ret_none! { wm_sys_key_up, co::WM::SYSKEYUP, wm::SysKeyUp,
		/// [`WM_SYSKEYUP`](crate::msg::wm::SysKeyUp) message.
		///
		/// Posted to the window with the keyboard focus when the user releases a
		/// key that was pressed while the ALT key was held down. It also occurs
		/// when no window currently has the keyboard focus; in this case, the
		/// `WM_SYSKEYUP` message is sent to the active window. The window that
		/// receives the message can distinguish between these two contexts by
		/// checking the context code in the lParam parameter.
	}

	wm_ret_none! { wm_theme_changed, co::WM::THEMECHANGED, wm::ThemeChanged,
		/// [`WM_THEMECHANGED`](crate::msg::wm::ThemeChanged) message.
		///
		/// Broadcast to every window following a theme change event. Examples of
		/// theme change events are the activation of a theme, the deactivation of
		/// a theme, or a transition from one theme to another.
	}

	wm_ret_none! { wm_window_pos_changed, co::WM::WINDOWPOSCHANGED, wm::WindowPosChanged,
		/// [`WM_WINDOWPOSCHANGED`](crate::msg::wm::WindowPosChanged) message.
		///
		/// Sent to a window whose size, position, or place in the Z order has
		/// changed as a result of a call to the
		/// [`SetWindowPos`](crate::HWND::SetWindowPos) function or another
		/// window-management function.
	}

	wm_ret_none! { wm_window_pos_changing, co::WM::WINDOWPOSCHANGING, wm::WindowPosChanging,
		/// [`WM_WINDOWPOSCHANGING`](crate::msg::wm::WindowPosChanging) message.
		///
		/// Sent to a window whose size, position, or place in the Z order is
		/// about to change as a result of a call to the
		/// [`SetWindowPos`](crate::HWND::SetWindowPos) function or another
		/// window-management function.
	}

	wm_ret_none! { wm_x_button_dbl_clk, co::WM::XBUTTONDBLCLK, wm::XButtonDblClk,
		/// [`WM_XBUTTONDBLCLK`](crate::msg::wm::XButtonDblClk) message.
		///
		/// Posted when the user double-clicks the first or second X button while
		/// the cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	wm_ret_none! { wm_x_button_down, co::WM::XBUTTONDOWN, wm::XButtonDown,
		/// [`WM_XBUTTONDOWN`](crate::msg::wm::XButtonDown) message.
		///
		/// Posted when the user presses the first or second X button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	wm_ret_none! { wm_x_button_up, co::WM::XBUTTONUP, wm::XButtonUp,
		/// [`WM_XBUTTONUP`](crate::msg::wm::XButtonUp) message.
		///
		/// Posted when the user releases the first or second X button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}
}
