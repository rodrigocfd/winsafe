use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// This trait is enabled with the `gui` feature, and exposes list view control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-list-view-control-reference-notifications).
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
pub trait GuiEventsListView: priv_ctrl_events::GuiEvents {
	fn_nfy_withparm_noret! { lvn_begin_drag, co::LVN::BEGINDRAG, NMLISTVIEW;
		/// [`LVN_BEGINDRAG`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-begindrag)
		/// notification.
	}

	fn_nfy_withparm_boolret! { lvn_begin_label_edit, co::LVN::BEGINLABELEDIT, NMLVDISPINFO;
		/// [`LVN_BEGINLABELEDIT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-beginlabeledit)
		/// notification.
	}

	fn_nfy_withparm_noret! { lvn_begin_r_drag, co::LVN::BEGINRDRAG, NMLISTVIEW;
		/// [`LVN_BEGINRDRAG`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-beginrdrag)
		/// notification.
	}

	fn_nfy_withparm_noret! { lvn_begin_scroll, co::LVN::BEGINSCROLL, NMLVSCROLL;
		/// [`LVN_BEGINSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-beginscroll)
		/// notification.
	}

	fn_nfy_withparm_noret! { lvn_column_click, co::LVN::COLUMNCLICK, NMLISTVIEW;
		/// [`LVN_COLUMNCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-columnclick)
		/// notification.
	}

	fn_nfy_withparm_noret! { lvn_column_drop_down, co::LVN::COLUMNDROPDOWN, NMLISTVIEW;
		/// [`LVN_COLUMNDROPDOWN`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-columndropdown)
		/// notification.
	}

	fn_nfy_withparm_noret! { lvn_column_overflow_click, co::LVN::COLUMNOVERFLOWCLICK, NMLISTVIEW;
		/// [`LVN_COLUMNOVERFLOWCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-columnoverflowclick)
		/// notification.
	}

	fn_nfy_noparm_boolret! { lvn_delete_all_items, co::LVN::DELETEALLITEMS;
		/// [`LVN_DELETEALLITEMS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-deleteallitems)
		/// notification.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// let list: gui::ListView;
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let list = gui::ListView::<()>::new(&wnd, gui::ListViewOpts::default());
		///
		/// list.on().lvn_delete_all_items(
		///     move || -> w::AnyResult<bool> {
		///         println!("All items deleted.");
		///         Ok(true) // supress subsequent WM_DELETEITEM notifications
		///     },
		/// );
		/// ```
	}

	fn_nfy_withparm_noret! { lvn_delete_item, co::LVN::DELETEITEM, NMLISTVIEW;
		/// [`LVN_DELETEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-deleteitem)
		/// notification.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// let list: gui::ListView;
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let list = gui::ListView::<()>::new(&wnd, gui::ListViewOpts::default());
		///
		/// list.on().lvn_delete_item(
		///     move |p: &w::NMLISTVIEW| -> w::AnyResult<()> {
		///         println!("Item: {}", p.iItem);
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	fn_nfy_withparm_boolret! { lvn_end_label_edit, co::LVN::ENDLABELEDIT, NMLVDISPINFO;
		/// [`LVN_ENDLABELEDIT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-endlabeledit)
		/// notification.
	}

	fn_nfy_withparm_noret! { lvn_end_scroll, co::LVN::ENDSCROLL, NMLVSCROLL;
		/// [`LVN_ENDSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-endscroll)
		/// notification.
	}

	fn_nfy_withmutparm_noret! { lvn_get_disp_info, co::LVN::GETDISPINFO, NMLVDISPINFO;
		/// [`LVN_GETDISPINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-getdispinfo)
		/// notification.
	}

	/// [`LVN_GETEMPTYMARKUP`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-getemptymarkup)
	/// notification.
	fn lvn_get_empty_markup<F>(&self, func: F)
	where
		F: Fn(&mut NMLVEMPTYMARKUP) -> AnyResult<bool> + 'static,
	{
		self.wm_notify(co::LVN::GETEMPTYMARKUP, move |p| {
			Ok(func(unsafe { p.cast_nmhdr_mut::<NMLVEMPTYMARKUP>() })? as _)
		});
	}

	fn_nfy_withparm_noret! { lvn_get_info_tip, co::LVN::GETINFOTIP, NMLVGETINFOTIP;
		/// [`LVN_GETINFOTIP`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-getinfotip)
		/// notification.
	}

	fn_nfy_withparm_noret! { lvn_hot_track, co::LVN::HOTTRACK, NMLISTVIEW;
		/// [`LVN_HOTTRACK`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-hottrack)
		/// notification.
	}

	fn_nfy_withparm_noret! { lvn_incremental_search, co::LVN::INCREMENTALSEARCH, NMLVFINDITEM;
		/// [`LVN_INCREMENTALSEARCH`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-incrementalsearch)
		/// notification.
	}

	fn_nfy_withparm_noret! { lvn_insert_item, co::LVN::INSERTITEM, NMLISTVIEW;
		/// [`LVN_INSERTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-insertitem)
		/// notification.
	}

	fn_nfy_withparm_noret! { lvn_item_activate, co::LVN::ITEMACTIVATE, NMITEMACTIVATE;
		/// [`LVN_ITEMACTIVATE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-itemactivate)
		/// notification.
	}

	fn_nfy_withparm_noret! { lvn_item_changed, co::LVN::ITEMCHANGED, NMLISTVIEW;
		/// [`LVN_ITEMCHANGED`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-itemchanged)
		/// notification.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// let list: gui::ListView;
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let list = gui::ListView::<()>::new(&wnd, gui::ListViewOpts::default());
		///
		/// list.on().lvn_item_changed(
		///     move |p: &w::NMLISTVIEW| -> w::AnyResult<()> {
		///         println!("Item: {}", p.iItem);
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	fn_nfy_withparm_boolret! { lvn_item_changing, co::LVN::ITEMCHANGING, NMLISTVIEW;
		/// [`LVN_ITEMCHANGING`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-itemchanging)
		/// notification.
	}

	fn_nfy_withparm_noret! { lvn_key_down, co::LVN::KEYDOWN, NMLVKEYDOWN;
		/// [`LVN_KEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-keydown)
		/// notification.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, co, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// let list: gui::ListView;
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let list = gui::ListView::<()>::new(&wnd, gui::ListViewOpts::default());
		///
		/// list.on().lvn_key_down(
		///     move |p: &w::NMLVKEYDOWN| -> w::AnyResult<()> {
		///         if p.wVKey == co::VK::DELETE {
		///             println!("DEL key was pressed.");
		///         }
		///         Ok(())
		///     },
		/// );
		/// ```
	}

	fn_nfy_withparm_noret! { lvn_link_click, co::LVN::LINKCLICK, NMLVLINK;
		/// [`LVN_LINKCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-linkclick)
		/// notification.
	}

	fn_nfy_noparm_noret! { lvn_marquee_begin, co::LVN::MARQUEEBEGIN;
		/// [`LVN_MARQUEEBEGIN`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-marqueebegin)
		/// notification.
	}

	fn_nfy_withparm_noret! { lvn_od_cache_hint, co::LVN::ODCACHEHINT, NMLVCACHEHINT;
		/// [`LVN_ODCACHEHINT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-odcachehint)
		/// notification.
	}

	/// [`LVN_ODFINDITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-odfinditem)
	/// notification.
	fn lvn_od_find_item<F>(&self, func: F)
	where
		F: Fn(&mut NMLVFINDITEM) -> AnyResult<Option<u32>> + 'static,
	{
		self.wm_notify(co::LVN::ODFINDITEM, move |p| {
			Ok(match func(unsafe { p.cast_nmhdr_mut::<NMLVFINDITEM>() })? {
				Some(idx) => idx as isize,
				None => -1,
			})
		});
	}

	fn_nfy_withparm_noret! { lvn_od_state_changed, co::LVN::ODSTATECHANGED, NMLVODSTATECHANGE;
		/// [`LVN_ODSTATECHANGED`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-odstatechanged)
		/// notification.
	}

	fn_nfy_withparm_noret! { lvn_set_disp_info, co::LVN::SETDISPINFO, NMLVDISPINFO;
		/// [`LVN_SETDISPINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/lvn-setdispinfo)
		/// notification.
	}

	fn_nfy_withparm_noret! { nm_click, co::NM::CLICK, NMITEMACTIVATE;
		/// [`NM_CLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-click-list-view)
		/// notification.
	}

	/// [`NM_CUSTOMDRAW`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-customdraw-list-view)
	/// notification.
	fn nm_custom_draw<F>(&self, func: F)
	where
		F: Fn(&mut NMLVCUSTOMDRAW) -> AnyResult<co::CDRF> + 'static,
	{
		self.wm_notify(co::NM::CUSTOMDRAW, move |p| {
			Ok(func(unsafe { p.cast_nmhdr_mut::<NMLVCUSTOMDRAW>() })?.raw() as _)
		});
	}

	fn_nfy_withparm_noret! { nm_dbl_clk, co::NM::DBLCLK, NMITEMACTIVATE;
		/// [`NM_DBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-dblclk-list-view)
		/// notification.
	}

	fn_nfy_noparm_i32ret! { nm_hover, co::NM::HOVER;
		/// [`NM_HOVER`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-hover-list-view)
		/// notification.
	}

	fn_nfy_noparm_noret! { nm_kill_focus, co::NM::KILLFOCUS;
		/// [`NM_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-killfocus-list-view)
		/// notification.
	}

	fn_nfy_withparm_i32ret! { nm_r_click, co::NM::RCLICK, NMITEMACTIVATE;
		/// [`NM_RCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-rclick-list-view)
		/// notification.
	}

	fn_nfy_withparm_noret! { nm_r_dbl_clk, co::NM::RDBLCLK, NMITEMACTIVATE;
		/// [`NM_RDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-rdblclk-list-view)
		/// notification.
	}

	fn_nfy_noparm_noret! { nm_released_capture, co::NM::RELEASEDCAPTURE;
		/// [`NM_RELEASEDCAPTURE`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-list-view-)
		/// notification.
	}

	fn_nfy_noparm_noret! { nm_return, co::NM::RETURN;
		/// [`NM_RETURN`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-return-list-view-)
		/// notification.
	}

	fn_nfy_noparm_noret! { nm_set_focus, co::NM::SETFOCUS;
		/// [`NM_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-setfocus-list-view-)
		/// notification.
	}
}

impl GuiEventsListView for BaseCtrlEvents {}
