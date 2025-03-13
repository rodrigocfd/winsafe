use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::gui::{events::*, privs::*, *};
use crate::msg::*;
use crate::prelude::*;

struct ProgressBarObj {
	base: BaseCtrl,
	_pin: PhantomPinned,
}

native_ctrl! { ProgressBar: ProgressBarObj;
	/// Native
	/// [progress bar](https://learn.microsoft.com/en-us/windows/win32/controls/progress-bar-control)
	/// control.
}

impl ProgressBar {
	/// Instantiates a new `ProgressBar` object, to be created on the parent
	/// window with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `ProgressBar` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: ProgressBarOpts) -> Self {
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(Arc::pin(ProgressBarObj {
			base: BaseCtrl::new(ctrl_id),
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
					"msctls_progress32",
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
					self2.set_position(opts.value);
				}
				if opts.state != co::PBST::NORMAL {
					self2.set_state(opts.state);
				}
				parent2
					.as_ref()
					.add_to_layout(self2.hwnd(), opts.resize_behavior)?;
				Ok(0) // ignored
			});

		new_self
	}

	/// Instantiates a new `ProgressBar` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `ProgressBar` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self {
		let new_self = Self(Arc::pin(ProgressBarObj {
			base: BaseCtrl::new(ctrl_id),
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
	/// [`pbm::GetPos`](crate::msg::pbm::GetPos) message.
	#[must_use]
	pub fn position(&self) -> u32 {
		unsafe { self.hwnd().SendMessage(pbm::GetPos {}) }
	}

	/// Retrieves the current minimum and maximum values by sending a
	/// [`pbm::GetRange`](crate::msg::pbm::GetRange) message. Default values are
	/// 0 and 100.
	#[must_use]
	pub fn range(&self) -> (u32, u32) {
		// For some reason, pbm::GetRange is returning all zeros when passing a
		// PBRANGE pointer.
		unsafe {
			let low = self
				.hwnd()
				.SendMessage(pbm::GetRange { return_low: true, ranges: None });
			let high = self
				.hwnd()
				.SendMessage(pbm::GetRange { return_low: false, ranges: None });
			(low as _, high as _)
		}
	}

	/// Sets or unsets the marquee mode by sending a
	/// [`pbm::SetMarquee`](crate::msg::pbm::SetMarquee) message combined with a
	/// [`SetWindowLongPtr`](crate::prelude::user_Hwnd::SetWindowLongPtr) call
	/// for a style change.
	pub fn set_marquee(&self, marquee: bool) {
		if marquee {
			// We must also adjust the window style before/after sending
			// PBM_SETMARQUEE message.
			let cur_style: co::PBS = self.hwnd().style().into();
			self.hwnd().set_style(cur_style | co::PBS::MARQUEE);
		}

		unsafe {
			self.hwnd()
				.SendMessage(pbm::SetMarquee { turn_on: marquee, time_ms: None });
		}

		if !marquee {
			let cur_style: co::PBS = self.hwnd().style().into();
			self.hwnd().set_style(cur_style & !co::PBS::MARQUEE);
		}
	}

	/// Sets the current position by sending a
	/// [`pbm::SetPos`](crate::msg::pbm::SetPos) message, returning the previous
	/// position.
	pub fn set_position(&self, position: u32) -> u32 {
		let cur_style: co::PBS = self.hwnd().style().into();
		if cur_style.has(co::PBS::MARQUEE) {
			self.set_marquee(false); // avoid crash
		}

		unsafe { self.hwnd().SendMessage(pbm::SetPos { position }) }
	}

	/// Sets the minimum and maximum values by sending a
	/// [`pbm::SetRange32`](crate::msg::pbm::SetRange32) message. Default values
	/// are 0 and 100.
	pub fn set_range(&self, min: u32, max: u32) {
		unsafe { self.hwnd().SendMessage(pbm::SetRange32 { min, max }) }
	}

	/// Sets the current state by sending a
	/// [`pbm::SetState`](crate::msg::pbm::SetState) message, retuning the
	/// previous state.
	pub fn set_state(&self, state: co::PBST) -> co::PBST {
		unsafe { self.hwnd().SendMessage(pbm::SetState { state }) }
	}

	/// Retrieves the current state by sending a
	/// [`pbm::GetState`](crate::msg::pbm::GetState) message.
	#[must_use]
	pub fn state(&self) -> co::PBST {
		unsafe { self.hwnd().SendMessage(pbm::GetState {}) }
	}
}

/// Options to create a [`ProgressBar`](crate::gui::ProgressBar)
/// programmatically with [`ProgressBar::new`](crate::gui::ProgressBar::new).
pub struct ProgressBarOpts {
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
	/// Progress bar styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `PBS::SMOOTH`.
	pub control_style: co::PBS,
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

	/// Initial range, min and max values.
	///
	/// Defaults to `(0, 100)`.
	pub range: (u32, u32),
	/// Initial progress value.
	///
	/// Defaults to `0`.
	pub value: u32,
	/// Initial state (normal, error or paused).
	///
	/// Defaults to `PBST::NORMAL`.
	pub state: co::PBST,
}

impl Default for ProgressBarOpts {
	fn default() -> Self {
		Self {
			position: dpi(0, 0),
			size: dpi(120, 23),
			control_style: co::PBS::SMOOTH,
			window_style: co::WS::CHILD | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			range: (0, 100),
			value: 0,
			state: co::PBST::NORMAL,
		}
	}
}
