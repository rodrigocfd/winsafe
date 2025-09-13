use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// This trait is enabled with the `gui` feature, and exposes month calendar control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-month-calendar-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`GuiEventsParent`](crate::prelude::GuiEventsParent) of the parent window,
/// who is the real responsible for the child event handling.
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait GuiEventsMonthCalendar: priv_ctrl_events::GuiEvents {
	fn_nfy_withparm_noret! { mcn_get_day_state, co::MCN::GETDAYSTATE, NMDAYSTATE;
		/// [`MCN_GETDAYSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcn-getdaystate)
		/// notification.
	}

	fn_nfy_withparm_noret! { mcn_sel_change, co::MCN::SELCHANGE, NMSELCHANGE;
		/// [`MCN_SELCHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcn-selchange)
		/// notification.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// let mcal: gui::MonthCalendar;
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let mcal = gui::MonthCalendar::new(&wnd, gui::MonthCalendarOpts::default());
		///
		/// mcal.on().mcn_sel_change(
		///     move |p: &w::NMSELCHANGE| -> w::AnyResult<()> {
		///         let d = &p.stSelStart;
		///         println!("{}-{}-{}", d.wYear, d.wMonth, d.wDay);
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	fn_nfy_withparm_noret! { mcn_select, co::MCN::SELECT, NMSELCHANGE;
		/// [`MCN_SELECT`](https://learn.microsoft.com/en-us/windows/win32/controls/mcn-select)
		/// notification.
	}

	fn_nfy_withparm_noret! { mcn_view_change, co::MCN::VIEWCHANGE, NMVIEWCHANGE;
		/// [`MCN_VIEWCHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcn-viewchange)
		/// notification.
	}

	fn_nfy_noparm_noret! { nm_released_capture, co::NM::RELEASEDCAPTURE;
		/// [`NM_RELEASEDCAPTURE`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-monthcal-)
		/// notification.
	}
}

impl GuiEventsMonthCalendar for BaseCtrlEvents {}
