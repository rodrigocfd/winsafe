use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::funcs::PostQuitMessage;
use crate::gui::events::{MonthCalendarEvents, WindowEvents};
use crate::gui::native_controls::native_control_base::{NativeControlBase, OptsId};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi};
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;
use crate::msg::mcm;
use crate::structs::{POINT, RECT, SIZE};

/// Native
/// [month calendar](https://docs.microsoft.com/en-us/windows/win32/controls/month-calendar-controls)
/// control.
#[derive(Clone)]
pub struct MonthCalendar(Arc<Obj>);

struct Obj { // actual fields of MonthCalendar
	base: NativeControlBase<MonthCalendarEvents>,
	opts_id: OptsId<MonthCalendarOpts>,
}

unsafe impl Send for MonthCalendar {}
unsafe impl Sync for MonthCalendar {}

impl Child for MonthCalendar {
	fn hctrl_ref(&self) -> &HWND {
		self.0.base.hctrl_ref()
	}
}

impl MonthCalendar {
	/// Instantiates a new `MonthCalendar` object, to be created on the parent
	/// window with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: MonthCalendarOpts) -> MonthCalendar {
		let opts = MonthCalendarOpts::define_ctrl_id(opts);
		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(
						parent,
						MonthCalendarEvents::new(parent, opts.ctrl_id),
					),
					opts_id: OptsId::Wnd(opts),
				},
			),
		);
		parent.privileged_events_ref().wm_create({
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});
		new_self
	}

	/// Instantiates a new `MonthCalendar` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> MonthCalendar {
		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(
						parent,
						MonthCalendarEvents::new(parent, ctrl_id),
					),
					opts_id: OptsId::Dlg(ctrl_id),
				},
			),
		);
		parent.privileged_events_ref().wm_init_dialog({
			let me = new_self.clone();
			move |_| { me.create(); true }
		});
		new_self
	}

	fn create(&self) {
		|| -> WinResult<()> {
			match &self.0.opts_id {
				OptsId::Wnd(opts) => {
					let mut pos = opts.position;
					multiply_dpi(Some(&mut pos), None)?;

					let our_hwnd = self.0.base.create_window( // may panic
						"SysMonthCal32", None, pos, SIZE::new(0, 0),
						opts.ctrl_id,
						opts.ex_window_style,
						opts.window_style | opts.month_calendar_style.into(),
					)?;

					let mut bounding_rect = RECT::default();
					our_hwnd.SendMessage(mcm::GetMinReqRect {
						bounding_rect: &mut bounding_rect,
					})?;
					our_hwnd.SetWindowPos(HwndPlace::None, 0, 0,
						bounding_rect.right, bounding_rect.bottom,
						co::SWP::NOZORDER | co::SWP::NOMOVE)?;

					Ok(())
				},
				OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

	hwnd_ctrlid_on_onsubclass!(MonthCalendarEvents);
}

//------------------------------------------------------------------------------

/// Options to create a [`MonthCalendar`](crate::gui::MonthCalendar)
/// programatically with
/// [`MonthCalendar::new`](crate::gui::MonthCalendar::new).
pub struct MonthCalendarOpts {
	/// Control position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Month calendar styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `MCS::NONE`.
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
	pub ex_window_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
}

impl Default for MonthCalendarOpts {
	fn default() -> Self {
		Self {
			position: POINT::new(0, 0),
			ctrl_id: 0,
			month_calendar_style: co::MCS::NONE,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			ex_window_style: co::WS_EX::LEFT,
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
