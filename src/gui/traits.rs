use crate::gui::events::WindowEvents;
use crate::handles::HWND;

/// Trait to any window which can host child controls.
pub trait Parent {
	/// Returns a reference to the window handle.
	fn hwnd_ref(&self) -> &HWND;

	/// Returns a reference to the user events object.
	///
	/// When an user event is added, it will overwrite the previous one.
	fn user_events_ref(&self) -> &WindowEvents;

	/// Returns a reference to the privileged events object, which is used
	/// internally to automate some tasks.
	///
	/// All privileged events are executed, and their result is ignored.
	fn privileged_events_ref(&self) -> &WindowEvents;
}

/// Trait to any child control.
pub trait Child {
	/// Returns a reference to the control handle.
	fn hctrl_ref(&self) -> &HWND;
}
