use std::any::Any;

use crate::gui::base::Base;
use crate::gui::{WindowControl, WindowMain, WindowModal};
use crate::handles::HWND;

/// Trait to any window which can host child controls.
pub trait Parent {
	/// Returns a reference to the `Any` trait, allowing downcasting.
	fn as_any(&self) -> &dyn Any;
}

/// Trait to any child control.
pub trait Child {
	/// Returns a reference to the [`HWND`](crate::HWND) of the child control.
	fn hwnd_ref(&self) -> &HWND;
}

pub(in crate::gui) fn baseref_from_parent(parent: &dyn Parent) -> &Base {
	if let Some(w) = parent.as_any().downcast_ref::<WindowMain>() {
		w.base_ref()
	} else if let Some(w) = parent.as_any().downcast_ref::<WindowModal>() {
		w.base_ref()
	} else if let Some(w) = parent.as_any().downcast_ref::<WindowControl>() {
		w.base_ref()
	} else {
		panic!("Unknown Parent downcasting, something really bad happened.")
	}
}
