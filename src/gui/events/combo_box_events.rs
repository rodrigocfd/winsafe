use std::ptr::NonNull;

use crate::co;
use crate::gui::events::MsgEvents;
use crate::gui::traits::Parent;

/// Exposes button
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-notifications).
pub struct ComboBoxEvents {
	parent_user_events: NonNull<MsgEvents>, // used only before parent creation
	ctrl_id: u16,
}

impl ComboBoxEvents {
	pub(crate) fn new(parent: &dyn Parent, ctrl_id: u16) -> ComboBoxEvents {
		Self {
			parent_user_events: NonNull::from(parent.user_events_ref()), // convert reference to pointer
			ctrl_id,
		}
	}

	fn parent_user_events(&self) -> &MsgEvents {
		unsafe { self.parent_user_events.as_ref() }
	}

	cmd_event! { cbn_close_up, co::CMD::CBN_CLOSEUP,
		/// [`CBN_CLOSEUP`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-closeup)
		/// notification.
	}

	cmd_event! { cbn_dbl_clk, co::CMD::CBN_DBLCLK,
		/// [`CBN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-dblclk)
		/// notification.
	}

	cmd_event! { cbn_drop_down, co::CMD::CBN_DROPDOWN,
		/// [`CBN_DROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-dropdown)
		/// notification.
	}

	cmd_event! { cbn_edit_change, co::CMD::CBN_EDITCHANGE,
		/// [`CBN_EDITCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-editchange)
		/// notification.
	}

	cmd_event! { cbn_edit_update, co::CMD::CBN_EDITUPDATE,
		/// [`CBN_EDITUPDATE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-editupdate)
		/// notification.
	}

	cmd_event! { cbn_err_space, co::CMD::CBN_ERRSPACE,
		/// [`CBN_ERRSPACE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-errspace)
		/// notification.
	}

	cmd_event! { cbn_kill_focus, co::CMD::CBN_KILLFOCUS,
		/// [`CBN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-killfocus)
		/// notification.
	}

	cmd_event! { cbn_sel_change, co::CMD::CBN_SELCHANGE,
		/// [`CBN_SELCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-selchange)
		/// notification.
	}

	cmd_event! { cbn_sel_end_cancel, co::CMD::CBN_SELENDCANCEL,
		/// [`CBN_SELENDCANCEL`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-selendcancel)
		/// notification.
	}

	cmd_event! { cbn_sel_end_ok, co::CMD::CBN_SELENDOK,
		/// [`CBN_SELENDOK`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-selendok)
		/// notification.
	}

	cmd_event! { cbn_set_focus, co::CMD::CBN_SETFOCUS,
		/// [`CBN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-setfocus)
		/// notification.
	}
}
