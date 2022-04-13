use crate::co;
use crate::gui::base::Base;
use crate::gui::events::base_events_proxy::BaseEventsProxy;
use crate::kernel::decl::ErrResult;

/// Exposes list box control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-box-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct ListBoxEvents(BaseEventsProxy);

impl ListBoxEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_cmd_noparm_noret! { lbn_dbl_clk, co::LBN::DBLCLK,
		/// [`LBN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-dblclk)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { lbn_err_space, co::LBN::ERRSPACE,
		/// [`LBN_ERRSPACE`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-errspace)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { lbn_kill_focus, co::LBN::KILLFOCUS,
		/// [`LBN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-killfocus)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { lbn_sel_cancel, co::LBN::SELCANCEL,
		/// [`LBN_SELCANCEL`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-selcancel)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { lbn_sel_change, co::LBN::SELCHANGE,
		/// [`LBN_SELCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-selchange)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { lbn_set_focus, co::LBN::SETFOCUS,
		/// [`LBN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-setfocus)
		/// command notification.
	}
}
