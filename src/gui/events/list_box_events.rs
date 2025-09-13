use crate::co;
use crate::gui::privs::*;

/// This trait is enabled with the `gui` feature, and exposes list box control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-list-box-control-reference-notifications).
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
pub trait GuiEventsListBox: priv_ctrl_events::GuiEvents {
	fn_cmd_noparm_noret! { lbn_dbl_clk, co::LBN::DBLCLK;
		/// [`LBN_DBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/lbn-dblclk)
		/// command notification.
	}

	fn_cmd_noparm_noret! { lbn_err_space, co::LBN::ERRSPACE;
		/// [`LBN_ERRSPACE`](https://learn.microsoft.com/en-us/windows/win32/controls/lbn-errspace)
		/// command notification.
	}

	fn_cmd_noparm_noret! { lbn_kill_focus, co::LBN::KILLFOCUS;
		/// [`LBN_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/lbn-killfocus)
		/// command notification.
	}

	fn_cmd_noparm_noret! { lbn_sel_cancel, co::LBN::SELCANCEL;
		/// [`LBN_SELCANCEL`](https://learn.microsoft.com/en-us/windows/win32/controls/lbn-selcancel)
		/// command notification.
	}

	fn_cmd_noparm_noret! { lbn_sel_change, co::LBN::SELCHANGE;
		/// [`LBN_SELCHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/lbn-selchange)
		/// command notification.
	}

	fn_cmd_noparm_noret! { lbn_set_focus, co::LBN::SETFOCUS;
		/// [`LBN_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/lbn-setfocus)
		/// command notification.
	}
}

impl GuiEventsListBox for BaseCtrlEvents {}
