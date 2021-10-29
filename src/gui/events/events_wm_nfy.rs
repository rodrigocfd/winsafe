use std::rc::Rc;

use crate::aliases::ErrResult;
use crate::co;
use crate::gui::events::func_store::FuncStore;
use crate::gui::events::events_wm::{
	EventsView,
	ProcessResult,
	sealed_events_wm::SealedEventsWm,
	WindowEvents,
};
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::msg::{MsgSendRecv, wm, WndMsg};

/// Exposes window
/// [messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues),
/// plus native control notifications.
///
/// You cannot directly instantiate this object, it is created internally by the
/// window.
pub struct WindowEventsAll {
	events_wm: WindowEvents,
	tmrs: FuncStore< // WM_TIMER messages
		u32,
		Box<dyn Fn() -> ErrResult<()>>, // return value is never meaningful
	>,
	cmds: FuncStore< // WM_COMMAND notifications
		(co::CMD, u16), // notif code, control ID
		Box<dyn Fn() -> ErrResult<()>>, // return value is never meaningful
	>,
	nfys: FuncStore< // WM_NOTIFY notifications
		(u16, co::NM), // idFrom, code
		Box<dyn Fn(wm::Notify) -> ErrResult<Option<isize>>>, // return value may be meaningful
	>,
}

impl WindowEventsAll {
	pub(in crate::gui) fn new() -> Self {
		Self {
			events_wm: WindowEvents::new(),
			tmrs: FuncStore::new(),
			cmds: FuncStore::new(),
			nfys: FuncStore::new(),
		}
	}

	/// Searches for the last added user function for the given message, and
	/// runs if it exists, returning the result.
	pub(in crate::gui) fn process_one_message(&self,
		wm_any: WndMsg) -> ErrResult<ProcessResult>
	{
		Ok(match wm_any.msg_id {
			co::WM::NOTIFY => {
				let wm_nfy = wm::Notify::from_generic_wm(wm_any);
				let key = (wm_nfy.nmhdr.idFrom(), wm_nfy.nmhdr.code);
				match self.nfys.find(key) {
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
				match self.cmds.find(key) {
					Some(func) => { // we have a stored function to handle this WM_COMMAND notification
						func()?; // execute user function
						ProcessResult::HandledWithoutRet
					},
					None => ProcessResult::NotHandled, // no stored WM_COMMAND notification
				}
			},
			co::WM::TIMER => {
				let wm_tmr = wm::Timer::from_generic_wm(wm_any);
				match self.tmrs.find(wm_tmr.timer_id) {
					Some(func) => { // we have a stored function to handle this WM_TIMER message
						func()?; // execute user function
						ProcessResult::HandledWithoutRet
					},
					None => ProcessResult::NotHandled, // no stored WM_TIMER message
				}
			}
			_ => self.events_wm.process_one_message(wm_any)?,
		})
	}

	/// Searches for all user functions for the given message, and runs all of
	/// them, discarding the results.
	pub(in crate::gui) fn process_all_messages(&self, wm_any: WndMsg) -> ErrResult<()> {
		Ok(match wm_any.msg_id {
			co::WM::NOTIFY => {
				let wm_nfy = wm::Notify::from_generic_wm(wm_any);
				let key = (wm_nfy.nmhdr.idFrom(), wm_nfy.nmhdr.code);
				for func in self.nfys.find_all(key) {
					func(wm_nfy)?; // execute stored function
				}
			},
			co::WM::COMMAND => {
				let wm_cmd = wm::Command::from_generic_wm(wm_any);
				let key = wm_cmd.event.code_id();
				for func in self.cmds.find_all(key) {
					func()?; // execute stored function
				}
			},
			co::WM::TIMER => {
				let wm_tmr = wm::Timer::from_generic_wm(wm_any);
				for func in self.tmrs.find_all(wm_tmr.timer_id) {
					func()?; // execute stored function
				}
			},
			_ => self.events_wm.process_all_messages(wm_any)?,
		})
	}

	/// [`WM_TIMER`](crate::msg::wm::Timer) message, narrowed to a specific
	/// timer ID.
	///
	/// Posted to the installing thread's message queue when a timer expires.
	pub fn wm_timer<F>(&self, timer_id: u32, func: F)
		where F: Fn() -> ErrResult<()> + 'static,
	{
		self.tmrs.insert(timer_id, Box::new(func));
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
		where F: Fn() -> ErrResult<()> + 'static,
	{
		self.cmds.insert((code, ctrl_id), Box::new(func));
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
	/// use winsafe::prelude::*;
	/// use winsafe::{co, gui, msg, ErrResult};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	///
	/// wnd.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), {
	///     let wnd = wnd.clone(); // pass into the closure
	///     move || -> ErrResult<()> {
	///         wnd.hwnd().SendMessage(msg::wm::Close {});
	///         Ok(())
	///     }
	/// });
	/// ```
	pub fn wm_command_accel_menu<F>(&self, ctrl_id: u16, func: F)
		where F: Fn() -> ErrResult<()> + 'static,
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
}

//------------------------------------------------------------------------------

impl SealedEventsWm for WindowEventsAll {
	fn add_msg<F>(&self, ident: co::WM, func: F)
		where F: Fn(WndMsg) -> ErrResult<Option<isize>> + 'static,
	{
		self.events_wm.add_msg(ident, Box::new(func));
	}
}

impl EventsView for WindowEventsAll {}

impl sealed_events_wm_nfy::SealedEventsWmNfy for WindowEventsAll {
	fn add_nfy<F>(&self, id_from: u16, code: co::NM, func: F)
		where F: Fn(wm::Notify) -> ErrResult<Option<isize>> + 'static,
	{
		self.nfys.insert((id_from, code), Box::new(func));
	}
}

impl EventsViewAll for WindowEventsAll {}

//------------------------------------------------------------------------------

pub(in crate::gui) mod sealed_events_wm_nfy {
	use super::{co, ErrResult, wm};

	pub trait SealedEventsWmNfy {
		/// Raw add notification.
		fn add_nfy<F>(&self, id_from: u16, code: co::NM, func: F)
			where F: Fn(wm::Notify) -> ErrResult<Option<isize>> + 'static;
	}
}

/// Exposes the methods of
/// [`WindowEventsAll`](crate::gui::events::WindowEventsAll).
pub trait EventsViewAll: sealed_events_wm_nfy::SealedEventsWmNfy + EventsView {
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
	fn wm_notify<F>(&self, id_from: i32, code: co::NM, func: F)
		where F: Fn(wm::Notify) -> ErrResult<isize> + 'static,
	{
		self.add_nfy(id_from as _, code, move |p| Ok(Some(func(p)?))); // return value is meaningful
	}
}
