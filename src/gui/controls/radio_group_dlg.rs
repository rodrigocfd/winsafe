use std::cell::UnsafeCell;
use std::ops::Index;
use std::sync::Arc;

use crate::co;
use crate::gui::RadioButtonDlg;
use crate::gui::events::RadioGroupEvents;
use crate::gui::traits::{Child, Parent};

/// A group of native
/// [radio button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#radio-buttons)
/// control, specifically to be used as the child of a dialog resource.
///
/// The radion button is actually a variation of the ordinary
/// [`Button`](crate::gui::Button): just a button with a specific style.
#[derive(Clone)]
pub struct RadioGroupDlg {
	obj: Arc<UnsafeCell<Obj>>,
}

struct Obj { // actual fields of RadioGroup
	radios: Vec<RadioButtonDlg>,
	parent_events: RadioGroupEvents,
}

unsafe impl Send for RadioGroupDlg {}
unsafe impl Sync for RadioGroupDlg {}

cref_mref!(RadioGroupDlg);

impl Child for RadioGroupDlg {
	fn create(&self) -> Result<(), co::ERROR> {
		for radio in self.mref().radios.iter_mut() {
			radio.create()?;
		}
		Ok(())
	}
}

impl Index<usize> for RadioGroupDlg {
	type Output = RadioButtonDlg;

	fn index(&self, i: usize) -> &Self::Output {
		&self.cref().radios[i]
	}
}

impl RadioGroupDlg {
	/// Creates a new RadioGroup object.
	///
	/// # Panic
	///
	/// Panics if no control IDs are passed.
	pub fn new(parent: &dyn Parent, ctrl_ids: &[u16]) -> RadioGroupDlg {
		if ctrl_ids.is_empty() {
			panic!("RadioGroup needs at least one RadioButton.");
		}

		let mut radios = Vec::with_capacity(ctrl_ids.len());

		for ctrl_id in ctrl_ids.iter() {
			radios.push(RadioButtonDlg::new(parent, *ctrl_id));
		}

		Self {
			obj: Arc::new(UnsafeCell::new(
				Obj {
					radios,
					parent_events: RadioGroupEvents::new(parent, ctrl_ids.to_vec()),
				},
			)),
		}
	}

	/// Exposes the radio group events.
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	pub fn on(&self) -> &RadioGroupEvents {
		let first_radio = self.index(0);

		if !first_radio.hwnd().is_null() {
			panic!("Cannot add events after the control is created.");
		} else if first_radio.is_parent_created() {
			panic!("Cannot add events after the parent window is created.");
		}
		&self.cref().parent_events
	}

	/// Returns an iterator over the internal
	/// [`RadioButton`](crate::gui::RadioButton) slice.
	///
	/// # Example
	///
	/// Changing the text of all radio buttons to `"One"`:
	///
	/// ```rust,ignore
	/// use winsafe::gui::RadioGroup;
	///
	/// let radio_group: RadioGroup; // initialize it somewhere...
	///
	/// for single_radio in me.rads.iter() {
	///   single_radio.hwnd().SetWindowText("One").unwrap();
	/// }
	/// ```
	pub fn iter(&self) -> std::slice::Iter<'_, RadioButtonDlg> {
		self.cref().radios.iter()
	}

	/// Returns the currently checked [`RadioButton`](crate::gui::RadioButton),
	/// if any.
	pub fn checked(&self) -> Option<&RadioButtonDlg> {
		for radio in self.cref().radios.iter() {
			if radio.is_checked() {
				return Some(radio);
			}
		}
		None
	}
}
