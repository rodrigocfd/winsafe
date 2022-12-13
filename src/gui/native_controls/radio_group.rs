use std::marker::PhantomPinned;
use std::ops::Index;
use std::pin::Pin;
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::Arc;

use crate::co;
use crate::gui::base::Base;
use crate::gui::events::RadioGroupEvents;
use crate::gui::layout_arranger::{Horz, Vert};
use crate::gui::native_controls::radio_button::{RadioButton, RadioButtonOpts};
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::prelude::{
	GuiChild, GuiEvents, GuiNativeControlEvents, GuiParent, GuiWindow, Handle,
};
use crate::user::decl::HWND;

struct Obj { // actual fields of RadioGroup
	parent_ptr: NonNull<Base>,
	radios: VeryUnsafeCell<Vec<RadioButton>>,
	events: RadioGroupEvents,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// A group of native [`RadioButton`](crate::gui::RadioButton) controls.
#[derive(Clone)]
pub struct RadioGroup(Pin<Arc<Obj>>);

unsafe impl Send for RadioGroup {}

impl Index<usize> for RadioGroup {
	type Output = RadioButton;

	fn index(&self, i: usize) -> &Self::Output {
		&self.0.radios[i]
	}
}

impl GuiNativeControlEvents<RadioGroupEvents> for RadioGroup {
	fn on(&self) -> &RadioGroupEvents {
		if *self.index(0).hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *unsafe { self.0.parent_ptr.as_ref() }.hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl RadioGroup {
	/// Instantiates a new `RadioGroup` object, each `RadioButton` to be created
	/// on the parent window with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if `opts` is empty.
	#[must_use]
	pub fn new(
		parent: &impl GuiParent,
		opts: &[RadioButtonOpts]) -> RadioGroup
	{
		if opts.is_empty() {
			panic!("RadioGroup needs at least one RadioButton.");
		}

		let radios = opts.iter().enumerate()
			.map(|(i, opt)| {
				let mut radio_opt = opt.manual_clone();
				if i == 0 { // first radio?
					radio_opt.window_style |= co::WS::TABSTOP | co::WS::GROUP;
				}
				RadioButton::new(parent, radio_opt)
			})
			.collect::<Vec<_>>();

		let ctrl_ids = radios.iter()
			.map(|r| r.ctrl_id()) // when the radio is created, the ctrl ID is defined
			.collect::<Vec<_>>();

		let horz_verts = Rc::new(
			opts.iter()
				.map(|opt| (opt.horz_resize, opt.vert_resize))
				.collect::<Vec<_>>(),
		);

		let parent_ref = unsafe { Base::from_guiparent(parent) };

		let new_self = Self(
			Arc::pin(
				Obj {
					parent_ptr: NonNull::from(parent_ref),
					radios: VeryUnsafeCell::new(radios),
					events: RadioGroupEvents::new(parent_ref, ctrl_ids),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		let horz_verts = horz_verts.clone();
		parent_ref.privileged_on().wm(parent_ref.creation_msg(), move |_| {
			self2.create(horz_verts.as_ref());
			Ok(None) // not meaningful
		});
		new_self
	}

	/// Instantiates a new `RadioGroup` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if `ctrls` is empty.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		ctrls: &[(u16, Horz, Vert)]) -> RadioGroup
	{
		if ctrls.is_empty() {
			panic!("RadioGroup needs at least one RadioButton.");
		}

		let radios = ctrls.iter()
			.map(|(ctrl_id, _, _)| RadioButton::new_dlg(parent, *ctrl_id))
			.collect::<Vec<_>>();

		let ctrl_ids = ctrls.iter()
			.map(|(ctrl_id, _, _)| *ctrl_id)
			.collect::<Vec<_>>();

		let horz_verts = ctrls.iter()
			.map(|(_, horz, vert)| (*horz, *vert))
			.collect::<Vec<_>>();

		let parent_ref = unsafe { Base::from_guiparent(parent) };

		let new_self = Self(
			Arc::pin(
				Obj {
					parent_ptr: NonNull::from(parent_ref),
					radios: VeryUnsafeCell::new(radios),
					events: RadioGroupEvents::new(parent_ref, ctrl_ids),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_ref.privileged_on().wm_init_dialog(move |_| {
			self2.create(horz_verts.as_ref());
			Ok(true) // not meaningful
		});
		new_self
	}

	fn create(&self, horz_vert: &[(Horz, Vert)]) {
		for (i, radio) in self.0.radios.iter().enumerate() {
			radio.create(horz_vert[i].0, horz_vert[i].1); // create each RadioButton sequentially
		}
	}

	/// Returns an iterator over the internal
	/// [`RadioButton`](crate::gui::RadioButton) slice.
	///
	/// # Example
	///
	/// Changing the text of all radio buttons to `"One"`:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::gui;
	///
	/// let radio_group: gui::RadioGroup; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let radio_group = gui::RadioGroup::new(&wnd, &[]);
	///
	/// for single_radio in radio_group.iter() {
	///     single_radio.hwnd().SetWindowText("One");
	/// }
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	pub fn iter(&self) -> std::slice::Iter<'_, RadioButton> {
		self.0.radios.iter()
	}

	/// Returns the currently checked [`RadioButton`](crate::gui::RadioButton)
	/// of this group, if any.
	#[must_use]
	pub fn checked(&self) -> Option<&RadioButton> {
		self.checked_index().map(|idx| &self.0.radios[idx])
	}

	/// Returns the index of the currently selected
	/// [`RadioButton`](crate::gui::RadioButton) of this group, if any.
	#[must_use]
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
	#[must_use]
	pub fn count(&self) -> usize {
		self.0.radios.len()
	}
}
