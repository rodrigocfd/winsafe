use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// This trait is enabled with the `gui` feature, and exposes up-down control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-up-down-control-reference-notifications).
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
pub trait GuiEventsUpDown: priv_ctrl_events::GuiEvents {
	fn_nfy_noparm_noret! { nm_released_capture, co::NM::RELEASEDCAPTURE;
		/// [`NM_RELEASEDCAPTURE`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-up-down-)
		/// notification.
	}

	fn_nfy_withparm_i32ret! { udn_delta_pos, co::UDN::DELTAPOS, NMUPDOWN;
		/// [`UDN_DELTAPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/udn-deltapos)
		/// notification.
	}
}

impl GuiEventsUpDown for BaseCtrlEvents {}
