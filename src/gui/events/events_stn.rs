use std::ptr::NonNull;

use crate::co;
use crate::gui::events::MsgEvents;
use crate::gui::traits::Parent;

/// Exposes label control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-static-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`MsgEvents`](crate::gui::events::MsgEvents) of the parent window, who is
/// the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
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

	cmd_event! { stn_clicked, co::STN::CLICKED.into(),
		/// [`STN_CLICKED`](https://docs.microsoft.com/en-us/windows/win32/controls/stn-clicked)
		/// notification.
		///
		/// Sent when the user clicks a static control that has the
		/// [`SS_NOTIFY`](crate::co::SS::NOTIFY) style.
	}

	cmd_event! { stn_dbl_clk, co::STN::DBLCLK.into(),
		/// [`STN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/stn-dblclk)
		/// notification.
		///
		/// Sent when the user double-clicks a static control that has the
		/// [`SS_NOTIFY`](crate::co::SS::NOTIFY) style.
	}

	cmd_event! { stn_disable, co::STN::DISABLE.into(),
		/// [`STN_DISABLE`](https://docs.microsoft.com/en-us/windows/win32/controls/stn-disable)
		/// notification.
		///
		/// Sent when a static control is disabled. The static control must have
		/// the [`SS_NOTIFY`](crate::co::SS::NOTIFY) style to receive this
		/// notification code.
	}

	cmd_event! { stn_enable, co::STN::ENABLE.into(),
		/// [`STN_ENABLE`](https://docs.microsoft.com/en-us/windows/win32/controls/stn-enable)
		/// notification.
		///
		/// Sent when a static control is enabled. The static control must have
		/// the [`SS_NOTIFY`](crate::co::SS::NOTIFY) style to receive this
		/// notification code.
	}
}
