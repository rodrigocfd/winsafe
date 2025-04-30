use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*};
use crate::msg::*;
use crate::prelude::*;

/// Exposes trackbar control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct TrackbarEvents(BaseCtrlEvents);

impl TrackbarEvents {
	#[must_use]
	pub(in crate::gui) fn new(parent: &impl AsRef<BaseWnd>, ctrl_id: u16) -> Self {
		Self(BaseCtrlEvents::new(parent, ctrl_id))
	}

	pub_fn_nfy_withparm_noret! { trbn_thumb_pos_changing, co::TRBN::THUMBPOSCHANGING, NMTRBTHUMBPOSCHANGING;
		/// [`TRBN_THUMBPOSCHANGING`](https://learn.microsoft.com/en-us/windows/win32/controls/trbn-thumbposchanging)
		/// notification.
	}

	/// [`WM_HSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-hscroll--trackbar-)
	/// notification.
	pub fn wm_h_scroll<F>(&self, func: F) -> &Self
	where
		F: Fn(wm::HScroll) -> AnyResult<()> + 'static,
	{
		let def_proc_val = self.0.wnd_ty().def_proc_val();
		self.0.wm(co::WM::HSCROLL, move |p| {
			func(unsafe { wm::HScroll::from_generic_wm(p) })?;
			Ok(def_proc_val)
		});
		self
	}

	/// [`WM_VSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-vscroll--trackbar-)
	/// notification.
	pub fn wm_v_scroll<F>(&self, func: F) -> &Self
	where
		F: Fn(wm::VScroll) -> AnyResult<()> + 'static,
	{
		let def_proc_val = self.0.wnd_ty().def_proc_val();
		self.0.wm(co::WM::VSCROLL, move |p| {
			func(unsafe { wm::VScroll::from_generic_wm(p) })?;
			Ok(def_proc_val)
		});
		self
	}

	/// [`NM_CUSTOMDRAW`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-customdraw-trackbar)
	/// notification.
	pub fn nm_custom_draw<F>(&self, func: F) -> &Self
	where
		F: Fn(&NMCUSTOMDRAW) -> AnyResult<co::CDRF> + 'static,
	{
		self.0.wm_notify(co::NM::CUSTOMDRAW, move |p| {
			Ok(func(unsafe { p.cast_nmhdr::<NMCUSTOMDRAW>() })?.raw() as _)
		});
		self
	}

	pub_fn_nfy_noparm_noret! { nm_released_capture, co::NM::RELEASEDCAPTURE;
		/// [`NM_RELEASEDCAPTURE`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-trackbar-)
		/// notification.
	}
}
