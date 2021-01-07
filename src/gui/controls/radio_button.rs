use crate::co;
use crate::funcs::GetSystemMetrics;
use crate::gui::controls::native_control_base::NativeControlBase;
use crate::gui::events::{ButtonEvents, MsgEvents};
use crate::gui::globals::{auto_ctrl_id, calc_text_bound_box, ui_font};
use crate::gui::traits::Parent;
use crate::handles::HWND;
use crate::msg::{BmGetCheck, BmSetCheck, WmCommand, WmSetFont};
use crate::structs::{POINT, SIZE};

/// Native
/// [radio button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#radio-buttons)
/// control.
///
/// The radion button is actually a variation of the ordinary
/// [`Button`](crate::gui::Button): just a button with a specific style.
///
/// You cannot directly instantiate this object, you must use
/// [`RadioGroup`](crate::gui::RadioGroup).
pub struct RadioButton {
	base: NativeControlBase,
	opts: RadioButtonOpts,
	parent_events: ButtonEvents,
}

impl RadioButton {
	pub(crate) fn new(parent: &dyn Parent, opts: RadioButtonOpts) -> RadioButton {
		let mut opts = opts;
		opts.define_ctrl_id();
		let ctrl_id = opts.ctrl_id;

		Self {
			base: NativeControlBase::new(parent.hwnd_ref()),
			opts,
			parent_events: ButtonEvents::new(parent, ctrl_id),
		}
	}

	pub(crate) fn create(&mut self) -> Result<(), co::ERROR> {
		let bound_box = Self::ideal_size(&self.opts.text)?;

		let our_hwnd = self.base.create_window( // may panic
			"BUTTON", Some(&self.opts.text), self.opts.pos, bound_box,
			self.opts.ctrl_id,
			self.opts.ex_window_style,
			self.opts.window_style | self.opts.button_style.into(),
		)?;

		our_hwnd.SendMessage(WmSetFont{ hfont: ui_font(), redraw: true });
		Ok(())
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
		self.opts.ctrl_id
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

	/// Calculates the ideal size to fit the check followed by the given text.
	fn ideal_size(text: &str) -> Result<SIZE, co::ERROR> {
		let mut bound_box = calc_text_bound_box(text)?;
		bound_box.cx += GetSystemMetrics(co::SM::CXMENUCHECK) // https://stackoverflow.com/a/1165052/6923555
			+ GetSystemMetrics(co::SM::CXEDGE);

		let cy_check = GetSystemMetrics(co::SM::CYMENUCHECK);
		if cy_check > bound_box.cy {
			bound_box.cy = cy_check; // if the check is taller than the font, use its height
		}

		Ok(bound_box)
	}
}

//------------------------------------------------------------------------------

/// Options for [`RadioButton::new`](crate::gui::RadioButton::new).
pub struct RadioButtonOpts {
	/// Text of the radio button to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
	/// Radio button position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to 0x0.
	pub pos: POINT,
	/// Radio button styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `co::BS::AUTORADIOBUTTON`.
	pub button_style: co::BS,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `co::WS_CHILD | co::WS_VISIBLE`.
	///
	/// The first RadioButton of a group should also have `co::WS_TABSTOP | co::WS_GROUP`.
	/// If this object being passed to a [`RadioGroup`](crate::gui::RadioGroup),
	/// this will be automatically set.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `co::WS_EX::LEFT`.
	pub ex_window_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
}

impl Default for RadioButtonOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			pos: POINT { x: 0, y: 0 },
			button_style: co::BS::AUTORADIOBUTTON,
			window_style: co::WS::CHILD | co::WS::VISIBLE,
			ex_window_style: co::WS_EX::LEFT,
			ctrl_id: 0,
		}
	}
}

impl RadioButtonOpts {
	fn define_ctrl_id(&mut self) {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
	}

	pub(crate) fn manual_clone(&self) -> RadioButtonOpts { // avoids a public clone method
		Self {
			text: self.text.clone(),
			pos: self.pos,
			button_style: self.button_style,
			window_style: self.window_style,
			ex_window_style: self.ex_window_style,
			ctrl_id: self.ctrl_id,
		}
	}
}
