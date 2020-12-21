use crate::gui::events::Events;
use crate::gui::Parent;
use crate::gui::window_base::WindowBase;
use crate::handles::HWND;

/// Main application window.
#[derive(Clone)]
pub struct WindowMain {
	base: WindowBase,
}

impl WindowMain {
	/// Creates a new `WindowMain` object.
	pub fn new() -> WindowMain {
		Self {
			base: WindowBase::new(),
		}
	}

	/// Returns the underlying handle for this window.
	pub fn hwnd(&self) -> HWND {
		self.base.hwnd()
	}
}

impl Parent for WindowMain {
	fn on(&self) -> Events {
		self.base.on()
	}
}