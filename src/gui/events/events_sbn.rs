use std::ptr::NonNull;

use crate::co;
use crate::gui::events::WindowEvents;
use crate::gui::traits::Parent;
use crate::structs::NMMOUSE;

/// Exposes status bar control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-status-bars-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct StatusBarEvents {
	parent_user_events: NonNull<WindowEvents>, // used only before parent creation
	ctrl_id: u16,
}

impl StatusBarEvents {
	pub(crate) fn new(parent: &dyn Parent, ctrl_id: u16) -> StatusBarEvents {
		Self {
			parent_user_events: NonNull::from(parent.user_events_ref()), // convert reference to pointer
			ctrl_id,
		}
	}

	fn parent_user_events(&self) -> &WindowEvents {
		unsafe { self.parent_user_events.as_ref() }
	}

	nfy_event_p_bool! { nm_click, co::NM::CLICK, NMMOUSE,
		/// [`NM_CLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-click-status-bar)
		/// notification.
		///
		/// Notifies that the user has clicked the left mouse button within the
		/// control.
	}

	nfy_event_p_bool! { nm_dbl_clk, co::NM::DBLCLK, NMMOUSE,
		/// [`NM_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-dblclk-status-bar)
		/// notification.
		///
		/// Notifies that the user has double-clicked the left mouse button within
		/// the control.
	}

	nfy_event_p_bool! { nm_rclick, co::NM::RCLICK, NMMOUSE,
		/// [`NM_RCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-rclick-status-bar)
		/// notification.
		///
		/// Notifies that the user has clicked the right mouse button within the
		/// control.
	}

	nfy_event_p_bool! { nm_r_dbl_clk, co::NM::RDBLCLK, NMMOUSE,
		/// [`NM_RDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-rdblclk-status-bar)
		/// notification.
		///
		/// Notifies that the user has double-clicked the right mouse button
		/// within the control.
	}

	nfy_event! { sbn_simple_mode_change, co::SBN::SIMPLEMODECHANGE.into(),
		/// [`SBN_SIMPLEMODECHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/sbn-simplemodechange)
		/// notification.
		///
		/// Sent by a status bar control when the simple mode changes due to a
		/// [`SB_SIMPLE`](crate::msg::sb::Simple) message.
	}
}
