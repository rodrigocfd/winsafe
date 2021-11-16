use std::any::Any;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::{AccelMenuCtrl, AccelMenuCtrlData, HwndPlace};
use crate::gui::events::{ButtonEvents, WindowEvents};
use crate::gui::native_controls::base_native_control::{
	BaseNativeControl,
	OptsId,
};
use crate::gui::privs::{
	auto_ctrl_id,
	calc_text_bound_box_check,
	multiply_dpi_or_dtu,
	ui_font,
};
use crate::gui::resizer::{Horz, Vert};
use crate::gui::traits::{
	AsAny,
	Child,
	FocusControl,
	NativeControl,
	NativeControlEvents,
	Parent,
	TextControl,
	Window,
};
use crate::handles::{Handle, HWND};
use crate::msg::{bm, wm};
use crate::structs::{POINT, SIZE};

/// Native
/// [radio button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#radio-buttons)
/// control, actually a variation of the ordinary
/// [`Button`](crate::gui::Button): just a button with a specific style.
///
/// You cannot directly instantiate this object, you must use
/// [`RadioGroup`](crate::gui::RadioGroup).
pub struct RadioButton(Obj);

struct Obj { // actual fields of RadioButton
	base: BaseNativeControl,
	opts_id: OptsId<RadioButtonOpts>,
	events: ButtonEvents,
}

impl AsAny for RadioButton {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl Window for RadioButton {
	fn hwnd(&self) -> HWND {
		self.0.base.hwnd()
	}
}

impl Child for RadioButton {
	fn ctrl_id(&self) -> u16 {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}
}

impl NativeControl for RadioButton {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl NativeControlEvents<ButtonEvents> for RadioButton {
	fn on(&self) -> &ButtonEvents {
		if !self.hwnd().is_null() {
			panic!("Cannot add events after the control creation.");
		} else if !self.0.base.parent_base().hwnd().is_null() {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl FocusControl for RadioButton {}
impl TextControl for RadioButton {}

impl RadioButton {
	pub(in crate::gui) fn new(
		parent: &impl Parent, opts: RadioButtonOpts) -> RadioButton
	{
		let opts = RadioButtonOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;
		Self(
			Obj {
				base: BaseNativeControl::new(parent.as_base()),
				opts_id: OptsId::Wnd(opts),
				events: ButtonEvents::new(parent.as_base(), ctrl_id),
			},
		)
	}

	pub(in crate::gui) fn new_dlg(
		parent: &impl Parent,
		ctrl_id: u16) -> RadioButton
	{
		Self(
			Obj {
				base: BaseNativeControl::new(parent.as_base()),
				opts_id: OptsId::Dlg(ctrl_id),
				events: ButtonEvents::new(parent.as_base(), ctrl_id),
			},
		)
	}

	pub(in crate::gui) fn create(&self, horz: Horz, vert: Vert) -> WinResult<()> {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				multiply_dpi_or_dtu(
					self.0.base.parent_base(), Some(&mut pos), None)?;

				let mut sz = opts.size;
					if sz.cx == -1 && sz.cy == -1 {
						sz = calc_text_bound_box_check(&opts.text)?; // resize to fit text
					} else {
						multiply_dpi_or_dtu(
							self.0.base.parent_base(), None, Some(&mut sz))?; // user-defined size
					}

				let our_hwnd = self.0.base.create_window( // may panic
					"BUTTON", Some(&opts.text), pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.button_style.into(),
				)?;

				our_hwnd.SendMessage(wm::SetFont { hfont: ui_font(), redraw: true });
				if opts.selected { self.select(true); }
			},
			OptsId::Dlg(ctrl_id) => {
				self.0.base.create_dlg(*ctrl_id)?; // may panic
			},
		}

		self.0.base.parent_base().add_to_resizer(self.hwnd(), horz, vert)?;
		self.hwnd().SendMessage(bm::SetDontClick { dont_click: true });
		Ok(())
	}

	/// Emulates the click event for the radio button by sending a
	/// [`bm::Click`](crate::msg::bm::Click) message.
	pub fn emulate_click(&self) {
		self.hwnd().SendMessage(bm::Click {});
	}

	/// Tells if this radio button is the currently selected one by sending a
	/// [`bm::GetCheck`](crate::msg::bm::GetCheck) message.
	pub fn is_selected(&self) -> bool {
		self.hwnd().SendMessage(bm::GetCheck {}) == co::BST::CHECKED
	}

	/// Sets the this radio button as the currently selected one by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message.
	pub fn select(&self, selected: bool) {
		self.hwnd().SendMessage(bm::SetCheck {
			state: if selected { co::BST::CHECKED } else { co::BST::UNCHECKED },
		});
	}

	/// Sets the this radio button as the currently selected one by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message, then sends a
	/// [`wm::Command`](crate::msg::wm::Command) message to the parent, so it
	/// can handle the event.
	pub fn select_and_trigger(&self, selected: bool) -> WinResult<()> {
		self.select(selected);
		self.hwnd().GetParent()?.SendMessage(wm::Command {
			event: AccelMenuCtrl::Ctrl(
				AccelMenuCtrlData {
					notif_code: co::BN::CLICKED.into(),
					ctrl_id: self.ctrl_id(),
					ctrl_hwnd: self.hwnd(),
				},
			),
		});
		Ok(())
	}

	/// Calls [`set_text`](crate::gui::TextControl::set_text) and resizes the
	/// control to exactly fit the new text.
	pub fn set_text_and_resize(&self, text: &str) -> WinResult<()> {
		self.set_text(text)?;
		let bound_box = calc_text_bound_box_check(text)?;
		self.hwnd().SetWindowPos(
			HwndPlace::None, POINT::default(), bound_box,
			co::SWP::NOZORDER | co::SWP::NOMOVE)
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`RadioButton`](crate::gui::RadioButton)
/// programmatically with [`RadioGroup::new`](crate::gui::RadioGroup::new).
pub struct RadioButtonOpts {
	/// Text of the control to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
	/// Control position within parent client area, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Control size, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to the size needed to fit the text.
	pub size: SIZE,
	/// Radio button styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `BS::AUTORADIOBUTTON`.
	pub button_style: co::BS,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE`.
	///
	/// The first RadioButton of a group should also have `WS::TABSTOP | WS::GROUP`.
	/// If this object being passed to a [`RadioGroup`](crate::gui::RadioGroup),
	/// this will be automatically set.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	pub window_ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
	/// Horizontal behavior when the parent is resized.
	///
	/// Defaults to `Horz::None`.
	pub horz_resize: Horz,
	/// Vertical behavior when the parent is resized.
	///
	/// Defaults to `Vert::None`.
	pub vert_resize: Vert,

	/// Initial selection state.
	///
	/// Defaults to `false`.
	pub selected: bool,
}

impl Default for RadioButtonOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: POINT::new(0, 0),
			size: SIZE::new(-1, -1), // will resize to fit the text
			button_style: co::BS::AUTORADIOBUTTON,
			window_style: co::WS::CHILD | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			horz_resize: Horz::None,
			vert_resize: Vert::None,
			selected: false,
		}
	}
}

impl RadioButtonOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}

	pub(in crate::gui) fn manual_clone(&self) -> RadioButtonOpts { // avoids a public clone method
		Self {
			text: self.text.clone(),
			position: self.position,
			size: self.size,
			button_style: self.button_style,
			window_style: self.window_style,
			window_ex_style: self.window_ex_style,
			ctrl_id: self.ctrl_id,
			horz_resize: self.horz_resize,
			vert_resize: self.vert_resize,
			selected: self.selected,
		}
	}
}