use crate::gui::events::Events;

/// Trait to any window which can host child controls.
pub trait Parent {
	/// Exposes the events that can be handled with a closure.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Closures must be attached to
	/// events before window creation.
	fn on(&self) -> Events;
}