use std::ptr::NonNull;

use crate::co;
use crate::gui::events::MsgEvents;
use crate::gui::traits::Parent;

/// Exposes label
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-static-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`MsgEvents`](crate::gui::events::MsgEvents) of the parent window, who is
/// the real responsible for the child event handling.
pub struct LabelEvents {
	parent_user_events: NonNull<MsgEvents>, // used only before parent creation
	ctrl_id: u16,
}

impl LabelEvents {
	pub(crate) fn new(parent: &dyn Parent, ctrl_id: u16) -> LabelEvents {
		Self {
			parent_user_events: NonNull::from(parent.user_events_ref()), // convert reference to pointer
			ctrl_id,
		}
	}

	fn parent_user_events(&self) -> &MsgEvents {
		unsafe { self.parent_user_events.as_ref() }
	}

	cmd_event! { stn_clicked, co::CMD::STN_CLICKED,
		/// [`STN_CLICKED`](https://docs.microsoft.com/en-us/windows/win32/controls/stn-clicked)
		/// notification.
	}

	cmd_event! { stn_dbl_clk, co::CMD::STN_DBLCLK,
		/// [`STN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/stn-dblclk)
		/// notification.
	}

	cmd_event! { stn_disable, co::CMD::STN_DISABLE,
		/// [`STN_DISABLE`](https://docs.microsoft.com/en-us/windows/win32/controls/stn-disable)
		/// notification.
	}

	cmd_event! { stn_enable, co::CMD::STN_ENABLE,
		/// [`STN_ENABLE`](https://docs.microsoft.com/en-us/windows/win32/controls/stn-enable)
		/// notification.
	}
}
