use crate::co;
use crate::gui::privs::*;

/// This trait is enabled with the `gui` feature, and exposes button control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications)
/// for a [`RadioGroup`](crate::gui::RadioGroup).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait GuiEventsRadioGroup: priv_ctrl_events::GuiEvents {
	fn_cmd_noparm_noret! { bn_clicked, co::BN::CLICKED;
		/// [`BN_CLICKED`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-clicked)
		/// command notification for all radio buttons in the group.
		///
		/// Sent when the user clicks a button.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// let radios: gui::RadioGroup;
		/// # let radios = gui::RadioGroup::new(&wnd, &[]);
		///
		/// radios.on().bn_clicked({
		///     let radios = radios.clone();
		///     move || -> w::AnyResult<()> {
		///         println!("Selected {}",
		///             radios.selected().unwrap()
		///                 .hwnd().GetWindowText()?,
		///         );
		///         Ok(())
		///     }
		/// });
		/// # w::SysResult::Ok(())
		/// ```
	}

	fn_cmd_noparm_noret! { bn_dbl_clk, co::BN::DBLCLK;
		/// [`BN_DBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-dblclk)
		/// command notification for all radio buttons in the group.
		///
		/// Sent when the user double-clicks a button. This notification code is
		/// sent automatically for
		/// [`BS::USERBUTTON`](crate::co::BS::USERBUTTON),
		/// [`BS::RADIOBUTTON`](crate::co::BS::RADIOBUTTON), and
		/// [`BS::OWNERDRAW`](crate::co::BS::OWNERDRAW) buttons. Other button
		/// types send only if they have the
		/// [`BS::NOTIFY`](crate::co::BS::NOTIFY) style.
	}

	fn_cmd_noparm_noret! { bn_kill_focus, co::BN::KILLFOCUS;
		/// [`BN_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-killfocus)
		/// command notification for all radio buttons in the group.
		///
		/// Sent when a button loses the keyboard focus. The button must have the
		/// [`BS::NOTIFY`](crate::co::BS::NOTIFY) style to send this notification
		/// code.
	}

	fn_cmd_noparm_noret! { bn_set_focus, co::BN::SETFOCUS;
		/// [`BN_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-setfocus)
		/// command notification for all radio buttons in the group.
		///
		/// Sent when a button receives the keyboard focus. The button must have
		/// the [`BS::NOTIFY`](crate::co::BS::NOTIFY) style to send this
		/// notification code.
	}
}

impl GuiEventsRadioGroup for BaseCtrlEvents {}
