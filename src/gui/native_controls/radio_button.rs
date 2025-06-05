use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*, *};
use crate::msg::*;
use crate::prelude::*;

struct RadioButtonObj {
	base: BaseCtrl,
	events: ButtonEvents,
	_pin: PhantomPinned,
}

native_ctrl! { RadioButton: RadioButtonObj => ButtonEvents;
	/// Native
	/// [radio button](https://learn.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#radio-buttons)
	/// control.
	///
	/// You cannot directly instantiate this object, you must use
	/// [`RadioGroup`](crate::gui::RadioGroup).
}

impl RadioButton {
	#[must_use]
	pub(in crate::gui) fn new(
		parent: &(impl GuiParent + 'static),
		opts: RadioButtonOpts,
		is_first: bool,
	) -> Self {
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(Arc::pin(RadioButtonObj {
			base: BaseCtrl::new(ctrl_id),
			events: ButtonEvents::new(parent, ctrl_id),
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent
			.as_ref()
			.before_on()
			.wm(parent.as_ref().wnd_ty().creation_msg(), move |_| {
				self2.0.base.create_window(
					opts.window_ex_style,
					"BUTTON",
					Some(&opts.text),
					if is_first {
						opts.window_style | co::WS::GROUP | co::WS::TABSTOP
					} else {
						opts.window_style & !(co::WS::GROUP | co::WS::TABSTOP)
					} | opts.control_style.into(),
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
				if opts.selected {
					self2.select(true);
				}
				parent2
					.as_ref()
					.add_to_layout(self2.hwnd(), opts.resize_behavior)?;
				Ok(0) // ignored
			});

		new_self
	}

	#[must_use]
	pub(in crate::gui) fn new_dlg(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		is_first: bool,
		resize_behavior: (Horz, Vert),
	) -> Self {
		let new_self = Self(Arc::pin(RadioButtonObj {
			base: BaseCtrl::new(ctrl_id),
			events: ButtonEvents::new(parent, ctrl_id),
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent.as_ref().before_on().wm_init_dialog(move |_| {
			self2.0.base.assign_dlg(&parent2)?;
			self2.hwnd().set_style(if is_first {
				self2.hwnd().style() | co::WS::GROUP | co::WS::TABSTOP
			} else {
				self2.hwnd().style() & !(co::WS::GROUP | co::WS::TABSTOP)
			});
			parent2
				.as_ref()
				.add_to_layout(self2.hwnd(), resize_behavior)?;
			Ok(true) // ignored
		});

		new_self
	}

	/// Tells if this radio button is the currently selected one by sending a
	/// [`bm::GetCheck`](crate::msg::bm::GetCheck) message.
	#[must_use]
	pub fn is_selected(&self) -> bool {
		unsafe { self.hwnd().SendMessage(bm::GetCheck {}) == co::BST::CHECKED }
	}

	/// Sets the this radio button as the currently selected one by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message.
	pub fn select(&self, selected: bool) {
		unsafe {
			self.hwnd().SendMessage(bm::SetCheck {
				state: if selected { co::BST::CHECKED } else { co::BST::UNCHECKED },
			});
		}
	}

	/// Sets the this radio button as the currently selected one by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message, then sends a
	/// [`wm::Command`](crate::msg::wm::Command) message to the parent, so it
	/// can handle the event.
	pub fn select_and_trigger(&self, selected: bool) -> SysResult<()> {
		self.select(selected);
		unsafe {
			self.hwnd().GetParent()?.SendMessage(wm::Command {
				event: AccelMenuCtrl::Ctrl {
					notif_code: co::BN::CLICKED.into(),
					ctrl_id: self.ctrl_id(),
					ctrl_hwnd: self.hwnd().raw_copy(),
				},
			});
		}
		Ok(())
	}

	/// Calls [`HWND::SetWindowText`](crate::HWND::SetWindowText) to set the
	/// text and resizes the control to exactly fit it.
	pub fn set_text_and_resize(&self, text: &str) -> SysResult<()> {
		let bound_box = text_calc::bound_box_with_check(&text_calc::remove_accel_ampersands(text))?;
		self.hwnd().SetWindowPos(
			HwndPlace::None,
			POINT::default(),
			bound_box,
			co::SWP::NOZORDER | co::SWP::NOMOVE,
		)?;
		self.hwnd().SetWindowText(text)?;
		Ok(())
	}

	/// Fires the click event for the button by sending a
	/// [`bm::Click`](crate::msg::bm::Click) message.
	pub fn trigger_click(&self) {
		unsafe {
			self.hwnd().SendMessage(bm::Click {});
		}
	}
}

/// Options to create a [`RadioButton`](crate::gui::RadioButton)
/// programmatically with [`RadioGroup::new`](crate::gui::RadioGroup::new).
#[derive(Clone)]
pub struct RadioButtonOpts {
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
	/// Radio button styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `BS::AUTORADIOBUTTON`.
	pub control_style: co::BS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE`.
	///
	/// The first RadioButton of a group will also have `WS::TABSTOP |
	/// WS::GROUP`. This will be automatically set by the owning
	/// [`RadioGroup`](crate::gui::RadioGroup).
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

	/// Initial selection state.
	///
	/// Defaults to `false`.
	pub selected: bool,
}

impl Default for RadioButtonOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: (0, 0),
			size: (0, 0), // will resize to fit the text
			control_style: co::BS::AUTORADIOBUTTON,
			window_style: co::WS::CHILD | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			selected: false,
		}
	}
}
