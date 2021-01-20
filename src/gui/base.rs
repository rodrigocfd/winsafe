use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::gui::events::{MsgEvents, ProcessResult};
use crate::gui::traits::Parent;
use crate::msg::Wm;
use crate::handles::{HINSTANCE, HWND};

/// Base to `WindowBase` and `DialogBase`.
pub struct Base {
	hwnd: HWND,
	user_events: MsgEvents, // ordinary window events, inserted by user: only last added is executed (overwrite previous)
	privileged_events: MsgEvents, // inserted internally to automate tasks: all will be executed
	ptr_parent_hwnd: Option<NonNull<HWND>>, // used only in control creation
}

impl Parent for Base {
	fn hwnd_ref(&self) -> &HWND {
		&self.hwnd
	}

	fn user_events_ref(&self) -> &MsgEvents {
		if !self.hwnd.is_null() {
			panic!("Cannot add event after window is created.");
		}
		&self.user_events
	}

	fn privileged_events_ref(&self) -> &MsgEvents {
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
			user_events: MsgEvents::new(),
			privileged_events: MsgEvents::new(),
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

	pub fn process_effective_message(&mut self, wm_any: Wm) -> ProcessResult {
		self.user_events.process_effective_message(wm_any)
	}

	pub fn process_privileged_messages(&mut self, wm_any: Wm) {
		self.privileged_events.process_all_messages(wm_any);
	}
}
