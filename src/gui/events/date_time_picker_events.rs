use std::ptr::NonNull;

use crate::co;
use crate::gui::events::MsgEvents;
use crate::gui::traits::Parent;
use crate::structs::{NMDATETIMECHANGE, NMDATETIMEFORMAT, NMDATETIMEFORMATQUERY, NMDATETIMEWMKEYDOWN};

/// Exposes date and time picker control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`MsgEvents`](crate::gui::events::MsgEvents) of the parent window, who is
/// the real responsible for the child event handling.
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

	nfy_event_p! { dtn_date_time_change, co::NM::DTN_DATETIMECHANGE, NMDATETIMECHANGE,
		/// [`DTN_DATETIMECHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-datetimechange)
		/// notification.
	}

	nfy_event! { dtn_drop_down, co::NM::DTN_DROPDOWN,
		/// [`DTN_DROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-dropdown)
		/// notification.
	}

	/// [`DTN_FORMAT`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-format)
	/// notification.
	pub fn dtn_format<F>(&self, func: F)
		where F: FnMut(&mut NMDATETIMEFORMAT) + 'static,
	{
		self.parent_user_events().add_nfy(self.ctrl_id, co::NM::DTN_FORMAT, {
			let mut func = func;
			move |p| {
				func(unsafe { p.cast_nmhdr_mut::<NMDATETIMEFORMAT>() });
				None
			}
		});
	}

	/// [`DTN_FORMATQUERY`](https://docs.microsoft.com/en-us/windows/win32/controls/dtn-formatquery)
	/// notification.
	pub fn dtn_format_query<F>(&self, func: F)
		where F: FnMut(&mut NMDATETIMEFORMATQUERY) + 'static,
	{
		self.parent_user_events().add_nfy(self.ctrl_id, co::NM::DTN_FORMATQUERY, {
			let mut func = func;
			move |p| {
				func(unsafe { p.cast_nmhdr_mut::<NMDATETIMEFORMATQUERY>() });
				None
			}
		});
	}

	nfy_event_p! { dtn_wm_key_down, co::NM::DTN_WMKEYDOWN, NMDATETIMEWMKEYDOWN,
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
