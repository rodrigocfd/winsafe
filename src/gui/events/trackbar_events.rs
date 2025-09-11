use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::msg::*;
use crate::prelude::*;

/// This trait is enabled with the `gui` feature, and exposes trackbar control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-notifications).
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
pub trait GuiEventsTrackbar: priv_ctrl_events::GuiEvents {
	fn_nfy_withparm_noret! { trbn_thumb_pos_changing, co::TRBN::THUMBPOSCHANGING, NMTRBTHUMBPOSCHANGING;
		/// [`TRBN_THUMBPOSCHANGING`](https://learn.microsoft.com/en-us/windows/win32/controls/trbn-thumbposchanging)
		/// notification.
	}

	/// [`WM_HSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-hscroll--trackbar-)
	/// notification.
	fn wm_h_scroll<F>(&self, func: F)
	where
		F: Fn(wm::HScroll) -> AnyResult<()> + 'static,
	{
		let def_proc_val = self.wnd_ty().def_proc_val();
		self.wm(co::WM::HSCROLL, move |p| {
			func(unsafe { wm::HScroll::from_generic_wm(p) })?;
			Ok(def_proc_val)
		});
	}

	/// [`WM_VSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-vscroll--trackbar-)
	/// notification.
	fn wm_v_scroll<F>(&self, func: F)
	where
		F: Fn(wm::VScroll) -> AnyResult<()> + 'static,
	{
		let def_proc_val = self.wnd_ty().def_proc_val();
		self.wm(co::WM::VSCROLL, move |p| {
			func(unsafe { wm::VScroll::from_generic_wm(p) })?;
			Ok(def_proc_val)
		});
	}

	/// [`NM_CUSTOMDRAW`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-customdraw-trackbar)
	/// notification.
	fn nm_custom_draw<F>(&self, func: F)
	where
		F: Fn(&NMCUSTOMDRAW) -> AnyResult<co::CDRF> + 'static,
	{
		self.wm_notify(co::NM::CUSTOMDRAW, move |p| {
			Ok(func(unsafe { p.cast_nmhdr::<NMCUSTOMDRAW>() })?.raw() as _)
		});
	}

	fn_nfy_noparm_noret! { nm_released_capture, co::NM::RELEASEDCAPTURE;
		/// [`NM_RELEASEDCAPTURE`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-trackbar-)
		/// notification.
	}
}

impl GuiEventsTrackbar for BaseCtrlEvents {}
