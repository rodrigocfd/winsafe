use std::ptr::NonNull;

use crate::co;
use crate::gui::events::MsgEvents;
use crate::gui::traits::Parent;

/// Exposes combo box control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`MsgEvents`](crate::gui::events::MsgEvents) of the parent window, who is
/// the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
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
		///
		/// Sent when the list box of a combo box has been closed.
	}

	cmd_event! { cbn_dbl_clk, co::CMD::CBN_DBLCLK,
		/// [`CBN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-dblclk)
		/// notification.
		///
		/// Sent when the user double-clicks a string in the list box of a combo
		/// box.
	}

	cmd_event! { cbn_drop_down, co::CMD::CBN_DROPDOWN,
		/// [`CBN_DROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-dropdown)
		/// notification.
		///
		/// Sent when the list box of a combo box is about to be made visible.
	}

	cmd_event! { cbn_edit_change, co::CMD::CBN_EDITCHANGE,
		/// [`CBN_EDITCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-editchange)
		/// notification.
		///
		/// Sent after the user has taken an action that may have altered the text
		/// in the edit control portion of a combo box. Unlike the
		/// [`CBN_EDITUPDATE`](crate::gui::events::ComboBoxEvents::cbn_edit_update)
		/// notification code, this notification code is sent after the system
		/// updates the screen.
	}

	cmd_event! { cbn_edit_update, co::CMD::CBN_EDITUPDATE,
		/// [`CBN_EDITUPDATE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-editupdate)
		/// notification.
		///
		/// Sent when the edit control portion of a combo box is about to display
		/// altered text. This notification code is sent after the control has
		/// formatted the text, but before it displays the text.
	}

	cmd_event! { cbn_err_space, co::CMD::CBN_ERRSPACE,
		/// [`CBN_ERRSPACE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-errspace)
		/// notification.
		///
		/// Sent when a combo box cannot allocate enough memory to meet a specific
		/// request.
	}

	cmd_event! { cbn_kill_focus, co::CMD::CBN_KILLFOCUS,
		/// [`CBN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-killfocus)
		/// notification.
		///
		/// Sent when a combo box loses the keyboard focus.
	}

	cmd_event! { cbn_sel_change, co::CMD::CBN_SELCHANGE,
		/// [`CBN_SELCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-selchange)
		/// notification.
		///
		/// Sent when the user changes the current selection in the list box of a
		/// combo box. The user can change the selection by clicking in the list
		/// box or by using the arrow keys.
	}

	cmd_event! { cbn_sel_end_cancel, co::CMD::CBN_SELENDCANCEL,
		/// [`CBN_SELENDCANCEL`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-selendcancel)
		/// notification.
		///
		/// Sent when the user selects an item, but then selects another control
		/// or closes the dialog box. It indicates the user's initial selection is
		/// to be ignored.
	}

	cmd_event! { cbn_sel_end_ok, co::CMD::CBN_SELENDOK,
		/// [`CBN_SELENDOK`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-selendok)
		/// notification.
		///
		/// Sent when the user selects a list item, or selects an item and then
		/// closes the list. It indicates that the user's selection is to be
		/// processed.
	}

	cmd_event! { cbn_set_focus, co::CMD::CBN_SETFOCUS,
		/// [`CBN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-setfocus)
		/// notification.
		///
		/// Sent when a combo box receives the keyboard focus.
	}
}
