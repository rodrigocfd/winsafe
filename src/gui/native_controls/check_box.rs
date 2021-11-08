use std::any::Any;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::{AccelMenuCtrl, AccelMenuCtrlData, HwndPlace};
use crate::gui::events::{ButtonEvents, EventsView, WindowEvents};
use crate::gui::native_controls::base_native_control::{BaseNativeControl, OptsId};
use crate::gui::privs::{auto_ctrl_id, calc_text_bound_box_check, multiply_dpi, ui_font};
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

/// Possible states of a [`CheckBox`](crate::gui::CheckBox) control.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CheckState {
	/// CheckBox is checked.
	Checked,
	/// CheckBox is grayed, indicating an indeterminate state. Applicable only
	/// if the CheckBox was created with [`BS::R3STATE`](crate::co::BS::R3STATE)
	/// or [`BS::AUTO3STATE`](crate::co::BS::AUTO3STATE) styles.
	Indeterminate,
	/// CheckBox is cleared.
	Unchecked,
}

//------------------------------------------------------------------------------

/// Native
/// [check box](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#check-boxes)
/// control, actually a variation of the ordinary
/// [`Button`](crate::gui::Button): just a button with a specific style.
#[derive(Clone)]
pub struct CheckBox(Arc<Obj>);

struct Obj { // actual fields of CheckBox
	base: BaseNativeControl,
	opts_id: OptsId<CheckBoxOpts>,
	events: ButtonEvents,
}

unsafe impl Send for CheckBox {}

impl AsAny for CheckBox {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl Window for CheckBox {
	fn hwnd(&self) -> HWND {
		self.0.base.hwnd()
	}
}

impl Child for CheckBox {
	fn ctrl_id(&self) -> u16 {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}
}

impl NativeControl for CheckBox {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl NativeControlEvents<ButtonEvents> for CheckBox {
	fn on(&self) -> &ButtonEvents {
		if !self.0.base.hwnd().is_null() {
			panic!("Cannot add events after the control creation.");
		} else if !self.0.base.parent_base().hwnd().is_null() {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl FocusControl for CheckBox {}
impl TextControl for CheckBox {}

impl CheckBox {
	/// Instantiates a new `CheckBox` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &impl Parent, opts: CheckBoxOpts) -> CheckBox {
		let opts = CheckBoxOpts::define_ctrl_id(opts);
		let (ctrl_id, horz, vert) = (opts.ctrl_id, opts.horz_resize, opts.vert_resize);
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Wnd(opts),
					events: ButtonEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm(parent.as_base().wmcreate_or_wminitdialog(), {
			let self2 = new_self.clone();
			move |_| { self2.create(horz, vert)?; Ok(0) }
		});
		new_self
	}

	/// Instantiates a new `CheckBox` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(
		parent: &impl Parent,
		ctrl_id: u16,
		horz_resize: Horz, vert_resize: Vert) -> CheckBox
	{
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Dlg(ctrl_id),
					events: ButtonEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm_init_dialog({
			let self2 = new_self.clone();
			move |_| { self2.create(horz_resize, vert_resize)?; Ok(true) }
		});
		new_self
	}

	fn create(&self, horz: Horz, vert: Vert) -> WinResult<()> {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				multiply_dpi(Some(&mut pos), None)?;

				let mut sz = opts.size;
				if sz.cx == -1 && sz.cy == -1 {
					sz = calc_text_bound_box_check(&opts.text)?; // resize to fit text
				} else {
					multiply_dpi(None, Some(&mut sz))?; // user-defined size
				}

				let our_hwnd = self.0.base.create_window(
					"BUTTON", Some(&opts.text), pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.button_style.into(),
				)?;

				our_hwnd.SendMessage(wm::SetFont { hfont: ui_font(), redraw: true });
				if opts.check_state != CheckState::Unchecked {
					self.set_check_state(opts.check_state);
				}
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ())?,
		}

		self.0.base.parent_base().add_to_resizer(self.hwnd(), horz, vert)
	}

	/// Retrieves the current check state by sending a
	/// [`bm::GetCheck`](crate::msg::bm::GetCheck) message.
	pub fn check_state(&self) -> CheckState {
		match self.hwnd().SendMessage(bm::GetCheck {}) {
			co::BST::CHECKED => CheckState::Checked,
			co::BST::INDETERMINATE => CheckState::Indeterminate,
			_ => CheckState::Unchecked,
		}
	}

	/// Emulates the click event for the check box by sending a
	/// [`bm::Click`](crate::msg::bm::Click) message.
	pub fn emulate_click(&self) {
		self.hwnd().SendMessage(bm::Click {});
	}

	/// Calls [`check_state`](crate::gui::CheckBox::check_state) and compares
	/// the result with
	/// [`CheckState::Checked`](crate::gui::CheckState::Checked).
	pub fn is_checked(&self) -> bool {
		self.check_state() == CheckState::Checked
	}

	/// Sets the current check state by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message.
	pub fn set_check_state(&self, state: CheckState) {
		self.hwnd().SendMessage(bm::SetCheck {
			state: match state {
				CheckState::Checked => co::BST::CHECKED,
				CheckState::Indeterminate => co::BST::INDETERMINATE,
				CheckState::Unchecked => co::BST::UNCHECKED,
			},
		});
	}

	/// Sets the current check state by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message, then sends a
	/// [`wm::Command`](crate::msg::wm::Command) message to the parent, so it
	/// can handle the event.
	pub fn set_check_state_and_trigger(&self, state: CheckState) -> WinResult<()> {
		self.set_check_state(state);
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

/// Options to create a [`CheckBox`](crate::gui::CheckBox) programmatically with
/// [`CheckBox::new`](crate::gui::CheckBox::new).
pub struct CheckBoxOpts {
	/// Text of the control to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
	/// Control position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Control size, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to the size needed to fit the text.
	pub size: SIZE,
	/// Check box styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `BS::AUTOCHECKBOX`.
	///
	/// Suggestions:
	/// * replace with `BS::AUTO3STATE` for a 3-state check box.
	pub button_style: co::BS,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
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

	/// Initial check state.
	///
	/// Defaults to `CheckState::Unchecked`.
	pub check_state: CheckState,
}

impl Default for CheckBoxOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: POINT::new(0, 0),
			size: SIZE::new(-1, -1), // will resize to fit the text
			button_style: co::BS::AUTOCHECKBOX,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			horz_resize: Horz::None,
			vert_resize: Vert::None,
			check_state: CheckState::Unchecked,
		}
	}
}

impl CheckBoxOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
