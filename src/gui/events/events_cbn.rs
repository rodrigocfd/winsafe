use crate::co;
use crate::gui::base::Base;
use crate::gui::events::base_events_proxy::BaseEventsProxy;

/// Exposes combo box control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ComboBoxEvents(BaseEventsProxy);

impl ComboBoxEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_cmd_ret0! { cbn_close_up, co::CBN::CLOSEUP,
		/// [`CBN_CLOSEUP`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-closeup)
		/// notification.
		///
		/// Sent when the list box of a combo box has been closed.
	}

	pub_fn_cmd_ret0! { cbn_dbl_clk, co::CBN::DBLCLK,
		/// [`CBN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-dblclk)
		/// notification.
		///
		/// Sent when the user double-clicks a string in the list box of a combo
		/// box.
	}

	pub_fn_cmd_ret0! { cbn_drop_down, co::CBN::DROPDOWN,
		/// [`CBN_DROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-dropdown)
		/// notification.
		///
		/// Sent when the list box of a combo box is about to be made visible.
	}

	pub_fn_cmd_ret0! { cbn_edit_change, co::CBN::EDITCHANGE,
		/// [`CBN_EDITCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-editchange)
		/// notification.
		///
		/// Sent after the user has taken an action that may have altered the
		/// text in the edit control portion of a combo box. Unlike the
		/// [`CBN_EDITUPDATE`](crate::gui::events::ComboBoxEvents::cbn_edit_update)
		/// notification code, this notification code is sent after the system
		/// updates the screen.
	}

	pub_fn_cmd_ret0! { cbn_edit_update, co::CBN::EDITUPDATE,
		/// [`CBN_EDITUPDATE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-editupdate)
		/// notification.
		///
		/// Sent when the edit control portion of a combo box is about to
		/// display altered text. This notification code is sent after the
		/// control has formatted the text, but before it displays the text.
	}

	pub_fn_cmd_ret0! { cbn_err_space, co::CBN::ERRSPACE,
		/// [`CBN_ERRSPACE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-errspace)
		/// notification.
		///
		/// Sent when a combo box cannot allocate enough memory to meet a
		/// specific request.
	}

	pub_fn_cmd_ret0! { cbn_kill_focus, co::CBN::KILLFOCUS,
		/// [`CBN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-killfocus)
		/// notification.
		///
		/// Sent when a combo box loses the keyboard focus.
	}

	pub_fn_cmd_ret0! { cbn_sel_change, co::CBN::SELCHANGE,
		/// [`CBN_SELCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-selchange)
		/// notification.
		///
		/// Sent when the user changes the current selection in the list box of
		/// a combo box. The user can change the selection by clicking in the
		/// list box or by using the arrow keys.
	}

	pub_fn_cmd_ret0! { cbn_sel_end_cancel, co::CBN::SELENDCANCEL,
		/// [`CBN_SELENDCANCEL`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-selendcancel)
		/// notification.
		///
		/// Sent when the user selects an item, but then selects another control
		/// or closes the dialog box. It indicates the user's initial selection
		/// is to be ignored.
	}

	pub_fn_cmd_ret0! { cbn_sel_end_ok, co::CBN::SELENDOK,
		/// [`CBN_SELENDOK`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-selendok)
		/// notification.
		///
		/// Sent when the user selects a list item, or selects an item and then
		/// closes the list. It indicates that the user's selection is to be
		/// processed.
	}

	pub_fn_cmd_ret0! { cbn_set_focus, co::CBN::SETFOCUS,
		/// [`CBN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-setfocus)
		/// notification.
		///
		/// Sent when a combo box receives the keyboard focus.
	}
}
