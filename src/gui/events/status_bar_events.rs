use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// Exposes status bar control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-status-bars-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window,
/// who is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by
/// the control.
pub struct StatusBarEvents(BaseCtrlEventsProxy);

impl StatusBarEvents {
	#[must_use]
	pub(in crate::gui) fn new(parent: &impl AsRef<Base>, ctrl_id: u16) -> Self {
		Self(BaseCtrlEventsProxy::new(parent, ctrl_id))
	}

	pub_fn_nfy_withparm_boolret! { nm_click, co::SBN::NM_CLICK, NMMOUSE;
		/// [`NM_CLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-click-status-bar)
		/// notification.
	}

	pub_fn_nfy_withparm_boolret! { nm_dbl_clk, co::SBN::NM_DBLCLK, NMMOUSE;
		/// [`NM_DBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-dblclk-status-bar)
		/// notification.
	}

	pub_fn_nfy_withparm_boolret! { nm_rclick, co::SBN::NM_RCLICK, NMMOUSE;
		/// [`NM_RCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-rclick-status-bar)
		/// notification.
	}

	pub_fn_nfy_withparm_boolret! { nm_r_dbl_clk, co::SBN::NM_RDBLCLK, NMMOUSE;
		/// [`NM_RDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-rdblclk-status-bar)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { sbn_simple_mode_change, co::SBN::SIMPLEMODECHANGE;
		/// [`SBN_SIMPLEMODECHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/sbn-simplemodechange)
		/// notification.
	}
}
