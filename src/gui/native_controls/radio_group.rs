use std::ops::Index;
use std::rc::Rc;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::gui::{RadioButton, RadioButtonOpts};
use crate::gui::events::RadioGroupEvents;
use crate::gui::resizer::{Horz, Vert};
use crate::gui::traits::{baseref_from_parent, Child, Parent};
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
	/// [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if no options are passed.
	pub fn new(parent: &dyn Parent, opts: &[RadioButtonOpts]) -> RadioGroup {
		if opts.is_empty() {
			panic!("RadioGroup needs at least one RadioButton.");
		}

		let parent_base_ref = baseref_from_parent(parent);

		let radios = opts.iter().enumerate().map(|(i, opt)| {
			let mut radio_opt = opt.manual_clone();
			if i == 0 { // first radio?
				radio_opt.window_style |= co::WS::TABSTOP | co::WS::GROUP;
			}
			RadioButton::new(parent, radio_opt)
		}).collect::<Vec<_>>();

		let ctrl_ids = opts.iter().map(|opt| opt.ctrl_id).collect::<Vec<_>>();
		let horz_verts = Rc::new(opts.iter().map(|opt| (opt.horz_resize, opt.vert_resize)).collect::<Vec<_>>());

		let new_self = Self(
			Arc::new(VeryUnsafeCell::new(
				Obj {
					radios,
					parent_events: RadioGroupEvents::new(parent_base_ref, ctrl_ids),
				},
			)),
		);

		parent_base_ref.privileged_events_ref().wm(parent_base_ref.create_or_initdlg(), {
			let me = new_self.clone();
			let horz_verts = horz_verts.clone();
			move |_| { me.create(horz_verts.as_ref())?; Ok(0) }
		});

		new_self
	}

	/// Instantiates a new `RadioGroup` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if no control IDs are passed.
	pub fn new_dlg(parent: &dyn Parent, ctrls: &[(u16, Horz, Vert)]) -> RadioGroup {
		if ctrls.is_empty() {
			panic!("RadioGroup needs at least one RadioButton.");
		}

		let parent_base_ref = baseref_from_parent(parent);

		let radios = ctrls.iter().map(|(ctrl_id, _, _)| RadioButton::new_dlg(parent, *ctrl_id)).collect::<Vec<_>>();
		let ctrl_ids = ctrls.iter().map(|(ctrl_id, _, _)| *ctrl_id).collect::<Vec<_>>();
		let horz_verts = Rc::new(ctrls.iter().map(|(_, horz, vert)| (*horz, *vert)).collect::<Vec<_>>());

		let new_self = Self(
			Arc::new(VeryUnsafeCell::new(
				Obj {
					radios,
					parent_events: RadioGroupEvents::new(parent_base_ref, ctrl_ids),
				},
			)),
		);

		parent_base_ref.privileged_events_ref().wm_init_dialog({
			let me = new_self.clone();
			let horz_verts = horz_verts.clone();
			move |_| { me.create(horz_verts.as_ref())?; Ok(true) }
		});

		new_self
	}

	fn create(&self, horz_vert: &[(Horz, Vert)]) -> WinResult<()> {
		for (i, radio) in self.0.as_mut().radios.iter_mut().enumerate() {
			radio.create(horz_vert[i].0, horz_vert[i].1)?;
		}
		Ok(())
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
		} else if !first_radio.parent_hwnd_ref().is_null() {
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
	///     single_radio.hwnd().SetWindowText("One")?;
	/// }
	/// ```
	pub fn iter(&self) -> std::slice::Iter<'_, RadioButton> {
		self.0.radios.iter()
	}

	/// Returns the [`RadioButton`](crate::gui::RadioButton) controls as a `Vec`
	/// of [`Child`](crate::gui::Child).
	pub fn as_child_vec(&self) -> Vec<&dyn Child> {
		self.0.radios.iter()
			.map(|rb| { let c: &dyn Child = rb; c })
			.collect::<Vec<_>>()
	}

	/// Returns the currently checked [`RadioButton`](crate::gui::RadioButton) of
	/// this group, if any.
	pub fn checked(&self) -> Option<&RadioButton> {
		self.checked_index().map(|idx| &self.0.radios[idx])
	}

	/// Returns the index of the currently selected
	/// [`RadioButton`](crate::gui::RadioButton) of this group, if any.
	pub fn checked_index(&self) -> Option<usize> {
		for (idx, radio) in self.0.radios.iter().enumerate() {
			if radio.is_selected() {
				return Some(idx);
			}
		}
		None
	}

	/// Returns the number of [`RadioButton`](crate::gui::RadioButton) controls
	/// in this group.
	pub fn count(&self) -> usize {
		self.0.radios.len()
	}
}
