use crate::co;
use crate::gui::base::Base;
use crate::gui::events::base_events_proxy::BaseEventsProxy;
use crate::structs::{NMDAYSTATE, NMSELCHANGE, NMVIEWCHANGE};

/// Exposes month calendar control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-month-calendar-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct MonthCalendarEvents(BaseEventsProxy);

impl MonthCalendarEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_nfy_ret0_param! { mcn_get_day_state, co::MCN::GETDAYSTATE.into(), NMDAYSTATE,
		/// [`MCN_GETDAYSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/mcn-getdaystate)
		/// notification.
		///
		/// Sent by a month calendar control to request information about how
		/// individual days should be displayed. This notification code is sent
		/// only by month calendar controls that use the
		/// [`MCS_DAYSTATE`](crate::co::MCS::DAYSTATE) style
	}

	pub_fn_nfy_ret0_param! { mcn_sel_change, co::MCN::SELCHANGE.into(), NMSELCHANGE,
		/// [`MCN_GETDAYSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/mcn-selchange)
		/// notification.
		///
		/// Sent by a month calendar control when the currently selected date or
		/// range of dates changes.
	}

	pub_fn_nfy_ret0_param! { mcn_select, co::MCN::SELECT.into(), NMSELCHANGE,
		/// [`MCN_SELECT`](https://docs.microsoft.com/en-us/windows/win32/controls/mcn-select)
		/// notification.
		///
		/// Sent by a month calendar control when the user makes an explicit
		/// date selection within a month calendar control.
	}

	pub_fn_nfy_ret0_param! { mcn_view_change, co::MCN::VIEWCHANGE.into(), NMVIEWCHANGE,
		/// [`MCN_VIEWCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/mcn-viewchange)
		/// notification.
		///
		/// Sent by a month calendar control when the current view changes.
	}

	pub_fn_nfy_ret0! { nm_released_capture, co::NM::RELEASEDCAPTURE,
		/// [`NM_RELEASEDCAPTURE`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-monthcal-)
		/// notification.
		///
		/// Notifies that the control is releasing mouse capture.
	}
}
