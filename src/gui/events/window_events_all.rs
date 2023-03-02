use std::cell::UnsafeCell;
use std::rc::Rc;

use crate::co;
use crate::gui::events::{ProcessResult, WindowEvents};
use crate::gui::events::func_store::FuncStore;
use crate::kernel::decl::AnyResult;
use crate::msg::{wm, WndMsg};
use crate::prelude::{GuiEvents, MsgSendRecv};

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
			u32,
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
	pub(in crate::gui) fn clear(&self) {
		unsafe {
			{ &mut *self.tmrs.get() }.clear();
			{ &mut *self.cmds.get() }.clear();
			{ &mut *self.nfys.get() }.clear();
		}
		self.window_events.clear();
	}

	/// Searches for the last added user function for the given message, and
	/// runs if it exists, returning the result.
	pub(in crate::gui) fn process_one_message(&self,
		wm_any: WndMsg) -> AnyResult<ProcessResult>
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
	pub(in crate::gui) fn process_all_messages(&self,
		wm_any: WndMsg) -> AnyResult<()>
	{
		Ok(match wm_any.msg_id {
			co::WM::NOTIFY => {
				let wm_nfy = wm::Notify::from_generic_wm(wm_any);
				let key = (wm_nfy.nmhdr.idFrom(), wm_nfy.nmhdr.code);
				let nfys = unsafe { &mut *self.nfys.get() };
				for func in nfys.find_all(key) {
					func(wm_nfy)?; // execute stored function
				}
			},
			co::WM::COMMAND => {
				let wm_cmd = wm::Command::from_generic_wm(wm_any);
				let key = wm_cmd.event.code_id();
				let cmds = unsafe { &mut *self.cmds.get() };
				for func in cmds.find_all(key) {
					func()?; // execute stored function
				}
			},
			co::WM::TIMER => {
				let wm_tmr = wm::Timer::from_generic_wm(wm_any);
				let tmrs = unsafe { &mut *self.tmrs.get() };
				for func in tmrs.find_all(wm_tmr.timer_id) {
					func()?; // execute stored function
				}
			},
			_ => self.window_events.process_all_messages(wm_any)?,
		})
	}
}

impl GuiEvents for WindowEventsAll {
	fn wm<F>(&self, ident: co::WM, func: F)
		where F: Fn(WndMsg) -> AnyResult<Option<isize>> + 'static,
	{
		self.window_events.wm(ident, func);
	}
}

impl GuiEventsAll for WindowEventsAll {
	fn wm_timer<F>(&self, timer_id: u32, func: F)
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

//------------------------------------------------------------------------------

/// Exposes methods to handle the basic window messages, plus timer and native
/// control notifications.
pub trait GuiEventsAll: GuiEvents {
	/// [`WM_TIMER`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-timer)
	/// message, narrowed to a specific timer ID.
	fn wm_timer<F>(&self, timer_id: u32, func: F)
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
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, gui, msg, AnyResult};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// const ID_BTN: u16 = 1000;
	///
	/// wnd.on().wm_command(co::BN::CLICKED, ID_BTN,
	///     move || -> AnyResult<()> {
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
