use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// This trait is enabled with the `gui` feature, and exposes date and time
/// picker control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait GuiEventsDateTimePicker: priv_ctrl_events::GuiEvents {
	fn_nfy_noparm_noret! { dtn_close_up, co::DTN::CLOSEUP;
		/// [`DTN_CLOSEUP`](https://learn.microsoft.com/en-us/windows/win32/controls/dtn-closeup)
		/// notification.
	}

	fn_nfy_withparm_noret! { dtn_date_time_change, co::DTN::DATETIMECHANGE, NMDATETIMECHANGE;
		/// [`DTN_DATETIMECHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/dtn-datetimechange)
		/// notification.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// let dtp: gui::DateTimePicker;
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let dtp = gui::DateTimePicker::new(&wnd, gui::DateTimePickerOpts::default());
		///
		/// dtp.on().dtn_date_time_change(
		///     move |p: &w::NMDATETIMECHANGE| -> w::AnyResult<()> {
		///         println!("{}-{}-{}", p.st.wYear, p.st.wMonth, p.st.wDay);
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	fn_nfy_noparm_noret! { dtn_drop_down, co::DTN::DROPDOWN;
		/// [`DTN_DROPDOWN`](https://learn.microsoft.com/en-us/windows/win32/controls/dtn-dropdown)
		/// notification.
	}

	fn_nfy_withmutparm_noret! { dtn_format, co::DTN::FORMAT, NMDATETIMEFORMAT;
		/// [`DTN_FORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/dtn-format)
		/// notification.
	}

	fn_nfy_withmutparm_noret! { dtn_format_query, co::DTN::FORMATQUERY, NMDATETIMEFORMATQUERY;
		/// [`DTN_FORMATQUERY`](https://learn.microsoft.com/en-us/windows/win32/controls/dtn-formatquery)
		/// notification.
	}

	fn_nfy_withmutparm_noret! { dtn_user_string, co::DTN::USERSTRING, NMDATETIMESTRING;
		/// [`DTN_USERSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/dtn-userstring)
		/// notification.
	}

	fn_nfy_withparm_noret! { dtn_wm_key_down, co::DTN::WMKEYDOWN, NMDATETIMEWMKEYDOWN;
		/// [`DTN_WMKEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/controls/dtn-wmkeydown)
		/// notification.
	}

	fn_nfy_noparm_noret! { nm_kill_focus, co::NM::KILLFOCUS;
		/// [`NM_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-killfocus-date-time)
		/// notification.
	}

	fn_nfy_noparm_noret! { nm_set_focus, co::NM::SETFOCUS;
		/// [`NM_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-setfocus-date-time-)
		/// notification.
	}
}

impl GuiEventsDateTimePicker for BaseCtrlEvents {}
