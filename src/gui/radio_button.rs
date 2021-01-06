use std::cell::UnsafeCell;
use std::sync::Arc;

use crate::co;
use crate::funcs::GetSystemMetrics;
use crate::gui::events::{ButtonEvents, MsgEvents};
use crate::gui::globals::{auto_ctrl_id, calc_text_bound_box, ui_font};
use crate::gui::native_control_base::NativeControlBase;
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;
use crate::msg::WmSetFont;
use crate::structs::{POINT, SIZE};

/// Native
/// [radio button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#radio-buttons)
/// control.
///
/// The radion button is actually a variation of the ordinary
/// [button](crate::gui::Button): just a button with a specific style.
#[derive(Clone)]
pub struct RadioButton {
	obj: Arc<UnsafeCell<Obj>>,
}

struct Obj { // actual fields of RadioButton
	base: NativeControlBase,
	opts: RadioButtonOpts,
	parent_events: ButtonEvents,
}

unsafe impl Send for RadioButton {}
unsafe impl Sync for RadioButton {}

cref_mref!(RadioButton);

impl Child for RadioButton {
	fn create(&self) -> Result<(), co::ERROR> {
		let opts = &self.cref().opts;
		let bound_box = Self::ideal_size(&opts.text)?;

		let our_hwnd = self.mref().base.create_window( // may panic
			"BUTTON", Some(&opts.text), opts.pos, bound_box, opts.ctrl_id,
			opts.ex_window_style,
			opts.window_style | opts.button_style.into(),
		)?;

		our_hwnd.SendMessage(WmSetFont{ hfont: ui_font(), redraw: true });
		Ok(())
	}
}

impl RadioButton {
	/// Creates a new RadioButton object.
	pub fn new<T: Parent>(parent: T, opts: RadioButtonOpts) -> RadioButton {
		let opts = opts.define_id();
		let ctrl_id = opts.ctrl_id;

		Self {
			obj: Arc::new(UnsafeCell::new(
				Obj {
					base: NativeControlBase::new(parent.hwnd_ref()),
					opts,
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
		self.cref().opts.ctrl_id
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
	fn define_id(self) -> RadioButtonOpts {
		let ctrl_id = if self.ctrl_id == 0 {
			auto_ctrl_id() // if user didn't set, auto generate ID
		} else {
			self.ctrl_id
		};
		Self { ctrl_id, ..self }
	}
}
