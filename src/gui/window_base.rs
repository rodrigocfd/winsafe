use crate::gui::events::Events;
use crate::handles::HWND;

/// Base to all ordinary windows.
#[derive(Clone)]
pub struct WindowBase {
	hwnd: HWND,
	events: Events,
}

impl WindowBase {
	pub fn new() -> WindowBase {
		Self {
			hwnd: HWND::default(),
			events: Events::new(),
		}
	}

	pub fn hwnd(&self) -> HWND {
		self.hwnd
	}

	pub fn on(&self) -> Events {
		self.events.clone()
	}
}