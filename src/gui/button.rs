use crate::gui::events::Events;
use crate::gui::native_control_base::NativeControlBase;
use crate::gui::Parent;
use crate::handles::HWND;

/// Native
/// [button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#push-buttons)
/// control.
#[derive(Clone)]
pub struct Button {
	base: NativeControlBase,
	parent_events: Events,
	subclass_events: Events,
}

impl Button {
	pub fn new(parent: &impl Parent) -> Button {
		Self {
			base: NativeControlBase::new(),
			parent_events: parent.on(),
			subclass_events: Events::new(),
		}
	}

	pub fn hwnd(&self) -> HWND {
		self.base.hwnd()
	}

	pub fn on(&self) -> Events {
		self.parent_events.clone()
	}

	pub fn on_subclass(&self) -> Events {
		self.subclass_events.clone()
	}
}