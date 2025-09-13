use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// This trait is enabled with the `gui` feature, and exposes status bar control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-status-bars-reference-notifications).
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
pub trait GuiEventsStatusBar: priv_ctrl_events::GuiEvents {
	fn_nfy_withparm_boolret! { nm_click, co::NM::CLICK, NMMOUSE;
		/// [`NM_CLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-click-status-bar)
		/// notification.
	}

	fn_nfy_withparm_boolret! { nm_dbl_clk, co::NM::DBLCLK, NMMOUSE;
		/// [`NM_DBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-dblclk-status-bar)
		/// notification.
	}

	fn_nfy_withparm_boolret! { nm_rclick, co::NM::RCLICK, NMMOUSE;
		/// [`NM_RCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-rclick-status-bar)
		/// notification.
	}

	fn_nfy_withparm_boolret! { nm_r_dbl_clk, co::NM::RDBLCLK, NMMOUSE;
		/// [`NM_RDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-rdblclk-status-bar)
		/// notification.
	}

	fn_nfy_noparm_noret! { sbn_simple_mode_change, co::SBN::SIMPLEMODECHANGE;
		/// [`SBN_SIMPLEMODECHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/sbn-simplemodechange)
		/// notification.
	}
}

impl GuiEventsStatusBar for BaseCtrlEvents {}
