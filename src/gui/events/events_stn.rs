use std::ptr::NonNull;

use crate::co;
use crate::gui::base::Base;
use crate::gui::events::WindowEvents;

pub_struct_ctrl_events_proxy! {
	/// Exposes label control
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-static-control-reference-notifications).
	///
	/// These event methods are just proxies to the
	/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window,
	/// who is the real responsible for the child event handling.
	///
	/// You cannot directly instantiate this object, it is created internally by
	/// the control.
	LabelEvents
}

impl LabelEvents {
	pub_fn_cmd_ret0! { stn_clicked, co::STN::CLICKED.into(),
		/// [`STN_CLICKED`](https://docs.microsoft.com/en-us/windows/win32/controls/stn-clicked)
		/// notification.
		///
		/// Sent when the user clicks a static control that has the
		/// [`SS_NOTIFY`](crate::co::SS::NOTIFY) style.
	}

	pub_fn_cmd_ret0! { stn_dbl_clk, co::STN::DBLCLK.into(),
		/// [`STN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/stn-dblclk)
		/// notification.
		///
		/// Sent when the user double-clicks a static control that has the
		/// [`SS_NOTIFY`](crate::co::SS::NOTIFY) style.
	}

	pub_fn_cmd_ret0! { stn_disable, co::STN::DISABLE.into(),
		/// [`STN_DISABLE`](https://docs.microsoft.com/en-us/windows/win32/controls/stn-disable)
		/// notification.
		///
		/// Sent when a static control is disabled. The static control must have
		/// the [`SS_NOTIFY`](crate::co::SS::NOTIFY) style to receive this
		/// notification code.
	}

	pub_fn_cmd_ret0! { stn_enable, co::STN::ENABLE.into(),
		/// [`STN_ENABLE`](https://docs.microsoft.com/en-us/windows/win32/controls/stn-enable)
		/// notification.
		///
		/// Sent when a static control is enabled. The static control must have
		/// the [`SS_NOTIFY`](crate::co::SS::NOTIFY) style to receive this
		/// notification code.
	}
}
