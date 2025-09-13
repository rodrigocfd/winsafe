use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// This trait is enabled with the `gui` feature, and exposes header control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-header-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`GuiEventsParent`](crate::prelude::GuiEventsParent) of the parent window,
/// who is the real responsible for the child event handling.
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait GuiEventsHeader: priv_ctrl_events::GuiEvents {
	fn_nfy_withparm_boolret! { hdn_begin_drag, co::HDN::BEGINDRAG, NMHEADER;
		/// [`HDN_BEGINDRAG`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-begindrag)
		/// notification.
	}

	fn_nfy_withparm_noret! { hdn_begin_filter_edit, co::HDN::BEGINFILTEREDIT, NMHEADER;
		/// [`HDN_BEGINFILTEREDIT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-beginfilteredit)
		/// notification.
	}

	fn_nfy_withparm_boolret! { hdn_begin_track, co::HDN::BEGINTRACK, NMHEADER;
		/// [`HDN_BEGINTRACK`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-begintrack)
		/// notification.
	}

	fn_nfy_withparm_noret! { hdn_divider_dbl_click, co::HDN::DIVIDERDBLCLICK, NMHEADER;
		/// [`HDN_DIVIDERDBLCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-dividerdblclick)
		/// notification.
	}

	fn_nfy_withparm_noret! { hdn_drop_down, co::HDN::DROPDOWN, NMHEADER;
		/// [`HDN_DROPDOWN`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-dropdown)
		/// notification.
	}

	fn_nfy_withparm_boolret! { hdn_end_drag, co::HDN::ENDDRAG, NMHEADER;
		/// [`HDN_ENDDRAG`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-enddrag)
		/// notification.
	}

	fn_nfy_withparm_noret! { hdn_end_filter_edit, co::HDN::ENDFILTEREDIT, NMHEADER;
		/// [`HDN_ENDFILTEREDIT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-endfilteredit)
		/// notification.
	}

	fn_nfy_withparm_noret! { hdn_end_track, co::HDN::ENDTRACK, NMHEADER;
		/// [`HDN_ENDTRACK`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-endtrack)
		/// notification.
	}

	fn_nfy_withparm_boolret! { hdn_filter_btn_click, co::HDN::FILTERBTNCLICK, NMHDFILTERBTNCLICK;
		/// [`HDN_FILTERBTNCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-filterbtnclick)
		/// notification.
	}

	fn_nfy_withparm_noret! { hdn_filter_change, co::HDN::FILTERCHANGE, NMHEADER;
		/// [`HDN_FILTERCHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-filterchange)
		/// notification.
	}

	/// [`HDN_GETDISPINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-getdispinfo)
	/// notification.
	fn hdn_get_disp_info<F>(&self, func: F)
	where
		F: Fn(&mut NMHDDISPINFO) -> AnyResult<isize> + 'static,
	{
		self.wm_notify(co::HDN::GETDISPINFO, move |p| {
			Ok(func(unsafe { p.cast_nmhdr_mut::<NMHDDISPINFO>() })?)
		});
	}

	fn_nfy_withparm_noret! { hdn_item_changed, co::HDN::ITEMCHANGED, NMHEADER;
		/// [`HDN_ITEMCHANGED`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-itemchanged)
		/// notification.
	}

	fn_nfy_withparm_noret! { hdn_item_changing, co::HDN::ITEMCHANGING, NMHEADER;
		/// [`HDN_ITEMCHANGING`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-itemchanging)
		/// notification.
	}

	fn_nfy_withparm_noret! { hdn_item_click, co::HDN::ITEMCLICK, NMHEADER;
		/// [`HDN_ITEMCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-itemclick)
		/// notification.
	}

	fn_nfy_withparm_noret! { hdn_item_dbl_click, co::HDN::ITEMDBLCLICK, NMHEADER;
		/// [`HDN_ITEMDBLCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-itemdblclick)
		/// notification.
	}

	fn_nfy_withparm_noret! { hdn_item_key_down, co::HDN::ITEMKEYDOWN, NMHEADER;
		/// [`HDN_ITEMKEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-itemkeydown)
		/// notification.
	}

	fn_nfy_withparm_noret! { hdn_item_state_icon_click, co::HDN::ITEMSTATEICONCLICK, NMHEADER;
		/// [`HDN_ITEMSTATEICONCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-itemstateiconclick)
		/// notification.
	}

	fn_nfy_withparm_noret! { hdn_overflow_click, co::HDN::OVERFLOWCLICK, NMHEADER;
		/// [`HDN_OVERFLOWCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-overflowclick)
		/// notification.
	}

	fn_nfy_withparm_boolret! { hdn_track, co::HDN::TRACK, NMHDFILTERBTNCLICK;
		/// [`HDN_TRACK`](https://learn.microsoft.com/en-us/windows/win32/controls/hdn-track)
		/// notification.
	}

	/// [`NM_CUSTOMDRAW`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-customdraw-header)
	/// notification.
	fn nm_custom_draw<F>(&self, func: F)
	where
		F: Fn(&mut NMCUSTOMDRAW) -> AnyResult<co::CDRF> + 'static,
	{
		self.wm_notify(co::NM::CUSTOMDRAW, move |p| {
			Ok(func(unsafe { p.cast_nmhdr_mut::<NMCUSTOMDRAW>() })?.raw() as _)
		});
	}

	fn_nfy_noparm_i32ret! { nm_r_click, co::NM::RCLICK;
		/// [`NM_RCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-rclick-header)
		/// notification.
	}

	fn_nfy_noparm_noret! { nm_released_capture, co::NM::RELEASEDCAPTURE;
		/// [`NM_RELEASEDCAPTURE`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-header-)
		/// notification.
	}
}

impl GuiEventsHeader for BaseCtrlEvents {}
