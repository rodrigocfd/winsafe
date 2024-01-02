use std::cell::UnsafeCell;
use std::rc::Rc;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*};
use crate::msg::*;
use crate::prelude::*;

/// Exposes window
/// [messages](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues),
/// plus timer and native control notifications.
///
/// You cannot directly instantiate this object, it is created internally by the
/// window.
pub struct WindowEventsAll {
	window_events: WindowEvents,
	tmrs: UnsafeCell<
		FuncStore< // WM_TIMER messages
			usize,
			Box<dyn Fn() -> AnyResult<()>>, // return value is never meaningful
		>,
	>,
	cmds: UnsafeCell<
		FuncStore< // WM_COMMAND notifications
			(co::CMD, u16), // notif code, control ID
			Box<dyn Fn() -> AnyResult<()>>, // return value is never meaningful
		>,
	>,
	nfys: UnsafeCell<
		FuncStore< // WM_NOTIFY notifications
			(u16, co::NM), // idFrom, code
			Box<dyn Fn(wm::Notify) -> AnyResult<Option<isize>>>, // return value may be meaningful
		>,
	>,
}

impl GuiEvents for WindowEventsAll {
	fn wm<F>(&self, ident: co::WM, func: F)
		where F: Fn(WndMsg) -> AnyResult<Option<isize>> + 'static,
	{
		self.window_events.wm(ident, func);
	}
}

impl GuiEventsAll for WindowEventsAll {
	fn wm_timer<F>(&self, timer_id: usize, func: F)
		where F: Fn() -> AnyResult<()> + 'static,
	{
		unsafe { &mut *self.tmrs.get() }.push(timer_id, Box::new(func));
	}

	fn wm_command<F>(&self, code: impl Into<co::CMD>, ctrl_id: u16, func: F)
		where F: Fn() -> AnyResult<()> + 'static,
	{
		let code: co::CMD = code.into();
		unsafe { &mut *self.cmds.get() }.push((code, ctrl_id), Box::new(func));
	}

	fn wm_notify<F>(&self, id_from: u16, code: impl Into<co::NM>, func: F)
		where F: Fn(wm::Notify) -> AnyResult<Option<isize>> + 'static,
	{
		let code: co::NM = code.into();
		unsafe { &mut *self.nfys.get() }.push((id_from, code), Box::new(func));
	}
}

impl WindowEventsAll {
	pub(in crate::gui) fn new() -> Self {
		Self {
			window_events: WindowEvents::new(),
			tmrs: UnsafeCell::new(FuncStore::new()),
			cmds: UnsafeCell::new(FuncStore::new()),
			nfys: UnsafeCell::new(FuncStore::new()),
		}
	}

	/// Removes all stored events.
	pub(in crate::gui) fn clear_events(&self) {
		unsafe {
			{ &mut *self.tmrs.get() }.clear();
			{ &mut *self.cmds.get() }.clear();
			{ &mut *self.nfys.get() }.clear();
		}
		self.window_events.clear_events();
	}

	/// Searches for the last added user function for the given message, and
	/// runs if it exists, returning the result.
	pub(in crate::gui) fn process_one_message(&self,
		wm_any: WndMsg,
	) -> AnyResult<ProcessResult>
	{
		Ok(match wm_any.msg_id {
			co::WM::NOTIFY => {
				let wm_nfy = wm::Notify::from_generic_wm(wm_any);
				let key = (wm_nfy.nmhdr.idFrom(), wm_nfy.nmhdr.code);
				let nfys = unsafe { &mut *self.nfys.get() };
				match nfys.find(key) {
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
				let cmds = unsafe { &mut *self.cmds.get() };
				match cmds.find(key) {
					Some(func) => { // we have a stored function to handle this WM_COMMAND notification
						func()?; // execute user function
						ProcessResult::HandledWithoutRet
					},
					None => ProcessResult::NotHandled, // no stored WM_COMMAND notification
				}
			},
			co::WM::TIMER => {
				let wm_tmr = wm::Timer::from_generic_wm(wm_any);
				let tmrs = unsafe { &mut *self.tmrs.get() };
				match tmrs.find(wm_tmr.timer_id) {
					Some(func) => { // we have a stored function to handle this WM_TIMER message
						func()?; // execute user function
						ProcessResult::HandledWithoutRet
					},
					None => ProcessResult::NotHandled, // no stored WM_TIMER message
				}
			}
			_ => self.window_events.process_one_message(wm_any)?,
		})
	}

	/// Searches for all user functions for the given message, and runs all of
	/// them, discarding the results.
	///
	/// Returns `true` if at least one message was processed.
	pub(in crate::gui) fn process_all_messages(&self,
		wm_any: WndMsg,
	) -> AnyResult<bool>
	{
		let mut at_least_one = false;

		match wm_any.msg_id {
			co::WM::NOTIFY => {
				let wm_nfy = wm::Notify::from_generic_wm(wm_any);
				let key = (wm_nfy.nmhdr.idFrom(), wm_nfy.nmhdr.code);
				let nfys = unsafe { &mut *self.nfys.get() };
				for func in nfys.find_all(key) {
					at_least_one = true;
					func(wm::Notify::from_generic_wm(wm_any))?; // execute stored function
				}
			},
			co::WM::COMMAND => {
				let wm_cmd = wm::Command::from_generic_wm(wm_any);
				let key = wm_cmd.event.code_id();
				let cmds = unsafe { &mut *self.cmds.get() };
				for func in cmds.find_all(key) {
					at_least_one = true;
					func()?; // execute stored function
				}
			},
			co::WM::TIMER => {
				let wm_tmr = wm::Timer::from_generic_wm(wm_any);
				let tmrs = unsafe { &mut *self.tmrs.get() };
				for func in tmrs.find_all(wm_tmr.timer_id) {
					at_least_one = true;
					func()?; // execute stored function
				}
			},
			_ => {
				at_least_one = self.window_events.process_all_messages(wm_any)?;
			},
		}

		Ok(at_least_one)
	}
}

//------------------------------------------------------------------------------

/// Exposes methods to handle the basic window messages, plus timer and native
/// control notifications.
pub trait GuiEventsAll: GuiEvents {
	/// [`WM_CREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create)
	/// message, sent only to non-dialog windows. Dialog windows receive
	/// [`WM_INITDIALOG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
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
	fn wm_create<F>(&self, func: F)
		where F: Fn(wm::Create) -> AnyResult<i32> + 'static,
	{
		self.wm(co::WM::CREATE,
			move |p| Ok(Some(func(wm::Create::from_generic_wm(p))? as _)));
	}

	fn_wm_withparm_boolret! { wm_init_dialog, co::WM::INITDIALOG, wm::InitDialog;
		/// [`WM_INITDIALOG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
		/// message, sent only to dialog windows. Non-dialog windows receive
		/// [`WM_CREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create)
		/// instead.
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
	}

	/// [`WM_TIMER`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-timer)
	/// message, narrowed to a specific timer ID.
	fn wm_timer<F>(&self, timer_id: usize, func: F)
		where F: Fn() -> AnyResult<()> + 'static;

	/// [`WM_COMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-command)
	/// message, for specific code and control ID.
	///
	/// A command notification must be narrowed by the
	/// [command code](crate::co::CMD) and the control ID, so the closure will
	/// be fired for that specific control at that specific event.
	///
	/// **Note:** Instead of using this event, you should always prefer the
	/// specific command notifications, which will give you the correct message
	/// parameters. This generic method should be used only when you have a
	/// custom, non-standard window notification.
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// const ID_BTN: u16 = 1000;
	///
	/// wnd.on().wm_command(co::BN::CLICKED, ID_BTN,
	///     move || -> w::AnyResult<()> {
	///         println!("Button clicked!");
	///         Ok(())
	///     },
	/// );
	/// ```
	fn wm_command<F>(&self, code: impl Into<co::CMD>, ctrl_id: u16, func: F)
		where F: Fn() -> AnyResult<()> + 'static;

	/// [`WM_COMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-command)
	/// message, handling both `CMD::Accelerator` and `CMD::Menu`, for a
	/// specific command ID.
	///
	/// Ideal to be used with menu commands whose IDs are shared with
	/// accelerators.
	fn wm_command_accel_menu<F>(&self, ctrl_id: u16, func: F)
		where F: Fn() -> AnyResult<()> + 'static,
	{
		let shared_func = Rc::new(func);

		self.wm_command(co::CMD::Menu, ctrl_id, {
			let shared_func = shared_func.clone();
			move || shared_func()
		});

		self.wm_command(co::CMD::Accelerator, ctrl_id, {
			let shared_func = shared_func.clone();
			move || shared_func()
		});
	}

	/// [`WM_NOTIFY`](crate::msg::wm::Notify) message, for specific ID and
	/// notification code.
	///
	/// **Note:** Instead of using this event, you should always prefer the
	/// specific notifications, which will give you the correct notification
	/// struct. This generic method should be used only when you have a custom,
	/// non-standard window notification.
	fn wm_notify<F>(&self, id_from: u16, code: impl Into<co::NM>, func: F)
		where F: Fn(wm::Notify) -> AnyResult<Option<isize>> + 'static;
}
