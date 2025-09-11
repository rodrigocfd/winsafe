use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// This trait is enabled with the `gui` feature, and exposes tab control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-tab-control-reference-notifications).
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
pub trait GuiEventsTab: priv_ctrl_events::GuiEvents {
	fn_nfy_noparm_noret! { nm_click, co::NM::CLICK;
		/// [`NM_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-click-tab)
		/// notification.
	}

	fn_nfy_noparm_i32ret! { nm_dbl_clk, co::NM::DBLCLK;
		/// [`NM_DBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-dblclk-tab)
		/// notification.
	}

	fn_nfy_noparm_i32ret! { nm_r_click, co::NM::RCLICK;
		/// [`NM_RCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-rclick-tab)
		/// notification.
	}

	fn_nfy_noparm_i32ret! { nm_r_dbl_clk, co::NM::RDBLCLK;
		/// [`NM_RDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-rdblclk-tab)
		/// notification.
	}

	fn_nfy_noparm_noret! { nm_released_capture, co::NM::RELEASEDCAPTURE;
		/// [`NM_RELEASEDCAPTURE`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-tab-)
		/// notification.
	}

	fn_nfy_noparm_noret! { tcn_focus_change, co::TCN::FOCUSCHANGE;
		/// [`TCN_FOCUSCHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/tcn-focuschange)
		/// notification.
	}

	fn_nfy_withparm_noret! { tcn_get_object, co::TCN::GETOBJECT, NMOBJECTNOTIFY;
		/// [`TCN_GETOBJECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcn-getobject)
		/// notification.
	}

	fn_nfy_withparm_noret! { tcn_key_down, co::TCN::KEYDOWN, NMTCKEYDOWN;
		/// [`TCN_KEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/controls/tcn-keydown)
		/// notification.
	}

	fn_nfy_noparm_noret! { tcn_sel_change, co::TCN::SELCHANGE;
		/// [`TCN_SELCHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/tcn-selchange)
		/// notification.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// let tab: gui::Tab;
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let tab = gui::Tab::new(&wnd, gui::TabOpts::default());
		///
		/// tab.on().tcn_sel_change(
		///     move || -> w::AnyResult<()> {
		///         println!("Changed tab.");
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	fn_nfy_noparm_boolret! { tcn_sel_changing, co::TCN::SELCHANGING;
		/// [`TCN_SELCHANGING`](https://learn.microsoft.com/en-us/windows/win32/controls/tcn-selchanging)
		/// notification.
	}
}

impl GuiEventsTab for BaseCtrlEvents {}
