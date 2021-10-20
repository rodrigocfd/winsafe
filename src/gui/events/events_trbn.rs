use crate::aliases::ErrResult;
use crate::co;
use crate::gui::events::sealed_events_wm_nfy::SealedEventsWmNfy;
use crate::gui::traits::ParentEvents;
use crate::structs::{NMCUSTOMDRAW, NMTRBTHUMBPOSCHANGING};

pub_struct_ctrl_events_proxy! {
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
	pub_fn_nfy_ret0_param! { trbn_thumb_pos_changing, co::TRBN::THUMBPOSCHANGING.into(), NMTRBTHUMBPOSCHANGING,
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
		where F: Fn(&NMCUSTOMDRAW) -> ErrResult<co::CDRF> + 'static,
	{
		self.parent_user_events().add_nfy(self.ctrl_id as _, co::NM::CUSTOMDRAW,
			move |p| Ok(Some(func(unsafe { p.cast_nmhdr::<NMCUSTOMDRAW>() })?.into())));
	}

	pub_fn_nfy_ret0! { nm_released_capture, co::NM::RELEASEDCAPTURE,
		/// [`NM_RELEASEDCAPTURE`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-trackbar-)
		/// notification.
		///
		/// Notifies a trackbar control's parent window that the control is
		/// releasing mouse capture.
	}
}
