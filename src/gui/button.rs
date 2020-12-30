use std::cell::UnsafeCell;
use std::sync::Arc;

use crate::gui::events::{ButtonEvents, MsgEvents};
use crate::gui::native_control_base::NativeControlBase;
use crate::gui::parent::Parent;
use crate::handles::HWND;

struct Obj {
	base: NativeControlBase,
	parent_events: ButtonEvents,
	subclass_events: MsgEvents,
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
					subclass_events: MsgEvents::new(),
				}
			)),
		}
	}

	/// Returns the underlying handle for this window.
	pub fn hwnd(&self) -> HWND {
		let self2 = unsafe { &*self.obj.get() };
		self2.base.hwnd()
	}

	/// Returns the control ID.
	pub fn ctrl_id(&self) -> u16 {
		let self2 = unsafe { &*self.obj.get() };
		self2.base.ctrl_id()
	}

	pub fn on(&self) -> ButtonEvents {
		let self2 = unsafe { &*self.obj.get() };
		self2.parent_events.clone()
	}

	pub fn on_subclass(&self) -> MsgEvents {
		let self2 = unsafe { &*self.obj.get() };
		self2.subclass_events.clone()
	}
}