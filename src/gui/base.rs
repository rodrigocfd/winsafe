use std::ptr::NonNull;

use crate::aliases::{ErrResult, WinResult};
use crate::co;
use crate::funcs::{DispatchMessage, GetMessage, TranslateMessage};
use crate::gui::events::{
	EventsView,
	sealed_events_wm::SealedEventsWm,
	ProcessResult,
	WindowEventsAll,
};
use crate::gui::privs::{post_quit_error, QUIT_ERROR};
use crate::gui::resizer::{Horz, Resizer, Vert};
use crate::handles::{Handle, HACCEL, HWND};
use crate::msg::WndMsg;
use crate::structs::MSG;

/// Base to `RawBase` and `DlgBase`.
///
/// While the parent module is private, the struct is public so it can be used
/// in sealed traits.
pub struct Base {
	hwnd: HWND,
	is_dialog: bool,
	parent_ptr: Option<NonNull<Self>>,
	user_events: WindowEventsAll, // ordinary window events, inserted by user: only last added is executed (overwrite previous)
	privileged_events: WindowEventsAll, // inserted internally to automate tasks: all will be executed
	resizer: Resizer,
}

impl Base {
	const WM_UI_THREAD: co::WM = co::WM(co::WM::APP.0 + 0x3fff);

	pub(in crate::gui) fn new(
		is_dialog: bool,
		parent_base: Option<&Base>) -> Self
	{
		let new_self = Self {
			hwnd: HWND::NULL,
			is_dialog,
			parent_ptr: parent_base.map(|parent_base| NonNull::from(parent_base)),
			user_events: WindowEventsAll::new(),
			privileged_events: WindowEventsAll::new(),
			resizer: Resizer::new(),
		};
		new_self.default_message_handlers();
		new_self
	}

	pub(in crate::gui) const fn hwnd(&self) -> HWND {
		self.hwnd
	}

	pub(in crate::gui) fn hwnd_mut(&mut self) -> &mut HWND {
		&mut self.hwnd
	}

	pub(in crate::gui) const fn is_dialog(&self) -> bool {
		self.is_dialog
	}

	pub(in crate::gui) const fn wmcreate_or_wminitdialog(&self) -> co::WM {
		if self.is_dialog { co::WM::INITDIALOG } else { co::WM::CREATE }
	}

	pub(in crate::gui) fn parent_base(&self) -> Option<&Self> {
		self.parent_ptr.as_ref().map(|ptr| unsafe { ptr.as_ref() })
	}

	pub(in crate::gui) fn on(&self) -> &WindowEventsAll {
		if !self.hwnd.is_null() {
			panic!("Cannot add event after window creation.");
		}
		&self.user_events
	}

	pub(in crate::gui) fn process_user_message(&self,
		wm_any: WndMsg) -> ErrResult<ProcessResult>
	{
		self.user_events.process_one_message(wm_any)
	}

	pub(in crate::gui) fn privileged_on(&self) -> &WindowEventsAll {
		if !self.hwnd.is_null() {
			panic!("Cannot add privileged event after window creation.");
		}
		&self.privileged_events
	}

	pub(in crate::gui) fn process_privileged_messages(&self,
		wm_any: WndMsg) -> ErrResult<()>
	{
		self.privileged_events.process_all_messages(wm_any)
	}

	pub(in crate::gui) fn add_to_resizer(&self,
		hchild: HWND, horz: Horz, vert: Vert) -> WinResult<()>
	{
		self.resizer.add(self.hwnd, hchild, horz, vert)
	}

	pub(in crate::gui) fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()>,
	{
		// This method is analog to SendMessage (synchronous), but intended to
		// be called from another thread, so a callback function can, tunelled
		// by wndproc, run in the original thread of the window, thus allowing
		// GUI updates. With this, the user doesn't have to deal with a custom
		// WM_ message.

		// https://users.rust-lang.org/t/sending-a-boxed-trait-over-ffi/21708/2
		let pack: Box<Box<dyn FnOnce() -> ErrResult<()>>> = Box::new(Box::new(func));
		let ptr_pack = Box::into_raw(pack);
		self.hwnd.SendMessage(WndMsg {
			msg_id: Self::WM_UI_THREAD,
			wparam: Self::WM_UI_THREAD.0 as _,
			lparam: ptr_pack as _,
		});
	}

	fn default_message_handlers(&self) {
		self.privileged_events.wm_size({
			let resizer = self.resizer.clone();
			move |p| { resizer.resize(&p)?; Ok(()) }
		});

		self.privileged_events.add_msg(Self::WM_UI_THREAD, |p| {
			if co::WM(p.wparam as _) == Self::WM_UI_THREAD { // additional safety check
				let ptr_pack = p.lparam as *mut Box<dyn FnOnce() -> ErrResult<()>>;
				let pack: Box<Box<dyn FnOnce() -> ErrResult<()>>> = unsafe { Box::from_raw(ptr_pack) };
				pack().unwrap_or_else(|err| post_quit_error(err));
			}
			Ok(None) // return value is not meaningful
		});
	}

	pub(in crate::gui) fn run_main_loop(
		haccel: Option<HACCEL>) -> ErrResult<i32>
	{
		let mut msg = MSG::default();

		loop {
			if !GetMessage(&mut msg, None, 0, 0)? {
				// WM_QUIT was sent, gracefully terminate the program.
				// wParam has the program exit code.
				// https://docs.microsoft.com/en-us/windows/win32/winmsg/using-messages-and-message-queues
				return match unsafe { QUIT_ERROR.take() } {
					Some(err) => Err(err),
					None => Ok(msg.wParam as _),
				};
			}

			// If a child window, will retrieve its top-level parent.
			// If a top-level, use itself.
			let hwnd_top_level = msg.hwnd.GetAncestor(co::GA::ROOT)
				.unwrap_or(msg.hwnd);

			// If we have an accelerator table, try to translate the message.
			if let Some(haccel) = haccel {
				if hwnd_top_level.TranslateAccelerator(haccel, &mut msg).is_ok() {
					continue; // message translated
				}
			}

			// Try to process keyboard actions for child controls.
			if hwnd_top_level.IsDialogMessage(&mut msg) {
				continue;
			}

			TranslateMessage(&msg);
			DispatchMessage(&msg);
		}
	}
}
