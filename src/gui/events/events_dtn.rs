use std::ptr::NonNull;

use crate::co;
use crate::gui::base::Base;
use crate::gui::events::WindowEvents;
use crate::structs::{
	NMDATETIMECHANGE,
	NMDATETIMEFORMAT,
	NMDATETIMEFORMATQUERY,
	NMDATETIMESTRING,
	NMDATETIMEWMKEYDOWN,
};

ctrl_events_proxy! {
	/// Exposes date and time picker control
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-notifications).
	///
	/// These event methods are just proxies to the
	/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window,
	/// who is the real responsible for the child event handling.
	///
	/// You cannot directly instantiate this object, it is created internally by
	/// the control.
	DateTimePickerEvents
}

impl DateTimePickerEvents {
	nfy_event! { dtn_close_up, co::DTN::CLOSEUP.into(),
		/// [`DTN_CLOSEUP`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-closeup)
		/// notification.
		///
		/// Sent by a date and time picker control when the user closes the
		/// drop-down month calendar. The month calendar is closed when the user
		/// chooses a date from the month calendar or clicks the drop-down arrow
		/// while the calendar is open.
	}

	nfy_event_p! { dtn_date_time_change, co::DTN::DATETIMECHANGE.into(), NMDATETIMECHANGE,
		/// [`DTN_DATETIMECHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-datetimechange)
		/// notification.
		///
		/// Sent by a date and time picker control whenever a change occurs.
	}

	nfy_event! { dtn_drop_down, co::DTN::DROPDOWN.into(),
		/// [`DTN_DROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-dropdown)
		/// notification.
		///
		/// Sent by a date and time picker control when the user activates the
		/// drop-down month calendar.
	}

	nfy_event_mut_p! { dtn_format, co::DTN::FORMAT.into(), NMDATETIMEFORMAT,
		/// [`DTN_FORMAT`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-format)
		/// notification.
		///
		/// Sent by a date and time picker control to request text to be
		/// displayed in a callback field.
	}

	nfy_event_mut_p! { dtn_format_query, co::DTN::FORMATQUERY.into(), NMDATETIMEFORMATQUERY,
		/// [`DTN_FORMATQUERY`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-formatquery)
		/// notification.
		///
		/// Sent by a date and time picker control to retrieve the maximum
		/// allowable size of the string that will be displayed in a callback
		/// field.
	}

	nfy_event_mut_p! { dtn_user_string, co::DTN::USERSTRING.into(), NMDATETIMESTRING,
		/// [`DTN_USERSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-userstring)
		/// notification.
		///
		/// Sent by a date and time picker (DTP) control when a user finishes
		/// editing a string in the control. This notification code is only sent
		/// by DTP controls that are set to the
		/// [`DTS_APPCANPARSE`](crate::co::DTS::APPCANPARSE) style.
	}

	nfy_event_p! { dtn_wm_key_down, co::DTN::WMKEYDOWN.into(), NMDATETIMEWMKEYDOWN,
		/// [`DTN_WMKEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-wmkeydown)
		/// notification.
		///
		/// Sent by a date and time picker control when the user types in a
		/// callback field.
	}

	nfy_event! { nm_kill_focus, co::NM::KILLFOCUS,
		/// [`NM_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-killfocus-date-time)
		/// notification.
		///
		/// Notifies that the control has lost the input focus.
	}

	nfy_event! { nm_set_focus, co::NM::SETFOCUS,
		/// [`NM_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-setfocus-date-time-)
		/// notification.
		///
		/// Notifies that the control has received the input focus.
	}
}
