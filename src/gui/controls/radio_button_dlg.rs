use crate::co;
use crate::gui::controls::native_control_base::NativeControlBase;
use crate::gui::events::{ButtonEvents, MsgEvents};
use crate::gui::traits::Parent;
use crate::handles::HWND;
use crate::msg::{BmGetCheck, BmSetCheck, WmCommand};

/// Native
/// [radio button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#radio-buttons)
/// control, specifically to be used as the child of a dialog resource.
///
/// The radion button is actually a variation of the ordinary
/// [`Button`](crate::gui::Button): just a button with a specific style.
///
/// You cannot directly instantiate this object, you must use
/// [`RadioGroup`](crate::gui::RadioGroup).
pub struct RadioButtonDlg {
	base: NativeControlBase,
	ctrl_id: u16,
	parent_events: ButtonEvents,
}

impl RadioButtonDlg {
	pub(crate) fn new(parent: &dyn Parent, ctrl_id: u16) -> RadioButtonDlg {
		Self {
			base: NativeControlBase::new(parent.hwnd_ref()),
			ctrl_id,
			parent_events: ButtonEvents::new(parent, ctrl_id),
		}
	}

	pub(crate) fn create(&mut self) -> Result<(), co::ERROR> {
		self.base.create_dlg(self.ctrl_id)
			.map(|_| ())
	}

	pub(crate) fn is_parent_created(&self) -> bool {
		self.base.is_parent_created()
	}

	/// Returns the underlying handle for this control.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is created.
	pub fn hwnd(&self) -> HWND {
		*self.base.hwnd()
	}

	/// Returns the control ID.
	pub fn ctrl_id(&self) -> u16 {
		self.ctrl_id
	}

	/// Exposes the radio button events.
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	/// ```
	pub fn on(&self) -> &ButtonEvents {
		if !self.hwnd().is_null() {
			panic!("Cannot add events after the control is created.");
		} else if self.base.is_parent_created() {
			panic!("Cannot add events after the parent window is created.");
		}
		&self.parent_events
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
		self.base.on_subclass()
	}

	/// Tells if this radio button is currently checked.
	pub fn is_checked(&self) -> bool {
		self.hwnd().SendMessage(BmGetCheck {}) == co::BST::CHECKED
	}

	/// Sets the current check state.
	pub fn set_check(&self, checked: bool) {
		self.hwnd().SendMessage(BmSetCheck {
			state: if checked { co::BST::CHECKED } else { co::BST::UNCHECKED },
		});
	}

	/// Fires the click event for the radio button.
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
