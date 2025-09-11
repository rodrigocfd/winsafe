use std::marker::PhantomPinned;
use std::ops::Index;
use std::pin::Pin;
use std::sync::Arc;

use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::prelude::*;

struct RadioGroupObj {
	radios: Vec<RadioButton>,
	events: BaseCtrlEvents,
	_pin: PhantomPinned,
}

/// A group of native [`RadioButton`](crate::gui::RadioButton) controls.
#[derive(Clone)]
pub struct RadioGroup(Pin<Arc<RadioGroupObj>>);

unsafe impl Send for RadioGroup {}

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
	/// Panics if `opts` is empty.
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `RadioGroup` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: &[RadioButtonOpts]) -> Self {
		if opts.is_empty() {
			panic!("RadioGroup needs at least one RadioButton.");
		}

		let (ctrl_ids, radios): (Vec<_>, Vec<_>) = opts
			.iter()
			.enumerate()
			.map(|(idx, opt)| {
				let radio = RadioButton::new(parent, opt.clone(), idx == 0);
				(radio.ctrl_id(), radio) // the constructor may set the ID
			})
			.unzip();

		Self(Arc::pin(RadioGroupObj {
			radios,
			events: BaseCtrlEvents::new_many(parent, ctrl_ids),
			_pin: PhantomPinned,
		}))
	}

	/// Instantiates a new `RadioGroup` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if `ctrls` is empty.
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `RadioGroup` in an event closure.
	#[must_use]
	pub fn new_dlg(parent: &(impl GuiParent + 'static), ctrls: &[(u16, Horz, Vert)]) -> Self {
		if ctrls.is_empty() {
			panic!("RadioGroup needs at least one RadioButton.");
		}

		let (ctrl_ids, radios): (Vec<_>, Vec<_>) = ctrls
			.iter()
			.enumerate()
			.map(|(idx, (ctrl_id, horz, vert))| {
				(*ctrl_id, RadioButton::new_dlg(parent, *ctrl_id, idx == 0, (*horz, *vert)))
			})
			.unzip();

		Self(Arc::pin(RadioGroupObj {
			radios,
			events: BaseCtrlEvents::new_many(parent, ctrl_ids),
			_pin: PhantomPinned,
		}))
	}

	/// Exposes the specific control events.
	///
	/// # Panics
	///
	/// Panics if the controls are already created. Events must be set before
	/// control creation.
	#[must_use]
	pub fn on(&self) -> &impl GuiEventsRadioGroup {
		if *self.0.radios[0].hwnd() != HWND::NULL {
			panic!("Cannot add events after control creation.");
		}
		&self.0.events
	}

	/// Returns the number of [`RadioButton`](crate::gui::RadioButton) controls
	/// in this group.
	#[must_use]
	pub fn count(&self) -> usize {
		self.0.radios.len()
	}

	/// Returns an iterator over the internal
	/// [`RadioButton`](crate::gui::RadioButton) objects.
	///
	/// # Example
	///
	/// Changing the text of all radio buttons to `"One"`:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let radio_group: gui::RadioGroup; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let radio_group = gui::RadioGroup::new(&wnd, &[]);
	///
	/// for radio in radio_group.iter() {
	///     radio.hwnd().SetWindowText("One");
	/// }
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn iter(&self) -> impl Iterator<Item = &RadioButton> {
		self.0.radios.iter()
	}

	/// Returns the currently checked [`RadioButton`](crate::gui::RadioButton)
	/// of this group, if any.
	#[must_use]
	pub fn selected(&self) -> Option<&RadioButton> {
		self.selected_index().map(|idx| &self.0.radios[idx])
	}

	/// Returns the index of the currently selected
	/// [`RadioButton`](crate::gui::RadioButton) of this group, if any.
	#[must_use]
	pub fn selected_index(&self) -> Option<usize> {
		self.0.radios.iter().position(|radio| radio.is_selected())
	}
}
