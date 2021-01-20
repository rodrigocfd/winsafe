use crate::gui::events::MsgEvents;
use crate::handles::HWND;

/// Trait to any window which can host child controls.
pub trait Parent {
	/// Returns a reference to the window handle.
	fn hwnd_ref(&self) -> &HWND;

	/// Returns a reference to the user events object.
	fn user_events_ref(&self) -> &MsgEvents;

	/// Returns a reference to the privileged events object.
	fn privileged_events_ref(&self) -> &MsgEvents;
}

/// Trait to any child control.
pub trait Child {
	/// Returns a reference to the control handle.
	fn hctrl_ref(&self) -> &HWND;
}
