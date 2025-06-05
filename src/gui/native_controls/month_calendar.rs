use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*, *};
use crate::msg::*;
use crate::prelude::*;

struct MonthCalendarObj {
	base: BaseCtrl,
	events: MonthCalendarEvents,
	_pin: PhantomPinned,
}

native_ctrl! { MonthCalendar: MonthCalendarObj => MonthCalendarEvents;
	/// Native
	/// [month calendar](https://learn.microsoft.com/en-us/windows/win32/controls/month-calendar-controls)
	/// control.
}

impl MonthCalendar {
	/// Instantiates a new `MonthCalendar` object, to be created on the parent
	/// window with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `MonthCalendar` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: MonthCalendarOpts) -> Self {
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(Arc::pin(MonthCalendarObj {
			base: BaseCtrl::new(ctrl_id),
			events: MonthCalendarEvents::new(parent, ctrl_id),
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
					"SysMonthCal32",
					None,
					opts.window_style | opts.control_style.into(),
					opts.position.into(),
					SIZE::default(),
					&parent2,
				)?;
				ui_font::set(self2.hwnd())?;

				let mut bounds_rect = RECT::default();
				unsafe {
					self2
						.hwnd()
						.SendMessage(mcm::GetMinReqRect { bounds_rect: &mut bounds_rect })?;
				}
				self2.hwnd().SetWindowPos(
					HwndPlace::None,
					POINT::default(),
					SIZE::new(bounds_rect.right, bounds_rect.bottom),
					co::SWP::NOZORDER | co::SWP::NOMOVE,
				)?;

				if opts.date.wDay != 0 {
					self2.set_date(&opts.date)?;
				}
				parent2
					.as_ref()
					.add_to_layout(self2.hwnd(), opts.resize_behavior)?;
				Ok(0) // ignored
			});

		new_self
	}

	/// Instantiates a new `MonthCalendar` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `MonthCalendar` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self {
		let new_self = Self(Arc::pin(MonthCalendarObj {
			base: BaseCtrl::new(ctrl_id),
			events: MonthCalendarEvents::new(parent, ctrl_id),
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
	/// [`mcm::GetCurSel`](crate::msg::mcm::GetCurSel) message.
	#[must_use]
	pub fn date(&self) -> SysResult<SYSTEMTIME> {
		let mut st = SYSTEMTIME::default();
		unsafe {
			self.hwnd().SendMessage(mcm::GetCurSel { info: &mut st })?;
		}
		Ok(st)
	}

	/// Sets the currently selected date by sending a
	/// [`mcm::SetCurSel`](crate::msg::mcm::SetCurSel) message.
	pub fn set_date(&self, st: &SYSTEMTIME) -> SysResult<()> {
		unsafe { self.hwnd().SendMessage(mcm::SetCurSel { info: st }) }
	}
}

/// Options to create a [`MonthCalendar`](crate::gui::MonthCalendar)
/// programmatically with
/// [`MonthCalendar::new`](crate::gui::MonthCalendar::new).
pub struct MonthCalendarOpts {
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Month calendar styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `MCS::NoValue`.
	pub control_style: co::MCS,
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
	/// **Note:** A `MonthCalendar` cannot be resized horizontally or
	/// vertically, so it will panic if you use `Horz::Resize` or
	/// `Vert::Resize`.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),

	/// Initial date.
	///
	/// Defaults to now.
	pub date: SYSTEMTIME,
}

impl Default for MonthCalendarOpts {
	fn default() -> Self {
		Self {
			position: dpi(0, 0),
			control_style: co::MCS::NoValue,
			window_style: co::WS::CHILD | co::WS::GROUP | co::WS::TABSTOP | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			date: SYSTEMTIME::default(),
		}
	}
}
