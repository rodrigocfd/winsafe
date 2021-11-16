use std::any::Any;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::gui::events::{EventsView, MonthCalendarEvents, WindowEvents};
use crate::gui::native_controls::base_native_control::{
	BaseNativeControl,
	OptsId,
};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi_or_dtu};
use crate::gui::resizer::{Horz, Vert};
use crate::gui::traits::{
	AsAny,
	Child,
	FocusControl,
	NativeControl,
	NativeControlEvents,
	Parent,
	Window,
};
use crate::handles::{Handle, HWND};
use crate::msg::mcm;
use crate::structs::{POINT, RECT, SIZE, SYSTEMTIME};

/// Native
/// [month calendar](https://docs.microsoft.com/en-us/windows/win32/controls/month-calendar-controls)
/// control.
#[derive(Clone)]
pub struct MonthCalendar(Arc<Obj>);

struct Obj { // actual fields of MonthCalendar
	base: BaseNativeControl,
	opts_id: OptsId<MonthCalendarOpts>,
	events: MonthCalendarEvents,
}

unsafe impl Send for MonthCalendar {}

impl AsAny for MonthCalendar {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl Window for MonthCalendar {
	fn hwnd(&self) -> HWND {
		self.0.base.hwnd()
	}
}

impl Child for MonthCalendar {
	fn ctrl_id(&self) -> u16 {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}
}

impl NativeControl for MonthCalendar {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl NativeControlEvents<MonthCalendarEvents> for MonthCalendar {
	fn on(&self) -> &MonthCalendarEvents {
		if !self.0.base.hwnd().is_null() {
			panic!("Cannot add events after the control creation.");
		} else if !self.0.base.parent_base().hwnd().is_null() {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl FocusControl for MonthCalendar {}

impl MonthCalendar {
	/// Instantiates a new `MonthCalendar` object, to be created on the parent
	/// window with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &impl Parent, opts: MonthCalendarOpts) -> MonthCalendar {
		let opts = MonthCalendarOpts::define_ctrl_id(opts);
		let (ctrl_id, horz, vert) = (opts.ctrl_id, opts.horz_resize, opts.vert_resize);
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Wnd(opts),
					events: MonthCalendarEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm(parent.as_base().wmcreate_or_wminitdialog(), {
			let self2 = new_self.clone();
			move |_| self2.create(horz, vert)
				.map_err(|e| e.into())
				.map(|_| 0)
		});
		new_self
	}

	/// Instantiates a new `MonthCalendar` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(
		parent: &impl Parent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert)) -> MonthCalendar
	{
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Dlg(ctrl_id),
					events: MonthCalendarEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm_init_dialog({
			let self2 = new_self.clone();
			move |_| self2.create(resize_behavior.0, resize_behavior.1)
				.map_err(|e| e.into())
				.map(|_| true)
		});
		new_self
	}

	fn create(&self, horz: Horz, vert: Vert) -> WinResult<()> {
		if horz == Horz::Resize {
			panic!("MonthCalendar cannot be resized with Horz::Resize.");
		} else if vert == Vert::Resize {
			panic!("MonthCalendar cannot be resized with Vert::Resize.");
		}

		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				multiply_dpi_or_dtu(
					self.0.base.parent_base(), Some(&mut pos), None)?;

				let our_hwnd = self.0.base.create_window(
					"SysMonthCal32", None, pos, SIZE::new(0, 0),
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.month_calendar_style.into(),
				)?;

				let mut bounding_rect = RECT::default();
				our_hwnd.SendMessage(mcm::GetMinReqRect {
					bounding_rect: &mut bounding_rect,
				})?;
				our_hwnd.SetWindowPos(HwndPlace::None, POINT::default(),
					SIZE::new(bounding_rect.right, bounding_rect.bottom),
					co::SWP::NOZORDER | co::SWP::NOMOVE)?;
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ())?,
		}

		self.0.base.parent_base().add_to_resizer(self.hwnd(), horz, vert)
	}

	/// Retrieves the currently selected date by sending a
	/// [`mcm::GetCurSel`](crate::msg::mcm::GetCurSel) message.
	pub fn date(&self, st: &mut SYSTEMTIME) -> WinResult<()> {
		self.hwnd().SendMessage(mcm::GetCurSel { info: st })
	}

	/// Sets the currently selected date by sending a
	/// [`mcm::SetCurSel`](crate::msg::mcm::SetCurSel) message.
	pub fn set_date(&self, st: &SYSTEMTIME) -> WinResult<()> {
		self.hwnd().SendMessage(mcm::SetCurSel { info: st })
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`MonthCalendar`](crate::gui::MonthCalendar)
/// programmatically with
/// [`MonthCalendar::new`](crate::gui::MonthCalendar::new).
pub struct MonthCalendarOpts {
	/// Control position within parent client area, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Month calendar styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `MCS::NoValue`.
	pub month_calendar_style: co::MCS,
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
	///
	/// **Note:** A `MonthCalendar` cannot be resized horizontally, so it will
	/// panic if you use `Horz::Resize`.
	pub horz_resize: Horz,
	/// Vertical behavior when the parent is resized.
	///
	/// Defaults to `Vert::None`.
	///
	/// **Note:** A `MonthCalendar` cannot be resized vertically, so it will
	/// panic if you use `Vert::Resize`.
	pub vert_resize: Vert,
}

impl Default for MonthCalendarOpts {
	fn default() -> Self {
		Self {
			position: POINT::new(0, 0),
			month_calendar_style: co::MCS::NoValue,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			horz_resize: Horz::None,
			vert_resize: Vert::None,
		}
	}
}

impl MonthCalendarOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
