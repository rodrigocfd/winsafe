use std::cell::UnsafeCell;
use std::ops::Index;
use std::sync::Arc;

use crate::co;
use crate::gui::{RadioButton, RadioButtonOpts};
use crate::gui::traits::{Child, Parent};

/// A group of native
/// [radio button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#radio-buttons)
/// control.
///
/// The radion button is actually a variation of the ordinary
/// [`Button`](crate::gui::Button): just a button with a specific style.
#[derive(Clone)]
pub struct RadioGroup {
	obj: Arc<UnsafeCell<Obj>>,
}

struct Obj { // actual fields of RadioGroup
	radios: Vec<RadioButton>,
}

unsafe impl Send for RadioGroup {}
unsafe impl Sync for RadioGroup {}

cref_mref!(RadioGroup);

impl Child for RadioGroup {
	fn create(&self) -> Result<(), co::ERROR> {
		for radio in self.mref().radios.iter_mut() {
			radio.create()?;
		}
		Ok(())
	}
}

impl Index<usize> for RadioGroup {
	type Output = RadioButton;

	fn index(&self, i: usize) -> &Self::Output {
		&self.cref().radios[i]
	}
}

impl RadioGroup {
	/// Creates a new RadioGroup object.
	pub fn new(parent: &dyn Parent, opts: &[RadioButtonOpts]) -> RadioGroup {
		let mut radios = Vec::with_capacity(opts.len());

		for (idx, radio_opts) in opts.iter().enumerate() {
			let mut radio_opts = radio_opts.manual_clone();
			if idx == 0 { // first radio?
				radio_opts.window_style |= co::WS::TABSTOP | co::WS::GROUP;
			}
			radios.push(RadioButton::new(parent, radio_opts));
		}

		Self {
			obj: Arc::new(UnsafeCell::new(
				Obj { radios },
			)),
		}
	}

	/// Returns the currently checked [`RadioButton`](crate::gui::RadioButton),
	/// if any.
	pub fn checked(&self) -> Option<&RadioButton> {
		for radio in self.cref().radios.iter() {
			if radio.is_checked() {
				return Some(radio);
			}
		}
		None
	}
}
