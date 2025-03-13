use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*, *};
use crate::msg::*;
use crate::prelude::*;

struct TrackbarObj {
	base: BaseCtrl,
	events: TrackbarEvents,
	_pin: PhantomPinned,
}

native_ctrl! { Trackbar: TrackbarObj => TrackbarEvents;
	/// Native
	/// [trackbar](https://learn.microsoft.com/en-us/windows/win32/controls/trackbar-controls)
	/// control.
}

impl Trackbar {
	/// Instantiates a new `Trackbar` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `Trackbar` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: TrackbarOpts) -> Self {
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(Arc::pin(TrackbarObj {
			base: BaseCtrl::new(ctrl_id),
			events: TrackbarEvents::new(parent, ctrl_id),
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
					"msctls_trackbar32",
					None,
					opts.window_style | opts.control_style.into(),
					opts.position.into(),
					opts.size.into(),
					&parent2,
				)?;
				if opts.range != (0, 100) {
					self2.set_range(opts.range.0, opts.range.1);
				}
				if opts.value != 0 {
					self2.set_pos(opts.value);
				}
				parent2
					.as_ref()
					.add_to_layout(self2.hwnd(), opts.resize_behavior)?;
				Ok(0) // ignored
			});

		new_self
	}

	/// Instantiates a new `Trackbar` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `Trackbar` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self {
		let new_self = Self(Arc::pin(TrackbarObj {
			base: BaseCtrl::new(ctrl_id),
			events: TrackbarEvents::new(parent, ctrl_id),
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

	/// Retrieves the current position by sending a
	/// [`trbm::GetPos`](crate::msg::trbm::GetPos) message.
	#[must_use]
	pub fn pos(&self) -> u32 {
		unsafe { self.hwnd().SendMessage(trbm::GetPos {}) }
	}

	/// Retrieves the minimum and maximum position values by sending
	/// [`trbm::GetRangeMin`](crate::msg::trbm::GetRangeMin) and
	/// [`trbm::GetRangeMax`](crate::msg::trbm::GetRangeMax) messages.
	#[must_use]
	pub fn range(&self) -> (u32, u32) {
		unsafe {
			(
				self.hwnd().SendMessage(trbm::GetRangeMin {}),
				self.hwnd().SendMessage(trbm::GetRangeMax {}),
			)
		}
	}

	/// Sets the current position by sending a
	/// [`trbm::SetPos`](crate::msg::trbm::SetPos) message.
	pub fn set_pos(&self, pos: u32) {
		unsafe {
			self.hwnd().SendMessage(trbm::SetPos { redraw: true, pos });
		}
	}

	/// Sets the minimum and maximum position values by sending
	/// [`trbm::SetRangeMin`](crate::msg::trbm::SetRangeMin) and
	/// [`trbm::SetRangeMax`](crate::msg::trbm::SetRangeMax) messages.
	pub fn set_range(&self, min: u32, max: u32) {
		unsafe {
			self.hwnd()
				.SendMessage(trbm::SetRangeMin { redraw: false, min });
			self.hwnd()
				.SendMessage(trbm::SetRangeMax { redraw: true, max });
		}
	}
}

/// Options to create a [`Trackbar`](crate::gui::Trackbar) programmatically with
/// [`Trackbar::new`](crate::gui::Trackbar::new).
pub struct TrackbarOpts {
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Width and height of control to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(120, 23)`.
	pub size: (i32, i32),
	/// Trackbar styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TBS::HORZ | TBS::AUTOTICKS`.
	pub control_style: co::TBS,
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

	/// The minimum and maximum position values.
	///
	/// Defaults to `(0, 100)`.
	pub range: (u32, u32),
	/// Initial position value.
	///
	/// Defaults to `0`.
	pub value: u32,
}

impl Default for TrackbarOpts {
	fn default() -> Self {
		Self {
			position: dpi(0, 0),
			size: dpi(120, 23),
			control_style: co::TBS::HORZ | co::TBS::AUTOTICKS,
			window_style: co::WS::CHILD | co::WS::GROUP | co::WS::TABSTOP | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			range: (0, 100),
			value: 0,
		}
	}
}
