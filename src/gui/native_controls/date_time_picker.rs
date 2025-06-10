use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*, *};
use crate::msg::*;
use crate::prelude::*;

struct DateTimePickerObj {
	base: BaseCtrl,
	events: DateTimePickerEvents,
	_pin: PhantomPinned,
}

native_ctrl! { DateTimePicker: DateTimePickerObj => DateTimePickerEvents;
	/// Native
	/// [date and time picker](https://learn.microsoft.com/en-us/windows/win32/controls/date-and-time-picker-controls)
	/// control.
}

impl DateTimePicker {
	/// Instantiates a new `DateTimePicker` object, to be created on the parent
	/// window with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `DateTimePicker` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: DateTimePickerOpts) -> Self {
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(Arc::pin(DateTimePickerObj {
			base: BaseCtrl::new(ctrl_id),
			events: DateTimePickerEvents::new(parent, ctrl_id),
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
					"SysDateTimePick32",
					None,
					opts.window_style | opts.control_style.into(),
					opts.position.into(),
					SIZE::with(opts.width, dpi_y(21)),
					&parent2,
				)?;
				if opts.width == 0 {
					let mut sz = SIZE::default();
					unsafe {
						self2.hwnd().SendMessage(dtm::GetIdealSize {
							size: &mut sz, // ask OS the ideal width
						});
					}
					sz.cy = dpi_y(21);
					self2.hwnd().SetWindowPos(
						HwndPlace::None,
						POINT::default(),
						sz,
						co::SWP::NOZORDER | co::SWP::NOMOVE,
					)?;
				}
				ui_font::set(self2.hwnd())?;
				if opts.date.wDay != 0 {
					// user defined a date
					self2.set_date(&opts.date)?;
				}
				parent2
					.as_ref()
					.add_to_layout(self2.hwnd(), opts.resize_behavior)?;
				Ok(0) // ignored
			});

		new_self
	}

	/// Instantiates a new `DateTimePicker` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `DateTimePicker` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self {
		let new_self = Self(Arc::pin(DateTimePickerObj {
			base: BaseCtrl::new(ctrl_id),
			events: DateTimePickerEvents::new(parent, ctrl_id),
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

	/// Retrieves the currently selected date by sending a
	/// [`dtm::GetSystemTime`](crate::msg::dtm::GetSystemTime) message.
	#[must_use]
	pub fn date(&self) -> SysResult<SYSTEMTIME> {
		let mut st = SYSTEMTIME::default();
		unsafe {
			self.hwnd()
				.SendMessage(dtm::GetSystemTime { system_time: &mut st })?;
		}
		Ok(st)
	}

	/// Sets the currently selected date by sending a
	/// [`dtm::SetSystemTime`](crate::msg::dtm::SetSystemTime) message.
	pub fn set_date(&self, st: &SYSTEMTIME) -> SysResult<()> {
		unsafe {
			self.hwnd()
				.SendMessage(dtm::SetSystemTime { system_time: Some(st) })
		}
	}
}

/// Options to create a [`DateTimePicker`](crate::gui::DateTimePicker)
/// programmatically with
/// [`DateTimePicker::new`](crate::gui::DateTimePicker::new).
pub struct DateTimePickerOpts {
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Control width to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to ideal width retrieved with
	/// [`dtm::GetIdealSize`](crate::msg::dtm::GetIdealSize) message, usually
	/// around `gui::dpi_x(250)`.
	pub width: i32,
	/// Date and time picker styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `DTS::LONGDATEFORMAT`.
	pub control_style: co::DTS,
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
	/// **Note:** A `DateTimePicker` cannot be resized vertically, so it will
	/// panic if you use `Vert::Resize`.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),

	/// Initial date to be displayed.
	///
	/// Defaults to now.
	pub date: SYSTEMTIME,
}

impl Default for DateTimePickerOpts {
	fn default() -> Self {
		Self {
			position: dpi(0, 0),
			width: dpi_x(0),
			control_style: co::DTS::LONGDATEFORMAT,
			window_style: co::WS::CHILD | co::WS::GROUP | co::WS::TABSTOP | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			date: SYSTEMTIME::default(), // wDay == 0
		}
	}
}
