use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::{DispatchMessage, GetMessage, TranslateMessage};
use crate::gui::events::{ProcessResult, WindowEvents};
use crate::gui::traits::Parent;
use crate::handles::{HACCEL, HINSTANCE, HWND};
use crate::msg::WndMsg;
use crate::structs::MSG;

/// Base to `RawBase` and `DlgBase`.
pub struct Base {
	hwnd: HWND,
	user_events: WindowEvents, // ordinary window events, inserted by user: only last added is executed (overwrite previous)
	privileged_events: WindowEvents, // inserted internally to automate tasks: all will be executed
	ptr_parent_hwnd: Option<NonNull<HWND>>, // used only in control creation
}

impl Parent for Base {
	fn hwnd_ref(&self) -> &HWND {
		&self.hwnd
	}

	fn user_events_ref(&self) -> &WindowEvents {
		if !self.hwnd.is_null() {
			panic!("Cannot add event after window is created.");
		}
		&self.user_events
	}

	fn privileged_events_ref(&self) -> &WindowEvents {
		if !self.hwnd.is_null() {
			panic!("Cannot add privileged event after window is created.");
		}
		&self.privileged_events
	}
}

impl Base {
	pub fn new(parent: Option<&dyn Parent>) -> Base {
		Self {
			hwnd: unsafe { HWND::null_handle() },
			user_events: WindowEvents::new(),
			privileged_events: WindowEvents::new(),
			ptr_parent_hwnd: parent.map(|parent| NonNull::from(parent.hwnd_ref())), // ref implicitly converted to pointer
		}
	}

	pub fn set_hwnd(&mut self, hwnd: HWND) {
		self.hwnd = hwnd;
	}

	pub fn parent_hwnd(&self) -> Option<HWND> {
		self.ptr_parent_hwnd.map(|ptr| unsafe { *ptr.as_ref() })
	}

	pub fn parent_hinstance(&self) -> WinResult<HINSTANCE> {
		Ok(match self.parent_hwnd() {
			Some(hparent) => hparent.hinstance(),
			None => HINSTANCE::GetModuleHandle(None)?,
		})
	}

	pub fn process_effective_message(&mut self, wm_any: WndMsg) -> ProcessResult {
		self.user_events.process_effective_message(wm_any)
	}

	pub fn process_privileged_messages(&mut self, wm_any: WndMsg) {
		self.privileged_events.process_all_messages(wm_any);
	}

	pub fn run_main_loop(haccel: Option<HACCEL>) -> WinResult<()> {
		loop {
			let mut msg = MSG::default();
			if !GetMessage(&mut msg, None, 0, 0)? {
				// WM_QUIT was sent, gracefully terminate the program.
				// wParam has the program exit code.
				// https://docs.microsoft.com/en-us/windows/win32/winmsg/using-messages-and-message-queues
				return match co::ERROR(msg.wParam as u32) {
					co::ERROR::SUCCESS => Ok(()),
					err => Err(err),
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
