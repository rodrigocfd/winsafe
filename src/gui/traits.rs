use std::any::Any;

use crate::gui::base::Base;
use crate::gui::{WindowControl, WindowMain, WindowModal};
use crate::handles::HWND;

/// Trait to any child control.
///
/// # Examples
///
/// Implementing `Child` for your custom control:
///
/// ```rust,ignore
/// using winsafe::{gui, HWND};
///
/// #[derive(Clone)]
/// pub struct MyCustomControl {
///     wnd: gui::WindowControl,
/// }
///
/// impl gui::Child for MyCustomControl {
///     fn hwnd_ref(&self) -> &HWND {
///         self.wnd.hwnd_ref()
///     }
/// }
/// ```
pub trait Child: Clone {
	/// Returns a reference to the [`HWND`](crate::HWND) of the child control.
	fn hwnd_ref(&self) -> &HWND;
}

/// Trait to any window which can host child controls.
///
/// **Note:** This is a
/// [sealed trait](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed)
/// which cannot be implemented outside the library.
pub trait Parent: Clone + private::Sealed {
	/// Returns a reference to the `Any` trait, allowing downcasting.
	fn as_any(&self) -> &dyn Any;
}

pub(in crate::gui) mod private {
	pub trait Sealed {}

	impl Sealed for crate::gui::WindowControl {}
	impl Sealed for crate::gui::WindowMain {}
	impl Sealed for crate::gui::WindowModal {}
}

pub(in crate::gui) fn baseref_from_parent(parent: &impl Parent) -> &Base {
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
