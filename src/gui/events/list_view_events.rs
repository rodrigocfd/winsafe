use crate::co;
use crate::comctl::decl::{
	NMITEMACTIVATE, NMLISTVIEW, NMLVCACHEHINT, NMLVCUSTOMDRAW, NMLVDISPINFO,
	NMLVEMPTYMARKUP, NMLVFINDITEM, NMLVGETINFOTIP, NMLVKEYDOWN, NMLVLINK,
	NMLVODSTATECHANGE, NMLVSCROLL,
};
use crate::gui::base::Base;
use crate::gui::events::base_events_proxy::BaseEventsProxy;
use crate::kernel::decl::ErrResult;

/// Exposes list view control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-view-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct ListViewEvents(BaseEventsProxy);

impl ListViewEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_nfy_withparm_noret! { lvn_begin_drag, co::LVN::BEGINDRAG, NMLISTVIEW,
		/// [`LVN_BEGINDRAG`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-begindrag)
		/// notification.
	}

	pub_fn_nfy_withparm_boolret! { lvn_begin_label_edit, co::LVN::BEGINLABELEDIT, NMLVDISPINFO,
		/// [`LVN_BEGINLABELEDIT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-beginlabeledit)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_begin_r_drag, co::LVN::BEGINRDRAG, NMLISTVIEW,
		/// [`LVN_BEGINRDRAG`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-beginrdrag)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_begin_scroll, co::LVN::BEGINSCROLL, NMLVSCROLL,
		/// [`LVN_BEGINSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-beginscroll)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_column_click, co::LVN::COLUMNCLICK, NMLISTVIEW,
		/// [`LVN_COLUMNCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-columnclick)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_column_drop_down, co::LVN::COLUMNDROPDOWN, NMLISTVIEW,
		/// [`LVN_COLUMNDROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-columndropdown)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_column_overflow_click, co::LVN::COLUMNOVERFLOWCLICK, NMLISTVIEW,
		/// [`LVN_COLUMNOVERFLOWCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-columnoverflowclick)
		/// notification.
	}

	pub_fn_nfy_withparm_boolret! { lvn_delete_all_items, co::LVN::DELETEALLITEMS, NMLISTVIEW,
		/// [`LVN_DELETEALLITEMS`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-deleteallitems)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_delete_item, co::LVN::DELETEITEM, NMLISTVIEW,
		/// [`LVN_DELETEITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-deleteitem)
		/// notification.
		///
		/// # Examples
		///
		/// ```rust,no_run
		/// use winsafe::prelude::*;
		/// use winsafe::{gui, ErrResult, NMLISTVIEW};
		///
		/// let list: gui::ListView; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let list = gui::ListView::new(&wnd, gui::ListViewOpts::default());
		///
		/// list.on().lvn_delete_item(|p: &NMLISTVIEW| -> ErrResult<()> {
		///     println!("Item: {}", p.iItem);
		///     Ok(())
		/// });
		/// ```
	}

	pub_fn_nfy_withparm_boolret! { lvn_end_label_edit, co::LVN::ENDLABELEDIT, NMLVDISPINFO,
		/// [`LVN_ENDLABELEDIT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-endlabeledit)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_end_scroll, co::LVN::ENDSCROLL, NMLVSCROLL,
		/// [`LVN_ENDSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-endscroll)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_get_disp_info, co::LVN::GETDISPINFO, NMLVDISPINFO,
		/// [`LVN_GETDISPINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-getdispinfo)
		/// notification.
	}

	/// [`LVN_GETEMPTYMARKUP`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-getemptymarkup)
	/// notification.
	pub fn lvn_get_empty_markup<F>(&self, func: F)
		where F: Fn(&mut NMLVEMPTYMARKUP) -> ErrResult<bool> + 'static,
	{
		self.0.wm_notify(co::LVN::GETEMPTYMARKUP,
			move |p| Ok(Some(func(unsafe { p.cast_nmhdr_mut::<NMLVEMPTYMARKUP>() })? as _)));
	}

	pub_fn_nfy_withparm_noret! { lvn_get_info_tip, co::LVN::GETINFOTIP, NMLVGETINFOTIP,
		/// [`LVN_GETINFOTIP`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-getinfotip)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_hot_track, co::LVN::HOTTRACK, NMLISTVIEW,
		/// [`LVN_HOTTRACK`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-hottrack)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_incremental_search, co::LVN::INCREMENTALSEARCH, NMLVFINDITEM,
		/// [`LVN_INCREMENTALSEARCH`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-incrementalsearch)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_insert_item, co::LVN::INSERTITEM, NMLISTVIEW,
		/// [`LVN_INSERTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-insertitem)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_item_activate, co::LVN::ITEMACTIVATE, NMITEMACTIVATE,
		/// [`LVN_ITEMACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-itemactivate)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_item_changed, co::LVN::ITEMCHANGED, NMLISTVIEW,
		/// [`LVN_ITEMCHANGED`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-itemchanged)
		/// notification.
		///
		/// # Examples
		///
		/// ```rust,no_run
		/// use winsafe::prelude::*;
		/// use winsafe::{gui, ErrResult, NMLISTVIEW};
		///
		/// let list: gui::ListView; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let list = gui::ListView::new(&wnd, gui::ListViewOpts::default());
		///
		/// list.on().lvn_item_changed(|p: &NMLISTVIEW| -> ErrResult<()> {
		///     println!("Item: {}", p.iItem);
		///     Ok(())
		/// });
		/// ```
	}

	pub_fn_nfy_withparm_boolret! { lvn_item_changing, co::LVN::ITEMCHANGING, NMLISTVIEW,
		/// [`LVN_ITEMCHANGING`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-itemchanging)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_key_down, co::LVN::KEYDOWN, NMLVKEYDOWN,
		/// [`LVN_KEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-keydown)
		/// notification.
		///
		/// # Examples
		///
		/// ```rust,no_run
		/// use winsafe::prelude::*;
		/// use winsafe::{co, gui, ErrResult, NMLVKEYDOWN};
		///
		/// let list: gui::ListView; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let list = gui::ListView::new(&wnd, gui::ListViewOpts::default());
		///
		/// list.on().lvn_key_down(|p: &NMLVKEYDOWN| -> ErrResult<()> {
		///     if p.wVKey == co::VK::DELETE {
		///         println!("DEL key was pressed.");
		///     }
		///     Ok(())
		/// });
		/// ```
	}

	pub_fn_nfy_withparm_noret! { lvn_link_click, co::LVN::LINKCLICK, NMLVLINK,
		/// [`LVN_LINKCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-linkclick)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { lvn_marquee_begin, co::LVN::MARQUEEBEGIN,
		/// [`LVN_MARQUEEBEGIN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-marqueebegin)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_od_cache_hint, co::LVN::ODCACHEHINT, NMLVCACHEHINT,
		/// [`LVN_ODCACHEHINT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-odcachehint)
		/// notification.
	}

	/// [`LVN_ODFINDITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-odfinditem)
	/// notification.
	pub fn lvn_od_find_item<F>(&self, func: F)
		where F: Fn(&mut NMLVFINDITEM) -> ErrResult<Option<u32>> + 'static,
	{
		self.0.wm_notify(co::LVN::ODFINDITEM, move |p| {
			Ok(Some(match func(unsafe { p.cast_nmhdr_mut::<NMLVFINDITEM>() })? {
				Some(idx) => idx as _,
				None => -1,
			}))
		});
	}

	pub_fn_nfy_withparm_noret! { lvn_od_state_changed, co::LVN::ODSTATECHANGED, NMLVODSTATECHANGE,
		/// [`LVN_ODSTATECHANGED`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-odstatechanged)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { lvn_set_disp_info, co::LVN::SETDISPINFO, NMLVDISPINFO,
		/// [`LVN_SETDISPINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-setdispinfo)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { nm_click, co::NM::CLICK, NMITEMACTIVATE,
		/// [`NM_CLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-click-list-view)
		/// notification.
	}

	/// [`NM_CUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-customdraw-list-view)
	/// notification.
	pub fn nm_custom_draw<F>(&self, func: F)
		where F: Fn(&NMLVCUSTOMDRAW) -> ErrResult<co::CDRF> + 'static,
	{
		self.0.wm_notify(co::NM::CUSTOMDRAW,
			move |p| Ok(Some(func(unsafe { p.cast_nmhdr::<NMLVCUSTOMDRAW>() })?.0 as _)));
	}

	pub_fn_nfy_withparm_noret! { nm_dbl_clk, co::NM::DBLCLK, NMITEMACTIVATE,
		/// [`NM_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-dblclk-list-view)
		/// notification.
	}

	pub_fn_nfy_noparm_i32ret! { nm_hover, co::NM::HOVER,
		/// [`NM_HOVER`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-hover-list-view)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { nm_kill_focus, co::NM::KILLFOCUS,
		/// [`NM_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-killfocus-list-view)
		/// notification.
	}

	pub_fn_nfy_withparm_i32ret! { nm_r_click, co::NM::RCLICK, NMITEMACTIVATE,
		/// [`NM_RCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-rclick-list-view)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { nm_r_dbl_clk, co::NM::RDBLCLK, NMITEMACTIVATE,
		/// [`NM_RDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-rdblclk-list-view)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { nm_released_capture, co::NM::RELEASEDCAPTURE,
		/// [`NM_RELEASEDCAPTURE`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-list-view-)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { nm_return, co::NM::RETURN,
		/// [`NM_RETURN`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-return-list-view-)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { nm_set_focus, co::NM::SETFOCUS,
		/// [`NM_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-setfocus-list-view-)
		/// notification.
	}
}
