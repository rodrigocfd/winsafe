use std::rc::Rc;

use crate::aliases::BoxResult;
use crate::co;
use crate::gui::events::func_store::FuncStore;
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::handles::{HBRUSH, HFONT, HICON, HMENU};
use crate::msg::{MsgSendRecv, wm, WndMsg};

/// The result of processing a message.
pub(crate) enum ProcessResult {
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
pub struct WindowEvents(VeryUnsafeCell<Obj>);

struct Obj { // actual fields of WindowEvents
	msgs: FuncStore< // ordinary WM messages
		co::WM,
		Box<dyn Fn(WndMsg) -> BoxResult<Option<isize>>>, // return value may be meaningful
	>,
	tmrs: FuncStore< // WM_TIMER messages
		u32,
		Box<dyn Fn() -> BoxResult<()>>, // return value is never meaningful
	>,
	cmds: FuncStore< // WM_COMMAND notifications
		(co::CMD, u16), // notif code, control ID
		Box<dyn Fn() -> BoxResult<()>>, // return value is never meaningful
	>,
	nfys: FuncStore< // WM_NOTIFY notifications
		(u16, co::NM), // idFrom, code
		Box<dyn Fn(wm::Notify) -> BoxResult<Option<isize>>>, // return value may be meaningful
	>,
}

impl WindowEvents {
	pub(crate) fn new() -> WindowEvents {
		Self(
			VeryUnsafeCell::new(
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

	/// Searches for the last added user function for the given message, and
	/// runs if it exists, returning the result.
	pub(crate) fn process_one_message(&self,
		wm_any: WndMsg) -> BoxResult<ProcessResult>
	{
		Ok(match wm_any.msg_id {
			co::WM::NOTIFY => {
				let wm_nfy = wm::Notify::from_generic_wm(wm_any);
				let key = (wm_nfy.nmhdr.idFrom(), wm_nfy.nmhdr.code);
				match self.0.nfys.find(key) {
					Some(func) => { // we have a stored function to handle this WM_NOTIFY notification
						match func(wm_nfy)? { // execute user function
							Some(res) => ProcessResult::HandledWithRet(res), // meaningful return value
							None => ProcessResult::HandledWithoutRet,
						}
					},
					None => ProcessResult::NotHandled, // no stored WM_NOTIFY notification
				}
			},
			co::WM::COMMAND => {
				let wm_cmd = wm::Command::from_generic_wm(wm_any);
				let key = wm_cmd.event.code_id();
				match self.0.cmds.find(key) {
					Some(func) => { // we have a stored function to handle this WM_COMMAND notification
						func()?; // execute user function
						ProcessResult::HandledWithoutRet
					},
					None => ProcessResult::NotHandled, // no stored WM_COMMAND notification
				}
			},
			co::WM::TIMER => {
				let wm_tmr = wm::Timer::from_generic_wm(wm_any);
				match self.0.tmrs.find(wm_tmr.timer_id) {
					Some(func) => { // we have a stored function to handle this WM_TIMER message
						func()?; // execute user function
						ProcessResult::HandledWithoutRet
					},
					None => ProcessResult::NotHandled, // no stored WM_TIMER message
				}
			}
			_ => { // any other message
				match self.0.msgs.find(wm_any.msg_id) {
					Some(func) => { // we have a stored function to handle this message
						match func(wm_any)? { // execute user function
							Some(res) => ProcessResult::HandledWithRet(res), // meaningful return value
							None => ProcessResult::HandledWithoutRet,
						}
					},
					None => ProcessResult::NotHandled, // no stored function
				}
			},
		})
	}

	/// Searches for all user functions for the given message, and runs all of
	/// them, discarding the results.
	pub(crate) fn process_all_messages(&self, wm_any: WndMsg) -> BoxResult<()> {
		Ok(match wm_any.msg_id {
			co::WM::NOTIFY => {
				let wm_nfy = wm::Notify::from_generic_wm(wm_any);
				let key = (wm_nfy.nmhdr.idFrom(), wm_nfy.nmhdr.code);
				for func in self.0.nfys.find_all(key) {
					func(wm_nfy)?; // execute stored function
				}
			},
			co::WM::COMMAND => {
				let wm_cmd = wm::Command::from_generic_wm(wm_any);
				let key = wm_cmd.event.code_id();
				for func in self.0.cmds.find_all(key) {
					func()?; // execute stored function
				}
			},
			co::WM::TIMER => {
				let wm_tmr = wm::Timer::from_generic_wm(wm_any);
				for func in self.0.tmrs.find_all(wm_tmr.timer_id) {
					func()?; // execute stored function
				}
			},
			_ => { // any other message
				for func in self.0.msgs.find_all(wm_any.msg_id) {
					func(wm_any)?; // execute stored function
				}
			},
		})
	}

	/// Raw add message.
	pub(crate) fn add_msg<F>(&self, ident: co::WM, func: F)
		where F: Fn(WndMsg) -> BoxResult<Option<isize>> + 'static,
	{
		self.0.as_mut().msgs.insert(ident, Box::new(func));
	}

	/// Raw add notification.
	pub(crate) fn add_nfy<F>(&self, id_from: u16, code: co::NM, func: F)
		where F: Fn(wm::Notify) -> BoxResult<Option<isize>> + 'static,
	{
		self.0.as_mut().nfys.insert((id_from, code), Box::new(func));
	}

	/// Event to any [window message](crate::co::WM).
	///
	/// **Note:** Instead of using this event, you should always prefer the
	/// specific events, which will give you the correct message parameters.
	/// This generic method should be used when you have a custom, non-standard
	/// window message.
	///
	/// # Examples
	///
	/// Handling a custom, user-defined message:
	///
	/// ```rust,ignore
	/// use winsafe::{co, gui, msg, BoxResult};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	///
	/// let CUSTOM_MSG = co::WM::from(0x1234);
	///
	/// wnd.on().wm(CUSTOM_MSG, {
	///     let wnd = wnd.clone(); // pass into the closure
	///     move |p: msg::WndMsg| -> BoxResult<isize> {
	///         println!("HWND: {}, msg ID: {}", wnd.hwnd(), p.msg_id);
	///         Ok(0)
	///     }
	/// });
	/// ```
	pub fn wm<F>(&self, ident: co::WM, func: F)
		where F: Fn(WndMsg) -> BoxResult<isize> + 'static,
	{
		self.add_msg(ident, move |p| Ok(Some(func(p)?))); // return value is meaningful
	}

	/// [`WM_TIMER`](crate::msg::wm::Timer) message, narrowed to a specific
	/// timer ID.
	///
	/// Posted to the installing thread's message queue when a timer expires.
	pub fn wm_timer<F>(&self, timer_id: u32, func: F)
		where F: Fn() -> BoxResult<()> + 'static,
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
		where F: Fn() -> BoxResult<()> + 'static,
	{
		self.0.as_mut().cmds.insert((code, ctrl_id), Box::new(func));
	}

	/// [`WM_COMMAND`](crate::msg::wm::Command) message, handling both
	/// `CMD::Accelerator` and `CMD::Menu`, for a specific command ID.
	///
	/// Ideal to be used with menu commands whose IDs are shared with
	/// accelerators.
	///
	/// # Examples
	///
	/// Closing the window on ESC key:
	///
	/// ```rust,ignore
	/// use winsafe::{co, gui, msg, BoxResult};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	///
	/// wnd.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), {
	///     let wnd = wnd.clone(); // pass into the closure
	///     move || -> BoxResult<()> {
	///         wnd.hwnd().SendMessage(msg::wm::Close {});
	///         Ok(())
	///     }
	/// });
	/// ```
	pub fn wm_command_accel_menu<F>(&self, ctrl_id: u16, func: F)
		where F: Fn() -> BoxResult<()> + 'static,
	{
		let shared_func = Rc::new(VeryUnsafeCell::new(func));

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
	/// A notification must be narrowed by the
	/// [notification code](crate::co::NM) and the control ID, so the closure
	/// will be fired for that specific control at the specific event.
	///
	/// **Note:** Instead of using this event, you should always prefer the
	/// specific notifications, which will give you the correct notification
	/// struct. This generic method should be used when you have a custom,
	/// non-standard window notification.
	pub fn wm_notify<F>(&self, id_from: i32, code: co::NM, func: F)
		where F: Fn(wm::Notify) -> BoxResult<isize> + 'static,
	{
		self.add_nfy(id_from as _, code, move |p| Ok(Some(func(p)?))); // return value is meaningful
	}

	pub_fn_wm_ret0_param! { wm_activate, co::WM::ACTIVATE, wm::Activate,
		/// [`WM_ACTIVATE`](crate::msg::wm::Activate) message.
		///
		/// Sent to both the window being activated and the window being
		/// deactivated. If the windows use the same input queue, the message is
		/// sent synchronously, first to the window procedure of the top-level
		/// window being deactivated, then to the window procedure of the
		/// top-level window being activated. If the windows use different input
		/// queues, the message is sent asynchronously, so the window is
		/// activated immediately.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * non-dialog [`WindowMain`](crate::gui::WindowMain).
	}

	pub_fn_wm_ret0_param! { wm_activate_app, co::WM::ACTIVATEAPP, wm::ActivateApp,
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
	/// for example, by clicking an application command button using the mouse
	/// or typing an application command key on the keyboard.
	pub fn wm_app_command<F>(&self, func: F)
		where F: Fn(wm::AppCommand) -> BoxResult<()> + 'static,
	{
		self.add_msg(co::WM::APPCOMMAND,
			move |p| { func(wm::AppCommand::from_generic_wm(p))?; Ok(Some(true as _)) });
	}

	pub_fn_wm_ret0! { wm_cancel_mode, co::WM::CANCELMODE,
		/// [`WM_CANCELMODE`](crate::msg::wm::CancelMode) message.
		///
		/// Sent to cancel certain modes, such as mouse capture. For example,
		/// the system sends this message to the active window when a dialog box
		/// or message box is displayed. Certain functions also send this
		/// message explicitly to the specified window regardless of whether it
		/// is the active window. For example, the
		/// [`HWND::EnableWindow`](crate::HWND::EnableWindow) function sends
		/// this message when disabling the specified window.
	}

	pub_fn_wm_ret0_param! { wm_capture_changed, co::WM::CAPTURECHANGED, wm::CaptureChanged,
		/// [`WM_CAPTURECHANGED`](crate::msg::wm::CaptureChanged) message.
		///
		/// Sent to the window that is losing the mouse capture.
	}

	pub_fn_wm_ret0_param! { wm_char, co::WM::CHAR, wm::Char,
		/// [`WM_CHAR`](crate::msg::wm::Char) message.
		///
		/// Posted to the window with the keyboard focus when a
		/// [`WM_KEYDOWN`](crate::msg::wm::KeyDown) message is translated by the
		/// [`TranslateMessage`](crate::TranslateMessage) function. The
		/// `WM_CHAR` message contains the character code of the key that was
		/// pressed.
	}

	pub_fn_wm_ret0! { wm_child_activate, co::WM::CHILDACTIVATE,
		/// [`WM_CHILDACTIVATE`](crate::msg::wm::ChildActivate) message.
		///
		/// Sent to a child window when the user clicks the window's title bar
		/// or when the window is activated, moved, or sized.
	}

	pub_fn_wm_ret0! { wm_close, co::WM::CLOSE,
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

	pub_fn_wm_ret0! { wm_context_menu, co::WM::CONTEXTMENU,
		/// [`WM_CONTEXTMENU`](crate::msg::wm::ContextMenu) message.
		///
		/// Notifies a window that the user desires a context menu to appear.
		/// The user may have clicked the right mouse button (right-clicked) in
		/// the window, pressed Shift+F10 or pressed the applications key
		/// (context menu key) available on some keyboards.
	}

	/// [`WM_CREATE`](crate::msg::wm::Create) message, sent only to non-dialog
	/// windows. Dialog windows receive
	/// [`WM_INITDIALOG`](crate::gui::events::WindowEvents::wm_init_dialog)
	/// instead.
	///
	/// Sent when an application requests that a window be created by calling
	/// the [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx) function. The
	/// message is sent before the function returns. The window procedure of the
	/// new window receives this message after the window is created, but before
	/// the window becomes visible.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{gui, msg, BoxResult};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	///
	/// wnd.on().wm_create({
	///     let wnd = wnd.clone(); // pass into the closure
	///     move |p: msg::wm::Create| -> BoxResult<i32> {
	///         println!("HWND: {}, client area: {}x{}",
	///             wnd.hwnd(),
	///             p.createstruct.cx,
	///             p.createstruct.cy,
	///         );
	///         Ok(0)
	///     }
	/// });
	/// ```
	pub fn wm_create<F>(&self, func: F)
		where F: Fn(wm::Create) -> BoxResult<i32> + 'static,
	{
		self.add_msg(co::WM::CREATE,
			move |p| Ok(Some(func(wm::Create::from_generic_wm(p))? as _)));
	}

	/// [`WM_CTLCOLORBTN`](crate::msg::wm::CtlColorBtn) message.
	///
	/// Sent to the parent window of a button before drawing the button. The
	/// parent window can change the button's text and background colors.
	/// However, only owner-drawn buttons respond to the parent window
	/// processing this message.
	pub fn wm_ctl_color_btn<F>(&self, func: F)
		where F: Fn(wm::CtlColorBtn) -> BoxResult<HBRUSH> + 'static,
	{
		self.add_msg(co::WM::CTLCOLORBTN,
			move |p| Ok(Some(func(wm::CtlColorBtn::from_generic_wm(p))?.ptr as _)));
	}

	pub_fn_wm_ctlcolor! { wm_ctl_color_dlg, co::WM::CTLCOLORDLG, wm::CtlColorDlg,
		/// [`WM_CTLCOLORDLG`](crate::msg::wm::CtlColorDlg) message.
		///
		/// Sent to a dialog box before the system draws the dialog box. By
		/// responding to this message, the dialog box can set its text and
		/// background colors using the specified display device context handle.
	}

	pub_fn_wm_ctlcolor! { wm_ctl_color_edit, co::WM::CTLCOLOREDIT, wm::CtlColorEdit,
		/// [`WM_CTLCOLOREDIT`](crate::msg::wm::CtlColorEdit) message.
		///
		/// An edit control that is not read-only or disabled sends the message
		/// to its parent window when the control is about to be drawn. By
		/// responding to this message, the parent window can use the specified
		/// device context handle to set the text and background colors of the
		/// edit control.
	}

	pub_fn_wm_ctlcolor! { wm_ctl_color_list_box, co::WM::CTLCOLORLISTBOX, wm::CtlColorListBox,
		/// [`WM_CTLCOLORLISTBOX`](crate::msg::wm::CtlColorListBox) message.
		///
		/// Sent to the parent window of a list box before the system draws the
		/// list box. By responding to this message, the parent window can set
		/// the text and background colors of the list box by using the
		/// specified display device context handle.
	}

	pub_fn_wm_ctlcolor! { wm_ctl_color_scroll_bar, co::WM::CTLCOLORSCROLLBAR, wm::CtlColorScrollBar,
		/// [`WM_CTLCOLORSCROLLBAR`](crate::msg::wm::CtlColorScrollBar) message.
		///
		/// Sent to the parent window of a scroll bar control when the control
		/// is about to be drawn. By responding to this message, the parent
		/// window can use the display context handle to set the background
		/// color of the scroll bar control.
	}

	pub_fn_wm_ctlcolor! { wm_ctl_color_static, co::WM::CTLCOLORSTATIC, wm::CtlColorStatic,
		/// [`WM_CTLCOLORSTATIC`](crate::msg::wm::CtlColorStatic) message.
		///
		/// A static control, or an edit control that is read-only or disabled,
		/// sends the message to its parent window when the control is about to
		/// be drawn. By responding to this message, the parent window can use
		/// the specified device context handle to set the text foreground and
		/// background colors of the static control.
	}

	pub_fn_wm_ret0_param! { wm_dead_char, co::WM::DEADCHAR, wm::DeadChar,
		/// [`WM_DEADCHAR`](crate::msg::wm::DeadChar) message.
		///
		/// Posted to the window with the keyboard focus when a
		/// [`WM_KEYUP`](crate::msg::wm::KeyUp) message is translated by the
		/// [`TranslateMessage`](crate::TranslateMessage) function.
		/// `WM_DEADCHAR` specifies a character code generated by a dead key. A
		/// dead key is a key that generates a character, such as the umlaut
		/// (double-dot), that is combined with another character to form a
		/// composite character. For example, the umlaut-O character (Ö) is
		/// generated by typing the dead key for the umlaut character, and then
		/// typing the O key.
	}

	pub_fn_wm_retbool_param! { wm_delete_item, co::WM::DELETEITEM, wm::DeleteItem,
		/// [`WM_DELETEITEM`](crate::msg::wm::DeleteItem) message.
		///
		/// Sent to the owner of a list box or combo box when the list box or
		/// combo box is destroyed or when items are removed.
	}

	pub_fn_wm_ret0! { wm_destroy, co::WM::DESTROY,
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
		///
		/// # Examples
		///
		/// ```rust,ignore
		/// use winsafe::{gui, BoxResult};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		///
		/// wnd.on().wm_destroy(|| -> BoxResult<()> {
		///     println!("Window is gone, goodbye!");
		///     Ok(())
		/// });
		/// ```
	}

	pub_fn_wm_ret0_param! { wm_display_change, co::WM::DISPLAYCHANGE, wm::DisplayChange,
		/// [`WM_DISPLAYCHANGE`](crate::msg::wm::DisplayChange) message.
		///
		/// Sent to all windows when the display resolution has changed.
	}

	pub_fn_wm_ret0_param! { wm_drop_files, co::WM::DROPFILES, wm::DropFiles,
		/// [`WM_DROPFILES`](crate::msg::wm::DropFiles) message.
		///
		/// Sent when the user drops a file on the window of an application that
		/// has registered itself as a recipient of dropped files.
		///
		/// # Examples
		///
		/// ```rust,ignore
		/// use winsafe::{gui, msg, BoxResult};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		///
		/// wnd.on().wm_drop_files(|p: msg::wm::DropFiles| -> BoxResult<()> {
		///     for dropped_file in p.hdrop.DragQueryFiles()?.iter() {
		///         println!("Dropped: {}", dropped_file);
		///     }
		///     Ok(())
		/// });
		/// ```
	}

	pub_fn_wm_ret0_param! { wm_enable, co::WM::ENABLE, wm::Enable,
		/// [`WM_ENABLE`](crate::msg::wm::Enable) message.
		///
		/// Sent when an application changes the enabled state of a window. It
		/// is sent to the window whose enabled state is changing. This message
		/// is sent before the [`HWND::EnableWindow`](crate::HWND::EnableWindow)
		/// function returns, but after the enabled state
		/// ([`WS::DISABLED`](crate::co::WS::DISABLED) style bit) of the window
		/// has changed.
	}

	pub_fn_wm_ret0_param! { wm_end_session, co::WM::ENDSESSION, wm::EndSession,
		/// [`WM_ENDSESSION`](crate::msg::wm::EndSession) message.
		///
		/// Sent to an application after the system processes the results of the
		/// [`WM_QUERYENDSESSION`](crate::gui::events::WindowEvents) message.
		/// The `WM_ENDSESSION` message informs the application whether the
		/// session is ending.
	}

	pub_fn_wm_ret0_param! { wm_enter_idle, co::WM::ENTERIDLE, wm::EnterIdle,
		/// [`WM_ENTERIDLE`](crate::msg::wm::EnterIdle) message.
		///
		/// Sent to the owner window of a modal dialog box or menu that is
		/// entering an idle state. A modal dialog box or menu enters an idle
		/// state when no messages are waiting in its queue after it has
		/// processed one or more previous messages.
	}

	pub_fn_wm_ret0_param! { wm_enter_menu_loop, co::WM::ENTERMENULOOP, wm::EnterMenuLoop,
		/// [`WM_ENTERMENULOOP`](crate::msg::wm::EnterMenuLoop) message.
		///
		/// Notifies an application's main window procedure that a menu modal
		/// loop has been entered.
	}

	pub_fn_wm_ret0_param! { wm_enter_size_move, co::WM::ENTERSIZEMOVE, wm::EnterSizeMove,
		/// [`WM_ENTERSIZEMOVE`](crate::msg::wm::EnterSizeMove) message.
		///
		/// Sent one time to a window after it enters the moving or sizing modal
		/// loop. The window enters the moving or sizing modal loop when the
		/// user clicks the window's title bar or sizing border, or when the
		/// window passes the
		/// [`WM_SYSCOMMAND`](crate::gui::events::WindowEvents::wm_sys_command)
		/// message to the `DefWindowProc` function and the `wParam` parameter
		/// of the message specifies the [`SC_MOVE`](crate::co::SC::MOVE) or
		/// [`SC_SIZE`](crate::co::SC::SIZE) value. The operation is complete
		/// when `DefWindowProc` returns.
		///
		/// The system sends the message regardless of whether the dragging of
		/// full windows is enabled.
	}

	/// [`WM_ERASEBKGND`](crate::msg::wm::EraseBkgnd) message.
	///
	/// Sent when the window background must be erased (for example, when a
	/// window is resized). The message is sent to prepare an invalidated
	/// portion of a window for painting.
	pub fn wm_erase_bkgnd<F>(&self, func: F)
		where F: Fn(wm::EraseBkgnd) -> BoxResult<i32> + 'static,
	{
		self.add_msg(co::WM::ERASEBKGND,
			move |p| Ok(Some(func(wm::EraseBkgnd::from_generic_wm(p))? as _)));
	}

	pub_fn_wm_ret0_param! { wm_exit_menu_loop, co::WM::EXITMENULOOP, wm::ExitMenuLoop,
		/// [`WM_EXITMENULOOP`](crate::msg::wm::ExitMenuLoop) message.
		///
		/// Notifies an application's main window procedure that a menu modal
		/// loop has been exited.
	}

	pub_fn_wm_ret0! { wm_exit_size_move, co::WM::EXITSIZEMOVE,
		/// [`WM_EXITSIZEMOVE`](crate::msg::wm::ExitSizeMove) message.
		///
		/// Sent one time to a window, after it has exited the moving or sizing
		/// modal loop. The window enters the moving or sizing modal loop when
		/// the user clicks the window's title bar or sizing border, or when the
		/// window passes the
		/// [`WM_SYSCOMMAND`](crate::gui::events::WindowEvents::wm_sys_command)
		/// message to the `DefWindowProc` function and the `wParam` parameter
		/// of the message specifies the [`SC_MOVE`](crate::co::SC::MOVE) or
		/// [`SC_SIZE`](crate::co::SC::SIZE) value. The operation is complete
		/// when `DefWindowProc` returns.
	}

	pub_fn_wm_retco_param! { wm_get_dlg_code, co::WM::GETDLGCODE, wm::GetDlgCode, co::DLGC,
		/// [`WM_GETDLGCODE`](crate::msg::wm::GetDlgCode) message.
		///
		/// By default, the system handles all keyboard input to the control;
		/// the system interprets certain types of keyboard input as dialog box
		/// navigation keys. To override this default behavior, the control can
		/// respond to the `WM_GETDLGCODE` message to indicate the types of
		/// input it wants to process itself.
	}

	/// [`WM_GETFONT`](crate::msg::wm::GetFont) message.
	///
	/// Retrieves the font with which the control is currently drawing its text.
	pub fn wm_get_font<F>(&self, func: F)
		where F: Fn() -> BoxResult<Option<HFONT>> + 'static,
	{
		self.add_msg(co::WM::GETFONT,
			move |_| Ok(Some(func()?.map(|h| h.ptr as _).unwrap_or_default())));
	}

	/// [`MN_GETHMENU`](crate::msg::wm::GetHMenu) message.
	///
	/// Retrieves the menu handle for the current window.
	pub fn wm_get_hmenu<F>(&self, func: F)
		where F: Fn() -> BoxResult<Option<HMENU>> + 'static
	{
		self.add_msg(co::WM::MN_GETHMENU,
			move |_| Ok(Some(func()?.map(|h| h.ptr as _).unwrap_or_default())));
	}

	pub_fn_wm_ret0_param! { wm_get_min_max_info, co::WM::GETMINMAXINFO, wm::GetMinMaxInfo,
		/// [`WM_GETMINMAXINFO`](crate::msg::wm::GetMinMaxInfo) message.
		///
		/// Sent to a window when the size or position of the window is about to
		/// change. An application can use this message to override the window's
		/// default maximized size and position, or its default minimum or
		/// maximum tracking size.
	}

	pub_fn_wm_ret0_param! { wm_get_title_bar_info_ex, co::WM::GETTITLEBARINFOEX, wm::GetTitleBarInfoEx,
		/// [`WM_GETTITLEBARINFOEX`](crate::msg::wm::GetTitleBarInfoEx) message.
		///
		/// Sent to request extended title bar information.
	}

	pub_fn_wm_ret0_param! { wm_help, co::WM::HELP, wm::Help,
		/// [`WM_HELP`](crate::msg::wm::Help) message.
		///
		/// Indicates that the user pressed the F1 key.
	}

	pub_fn_wm_ret0_param! { wm_h_scroll, co::WM::HSCROLL, wm::HScroll,
		/// [`WM_HSCROLL`](crate::msg::wm::HScroll) message.
		///
		/// The WM_HSCROLL message is sent to a window when a scroll event
		/// occurs in the window's standard horizontal scroll bar. This message
		/// is also sent to the owner of a horizontal scroll bar control when a
		/// scroll event occurs in the control.
	}

	pub_fn_wm_retbool_param! { wm_init_dialog, co::WM::INITDIALOG, wm::InitDialog,
		/// [`WM_INITDIALOG`](crate::msg::wm::InitDialog) message, sent only to
		/// dialog windows. Non-dialog windows receive
		/// [`WM_CREATE`](crate::gui::events::WindowEvents::wm_create) instead.
		///
		/// Sent to the dialog box procedure immediately before a dialog box is
		/// displayed. Dialog box procedures typically use this message to
		/// initialize controls and carry out any other initialization tasks that
		/// affect the appearance of the dialog box.
		///
		/// # Examples
		///
		/// ```rust,ignore
		/// use winsafe::{gui, msg, BoxResult};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		///
		/// wnd.on().wm_init_dialog({
		///     let wnd = wnd.clone(); // pass into the closure
		///     move |p: msg::wm::InitDialog| -> BoxResult<bool> {
		///         println!("Focused HWND: {}", p.hwnd_focus);
		///         Ok(true)
		///     }
		/// });
		/// ```
	}

	pub_fn_wm_ret0_param! { wm_init_menu_popup, co::WM::INITMENUPOPUP, wm::InitMenuPopup,
		/// [`WM_INITMENUPOPUP`](crate::msg::wm::InitMenuPopup) message.
		///
		/// Sent when a drop-down menu or submenu is about to become active.
		/// This allows an application to modify the menu before it is
		/// displayed, without changing the entire menu.
	}

	pub_fn_wm_ret0_param! { wm_key_down, co::WM::KEYDOWN, wm::KeyDown,
		/// [`WM_KEYDOWN`](crate::msg::wm::KeyDown) message.
		///
		/// Posted to the window with the keyboard focus when a nonsystem key is
		/// pressed. A nonsystem key is a key that is pressed when the ALT key
		/// is not pressed.
	}

	pub_fn_wm_ret0_param! { wm_key_up, co::WM::KEYUP, wm::KeyUp,
		/// [`WM_KEYUP`](crate::msg::wm::KeyUp) message.
		///
		/// Posted to the window with the keyboard focus when a nonsystem key is
		/// released. A nonsystem key is a key that is pressed when the ALT key
		/// is not pressed, or a keyboard key that is pressed when a window has
		/// the keyboard focus.
	}

	pub_fn_wm_ret0_param! { wm_kill_focus, co::WM::KILLFOCUS, wm::KillFocus,
		/// [`WM_KILLFOCUS`](crate::msg::wm::KillFocus) message.
		///
		/// Sent to a window immediately before it loses the keyboard focus.
	}

	pub_fn_wm_ret0_param! { wm_l_button_dbl_clk, co::WM::LBUTTONDBLCLK, wm::LButtonDblClk,
		/// [`WM_LBUTTONDBLCLK`](crate::msg::wm::LButtonDblClk) message.
		///
		/// Posted when the user double-clicks the left mouse button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	pub_fn_wm_ret0_param! { wm_l_button_down, co::WM::LBUTTONDOWN, wm::LButtonDown,
		/// [`WM_LBUTTONDOWN`](crate::msg::wm::LButtonDown) message.
		///
		/// Posted when the user presses the left mouse button while the cursor
		/// is in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
		///
		/// # Examples
		///
		/// ```rust,ignore
		/// use winsafe::{gui, msg, BoxResult};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		///
		/// wnd.on().wm_l_button_down({
		///     let wnd = wnd.clone(); // pass into the closure
		///     move |p: msg::wm::LButtonDown| -> BoxResult<()> {
		///         println!("Point: {}x{}", p.coords.x, p.coords.y);
		///         Ok(())
		///     }
		/// });
		/// ```
	}

	pub_fn_wm_ret0_param! { wm_l_button_up, co::WM::LBUTTONUP, wm::LButtonUp,
		/// [`WM_LBUTTONUP`](crate::msg::wm::LButtonUp) message.
		///
		/// Posted when the user releases the left mouse button while the cursor
		/// is in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
	}

	pub_fn_wm_ret0_param! { wm_m_button_dbl_clk, co::WM::MBUTTONDBLCLK, wm::MButtonDblClk,
		/// [`WM_MBUTTONDBLCLK`](crate::msg::wm::MButtonDblClk) message.
		///
		/// Posted when the user double-clicks the middle mouse button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	pub_fn_wm_ret0_param! { wm_m_button_down, co::WM::MBUTTONDOWN, wm::MButtonDown,
		/// [`WM_MBUTTONDOWN`](crate::msg::wm::MButtonDown) message.
		///
		/// Posted when the user presses the middle mouse button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	pub_fn_wm_ret0_param! { wm_m_button_up, co::WM::MBUTTONUP, wm::MButtonUp,
		/// [`WM_MBUTTONUP`](crate::msg::wm::MButtonUp) message.
		///
		/// Posted when the user releases the middle mouse button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	pub_fn_wm_ret0_param! { wm_menu_command, co::WM::MENUCOMMAND, wm::MenuCommand,
		/// [`WM_MENUCOMMAND`](crate::msg::wm::MenuCommand) message.
		///
		/// Sent when the user makes a selection from a menu.
	}

	pub_fn_wm_retco_param! { wm_menu_drag, co::WM::MENUDRAG, wm::MenuDrag, co::MND,
		/// [`WM_MENUDRAG`](crate::msg::wm::MenuDrag) message.
		///
		/// Sent to the owner of a drag-and-drop menu when the user drags a menu
		/// item.
	}

	pub_fn_wm_ret0_param! { wm_menu_r_button_up, co::WM::MENURBUTTONUP, wm::MenuRButtonUp,
		/// [`WM_MENURBUTTONUP`](crate::msg::wm::MenuRButtonUp) message.
		///
		/// Sent when the user releases the right mouse button while the cursor
		/// is on a menu item.
	}

	pub_fn_wm_ret0_param! { wm_mouse_hover, co::WM::MOUSEHOVER, wm::MouseHover,
		/// [`WM_MOUSEHOVER`](crate::msg::wm::MouseHover) message.
		///
		/// Posted to a window when the cursor hovers over the client area of
		/// the window for the period of time specified in a prior call to
		/// [`TrackMouseEvent`](crate::TrackMouseEvent).
	}

	pub_fn_wm_ret0! { wm_mouse_leave, co::WM::MOUSELEAVE,
		/// [`WM_MOUSELEAVE`](crate::msg::wm::MouseLeave) message.
		///
		/// Posted to a window when the cursor leaves the client area of the
		/// window specified in a prior call to
		/// [`TrackMouseEvent`](crate::TrackMouseEvent).
	}

	pub_fn_wm_ret0_param! { wm_mouse_move, co::WM::MOUSEMOVE, wm::MouseMove,
		/// [`WM_MOUSEMOVE`](crate::msg::wm::MouseMove) message.
		///
		/// Posted to a window when the cursor moves. If the mouse is not
		/// captured, the message is posted to the window that contains the
		/// cursor. Otherwise, the message is posted to the window that has
		/// captured the mouse.
	}

	pub_fn_wm_ret0_param! { wm_move, co::WM::MOVE, wm::Move,
		/// [`WM_MOVE`](crate::msg::wm::Move) message.
		///
		/// Sent after a window has been moved.
	}

	pub_fn_wm_ret0_param! { wm_moving, co::WM::MOVING, wm::Moving,
		/// [`WM_MOVING`](crate::msg::wm::Moving) message.
		///
		/// Sent to a window that the user is moving. By processing this
		/// message, an application can monitor the position of the drag
		/// rectangle and, if needed, change its position.
	}

	pub_fn_wm_retco_param! { wm_nc_calc_size, co::WM::NCCALCSIZE, wm::NcCalcSize, co::WVR,
		/// [`WM_NCCALCSIZE`](crate::msg::wm::NcCalcSize) message.
		///
		/// Sent when the size and position of a window's client area must be
		/// calculated. By processing this message, an application can control
		/// the content of the window's client area when the size or position of
		/// the window changes.
	}

	pub_fn_wm_retbool_param! { wm_nc_create, co::WM::NCCREATE, wm::NcCreate,
		/// [`WM_NCCREATE`](crate::msg::wm::NcCreate) message.
		///
		/// Sent prior to the
		/// [`WM_CREATE`](crate::gui::events::WindowEvents::wm_create) message when
		/// a window is first created.
	}

	pub_fn_wm_ret0! { wm_nc_destroy, co::WM::NCDESTROY,
		/// [`WM_NCDESTROY`](crate::msg::wm::NcDestroy) message.
		///
		/// Notifies a window that its nonclient area is being destroyed. The
		/// [`HWND::DestroyWindow`](crate::HWND::DestroyWindow) function sends
		/// the message to the window following the
		/// [`WM_DESTROY`](crate::gui::events::WindowEvents::wm_destroy)
		/// message. `WM_DESTROY` is used to free the allocated memory object
		/// associated with the window.
		///
		/// The `WM_NCDESTROY` message is sent after the child windows have been
		/// destroyed. In contrast, `WM_DESTROY` is sent before the child
		/// windows are destroyed.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * non-dialog [`WindowMain`](crate::gui::WindowMain);
		/// * dialog [`WindowMain`](crate::gui::WindowMain).
	}

	pub_fn_wm_retco_param! { wm_nc_hit_test, co::WM::NCHITTEST, wm::NcHitTest, co::HT,
		/// [`WM_NCHITTEST`](crate::msg::wm::NcHitTest) message.
		///
		/// Sent to a window in order to determine what part of the window
		/// corresponds to a particular screen coordinate. This can happen, for
		/// example, when the cursor moves, when a mouse button is pressed or
		/// released, or in response to a call to a function such as
		/// [`HWND::WindowFromPoint`](crate::HWND::WindowFromPoint). If the
		/// mouse is not captured, the message is sent to the window beneath the
		/// cursor. Otherwise, the message is sent to the window that has
		/// captured the mouse.
	}

	pub_fn_wm_ret0_param! { wm_nc_paint, co::WM::NCPAINT, wm::NcPaint,
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

	pub_fn_wm_ret0_param! { wm_next_dlg_ctl, co::WM::NEXTDLGCTL, wm::NextDlgCtl,
		/// [`WM_NEXTDLGCTL`](crate::msg::wm::NextDlgCtl) message.
		///
		/// Sent to a dialog box procedure to set the keyboard focus to a
		/// different control in the dialog box.
	}

	pub_fn_wm_ret0! { wm_null, co::WM::NULL,
		/// [`WM_NULL`](crate::msg::wm::Null) message.
		///
		/// Performs no operation. An application sends the message if it wants
		/// to post a message that the recipient window will ignore.
	}

	pub_fn_wm_ret0! { wm_paint, co::WM::PAINT,
		/// [`WM_PAINT`](crate::msg::wm::Paint) message.
		///
		/// Sent when the system or another application makes a request to paint
		/// a portion of an application's window. The message is sent when the
		/// [`HWND::UpdateWindow`](crate::HWND::UpdateWindow) or
		/// [`HWND::RedrawWindow`](crate::HWND::RedrawWindow) function is
		/// called, or by the [`DispatchMessage`](crate::DispatchMessage)
		/// function when the application obtains a `WM_PAINT` message by using
		/// the [`GetMessage`](crate::GetMessage) or
		/// [`PeekMessage`](crate::PeekMessage) function.
	}

	pub_fn_wm_ret0_param! { wm_parent_notify, co::WM::PARENTNOTIFY, wm::ParentNotify,
		/// [`WM_PARENTNOTIFY`](crate::msg::wm::ParentNotify) message.
		///
		/// Sent to a window when a significant action occurs on a descendant
		/// window.
	}

	/// [`WM_QUERYOPEN`](crate::msg::wm::QueryOpen) message.
	///
	/// Sent to an icon when the user requests that the window be restored to
	/// its previous size and position.
	pub fn wm_query_open<F>(&self, func: F)
		where F: Fn() -> BoxResult<bool> + 'static,
	{
		self.add_msg(co::WM::QUERYOPEN, move |_| Ok(Some(func()? as _)));
	}

	pub_fn_wm_ret0_param! { wm_r_button_dbl_clk, co::WM::RBUTTONDBLCLK, wm::RButtonDblClk,
		/// [`WM_RBUTTONDBLCLK`](crate::msg::wm::RButtonDblClk) message.
		///
		/// Posted when the user double-clicks the right mouse button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	pub_fn_wm_ret0_param! { wm_r_button_down, co::WM::RBUTTONDOWN, wm::RButtonDown,
		/// [`WM_RBUTTONDOWN`](crate::msg::wm::RButtonDown) message.
		///
		/// Posted when the user presses the right mouse button while the cursor
		/// is in the client area of a window. If the mouse is not captured, the
		/// message is posted to the window beneath the cursor. Otherwise, the
		/// message is posted to the window that has captured the mouse.
	}

	pub_fn_wm_ret0_param! { wm_r_button_up, co::WM::RBUTTONUP, wm::RButtonUp,
		/// [`WM_RBUTTONUP`](crate::msg::wm::RButtonUp) message.
		///
		/// Posted when the user releases the right mouse button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	pub_fn_wm_retbool_param! { wm_set_cursor, co::WM::SETCURSOR, wm::SetCursor,
		/// [`WM_SETCURSOR`](crate::msg::wm::SetCursor) message.
		///
		/// Sent to a window if the mouse causes the cursor to move within a
		/// window and mouse input is not captured.
	}

	pub_fn_wm_ret0_param! { wm_set_focus, co::WM::SETFOCUS, wm::SetFocus,
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

	pub_fn_wm_ret0_param! { wm_set_font, co::WM::SETFONT, wm::SetFont,
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
		where F: Fn(wm::SetIcon) -> BoxResult<Option<HICON>> + 'static,
	{
		self.add_msg(co::WM::SETICON, move |p|
			Ok(Some(
				func(wm::SetIcon::from_generic_wm(p))?
					.map(|h| h.ptr as _).unwrap_or_default(),
			))
		);
	}

	pub_fn_wm_ret0_param! { wm_set_redraw, co::WM::SETREDRAW, wm::SetRedraw,
		/// [`WM_SETRDRAW`](crate::msg::wm::SetRedraw) message.
		///
		/// Sent to a window to allow changes in that window to be redrawn, or
		/// to prevent changes in that window from being redrawn.
	}

	pub_fn_wm_ret0_param! { wm_show_window, co::WM::SHOWWINDOW, wm::ShowWindow,
		/// [`WM_SHOWWINDOW`](crate::msg::wm::ShowWindow) message.
		///
		/// Sent to a window when the window is about to be hidden or shown.
	}

	pub_fn_wm_ret0_param! { wm_size, co::WM::SIZE, wm::Size,
		/// [`WM_SIZE`](crate::msg::wm::Size) message.
		///
		/// Sent to a window after its size has changed.
		///
		/// # Examples
		///
		/// ```rust,ignore
		/// use winsafe::{gui, msg, BoxResult};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		///
		/// wnd.on().wm_size({
		///     let wnd = wnd.clone(); // pass into the closure
		///     move |p: msg::wm::Size| -> BoxResult<()> {
		///         println!("HWND: {}, client area: {}x{}",
		///             wnd.hwnd(),
		///             p.width,
		///             p.height,
		///         );
		///         Ok(())
		///     }
		/// });
		/// ```
	}

	pub_fn_wm_ret0_param! { wm_sizing, co::WM::SIZING, wm::Sizing,
		/// [`WM_SIZING`](crate::msg::wm::Sizing) message.
		///
		/// Sent to a window that the user is resizing. By processing this
		/// message, an application can monitor the size and position of the
		/// drag rectangle and, if needed, change its size or position.
	}

	pub_fn_wm_ret0_param! { wm_style_changed, co::WM::STYLECHANGED, wm::StyleChanged,
		/// [`WM_STYLECHANGED`](crate::msg::wm::StyleChanged) message.
		///
		/// Sent to a window after the
		/// [`HWND::SetWindowLongPtr`](crate::HWND::SetWindowLongPtr) function
		/// has changed one or more of the window's styles.
	}

	pub_fn_wm_ret0_param! { wm_style_changing, co::WM::STYLECHANGING, wm::StyleChanging,
		/// [`WM_STYLECHANGING`](crate::msg::wm::StyleChanging) message.
		///
		/// Sent to a window when the
		/// [`HWND::SetWindowLongPtr`](crate::HWND::SetWindowLongPtr) function
		/// is about to change one or more of the window's styles.
	}

	pub_fn_wm_ret0! { wm_sync_paint, co::WM::SYNCPAINT,
		/// [`WM_SYNCPAINT`](crate::msg::wm::SyncPaint) message.
		///
		/// Used to synchronize painting while avoiding linking independent GUI
		/// threads.
	}

	pub_fn_wm_ret0_param! { wm_sys_char, co::WM::SYSCHAR, wm::SysChar,
		/// [`WM_SYSCHAR`](crate::msg::wm::SysChar) message.
		///
		/// Posted to the window with the keyboard focus when a
		/// [`WM_SYSKEYDOWN`](crate::msg::wm::SysKeyDown) message is translated
		/// by the [`TranslateMessage`](crate::TranslateMessage) function. It
		/// specifies the character code of a system character key that is, a
		/// character key that is pressed while the ALT key is down.
	}

	pub_fn_wm_ret0_param! { wm_sys_command, co::WM::SYSCOMMAND, wm::SysCommand,
		/// [`WM_SYSCOMMAND`](crate::msg::wm::SysCommand) message.
		///
		/// A window receives this message when the user chooses a command from
		/// the Window menu (formerly known as the system or control menu) or
		/// when the user chooses the maximize button, minimize button, restore
		/// button, or close button.
	}

	pub_fn_wm_ret0_param! { wm_sys_dead_char, co::WM::SYSDEADCHAR, wm::SysDeadChar,
		/// [`WM_SYSDEADCHAR`](crate::msg::wm::SysDeadChar) message.
		///
		/// Sent to the window with the keyboard focus when a
		/// [`WM_SYSKEYDOWN`](crate::msg::wm::SysKeyDown) message is translated
		/// by the [`TranslateMessage`](crate::TranslateMessage) function.
		/// `WM_SYSDEADCHAR` specifies the character code of a system dead key
		/// that is, a dead key that is pressed while holding down the ALT key.
	}

	pub_fn_wm_ret0_param! { wm_sys_key_down, co::WM::SYSKEYDOWN, wm::SysKeyDown,
		/// [`WM_SYSKEYDOWN`](crate::msg::wm::SysKeyDown) message.
		///
		/// Posted to the window with the keyboard focus when the user presses
		/// the F10 key (which activates the menu bar) or holds down the ALT key
		/// and then presses another key. It also occurs when no window
		/// currently has the keyboard focus; in this case, the `WM_SYSKEYDOWN`
		/// message is sent to the active window. The window that receives the
		/// message can distinguish between these two contexts by checking the
		/// context code in the lParam parameter.
	}

	pub_fn_wm_ret0_param! { wm_sys_key_up, co::WM::SYSKEYUP, wm::SysKeyUp,
		/// [`WM_SYSKEYUP`](crate::msg::wm::SysKeyUp) message.
		///
		/// Posted to the window with the keyboard focus when the user releases
		/// a key that was pressed while the ALT key was held down. It also
		/// occurs when no window currently has the keyboard focus; in this
		/// case, the `WM_SYSKEYUP` message is sent to the active window. The
		/// window that receives the message can distinguish between these two
		/// contexts by checking the context code in the lParam parameter.
	}

	pub_fn_wm_ret0! { wm_theme_changed, co::WM::THEMECHANGED,
		/// [`WM_THEMECHANGED`](crate::msg::wm::ThemeChanged) message.
		///
		/// Broadcast to every window following a theme change event. Examples
		/// of theme change events are the activation of a theme, the
		/// deactivation of a theme, or a transition from one theme to another.
	}

	pub_fn_wm_ret0_param! { wm_uninit_menu_popup, co::WM::UNINITMENUPOPUP, wm::UninitMenuPopup,
		/// [`WM_UNINITMENUPOPUP`](crate::msg::wm::UninitMenuPopup) message.
		///
		/// Sent when a drop-down menu or submenu has been destroyed.
	}

	pub_fn_wm_ret0_param! { wm_v_scroll, co::WM::VSCROLL, wm::VScroll,
		/// [`WM_VSCROLL`](crate::msg::wm::VScroll) message.
		///
		/// The WM_VSCROLL message is sent to a window when a scroll event
		/// occurs in the window's standard vertical scroll bar. This message is
		/// also sent to the owner of a vertical scroll bar control when a
		/// scroll event occurs in the control.
	}

	pub_fn_wm_ret0_param! { wm_window_pos_changed, co::WM::WINDOWPOSCHANGED, wm::WindowPosChanged,
		/// [`WM_WINDOWPOSCHANGED`](crate::msg::wm::WindowPosChanged) message.
		///
		/// Sent to a window whose size, position, or place in the Z order has
		/// changed as a result of a call to the
		/// [`HWND::SetWindowPos`](crate::HWND::SetWindowPos) function or
		/// another window-management function.
	}

	pub_fn_wm_ret0_param! { wm_window_pos_changing, co::WM::WINDOWPOSCHANGING, wm::WindowPosChanging,
		/// [`WM_WINDOWPOSCHANGING`](crate::msg::wm::WindowPosChanging) message.
		///
		/// Sent to a window whose size, position, or place in the Z order is
		/// about to change as a result of a call to the
		/// [`HWND::SetWindowPos`](crate::HWND::SetWindowPos) function or
		/// another window-management function.
	}

	pub_fn_wm_ret0_param! { wm_x_button_dbl_clk, co::WM::XBUTTONDBLCLK, wm::XButtonDblClk,
		/// [`WM_XBUTTONDBLCLK`](crate::msg::wm::XButtonDblClk) message.
		///
		/// Posted when the user double-clicks the first or second X button
		/// while the cursor is in the client area of a window. If the mouse is
		/// not captured, the message is posted to the window beneath the
		/// cursor. Otherwise, the message is posted to the window that has
		/// captured the mouse.
	}

	pub_fn_wm_ret0_param! { wm_x_button_down, co::WM::XBUTTONDOWN, wm::XButtonDown,
		/// [`WM_XBUTTONDOWN`](crate::msg::wm::XButtonDown) message.
		///
		/// Posted when the user presses the first or second X button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}

	pub_fn_wm_ret0_param! { wm_x_button_up, co::WM::XBUTTONUP, wm::XButtonUp,
		/// [`WM_XBUTTONUP`](crate::msg::wm::XButtonUp) message.
		///
		/// Posted when the user releases the first or second X button while the
		/// cursor is in the client area of a window. If the mouse is not
		/// captured, the message is posted to the window beneath the cursor.
		/// Otherwise, the message is posted to the window that has captured the
		/// mouse.
	}
}
