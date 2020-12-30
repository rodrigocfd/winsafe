use std::cell::UnsafeCell;
use std::sync::Arc;

use crate::gui::events::{ButtonEvents, MsgEvents};
use crate::gui::native_control_base::NativeControlBase;
use crate::gui::parent::Parent;
use crate::handles::HWND;

struct Obj {
	base: NativeControlBase,
	parent_events: ButtonEvents,
}

//------------------------------------------------------------------------------

/// Native
/// [button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#push-buttons)
/// control.
#[derive(Clone)]
pub struct Button {
	obj: Arc<UnsafeCell<Obj>>,
}

unsafe impl Send for Button {}
unsafe impl Sync for Button {}

cref_mref!(Button);

impl Button {
	/// Creates a new Button object.
	pub fn new<T: Parent>(parent: T) -> Button {
		Self::new_with_id(parent, NativeControlBase::auto_ctrl_id())
	}

	/// Creates a new Button object with a specific control ID.
	pub fn new_with_id<T: Parent>(parent: T, ctrl_id: u16) -> Button {
		Self {
			obj: Arc::new(UnsafeCell::new(
				Obj {
					base: NativeControlBase::new_with_id(ctrl_id),
					parent_events: ButtonEvents::new(parent.on(), ctrl_id),
				}
			)),
		}
	}

	/// Returns the underlying handle for this window.
	pub fn hwnd(&self) -> HWND {
		self.cref().base.hwnd()
	}

	/// Returns the control ID.
	pub fn ctrl_id(&self) -> u16 {
		self.cref().base.ctrl_id()
	}

	/// Exposes the events that can be handled with a closure.
	///
	/// # Panics
	///
	/// Panics if the control is already created. Closures must be attached to
	/// events before control creation.
	pub fn on(&self) -> ButtonEvents {
		self.cref().parent_events.clone()
	}

	/// Exposes the subclassing handler methods. If at least one handle is added,
	/// the control will be subclassed.
	///
	/// # Panics
	///
	/// Panics if the control is already created. Closures must be attached to
	/// events before control creation.
	pub fn on_subclass(&self) -> MsgEvents {
		self.cref().base.on_subclass()
	}

	/// Physically creates the control within the parent window.
	///
	/// # Panics
	///
	/// Panics if the control is already created.
	pub fn create(&self) {
		if !self.cref().base.hwnd().is_null() {
			panic!("Cannot create Button twice.");
		}


	}
}
