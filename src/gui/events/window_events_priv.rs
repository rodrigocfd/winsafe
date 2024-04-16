use std::cell::UnsafeCell;

use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::msg::*;
use crate::prelude::*;

/// Exposes window messages for before/after-user events, used internally.
pub(in crate::gui) struct WindowEventsPriv {
	is_dialog: bool,
	msgs: UnsafeCell< // ordinary WM messages
		FuncStore<
			co::WM,
			Box<dyn Fn(&HWND, WndMsg) -> AnyResult<()>>, // return value is discarded
		>,
	>,
	nfys: UnsafeCell<
		FuncStore< // WM_NOTIFY notifications
			(u16, co::NM), // idFrom, code
			Box<dyn Fn(&HWND, wm::Notify) -> AnyResult<()>>, // return value is discarded
		>,
	>,
}

impl WindowEventsPriv {
	pub(in crate::gui) fn new(is_dialog: bool) -> Self {
		Self {
			is_dialog,
			msgs: UnsafeCell::new(FuncStore::new()),
			nfys: UnsafeCell::new(FuncStore::new()),
		}
	}

	/// Removes all stored events.
	pub(in crate::gui) fn clear_events(&self) {
		unsafe { &mut *self.msgs.get() }.clear();
	}

	/// Searches for all user functions for the given message, and runs all of
	/// them, discarding the results.
	///
	/// Returns `true` if at least one message was processed.
	pub(in crate::gui) fn process_all_messages(&self,
		hwnd: &HWND,
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
					func(hwnd, wm::Notify::from_generic_wm(wm_any))?; // execute stored function
				}
			},
			_ => {
				let msgs = unsafe { &mut *self.msgs.get() };
				for func in msgs.find_all(wm_any.msg_id) {
					at_least_one = true;
					func(hwnd, wm_any)?; // execute each stored function
				}
			},
		}

		Ok(at_least_one)
	}

	/// Ordinary message handling.
	pub(in crate::gui) fn wm<F>(&self, ident: co::WM, func: F)
		where F: Fn(&HWND, WndMsg) -> AnyResult<()> + 'static,
	{
		unsafe { &mut *self.msgs.get() }.push(ident, Box::new(func));
	}

	/// If a dialog window, will handle `co::WM::INITDIALOG`, otherwise will
	/// handle `co::WM::CREATE`.
	pub(in crate::gui) fn wm_create_or_initdialog<F>(&self, func: F)
		where F: Fn(&HWND, WndMsg) -> AnyResult<()> + 'static,
	{
		unsafe { &mut *self.msgs.get() }.push(
			if self.is_dialog { co::WM::INITDIALOG } else { co::WM::CREATE },
			Box::new(func),
		);
	}

	/// General `WM_NOTIFY` handling.
	pub(in crate::gui) fn wm_notify<F>(&self,
		id_from: u16,
		code: impl Into<co::NM>,
		func: F,
	)
		where F: Fn(&HWND, wm::Notify) -> AnyResult<()> + 'static,
	{
		let code: co::NM = code.into();
		unsafe { &mut *self.nfys.get() }.push((id_from, code), Box::new(func));
	}
}
