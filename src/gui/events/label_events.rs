use crate::co;
use crate::gui::privs::*;

/// This trait is enabled with the `gui` feature, and exposes label control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-static-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`GuiEventsParent`](crate::prelude::GuiEventsParent) of the parent window,
/// who is the real responsible for the child event handling.
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait GuiEventsLabel: priv_ctrl_events::GuiEvents {
	fn_cmd_noparm_noret! { stn_clicked, co::STN::CLICKED;
		/// [`STN_CLICKED`](https://learn.microsoft.com/en-us/windows/win32/controls/stn-clicked)
		/// notification.
	}

	fn_cmd_noparm_noret! { stn_dbl_clk, co::STN::DBLCLK;
		/// [`STN_DBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/stn-dblclk)
		/// notification.
	}

	fn_cmd_noparm_noret! { stn_disable, co::STN::DISABLE;
		/// [`STN_DISABLE`](https://learn.microsoft.com/en-us/windows/win32/controls/stn-disable)
		/// notification.
	}

	fn_cmd_noparm_noret! { stn_enable, co::STN::ENABLE;
		/// [`STN_ENABLE`](https://learn.microsoft.com/en-us/windows/win32/controls/stn-enable)
		/// notification.
	}
}

impl GuiEventsLabel for BaseCtrlEvents {}
