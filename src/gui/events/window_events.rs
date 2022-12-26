use crate::co;
use crate::gdi::decl::HFONT;
use crate::gui::events::func_store::FuncStore;
use crate::kernel::decl::AnyResult;
use crate::msg::{wm, WndMsg};
use crate::prelude::MsgSendRecv;
use crate::user::decl::{HICON, HMENU};

/// The result of processing a message.
pub(in crate::gui) enum ProcessResult {
	/// Message was not handled because no function was found.
	NotHandled,
	/// Message handled, and return value is meaningful.
	HandledWithRet(isize),
	/// Message handled, but you should return the default value (0 or FALSE).
	HandledWithoutRet,
}

//------------------------------------------------------------------------------

/// Exposes window
/// [messages](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
///
/// You cannot directly instantiate this object, it is created internally by the
/// window.
pub struct WindowEvents {
	msgs: FuncStore< // ordinary WM messages
		co::WM,
		Box<dyn Fn(WndMsg) -> AnyResult<Option<isize>>>, // return value may be meaningful
	>,
}

impl WindowEvents {
	pub(in crate::gui) fn new() -> Self {
		Self { msgs: FuncStore::new() }
	}

	pub(in crate::gui) fn is_empty(&self) -> bool {
		self.msgs.is_empty()
	}

	/// Removes all stored events.
	pub(in crate::gui) fn clear(&self) {
		self.msgs.clear();
	}

	/// Searches for the last added user function for the given message, and
	/// runs if it exists, returning the result.
	pub(in crate::gui) fn process_one_message(&self,
		wm_any: WndMsg) -> AnyResult<ProcessResult>
	{
		Ok(match self.msgs.find(wm_any.msg_id) {
			Some(func) => { // we have a stored function to handle this message
				match func(wm_any)? { // execute user function
					Some(res) => ProcessResult::HandledWithRet(res), // meaningful return value
					None => ProcessResult::HandledWithoutRet,
				}
			},
			None => ProcessResult::NotHandled, // no stored function
		})
	}

	/// Searches for all user functions for the given message, and runs all of
	/// them, discarding the results.
	pub(in crate::gui) fn process_all_messages(&self,
		wm_any: WndMsg) -> AnyResult<()>
	{
		for func in self.msgs.find_all(wm_any.msg_id) {
			func(wm_any)?; // execute each stored function
		}
		Ok(())
	}
}

//------------------------------------------------------------------------------

impl GuiEvents for WindowEvents {
	fn wm<F>(&self, ident: co::WM, func: F)
		where F: Fn(WndMsg) -> AnyResult<Option<isize>> + 'static,
	{
		self.msgs.push(ident, Box::new(func));
	}
}

/// Exposes the basic window message methods of
/// [`WindowEvents`](crate::gui::events::WindowEvents).
pub trait GuiEvents {
	/// Event to any [window message](crate::co::WM).
	///
	/// **Note:** Instead of using this event, you should always prefer the
	/// specific events, which will give you the correct message parameters.
	/// This generic method should be used only when you have a custom,
	/// non-standard window message – which should be pretty rare.
	///
	/// # Examples
	///
	/// Handling a custom, user-defined message:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, gui, msg, AnyResult};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// let CUSTOM_MSG = co::WM::from(0x1234);
	///
	/// wnd.on().wm(CUSTOM_MSG, {
	///     let wnd = wnd.clone(); // to pass into the closure
	///     move |p: msg::WndMsg| -> AnyResult<Option<isize>> {
	///         println!("HWND: {}, msg ID: {}", wnd.hwnd(), p.msg_id);
	///         Ok(Some(0))
	///     }
	/// });
	/// ```
	fn wm<F>(&self, ident: co::WM, func: F)
		where F: Fn(WndMsg) -> AnyResult<Option<isize>> + 'static;

	fn_wm_withparm_noret! { wm_activate, co::WM::ACTIVATE, wm::Activate,
		/// [`WM_ACTIVATE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-activate)
		/// message.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * non-dialog [`WindowMain`](crate::gui::WindowMain).
	}

	fn_wm_withparm_noret! { wm_activate_app, co::WM::ACTIVATEAPP, wm::ActivateApp,
		/// [`WM_ACTIVATEAPP`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-activateapp)
		/// message.
	}

	/// [`WM_APPCOMMAND`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-appcommand)
	/// message.
	fn wm_app_command<F>(&self, func: F)
		where F: Fn(wm::AppCommand) -> AnyResult<()> + 'static,
	{
		self.wm(co::WM::APPCOMMAND, move |p| {
			func(wm::AppCommand::from_generic_wm(p))?;
			Ok(Some(1)) // TRUE
		});
	}

	fn_wm_noparm_noret! { wm_cancel_mode, co::WM::CANCELMODE,
		/// [`WM_CANCELMODE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-cancelmode)
		/// message.
	}

	fn_wm_withparm_noret! { wm_capture_changed, co::WM::CAPTURECHANGED, wm::CaptureChanged,
		/// [`WM_CAPTURECHANGED`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-capturechanged)
		/// message.
	}

	fn_wm_withparm_noret! { wm_char, co::WM::CHAR, wm::Char,
		/// [`WM_CHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-char)
		/// message.
	}

	fn_wm_noparm_noret! { wm_child_activate, co::WM::CHILDACTIVATE,
		/// [`WM_CHILDACTIVATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-childactivate)
		/// message.
	}

	fn_wm_noparm_noret! { wm_close, co::WM::CLOSE,
		/// [`WM_CLOSE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-close)
		/// message.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * dialog [`WindowMain`](crate::gui::WindowMain);
		/// * dialog [`WindowModal`](crate::gui::WindowModal);
		/// * non-dialog [`WindowModal`](crate::gui::WindowModal).
	}

	fn_wm_noparm_noret! { wm_context_menu, co::WM::CONTEXTMENU,
		/// [`WM_CONTEXTMENU`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-contextmenu)
		/// message.
	}

	/// [`WM_CREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create)
	/// message, sent only to non-dialog windows. Dialog windows receive
	/// [`WM_INITDIALOG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
	/// instead.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{gui, msg, AnyResult};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// wnd.on().wm_create({
	///     let wnd = wnd.clone(); // to pass into the closure
	///     move |p: msg::wm::Create| -> AnyResult<i32> {
	///         println!("HWND: {}, client area: {}x{}",
	///             wnd.hwnd(),
	///             p.createstruct.cx,
	///             p.createstruct.cy,
	///         );
	///         Ok(0)
	///     }
	/// });
	/// ```
	fn wm_create<F>(&self, func: F)
		where F: Fn(wm::Create) -> AnyResult<i32> + 'static,
	{
		self.wm(co::WM::CREATE,
			move |p| Ok(Some(func(wm::Create::from_generic_wm(p))? as _)));
	}

	fn_wm_ctlcolor! { wm_ctl_color_btn, co::WM::CTLCOLORBTN, wm::CtlColorBtn,
		/// [`WM_CTLCOLORBTN`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorbtn)
		/// message.
	}

	fn_wm_ctlcolor! { wm_ctl_color_dlg, co::WM::CTLCOLORDLG, wm::CtlColorDlg,
		/// [`WM_CTLCOLORDLG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-ctlcolordlg)
		/// message.
	}

	fn_wm_ctlcolor! { wm_ctl_color_edit, co::WM::CTLCOLOREDIT, wm::CtlColorEdit,
		/// [`WM_CTLCOLOREDIT`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcoloredit)
		/// message.
	}

	fn_wm_ctlcolor! { wm_ctl_color_list_box, co::WM::CTLCOLORLISTBOX, wm::CtlColorListBox,
		/// [`WM_CTLCOLORLISTBOX`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorlistbox)
		/// message.
	}

	fn_wm_ctlcolor! { wm_ctl_color_scroll_bar, co::WM::CTLCOLORSCROLLBAR, wm::CtlColorScrollBar,
		/// [`WM_CTLCOLORSCROLLBAR`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorscrollbar)
		/// message.
	}

	fn_wm_ctlcolor! { wm_ctl_color_static, co::WM::CTLCOLORSTATIC, wm::CtlColorStatic,
		/// [`WM_CTLCOLORSTATIC`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorstatic)
		/// message.
	}

	fn_wm_withparm_noret! { wm_dead_char, co::WM::DEADCHAR, wm::DeadChar,
		/// [`WM_DEADCHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-deadchar)
		/// message.
	}

	fn_wm_withparm_boolret! { wm_delete_item, co::WM::DELETEITEM, wm::DeleteItem,
		/// [`WM_DELETEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-deleteitem)
		/// message.
	}

	fn_wm_noparm_noret! { wm_destroy, co::WM::DESTROY,
		/// [`WM_DESTROY`](crate::msg::wm::Destroy) message.
		///
		/// # Examples
		///
		/// ```rust,no_run
		/// use winsafe::prelude::*;
		/// use winsafe::{gui, AnyResult};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// wnd.on().wm_destroy(move || -> AnyResult<()> {
		///     println!("Window is gone, goodbye!");
		///     Ok(())
		/// });
		/// ```
	}

	fn_wm_withparm_noret! { wm_display_change, co::WM::DISPLAYCHANGE, wm::DisplayChange,
		/// [`WM_DISPLAYCHANGE`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-displaychange)
		/// message.
	}

	fn_wm_withparm_noret! { wm_drop_files, co::WM::DROPFILES, wm::DropFiles,
		/// [`WM_DROPFILES`](https://learn.microsoft.com/en-us/windows/win32/shell/wm-dropfiles)
		/// message.
		///
		/// # Examples
		///
		/// ```rust,no_run
		/// use winsafe::prelude::*;
		/// use winsafe::{gui, msg, AnyResult};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// wnd.on().wm_drop_files(move |p: msg::wm::DropFiles| -> AnyResult<()> {
		///     for dropped_file in p.hdrop.iter()? {
		///         let dropped_file = dropped_file?;
		///         println!("Dropped: {}", dropped_file);
		///     }
		///     Ok(())
		/// });
		/// ```
	}

	fn_wm_withparm_noret! { wm_enable, co::WM::ENABLE, wm::Enable,
		/// [`WM_ENABLE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-enable)
		/// message.
	}

	fn_wm_withparm_noret! { wm_end_session, co::WM::ENDSESSION, wm::EndSession,
		/// [`WM_ENDSESSION`](https://learn.microsoft.com/en-us/windows/win32/shutdown/wm-endsession)
		/// message.
	}

	fn_wm_withparm_noret! { wm_enter_idle, co::WM::ENTERIDLE, wm::EnterIdle,
		/// [`WM_ENTERIDLE`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-enteridle)
		/// message.
	}

	fn_wm_withparm_noret! { wm_enter_menu_loop, co::WM::ENTERMENULOOP, wm::EnterMenuLoop,
		/// [`WM_ENTERMENULOOP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-entermenuloop)
		/// message.
	}

	fn_wm_noparm_noret! { wm_enter_size_move, co::WM::ENTERSIZEMOVE,
		/// [`WM_ENTERSIZEMOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-entersizemove)
		/// message.
	}

	/// [`WM_ERASEBKGND`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-erasebkgnd)
	/// message.
	fn wm_erase_bkgnd<F>(&self, func: F)
		where F: Fn(wm::EraseBkgnd) -> AnyResult<i32> + 'static,
	{
		self.wm(co::WM::ERASEBKGND,
			move |p| Ok(Some(func(wm::EraseBkgnd::from_generic_wm(p))? as _)));
	}

	fn_wm_withparm_noret! { wm_exit_menu_loop, co::WM::EXITMENULOOP, wm::ExitMenuLoop,
		/// [`WM_EXITMENULOOP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-exitmenuloop)
		/// message.
	}

	fn_wm_noparm_noret! { wm_exit_size_move, co::WM::EXITSIZEMOVE,
		/// [`WM_EXITSIZEMOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-exitsizemove)
		/// message.
	}

	fn_wm_withparm_coret! { wm_get_dlg_code, co::WM::GETDLGCODE, wm::GetDlgCode, co::DLGC,
		/// [`WM_GETDLGCODE`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-getdlgcode)
		/// message.
	}

	/// [`WM_GETFONT`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-getfont)
	/// message.
	fn wm_get_font<F>(&self, func: F)
		where F: Fn() -> AnyResult<Option<HFONT>> + 'static,
	{
		self.wm(co::WM::GETFONT,
			move |_| Ok(Some(func()?.map(|h| h.0 as _).unwrap_or_default())));
	}

	/// [`WM_GETHMENU`](https://learn.microsoft.com/en-us/windows/win32/winmsg/mn-gethmenu)
	/// message. Originally has `MN` prefix.
	fn wm_get_hmenu<F>(&self, func: F)
		where F: Fn() -> AnyResult<Option<HMENU>> + 'static
	{
		self.wm(co::WM::MN_GETHMENU,
			move |_| Ok(Some(func()?.map(|h| h.0 as _).unwrap_or_default())));
	}

	fn_wm_withparm_noret! { wm_get_min_max_info, co::WM::GETMINMAXINFO, wm::GetMinMaxInfo,
		/// [`WM_GETMINMAXINFO`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-getminmaxinfo)
		/// message.
	}

	/// [`WM_GETTEXT`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-gettext)
	/// message.
	fn wm_get_text<F>(&self, func: F)
		where F: Fn(wm::GetText) -> AnyResult<u32> + 'static,
	{
		self.wm(co::WM::GETTEXT,
			move |p| Ok(Some(func(wm::GetText::from_generic_wm(p))? as _)));
	}

	/// [`WM_GETTEXTLENGTH`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-gettextlength)
	/// message.
	fn wm_get_text_length<F>(&self, func: F)
		where F: Fn() -> AnyResult<u32> + 'static,
	{
		self.wm(co::WM::GETTEXTLENGTH,
			move |_| Ok(Some(func()? as _)));
	}

	fn_wm_withparm_noret! { wm_get_title_bar_info_ex, co::WM::GETTITLEBARINFOEX, wm::GetTitleBarInfoEx,
		/// [`WM_GETTITLEBARINFOEX`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-gettitlebarinfoex)
		/// message.
	}

	fn_wm_withparm_noret! { wm_help, co::WM::HELP, wm::Help,
		/// [`WM_HELP`](https://learn.microsoft.com/en-us/windows/win32/shell/wm-help)
		/// message.
	}

	fn_wm_withparm_noret! { wm_h_scroll, co::WM::HSCROLL, wm::HScroll,
		/// [`WM_HSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-hscroll)
		/// message.
	}

	fn_wm_withparm_boolret! { wm_init_dialog, co::WM::INITDIALOG, wm::InitDialog,
		/// [`WM_INITDIALOG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
		/// message, sent only to dialog windows. Non-dialog windows receive
		/// [`WM_CREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create)
		/// instead.
		///
		/// # Examples
		///
		/// ```rust,no_run
		/// use winsafe::prelude::*;
		/// use winsafe::{gui, msg, AnyResult};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// let wnd = wnd.clone(); // to pass into the closure
		/// wnd.on().wm_init_dialog(move |p: msg::wm::InitDialog| -> AnyResult<bool> {
		///     println!("Focused HWND: {}", p.hwnd_focus);
		///     Ok(true)
		/// });
		/// ```
	}

	fn_wm_withparm_noret! { wm_init_menu_popup, co::WM::INITMENUPOPUP, wm::InitMenuPopup,
		/// [`WM_INITMENUPOPUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-initmenupopup)
		/// message.
	}

	fn_wm_withparm_noret! { wm_key_down, co::WM::KEYDOWN, wm::KeyDown,
		/// [`WM_KEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keydown)
		/// message.
	}

	fn_wm_withparm_noret! { wm_key_up, co::WM::KEYUP, wm::KeyUp,
		/// [`WM_KEYUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keyup)
		/// message.
	}

	fn_wm_withparm_noret! { wm_kill_focus, co::WM::KILLFOCUS, wm::KillFocus,
		/// [`WM_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-killfocus)
		/// message.
	}

	fn_wm_withparm_noret! { wm_l_button_dbl_clk, co::WM::LBUTTONDBLCLK, wm::LButtonDblClk,
		/// [`WM_LBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondblclk)
		/// message.
		///
		/// # Examples
		///
		/// ```rust,no_run
		/// use winsafe::prelude::*;
		/// use winsafe::{gui, msg, AnyResult};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// let wnd = wnd.clone(); // to pass into the closure
		/// wnd.on().wm_l_button_dbl_clk(move |p: msg::wm::LButtonDblClk| -> AnyResult<()> {
		///     println!("Point: {}x{}", p.coords.x, p.coords.y);
		///     Ok(())
		/// });
		/// ```
	}

	fn_wm_withparm_noret! { wm_l_button_down, co::WM::LBUTTONDOWN, wm::LButtonDown,
		/// [`WM_LBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown)
		/// message.
		///
		/// # Examples
		///
		/// ```rust,no_run
		/// use winsafe::prelude::*;
		/// use winsafe::{gui, msg, AnyResult};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// let wnd = wnd.clone(); // to pass into the closure
		/// wnd.on().wm_l_button_down(move |p: msg::wm::LButtonDown| -> AnyResult<()> {
		///     println!("Point: {}x{}", p.coords.x, p.coords.y);
		///     Ok(())
		/// });
		/// ```
	}

	fn_wm_withparm_noret! { wm_l_button_up, co::WM::LBUTTONUP, wm::LButtonUp,
		/// [`WM_LBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttonup)
		/// message.
	}

	fn_wm_withparm_noret! { wm_m_button_dbl_clk, co::WM::MBUTTONDBLCLK, wm::MButtonDblClk,
		/// [`WM_MBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondblclk)
		/// message.
	}

	fn_wm_withparm_noret! { wm_m_button_down, co::WM::MBUTTONDOWN, wm::MButtonDown,
		/// [`WM_MBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondown)
		/// message.
	}

	fn_wm_withparm_noret! { wm_m_button_up, co::WM::MBUTTONUP, wm::MButtonUp,
		/// [`WM_MBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttonup)
		/// message.
	}

	fn_wm_withparm_noret! { wm_menu_command, co::WM::MENUCOMMAND, wm::MenuCommand,
		/// [`WM_MENUCOMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menucommand)
		/// message.
	}

	fn_wm_withparm_coret! { wm_menu_drag, co::WM::MENUDRAG, wm::MenuDrag, co::MND,
		/// [`WM_MENUDRAG`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menudrag)
		/// message.
	}

	fn_wm_withparm_noret! { wm_menu_r_button_up, co::WM::MENURBUTTONUP, wm::MenuRButtonUp,
		/// [`WM_MENURBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menurbuttonup)
		/// message.
	}

	fn_wm_withparm_noret! { wm_mouse_hover, co::WM::MOUSEHOVER, wm::MouseHover,
		/// [`WM_MOUSEHOVER`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousehover)
		/// message.
	}

	fn_wm_noparm_noret! { wm_mouse_leave, co::WM::MOUSELEAVE,
		/// [`WM_MOUSELEAVE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mouseleave)
		/// message.
	}

	fn_wm_withparm_noret! { wm_mouse_move, co::WM::MOUSEMOVE, wm::MouseMove,
		/// [`WM_MOUSEMOVE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove)
		/// message.
	}

	fn_wm_withparm_noret! { wm_move, co::WM::MOVE, wm::Move,
		/// [`WM_MOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-move)
		/// message.
	}

	fn_wm_withparm_noret! { wm_moving, co::WM::MOVING, wm::Moving,
		/// [`WM_MOVING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-moving)
		/// message.
	}

	fn_wm_withparm_coret! { wm_nc_calc_size, co::WM::NCCALCSIZE, wm::NcCalcSize, co::WVR,
		/// [`WM_NCCALCSIZE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize)
		/// message.
	}

	fn_wm_withparm_boolret! { wm_nc_create, co::WM::NCCREATE, wm::NcCreate,
		/// [`WM_NCCREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccreate)
		/// message.
	}

	fn_wm_noparm_noret! { wm_nc_destroy, co::WM::NCDESTROY,
		/// [`WM_NCDESTROY`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-ncdestroy)
		/// message.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * non-dialog [`WindowMain`](crate::gui::WindowMain);
		/// * dialog [`WindowMain`](crate::gui::WindowMain).
	}

	fn_wm_withparm_coret! { wm_nc_hit_test, co::WM::NCHITTEST, wm::NcHitTest, co::HT,
		/// [`WM_NCHITTEST`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-nchittest)
		/// message.
	}

	fn_wm_withparm_noret! { wm_nc_paint, co::WM::NCPAINT, wm::NcPaint,
		/// [`WM_NCPAINT`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-ncpaint)
		/// message.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * non-dialog [`WindowControl`](crate::gui::WindowControl);
		/// * dialog [`WindowControl`](crate::gui::WindowControl).
	}

	fn_wm_withparm_noret! { wm_next_dlg_ctl, co::WM::NEXTDLGCTL, wm::NextDlgCtl,
		/// [`WM_NEXTDLGCTL`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-nextdlgctl)
		/// message.
	}

	fn_wm_noparm_noret! { wm_null, co::WM::NULL,
		/// [`WM_NULL`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-null)
		/// message.
	}

	fn_wm_noparm_noret! { wm_paint, co::WM::PAINT,
		/// [`WM_PAINT`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-paint)
		/// message.
		///
		/// # Examples
		///
		/// ```rust,no_run
		/// use winsafe::prelude::*;
		/// use winsafe::{gui, AnyResult};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// wnd.on().wm_paint({
		///     let wnd = wnd.clone(); // to pass into the closure
		///     move || -> AnyResult<()> {
		///         let hdc = wnd.hwnd().BeginPaint()?;
		///
		///         // hdc painting...
		///
		///         Ok(())
		///     }
		/// });
		/// ```
	}

	fn_wm_withparm_noret! { wm_parent_notify, co::WM::PARENTNOTIFY, wm::ParentNotify,
		/// [`WM_PARENTNOTIFY`](https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-parentnotify)
		/// message.
	}

	fn_wm_noparm_boolret! { wm_query_open, co::WM::QUERYOPEN,
		/// [`WM_QUERYOPEN`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-queryopen)
		/// message.
	}

	fn_wm_withparm_noret! { wm_r_button_dbl_clk, co::WM::RBUTTONDBLCLK, wm::RButtonDblClk,
		/// [`WM_RBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondblclk)
		/// message.
	}

	fn_wm_withparm_noret! { wm_r_button_down, co::WM::RBUTTONDOWN, wm::RButtonDown,
		/// [`WM_RBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondown)
		/// message.
	}

	fn_wm_withparm_noret! { wm_r_button_up, co::WM::RBUTTONUP, wm::RButtonUp,
		/// [`WM_RBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttonup)
	}

	fn_wm_withparm_boolret! { wm_set_cursor, co::WM::SETCURSOR, wm::SetCursor,
		/// [`WM_SETCURSOR`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-setcursor)
		/// message.
	}

	fn_wm_withparm_noret! { wm_set_focus, co::WM::SETFOCUS, wm::SetFocus,
		/// [`WM_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-setfocus)
		/// message.
		///
		/// # Default handling
		///
		/// If you handle this event, you'll overwrite the default handling in:
		///
		/// * non-dialog [`WindowMain`](crate::gui::WindowMain);
		/// * non-dialog [`WindowModal`](crate::gui::WindowModal).
	}

	fn_wm_withparm_noret! { wm_set_font, co::WM::SETFONT, wm::SetFont,
		/// [`WM_SETFONT`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-setfont)
		/// message.
	}

	/// [`WM_SETICON`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-seticon)
	/// message.
	fn wm_set_icon<F>(&self, func: F)
		where F: Fn(wm::SetIcon) -> AnyResult<Option<HICON>> + 'static,
	{
		self.wm(co::WM::SETICON, move |p|
			Ok(Some(
				func(wm::SetIcon::from_generic_wm(p))?
					.map(|h| h.0 as _).unwrap_or_default(),
			))
		);
	}

	fn_wm_withparm_noret! { wm_set_redraw, co::WM::SETREDRAW, wm::SetRedraw,
		/// [`WM_SETREDRAW`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-setredraw)
		/// message.
	}

	fn_wm_withparm_boolret! { wm_set_text, co::WM::SETTEXT, wm::SetText,
		/// [`WM_SETTEXT`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-settext)
		/// message.
	}

	fn_wm_withparm_noret! { wm_show_window, co::WM::SHOWWINDOW, wm::ShowWindow,
		/// [`WM_SHOWWINDOW`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow)
		/// message.
	}

	fn_wm_withparm_noret! { wm_size, co::WM::SIZE, wm::Size,
		/// [`WM_SIZE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-size)
		/// message.
		///
		/// # Examples
		///
		/// ```rust,no_run
		/// use winsafe::prelude::*;
		/// use winsafe::{gui, msg, AnyResult};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// wnd.on().wm_size({
		///     let wnd = wnd.clone(); // to pass into the closure
		///     move |p: msg::wm::Size| -> AnyResult<()> {
		///         println!("HWND: {}, client area: {}x{}",
		///             wnd.hwnd(),
		///             p.client_area.cx,
		///             p.client_area.cy,
		///         );
		///         Ok(())
		///     }
		/// });
		/// ```
	}

	fn_wm_withparm_noret! { wm_sizing, co::WM::SIZING, wm::Sizing,
		/// [`WM_SIZING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-sizing)
		/// message.
	}

	fn_wm_withparm_noret! { wm_style_changed, co::WM::STYLECHANGED, wm::StyleChanged,
		/// [`WM_STYLECHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanged)
		/// message.
	}

	fn_wm_withparm_noret! { wm_style_changing, co::WM::STYLECHANGING, wm::StyleChanging,
		/// [`WM_STYLECHANGING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanging)
		/// message.
	}

	fn_wm_noparm_noret! { wm_sync_paint, co::WM::SYNCPAINT,
		/// [`WM_SYNCPAINT`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-syncpaint)
		/// message.
	}

	fn_wm_withparm_noret! { wm_sys_char, co::WM::SYSCHAR, wm::SysChar,
		/// [`WM_SYSCHAR`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syschar)
		/// message.
	}

	fn_wm_withparm_noret! { wm_sys_command, co::WM::SYSCOMMAND, wm::SysCommand,
		/// [`WM_SYSCOMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syscommand)
		/// message.
	}

	fn_wm_withparm_noret! { wm_sys_dead_char, co::WM::SYSDEADCHAR, wm::SysDeadChar,
		/// [`WM_SYSDEADCHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-sysdeadchar)
		/// message.
	}

	fn_wm_withparm_noret! { wm_sys_key_down, co::WM::SYSKEYDOWN, wm::SysKeyDown,
		/// [`WM_SYSKEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-syskeydown)
		/// message.
	}

	fn_wm_withparm_noret! { wm_sys_key_up, co::WM::SYSKEYUP, wm::SysKeyUp,
		/// [`WM_SYSKEYUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-syskeyup)
		/// message.
	}

	fn_wm_noparm_noret! { wm_theme_changed, co::WM::THEMECHANGED,
		/// [`WM_THEMECHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-themechanged)
		/// message.
	}

	fn_wm_withparm_noret! { wm_uninit_menu_popup, co::WM::UNINITMENUPOPUP, wm::UninitMenuPopup,
		/// [`WM_UNINITMENUPOPUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-uninitmenupopup)
		/// message.
	}

	fn_wm_noparm_boolret! { wm_undo, co::WM::UNDO,
		/// [`WM_UNDO`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-undo)
		/// message.
	}

	fn_wm_withparm_noret! { wm_v_scroll, co::WM::VSCROLL, wm::VScroll,
		/// [`WM_VSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-vscroll)
		/// message.
	}

	fn_wm_withparm_noret! { wm_window_pos_changed, co::WM::WINDOWPOSCHANGED, wm::WindowPosChanged,
		/// [`WM_WINDOWPOSCHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanged)
		/// message.
	}

	fn_wm_withparm_noret! { wm_window_pos_changing, co::WM::WINDOWPOSCHANGING, wm::WindowPosChanging,
		/// [`WM_WINDOWPOSCHANGING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanging)
		/// message.
	}

	fn_wm_withparm_noret! { wm_x_button_dbl_clk, co::WM::XBUTTONDBLCLK, wm::XButtonDblClk,
		/// [`WM_XBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondblclk)
		/// message.
	}

	fn_wm_withparm_noret! { wm_x_button_down, co::WM::XBUTTONDOWN, wm::XButtonDown,
		/// [`WM_XBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondown)
		/// message.
	}

	fn_wm_withparm_noret! { wm_x_button_up, co::WM::XBUTTONUP, wm::XButtonUp,
		/// [`WM_XBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttonup)
		/// message.
	}
}
