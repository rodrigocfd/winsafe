use crate::co;
use crate::comctl::decl::{NMCUSTOMDRAW, NMTRBTHUMBPOSCHANGING};
use crate::gui::base::Base;
use crate::gui::events::base_events_proxy::BaseEventsProxy;
use crate::kernel::decl::ErrResult;

/// Exposes trackbar control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct TrackbarEvents(BaseEventsProxy);

impl TrackbarEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_nfy_ret0_param! { trbn_thumb_pos_changing, co::TRBN::THUMBPOSCHANGING, NMTRBTHUMBPOSCHANGING,
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
		self.0.add_nfy(co::NM::CUSTOMDRAW,
			move |p| Ok(Some(func(unsafe { p.cast_nmhdr::<NMCUSTOMDRAW>() })?.0 as _)));
	}

	pub_fn_nfy_ret0! { nm_released_capture, co::NM::RELEASEDCAPTURE,
		/// [`NM_RELEASEDCAPTURE`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-trackbar-)
		/// notification.
		///
		/// Notifies a trackbar control's parent window that the control is
		/// releasing mouse capture.
	}
}
