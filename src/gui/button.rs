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
	/// Creates a new Button object.
	pub fn new(parent: &impl Parent) -> Button {
		Self {
			base: NativeControlBase::new(),
			parent_events: parent.on(),
			subclass_events: Events::new(),
		}
	}

	/// Returns the underlying handle for this window.
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

//------------------------------------------------------------------------------

/// Allows adding closures to handle button
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications).
pub struct EventsButton {
	parent_events: Events,
}

impl EventsButton {
	pub(super) fn new(parent_events: Events) -> EventsButton {
		Self {
			parent_events
		}
	}

	pub fn BnClicked<F>(&self, func: F)
		where F: FnMut() + Send + Sync + 'static,
	{

	}
}