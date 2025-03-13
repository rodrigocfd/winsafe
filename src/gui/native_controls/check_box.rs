use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*, *};
use crate::msg::*;
use crate::prelude::*;

struct CheckBoxObj {
	base: BaseCtrl,
	events: ButtonEvents,
	_pin: PhantomPinned,
}

native_ctrl! { CheckBox: CheckBoxObj => ButtonEvents;
	/// Native
	/// [check box](https://learn.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#check-boxes)
	/// control, actually a variation of the ordinary
	/// [`Button`](crate::gui::Button): just a button with a specific style.
}

impl CheckBox {
	/// Instantiates a new `CheckBox` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `CheckBox` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: CheckBoxOpts) -> Self {
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(Arc::pin(CheckBoxObj {
			base: BaseCtrl::new(ctrl_id),
			events: ButtonEvents::new(parent, ctrl_id),
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent
			.as_ref()
			.before_on()
			.wm(parent.as_ref().is_dlg().create_msg(), move |_| {
				self2.0.base.create_window(
					opts.window_ex_style,
					"BUTTON",
					Some(&opts.text),
					opts.window_style | opts.control_style.into(),
					opts.position.into(),
					if opts.size == (0, 0) {
						text_calc::bound_box_with_check(&text_calc::remove_accel_ampersands(
							&opts.text,
						))?
					} else {
						opts.size.into()
					},
					&parent2,
				)?;
				ui_font::set(self2.hwnd())?;
				parent2
					.as_ref()
					.add_to_layout(self2.hwnd(), opts.resize_behavior)?;
				Ok(0) // ignored
			});

		new_self
	}

	/// Instantiates a new `CheckBox` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `CheckBox` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self {
		let new_self = Self(Arc::pin(CheckBoxObj {
			base: BaseCtrl::new(ctrl_id),
			events: ButtonEvents::new(parent, ctrl_id),
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent.as_ref().before_on().wm_init_dialog(move |_| {
			self2.0.base.assign_dlg(&parent2)?;
			parent2
				.as_ref()
				.add_to_layout(self2.hwnd(), resize_behavior)?;
			Ok(true) // ignored
		});

		new_self
	}

	/// Sends a [`bm::GetCheck`](crate::msg::bm::GetCheck) message and returns
	/// `true` if current state is `co::BST::CHECKED`.
	#[must_use]
	pub fn is_checked(&self) -> bool {
		self.state() == co::BST::CHECKED
	}

	/// Sets or unsets the check mark by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message.
	pub fn set_check(&self, check: bool) {
		self.set_state(if check { co::BST::CHECKED } else { co::BST::UNCHECKED });
	}

	/// Sets the current check state by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message, then sends a
	/// [`wm::Command`](crate::msg::wm::Command) message to the parent, so it
	/// can handle the event.
	pub fn set_check_and_trigger(&self, check: bool) -> SysResult<()> {
		self.set_state_and_trigger(if check { co::BST::CHECKED } else { co::BST::UNCHECKED })
	}

	/// Sets the current check state by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message.
	pub fn set_state(&self, state: co::BST) {
		unsafe {
			self.hwnd().SendMessage(bm::SetCheck { state });
		}
	}

	/// Sets the current check state by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message, then sends a
	/// [`wm::Command`](crate::msg::wm::Command) message to the parent, so it
	/// can handle the event.
	pub fn set_state_and_trigger(&self, state: co::BST) -> SysResult<()> {
		self.set_state(state);
		unsafe {
			self.hwnd().GetParent()?.SendMessage(wm::Command {
				event: AccelMenuCtrl::Ctrl(AccelMenuCtrlData {
					notif_code: co::BN::CLICKED.into(),
					ctrl_id: self.ctrl_id(),
					ctrl_hwnd: self.hwnd().raw_copy(),
				}),
			});
		}
		Ok(())
	}

	/// Calls [`HWND::SetWindowText`](crate::prelude::user_Hwnd::SetWindowText)
	/// to set the text and resizes the control to exactly fit it.
	pub fn set_text_and_resize(&self, text: &str) -> SysResult<()> {
		self.hwnd().SetWindowText(text)?;
		let bound_box = text_calc::bound_box_with_check(&text_calc::remove_accel_ampersands(text))?;
		self.hwnd().SetWindowPos(
			HwndPlace::None,
			POINT::default(),
			bound_box,
			co::SWP::NOZORDER | co::SWP::NOMOVE,
		)?;
		Ok(())
	}

	/// Retrieves the current check state by sending a
	/// [`bm::GetCheck`](crate::msg::bm::GetCheck) message.
	#[must_use]
	pub fn state(&self) -> co::BST {
		unsafe { self.hwnd().SendMessage(bm::GetCheck {}) }
	}

	/// Fires the click event for the check box by sending a
	/// [`bm::Click`](crate::msg::bm::Click) message.
	pub fn trigger_click(&self) {
		unsafe {
			self.hwnd().SendMessage(bm::Click {});
		}
	}
}

/// Options to create a [`CheckBox`](crate::gui::CheckBox) programmatically with
/// [`CheckBox::new`](crate::gui::CheckBox::new).
pub struct CheckBoxOpts {
	/// Text of the control to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Width and height of control to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to the size needed to fit the text.
	pub size: (i32, i32),
	/// Check box styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `BS::AUTOCHECKBOX`.
	///
	/// Suggestions:
	/// * replace with `BS::AUTO3STATE` for a 3-state check box.
	pub control_style: co::BS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::GROUP | WS::TABSTOP | WS::VISIBLE`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	pub window_ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
	/// Horizontal and vertical behavior of the control when the parent window
	/// is resized.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),

	/// Initial check state.
	///
	/// Defaults to `co::BST::UNCHECKED`.
	pub check_state: co::BST,
}

impl Default for CheckBoxOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: dpi(0, 0),
			size: (0, 0), // will resize to fit the text
			control_style: co::BS::AUTOCHECKBOX,
			window_style: co::WS::CHILD | co::WS::GROUP | co::WS::TABSTOP | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			check_state: co::BST::UNCHECKED,
		}
	}
}
