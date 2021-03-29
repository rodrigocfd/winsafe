use std::ptr::NonNull;

use crate::co;
use crate::gui::base::Base;
use crate::gui::events::WindowEvents;

ctrl_events_proxy! {
	/// Exposes list box control
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-box-control-reference-notifications).
	///
	/// These event methods are just proxies to the
	/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
	/// is the real responsible for the child event handling.
	///
	/// You cannot directly instantiate this object, it is created internally by the
	/// control.
	ListBoxEvents
}

impl ListBoxEvents {
	cmd_event! { lbn_dbl_clk, co::LBN::DBLCLK.into(),
		/// [`LBN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-dblclk)
		/// notification.
		///
		/// Notifies the application that the user has double-clicked an item in a
		/// list box.
	}

	cmd_event! { lbn_err_space, co::LBN::ERRSPACE.into(),
		/// [`LBN_ERRSPACE`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-errspace)
		/// notification.
		///
		/// Notifies the application that the list box cannot allocate enough
		/// memory to meet a specific request.
	}

	cmd_event! { lbn_kill_focus, co::LBN::KILLFOCUS.into(),
		/// [`LBN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-killfocus)
		/// notification.
		///
		/// Notifies the application that the list box has lost the keyboard focus.
	}

	cmd_event! { lbn_sel_cancel, co::LBN::SELCANCEL.into(),
		/// [`LBN_SELCANCEL`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-selcancel)
		/// notification.
		///
		/// Notifies the application that the user has canceled the selection in a
		/// list box.
	}

	cmd_event! { lbn_sel_change, co::LBN::SELCHANGE.into(),
		/// [`LBN_SELCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-selchange)
		/// notification.
		///
		/// Notifies the application that the selection in a list box has changed
		/// as a result of user input.
	}

	cmd_event! { lbn_set_focus, co::LBN::SETFOCUS.into(),
		/// [`LBN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/lbn-setfocus)
		/// notification.
		///
		/// Notifies the application that the list box has received the keyboard
		/// focus.
	}
}
