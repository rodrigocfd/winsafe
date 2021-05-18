use std::ops::Index;
use std::sync::Arc;

use crate::co;
use crate::funcs::PostQuitMessage;
use crate::gui::{RadioButton, RadioButtonOpts};
use crate::gui::events::RadioGroupEvents;
use crate::gui::traits::{baseref_from_parent, Parent};
use crate::gui::very_unsafe_cell::VeryUnsafeCell;

/// A group of native [`RadioButton`](crate::gui::RadioButton) controls.
#[derive(Clone)]
pub struct RadioGroup(Arc<VeryUnsafeCell<Obj>>);

struct Obj { // actual fields of RadioGroup
	radios: Vec<RadioButton>,
	parent_events: RadioGroupEvents,
}

unsafe impl Send for RadioGroup {}
unsafe impl Sync for RadioGroup {}

impl Index<usize> for RadioGroup {
	type Output = RadioButton;

	fn index(&self, i: usize) -> &Self::Output {
		&self.0.radios[i]
	}
}

impl RadioGroup {
	/// Instantiates a new `RadioGroup` object, each `RadioButton` to be created
	/// on the parent window with
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if no options are passed.
	pub fn new(parent: &dyn Parent, opts: &[RadioButtonOpts]) -> RadioGroup {
		if opts.is_empty() {
			panic!("RadioGroup needs at least one RadioButton.");
		}

		let parent_ref = baseref_from_parent(parent);
		let mut ctrl_ids = Vec::with_capacity(opts.len());
		let mut radios = Vec::with_capacity(opts.len());

		for (idx, radio_opts) in opts.iter().enumerate() {
			let mut radio_opts = radio_opts.manual_clone();
			if idx == 0 { // first radio?
				radio_opts.window_style |= co::WS::TABSTOP | co::WS::GROUP;
			}

			let new_radio = RadioButton::new(parent, radio_opts);
			ctrl_ids.push(new_radio.ctrl_id());
			radios.push(new_radio);
		}

		let new_self = Self(
			Arc::new(VeryUnsafeCell::new(
				Obj {
					radios,
					parent_events: RadioGroupEvents::new(parent_ref, ctrl_ids),
				},
			)),
		);

		parent_ref.privileged_events_ref().wm(parent_ref.creation_wm(), {
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});

		new_self
	}

	/// Instantiates a new `RadioGroup` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if no control IDs are passed.
	pub fn new_dlg(parent: &dyn Parent, ctrl_ids: &[i32]) -> RadioGroup {
		if ctrl_ids.is_empty() {
			panic!("RadioGroup needs at least one RadioButton.");
		}

		let parent_ref = baseref_from_parent(parent);
		let mut radios = Vec::with_capacity(ctrl_ids.len());

		for ctrl_id in ctrl_ids.iter() {
			radios.push(RadioButton::new_dlg(parent, *ctrl_id));
		}

		let new_self = Self(
			Arc::new(VeryUnsafeCell::new(
				Obj {
					radios,
					parent_events: RadioGroupEvents::new(parent_ref, ctrl_ids.to_vec()),
				},
			)),
		);

		parent_ref.privileged_events_ref().wm_init_dialog({
			let me = new_self.clone();
			move |_| { me.create(); true }
		});

		new_self
	}

	fn create(&self) {
		for radio in self.0.as_mut().radios.iter_mut() {
			radio.create()
				.unwrap_or_else(|err| PostQuitMessage(err));
		}
	}

	/// Exposes the radio group events.
	///
	/// These event methods are just proxies to the
	/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window,
	/// who is the real responsible for the child event handling.
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	pub fn on(&self) -> &RadioGroupEvents {
		let first_radio = self.index(0);

		if !first_radio.hwnd().is_null() {
			panic!("Cannot add events after the control is created.");
		} else if !first_radio.base_ref().parent_ref().hwnd_ref().is_null() {
			panic!("Cannot add events after the parent window is created.");
		}
		&self.0.parent_events
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
	/// let radio_group: RadioGroup; // initialized somewhere
	///
	/// for single_radio in me.rads.iter() {
	///     single_radio.hwnd().SetWindowText("One").unwrap();
	/// }
	/// ```
	pub fn iter(&self) -> std::slice::Iter<'_, RadioButton> {
		self.0.radios.iter()
	}

	/// Returns the currently checked [`RadioButton`](crate::gui::RadioButton) of
	/// this group, if any.
	pub fn checked(&self) -> Option<&RadioButton> {
		for radio in self.0.radios.iter() {
			if radio.is_checked() {
				return Some(radio);
			}
		}
		None
	}
}
