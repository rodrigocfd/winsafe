use crate::co;
use crate::gui::base::Base;
use crate::gui::events::base_events_proxy::BaseEventsProxy;

/// Exposes list box control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-box-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListBoxEvents(BaseEventsProxy);

impl ListBoxEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_cmd_ret0! { lbn_dbl_clk, co::LBN::DBLCLK,
		/// [`LBN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-dblclk)
		/// notification.
		///
		/// Notifies the application that the user has double-clicked an item in
		/// a list box.
	}

	pub_fn_cmd_ret0! { lbn_err_space, co::LBN::ERRSPACE,
		/// [`LBN_ERRSPACE`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-errspace)
		/// notification.
		///
		/// Notifies the application that the list box cannot allocate enough
		/// memory to meet a specific request.
	}

	pub_fn_cmd_ret0! { lbn_kill_focus, co::LBN::KILLFOCUS,
		/// [`LBN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-killfocus)
		/// notification.
		///
		/// Notifies the application that the list box has lost the keyboard focus.
	}

	pub_fn_cmd_ret0! { lbn_sel_cancel, co::LBN::SELCANCEL,
		/// [`LBN_SELCANCEL`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-selcancel)
		/// notification.
		///
		/// Notifies the application that the user has canceled the selection in
		/// a list box.
	}

	pub_fn_cmd_ret0! { lbn_sel_change, co::LBN::SELCHANGE,
		/// [`LBN_SELCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-selchange)
		/// notification.
		///
		/// Notifies the application that the selection in a list box has
		/// changed as a result of user input.
	}

	pub_fn_cmd_ret0! { lbn_set_focus, co::LBN::SETFOCUS,
		/// [`LBN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-setfocus)
		/// notification.
		///
		/// Notifies the application that the list box has received the keyboard
		/// focus.
	}
}
