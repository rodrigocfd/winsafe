use std::ptr::NonNull;

use crate::co;
use crate::gui::base::Base;
use crate::gui::events::WindowEvents;
use crate::structs::{NMCUSTOMDRAW, NMTRBTHUMBPOSCHANGING};

ctrl_events_proxy! {
	/// Exposes trackbar control
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-notifications).
	///
	/// These event methods are just proxies to the
	/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window,
	/// who is the real responsible for the child event handling.
	///
	/// You cannot directly instantiate this object, it is created internally by
	/// the control.
	TrackbarEvents
}

impl TrackbarEvents {
	nfy_event_p! { trbn_thumb_pos_changing, co::TRBN::THUMBPOSCHANGING.into(), NMTRBTHUMBPOSCHANGING,
		/// [`TRBN_THUMBPOSCHANGING`](https://docs.microsoft.com/en-us/windows/win32/controls/trbn-thumbposchanging)
		/// notification.
		///
		/// Notifies that the thumb position on a trackbar is changing.
	}

	/// [`NM_CUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-customdraw-trackbar)
	/// notification.
	///
	/// Sent by a trackbar control to notify its parent windows about drawing
	/// operations.
	pub fn nm_custom_draw<F>(&self, func: F)
		where F: FnMut(&NMCUSTOMDRAW) -> co::CDRF + 'static,
	{
		self.parent_user_events().add_nfy(self.ctrl_id, co::NM::CUSTOMDRAW, {
			let mut func = func;
			move |p| Some(func(unsafe { p.cast_nmhdr::<NMCUSTOMDRAW>() }).into())
		});
	}

	nfy_event! { nm_released_capture, co::NM::RELEASEDCAPTURE,
		/// [`NM_RELEASEDCAPTURE`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-trackbar-)
		/// notification.
		///
		/// Notifies a trackbar control's parent window that the control is
		/// releasing mouse capture.
	}
}
