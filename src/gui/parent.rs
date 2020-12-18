use crate::gui::events::Events;

/// Trait to any window which can host child controls.
pub trait Parent : Clone {
	fn on(&self) -> Events;
}