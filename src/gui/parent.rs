use crate::gui::events::MsgEvents;
use crate::handles::HWND;

/// Trait to any window which can host child controls.
pub trait Parent {
	/// Returns the underlying handle for this window.
	fn hwnd(&self) -> HWND;

	/// Exposes the window events.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Closures must be attached to
	/// events before window creation.
	fn on(&self) -> MsgEvents;
}
