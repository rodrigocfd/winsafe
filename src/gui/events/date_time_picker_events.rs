use std::ptr::NonNull;

use crate::co;
use crate::gui::events::MsgEvents;
use crate::gui::traits::Parent;
use crate::structs as s;

/// Exposes date and time picker control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`MsgEvents`](crate::gui::events::MsgEvents) of the parent window, who is
/// the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct DateTimePickerEvents {
	parent_user_events: NonNull<MsgEvents>, // used only before parent creation
	ctrl_id: u16,
}

impl DateTimePickerEvents {
	pub(crate) fn new(parent: &dyn Parent, ctrl_id: u16) -> DateTimePickerEvents {
		Self {
			parent_user_events: NonNull::from(parent.user_events_ref()), // convert reference to pointer
			ctrl_id,
		}
	}

	fn parent_user_events(&self) -> &MsgEvents {
		unsafe { self.parent_user_events.as_ref() }
	}

	nfy_event! { dtn_close_up, co::NM::DTN_CLOSEUP,
		/// [`DTN_CLOSEUP`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-closeup)
		/// notification.
	}

	nfy_event_p! { dtn_date_time_change, co::NM::DTN_DATETIMECHANGE, s::NMDATETIMECHANGE,
		/// [`DTN_DATETIMECHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-datetimechange)
		/// notification.
	}

	nfy_event! { dtn_drop_down, co::NM::DTN_DROPDOWN,
		/// [`DTN_DROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-dropdown)
		/// notification.
	}

	nfy_event_mut_p! { dtn_format, co::NM::DTN_FORMAT, s::NMDATETIMEFORMAT,
		/// [`DTN_FORMAT`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-format)
		/// notification.
	}

	nfy_event_mut_p! { dtn_format_query, co::NM::DTN_FORMATQUERY, s::NMDATETIMEFORMATQUERY,
		/// [`DTN_FORMATQUERY`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-formatquery)
		/// notification.
	}

	nfy_event_mut_p! { dtn_user_string, co::NM::DTN_USERSTRING, s::NMDATETIMESTRING,
		/// [`DTN_USERSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-userstring)
		/// notification.
	}

	nfy_event_p! { dtn_wm_key_down, co::NM::DTN_WMKEYDOWN, s::NMDATETIMEWMKEYDOWN,
		/// [`DTN_WMKEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-wmkeydown)
		/// notification.
	}

	nfy_event! { nm_kill_focus, co::NM::KILLFOCUS,
		/// [`NM_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-killfocus-date-time)
		/// notification.
	}

	nfy_event! { nm_set_focus, co::NM::SETFOCUS,
		/// [`NM_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-setfocus-date-time-)
		/// notification.
	}
}
