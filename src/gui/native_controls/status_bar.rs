use std::sync::Arc;

use crate::gui::events::StatusBarEvents;
use crate::gui::native_controls::native_control_base::NativeControlBase;
use crate::gui::privs::auto_ctrl_id;
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;

/// Native
/// [status bar](https://docs.microsoft.com/en-us/windows/win32/controls/status-bars)
/// control.
#[derive(Clone)]
pub struct StatusBar(Arc<Obj>);

struct Obj { // actual fields of StatusBar
	base: NativeControlBase<StatusBarEvents>,
	ctrl_id: u16,
}

unsafe impl Send for StatusBar {}
unsafe impl Sync for StatusBar {}

impl Child for StatusBar {
	fn hctrl_ref(&self) -> &HWND {
		self.0.base.hctrl_ref()
	}
}

impl StatusBar {
	/// Instantiates a new `StatusBar` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent) -> StatusBar {
		let ctrl_id = auto_ctrl_id();
		let me = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(
						parent,
						StatusBarEvents::new(parent, ctrl_id),
					),
					ctrl_id,
				},
			),
		);
		parent.privileged_events_ref().wm_create({
			let me = me.clone();
			move |_| { me.create(); 0 }
		});
		me
	}

	fn create(&self) {

	}
}
