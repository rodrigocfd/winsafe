use std::ptr::NonNull;

use crate::aliases::{ErrResult, WinResult};
use crate::co;
use crate::funcs::{DispatchMessage, GetMessage, TranslateMessage};
use crate::gui::events::{ProcessResult, WindowEvents};
use crate::gui::privs::{post_quit_error, QUIT_ERROR};
use crate::handles::{HACCEL, HINSTANCE, HWND};
use crate::msg::WndMsg;
use crate::structs::MSG;

/// Base to `RawBase` and `DlgBase`.
pub(in crate::gui) struct Base {
	hwnd: HWND,
	is_dialog: bool,
	ptr_parent: Option<NonNull<Base>>, // parent of this window
	user_events: WindowEvents, // ordinary window events, inserted by user: only last added is executed (overwrite previous)
	privileged_events: WindowEvents, // inserted internally to automate tasks: all will be executed
}

impl Base {
	const WM_UI_THREAD: co::WM = co::WM(co::WM::APP.0 + 0x3fff);

	pub(in crate::gui) fn new(
		parent_base_ref: Option<&Base>, is_dialog: bool) -> Base
	{
		Self {
			hwnd: HWND::NULL,
			is_dialog,
			ptr_parent: parent_base_ref.map(|pb_ref| NonNull::from(pb_ref)),
			user_events: WindowEvents::new(),
			privileged_events: WindowEvents::new(),
		}
	}

	pub(in crate::gui) fn hwnd_ref(&self) -> &HWND {
		&self.hwnd
	}

	pub(in crate::gui) fn set_hwnd(&mut self, hwnd: HWND) {
		self.hwnd = hwnd;
	}

	pub(in crate::gui) fn creation_wm(&self) -> co::WM {
		if self.is_dialog { co::WM::INITDIALOG } else { co::WM::CREATE }
	}

	pub(in crate::gui) fn parent_base_ref(&self) -> Option<&Base> {
		self.ptr_parent.as_ref().map(|ptr| unsafe { ptr.as_ref() })
	}

	pub(in crate::gui) fn parent_hinstance(&self) -> WinResult<HINSTANCE> {
		self.parent_base_ref().map_or_else(
			|| HINSTANCE::GetModuleHandle(None),
			|parent| Ok(parent.hwnd_ref().hinstance()),
		)
	}

	pub(in crate::gui) fn user_events_ref(&self) -> &WindowEvents {
		if !self.hwnd.is_null() {
			panic!("Cannot add event after window is created.");
		}
		&self.user_events
	}

	pub(in crate::gui) fn privileged_events_ref(&self) -> &WindowEvents {
		if !self.hwnd.is_null() {
			panic!("Cannot add privileged event after window is created.");
		}
		&self.privileged_events
	}

	pub(in crate::gui) fn process_one_message(&mut self,
		wm_any: WndMsg) -> ErrResult<ProcessResult>
	{
		self.user_events.process_one_message(wm_any)
	}

	pub(in crate::gui) fn process_privileged_messages(&mut self,
		wm_any: WndMsg) -> ErrResult<()>
	{
		self.privileged_events.process_all_messages(wm_any)
	}

	pub(in crate::gui) fn ui_thread_message_handler(&self) {
		self.privileged_events.wm(Self::WM_UI_THREAD, |p| {
			if co::WM(p.wparam as _) == Self::WM_UI_THREAD { // additional safety check
				let ptr_pack = p.lparam as *mut Box<dyn FnOnce() -> ErrResult<()>>;
				let pack: Box<Box<dyn FnOnce() -> ErrResult<()>>> = unsafe { Box::from_raw(ptr_pack) };
				pack().unwrap_or_else(|err| post_quit_error(err));
			}
			Ok(0)
		});
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
