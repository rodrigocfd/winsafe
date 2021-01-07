use std::cell::UnsafeCell;
use std::sync::Arc;

use crate::co;
use crate::gui::controls::native_control_base::NativeControlBase;
use crate::gui::events::{ButtonEvents, MsgEvents};
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;
use crate::msg::WmCommand;

/// Native
/// [button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#push-buttons)
/// control, specifically to be used as the child of a dialog resource.
#[derive(Clone)]
pub struct ButtonDlg {
	obj: Arc<UnsafeCell<Obj>>,
}

struct Obj { // actual fields of ButtonDlg
	base: NativeControlBase,
	ctrl_id: u16,
	parent_events: ButtonEvents,
}

unsafe impl Send for ButtonDlg {}
unsafe impl Sync for ButtonDlg {}

cref_mref!(ButtonDlg);

impl Child for ButtonDlg {
	fn create(&self) -> Result<(), co::ERROR> {
		self.mref().base
			.create_dlg(self.cref().ctrl_id)
			.map(|_| ())
	}
}

impl ButtonDlg {
	/// Creates a new Button object.
	pub fn new(parent: &dyn Parent, ctrl_id: u16) -> ButtonDlg {
		Self {
			obj: Arc::new(UnsafeCell::new(
				Obj {
					base: NativeControlBase::new(parent.hwnd_ref()),
					ctrl_id,
					parent_events: ButtonEvents::new(parent, ctrl_id),
				},
			)),
		}
	}

	/// Returns the underlying handle for this control.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is created.
	pub fn hwnd(&self) -> HWND {
		*self.cref().base.hwnd()
	}

	/// Returns the control ID.
	pub fn ctrl_id(&self) -> u16 {
		self.cref().ctrl_id
	}

	/// Exposes the button events.
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	///
	/// # Examples
	///
	/// When button is clicked, becomes disabled:
	///
	/// ```rust,ignore
	/// use winsafe::gui::Button;
	///
	/// let btn: Button; // initialize it somewhere...
	///
	/// btn.on().bn_clicked({
	///   let btn = btn.clone(); // pass into closure
	///   move || {
	///     btn.EnableWindow(false);
	///   }
	/// });
	/// ```
	pub fn on(&self) -> &ButtonEvents {
		if !self.hwnd().is_null() {
			panic!("Cannot add events after the control is created.");
		} else if self.cref().base.is_parent_created() {
			panic!("Cannot add events after the parent window is created.");
		}
		&self.cref().parent_events
	}

	/// Exposes the subclass events. If at least one event exists, the control
	/// will be
	/// [subclassed](https://docs.microsoft.com/en-us/windows/win32/controls/subclassing-overview).
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	pub fn on_subclass(&self) -> &MsgEvents {
		self.cref().base.on_subclass()
	}

	/// Fires the click event for the button.
	pub fn trigger_click(&self) {
		self.hwnd().SendMessage(
			WmCommand {
				code: co::CMD::BN_CLICKED,
				ctrl_id: self.ctrl_id(),
				ctrl_hwnd: Some(self.hwnd()),
			},
		);
	}
}
