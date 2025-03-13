use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*, *};
use crate::prelude::*;

struct LabelObj {
	base: BaseCtrl,
	events: LabelEvents,
	_pin: PhantomPinned,
}

native_ctrl! { Label: LabelObj => LabelEvents;
	/// Native
	/// [label](https://learn.microsoft.com/en-us/windows/win32/controls/about-static-controls)
	/// control.
}

impl Label {
	/// Instantiates a new `Label` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `Label` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: LabelOpts) -> Self {
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(Arc::pin(LabelObj {
			base: BaseCtrl::new(ctrl_id),
			events: LabelEvents::new(parent, ctrl_id),
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
					"STATIC",
					Some(&opts.text),
					opts.window_style | opts.control_style.into(),
					opts.position.into(),
					if opts.size == (0, 0) {
						text_calc::bound_box(&text_calc::remove_accel_ampersands(&opts.text))?
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

	/// Instantiates a new `Label` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `Label` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self {
		let new_self = Self(Arc::pin(LabelObj {
			base: BaseCtrl::new(ctrl_id),
			events: LabelEvents::new(parent, ctrl_id),
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

	/// Calls [`HWND::SetWindowText`](crate::prelude::user_Hwnd::SetWindowText)
	/// to set the text and resizes the control to exactly fit it.
	pub fn set_text_and_resize(&self, text: &str) -> SysResult<()> {
		self.hwnd().SetWindowText(text)?;
		let bound_box = text_calc::bound_box(&text_calc::remove_accel_ampersands(text))?;
		self.hwnd().SetWindowPos(
			HwndPlace::None,
			POINT::default(),
			bound_box,
			co::SWP::NOZORDER | co::SWP::NOMOVE,
		)?;
		Ok(())
	}
}

/// Options to create a [`Label`](crate::gui::Label) programmatically with
/// [`Label::new`](crate::gui::Label::new).
pub struct LabelOpts {
	/// Text of the control to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to "Label".
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
	/// Label styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `SS::LEFT | SS:NOTIFY`.
	pub control_style: co::SS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE`.
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
}

impl Default for LabelOpts {
	fn default() -> Self {
		Self {
			text: "Label".to_owned(),
			position: dpi(0, 0),
			size: (0, 0), // will resize to fit the text
			control_style: co::SS::LEFT | co::SS::NOTIFY,
			window_style: co::WS::CHILD | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
		}
	}
}
