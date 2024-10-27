use crate::co;
use crate::decl::*;
use crate::gui::{*, privs::*};

/// Exposes trackbar control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct TrackbarEvents(BaseCtrlEventsProxy);

impl TrackbarEvents {
	#[must_use]
	pub(in crate::gui) fn new(parent: &impl AsRef<Base>, ctrl_id: u16) -> Self {
		Self(BaseCtrlEventsProxy::new(parent, ctrl_id))
	}

	pub_fn_nfy_withparm_noret! { trbn_thumb_pos_changing, co::TRBN::THUMBPOSCHANGING, NMTRBTHUMBPOSCHANGING;
		/// [`TRBN_THUMBPOSCHANGING`](https://learn.microsoft.com/en-us/windows/win32/controls/trbn-thumbposchanging)
		/// notification.
	}

	/// [`NM_CUSTOMDRAW`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-customdraw-trackbar)
	/// notification.
	pub fn nm_custom_draw<F>(&self, func: F)
		where F: Fn(&NMCUSTOMDRAW) -> AnyResult<co::CDRF> + 'static,
	{
		self.0.wm_notify(co::TRBN::NM_CUSTOMDRAW, move |p| {
			let ret_val = func(unsafe { p.cast_nmhdr::<NMCUSTOMDRAW>() })?.raw() as isize;
			Ok(WmRet::HandledWithRet(ret_val))
		});
	}

	pub_fn_nfy_noparm_noret! { nm_released_capture, co::TRBN::NM_RELEASEDCAPTURE;
		/// [`NM_RELEASEDCAPTURE`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-trackbar-)
		/// notification.
	}
}
