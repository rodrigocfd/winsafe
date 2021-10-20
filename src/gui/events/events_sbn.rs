use crate::co;
use crate::gui::base::Base;
use crate::gui::events::base_events_proxy::BaseEventsProxy;
use crate::structs::NMMOUSE;

/// Exposes status bar control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-status-bars-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window,
/// who is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by
/// the control.
pub struct StatusBarEvents(BaseEventsProxy);

impl StatusBarEvents {
	pub(in crate::gui) fn new(parent_base_ref: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base_ref, ctrl_id))
	}

	pub_fn_nfy_retbool_param! { nm_click, co::NM::CLICK, NMMOUSE,
		/// [`NM_CLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-click-status-bar)
		/// notification.
		///
		/// Notifies that the user has clicked the left mouse button within the
		/// control.
	}

	pub_fn_nfy_retbool_param! { nm_dbl_clk, co::NM::DBLCLK, NMMOUSE,
		/// [`NM_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-dblclk-status-bar)
		/// notification.
		///
		/// Notifies that the user has double-clicked the left mouse button
		/// within the control.
	}

	pub_fn_nfy_retbool_param! { nm_rclick, co::NM::RCLICK, NMMOUSE,
		/// [`NM_RCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-rclick-status-bar)
		/// notification.
		///
		/// Notifies that the user has clicked the right mouse button within the
		/// control.
	}

	pub_fn_nfy_retbool_param! { nm_r_dbl_clk, co::NM::RDBLCLK, NMMOUSE,
		/// [`NM_RDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-rdblclk-status-bar)
		/// notification.
		///
		/// Notifies that the user has double-clicked the right mouse button
		/// within the control.
	}

	pub_fn_nfy_ret0! { sbn_simple_mode_change, co::SBN::SIMPLEMODECHANGE.into(),
		/// [`SBN_SIMPLEMODECHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/sbn-simplemodechange)
		/// notification.
		///
		/// Sent by a status bar control when the simple mode changes due to a
		/// [`sb::Simple`](crate::msg::sb::Simple) message.
	}
}
