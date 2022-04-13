use crate::co;
use crate::comctl::decl::{
	NMDATETIMECHANGE, NMDATETIMEFORMAT, NMDATETIMEFORMATQUERY, NMDATETIMESTRING,
	NMDATETIMEWMKEYDOWN,
};
use crate::gui::base::Base;
use crate::gui::events::base_events_proxy::BaseEventsProxy;
use crate::kernel::decl::ErrResult;

/// Exposes date and time picker control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct DateTimePickerEvents(BaseEventsProxy);

impl DateTimePickerEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_nfy_noparm_noret! { dtn_close_up, co::DTN::CLOSEUP,
		/// [`DTN_CLOSEUP`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-closeup)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { dtn_date_time_change, co::DTN::DATETIMECHANGE, NMDATETIMECHANGE,
		/// [`DTN_DATETIMECHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-datetimechange)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { dtn_drop_down, co::DTN::DROPDOWN,
		/// [`DTN_DROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-dropdown)
		/// notification.
	}

	pub_fn_nfy_withmutparm_noret! { dtn_format, co::DTN::FORMAT, NMDATETIMEFORMAT,
		/// [`DTN_FORMAT`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-format)
		/// notification.
	}

	pub_fn_nfy_withmutparm_noret! { dtn_format_query, co::DTN::FORMATQUERY, NMDATETIMEFORMATQUERY,
		/// [`DTN_FORMATQUERY`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-formatquery)
		/// notification.
	}

	pub_fn_nfy_withmutparm_noret! { dtn_user_string, co::DTN::USERSTRING, NMDATETIMESTRING,
		/// [`DTN_USERSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-userstring)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { dtn_wm_key_down, co::DTN::WMKEYDOWN, NMDATETIMEWMKEYDOWN,
		/// [`DTN_WMKEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-wmkeydown)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { nm_kill_focus, co::NM::KILLFOCUS,
		/// [`NM_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-killfocus-date-time)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { nm_set_focus, co::NM::SETFOCUS,
		/// [`NM_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-setfocus-date-time-)
		/// notification.
	}
}
