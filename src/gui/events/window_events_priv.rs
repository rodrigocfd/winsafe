use std::cell::UnsafeCell;

use crate::co;
use crate::decl::*;
use crate::gui::{*, privs::*};
use crate::msg::*;
use crate::prelude::*;

/// Exposes window messages for internal before/after user events.
pub(in crate::gui) struct WindowEventsPriv {
	is_dialog: bool,
	msgs: UnsafeCell<
		FuncStore< // ordinary WM messages
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
			(u16, co::NM), // idFrom, code
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

impl WindowEventsPriv {
	#[must_use]
	pub(in crate::gui) const fn new(is_dialog: bool) -> Self {
		Self {
			is_dialog,
			msgs: UnsafeCell::new(FuncStore::new()),
			cmds: UnsafeCell::new(FuncStore::new()),
			nfys: UnsafeCell::new(FuncStore::new()),
			tmrs: UnsafeCell::new(FuncStore::new()),
		}
	}

	pub(in crate::gui) fn is_empty(&self) -> bool {
		unsafe {
			{ &*self.msgs.get() }.is_empty()
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

		if wm_any.msg_id == co::WM::COMMAND {
			let wm_cmd = wm::Command::from_generic_wm(wm_any);
			let key_cmd = wm_cmd.event.id_code();
			let cmds = unsafe { &*self.cmds.get() };
			for func in cmds.filter(key_cmd) {
				match func()? {
					WmRet::HandledWithRet(_)
						| WmRet::HandledOk => { at_least_one = true; }
					_ => {},
				}
			}
		} else if wm_any.msg_id == co::WM::NOTIFY {
			let wm_nfy = wm::Notify::from_generic_wm(wm_any);
			let key_nfy = (wm_nfy.nmhdr.idFrom(), wm_nfy.nmhdr.code);
			let nfys = unsafe { &*self.nfys.get() };
			for func in nfys.filter(key_nfy) {
				match func(wm::Notify::from_generic_wm(wm_any))? { // wm::Notify cannot be Copy
					WmRet::HandledWithRet(_)
						| WmRet::HandledOk => { at_least_one = true; }
					_ => {},
				}
			}
		} else if wm_any.msg_id == co::WM::TIMER {
			let wm_tmr = wm::Timer::from_generic_wm(wm_any);
			let tmrs = unsafe { &*self.tmrs.get() };
			for func in tmrs.filter(wm_tmr.timer_id) {
				func()?;
				at_least_one = true;
			}
		}

		let msgs = unsafe { &*self.msgs.get() };
		for func in msgs.filter(wm_any.msg_id) {
			match func(hwnd, wm_any)? {
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
		if wm_any.msg_id == co::WM::COMMAND {
			let wm_cmd = wm::Command::from_generic_wm(wm_any);
			let key_cmd = wm_cmd.event.id_code();
			let cmds = unsafe { &*self.cmds.get() };
			for func in cmds.filter_rev(key_cmd) {
				match func()? {
					WmRet::NotHandled => {},
					r => return Ok(r), // handled: stop here
				}
			}
		} else if wm_any.msg_id == co::WM::NOTIFY {
			let wm_nfy = wm::Notify::from_generic_wm(wm_any);
			let key_nfy = (wm_nfy.nmhdr.idFrom(), wm_nfy.nmhdr.code);
			let nfys = unsafe { &*self.nfys.get() };
			for func in nfys.filter_rev(key_nfy) {
				match func(wm::Notify::from_generic_wm(wm_any))? { // wm::Notify cannot be Copy
					WmRet::NotHandled => {},
					r => return Ok(r), // handled: stop here
				}
			}
		} else if wm_any.msg_id == co::WM::TIMER {
			let wm_tmr = wm::Timer::from_generic_wm(wm_any);
			let tmrs = unsafe { &*self.tmrs.get() };
			if let Some(func) = tmrs.filter_rev(wm_tmr.timer_id).next() { // just execute the last, if any
				func()?;
				return Ok(WmRet::HandledOk);
			}
		}

		let msgs = unsafe { &*self.msgs.get() };
		for func in msgs.filter_rev(wm_any.msg_id) {
			match func(hwnd, wm_any)? {
				WmRet::NotHandled => {},
				r => return Ok(r), // handled: stop here
			}
		}
		Ok(WmRet::NotHandled)
	}

	pub(in crate::gui) fn wm<F>(&self, ident: co::WM, func: F)
		where F: Fn(&HWND, WndMsg) -> AnyResult<WmRet> + 'static,
	{
		unsafe { &mut *self.msgs.get() }.push(ident, Box::new(func));
	}

	/// If a dialog window, will handle `co::WM::INITDIALOG`, otherwise will
	/// handle `co::WM::CREATE`.
	pub(in crate::gui) fn wm_create_or_initdialog<F>(&self, func: F)
		where F: Fn(&HWND, WndMsg) -> AnyResult<WmRet> + 'static,
	{
		self.wm(
			if self.is_dialog { co::WM::INITDIALOG } else { co::WM::CREATE },
			Box::new(func),
		);
	}

	pub(in crate::gui) fn wm_command<F>(&self,
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

	pub(in crate::gui) fn wm_notify<F>(&self,
		id_from: impl Into<u16>,
		code: impl Into<co::NM>,
		func: F,
	)
		where F: Fn(wm::Notify) -> AnyResult<WmRet> + 'static,
	{
		let code: co::NM = code.into();
		unsafe { &mut *self.nfys.get() }.push(
			(id_from.into(), code),
			Box::new(func),
		);
	}

	pub(in crate::gui) fn wm_timer<F>(&self, timer_id: usize, func: F)
		where F: Fn() -> AnyResult<()> + 'static,
	{
		unsafe { &mut *self.tmrs.get() }.push(timer_id, Box::new(func));
	}
}
