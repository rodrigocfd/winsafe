use crate::co;
use crate::gui::base::Base;
use crate::gui::events::base_events_proxy::BaseEventsProxy;
use crate::kernel::decl::AnyResult;

/// Exposes combo box control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct ComboBoxEvents(BaseEventsProxy);

impl ComboBoxEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_cmd_noparm_noret! { cbn_close_up, co::CBN::CLOSEUP,
		/// [`CBN_CLOSEUP`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-closeup)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_dbl_clk, co::CBN::DBLCLK,
		/// [`CBN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-dblclk)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_drop_down, co::CBN::DROPDOWN,
		/// [`CBN_DROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-dropdown)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_edit_change, co::CBN::EDITCHANGE,
		/// [`CBN_EDITCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-editchange)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_edit_update, co::CBN::EDITUPDATE,
		/// [`CBN_EDITUPDATE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-editupdate)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_err_space, co::CBN::ERRSPACE,
		/// [`CBN_ERRSPACE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-errspace)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_kill_focus, co::CBN::KILLFOCUS,
		/// [`CBN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-killfocus)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_sel_change, co::CBN::SELCHANGE,
		/// [`CBN_SELCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-selchange)
		/// command notification.
		///
		/// # Examples
		///
		/// ```rust,no_run
		/// use winsafe::prelude::*;
		/// use winsafe::{gui, AnyResult};
		///
		/// let cmb: gui::ComboBox; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let cmb = gui::ComboBox::new(&wnd, gui::ComboBoxOpts::default());
		///
		/// cmb.on().cbn_sel_change({
		///     let cmb = cmb.clone(); // to pass into the closure
		///     move || -> AnyResult<()> {
		///         if let Some(sel_text) = cmb.items().selected_text() {
		///             println!("New selected text: {}", sel_text);
		///         }
		///         Ok(())
		///     }
		/// });
		/// ```
	}

	pub_fn_cmd_noparm_noret! { cbn_sel_end_cancel, co::CBN::SELENDCANCEL,
		/// [`CBN_SELENDCANCEL`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-selendcancel)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_sel_end_ok, co::CBN::SELENDOK,
		/// [`CBN_SELENDOK`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-selendok)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { cbn_set_focus, co::CBN::SETFOCUS,
		/// [`CBN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/cbn-setfocus)
		/// command notification.
	}
}
