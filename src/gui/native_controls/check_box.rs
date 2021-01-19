use std::sync::Arc;

use crate::co;
use crate::gui::events::{ButtonEvents, MsgEvents};
use crate::gui::native_controls::native_control_base::{NativeControlBase, OptsId};
use crate::gui::privs::{auto_ctrl_id, calc_text_bound_box_check, multiply_dpi, ui_font};
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;
use crate::msg::{BmClick, BmGetCheck, BmSetCheck, WmSetFont};
use crate::structs::POINT;

/// Native
/// [check box](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#check-boxes)
/// control.
///
/// The check box is actually a variation of the ordinary
/// [`Button`](crate::gui::Button): just a button with a specific style.
#[derive(Clone)]
pub struct CheckBox {
	base: Arc<
		NativeControlBase<ButtonEvents, CheckBoxOpts>,
	>,
}

unsafe impl Send for CheckBox {}
unsafe impl Sync for CheckBox {}

impl Child for CheckBox {
	fn hctrl_ref(&self) -> &HWND {
		self.base.hctrl_ref()
	}
}

impl CheckBox {
	/// Instantiates a new `CheckBox` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: CheckBoxOpts) -> CheckBox {
		let opts = CheckBoxOpts::define_ctrl_id(opts);
		let me = Self {
			base: Arc::new(
				NativeControlBase::new(
					parent,
					ButtonEvents::new(parent, opts.ctrl_id),
					OptsId::Wnd(opts),
				),
			),
		};
		me.add_creation_to_parent(parent);
		me
	}

	/// Instantiates a new `CheckBox` object, to be loaded from a dialog resource
	/// with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> CheckBox {
		let me = Self {
			base: Arc::new(
				NativeControlBase::new(
					parent,
					ButtonEvents::new(parent, ctrl_id),
					OptsId::Dlg(ctrl_id),
				),
			),
		};
		me.add_creation_to_parent(parent);
		me
	}

	fn add_creation_to_parent(&self, parent: &dyn Parent) {
		let me = self.clone();
		parent.add_child_to_be_created(
			Box::new(move || {
				match me.base.opts_id() {
					OptsId::Wnd(opts) => {
						let mut pos = opts.position;
						multiply_dpi(Some(&mut pos), None)?;

						let bound_box = calc_text_bound_box_check(&opts.text)?;

						let our_hwnd = me.base.create_window( // may panic
							"BUTTON", Some(&opts.text), pos, bound_box,
							opts.ctrl_id,
							opts.ex_window_style,
							opts.window_style | opts.button_style.into(),
						)?;

						our_hwnd.SendMessage(WmSetFont{ hfont: ui_font(), redraw: true });
						Ok(())
					},
					OptsId::Dlg(ctrl_id) => me.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
				}
			})
		)
	}

	/// Returns the underlying handle for this control.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is created.
	pub fn hwnd(&self) -> HWND {
		*self.hctrl_ref()
	}

	/// Returns the control ID.
	pub fn ctrl_id(&self) -> u16 {
		match self.base.opts_id() {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}

	/// Exposes the check box events.
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	pub fn on(&self) -> &ButtonEvents {
		self.base.on()
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

	/// Tells if this check box is currently checked.
	pub fn is_checked(&self) -> bool {
		self.hwnd().SendMessage(BmGetCheck {}) == co::BST::CHECKED
	}

	/// Sets the current check state.
	pub fn set_check(&self, checked: bool) {
		self.hwnd().SendMessage(BmSetCheck {
			state: if checked { co::BST::CHECKED } else { co::BST::UNCHECKED },
		});
	}

	/// Fires the click event for the radio button. The event is asynchronous,
	/// the method returns immediately.
	pub fn trigger_click(&self) {
		self.hwnd().PostMessage(BmClick {}).unwrap();
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`CheckBox`](crate::gui::CheckBox) programatically with
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
	pub ex_window_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
}

impl Default for CheckBoxOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: POINT::new(0, 0),
			button_style: co::BS::AUTOCHECKBOX,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			ex_window_style: co::WS_EX::LEFT,
			ctrl_id: 0,
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
