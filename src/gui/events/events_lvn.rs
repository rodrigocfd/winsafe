use crate::co;
use crate::structs::{
	NMITEMACTIVATE,
	NMLISTVIEW,
	NMLVCACHEHINT,
	NMLVCUSTOMDRAW,
	NMLVDISPINFO,
	NMLVEMPTYMARKUP,
	NMLVFINDITEM,
	NMLVGETINFOTIP,
	NMLVKEYDOWN,
	NMLVLINK,
	NMLVODSTATECHANGE,
	NMLVSCROLL,
};

pub_struct_ctrl_events_proxy! {
	/// Exposes list view control
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-view-control-reference-notifications).
	///
	/// These event methods are just proxies to the
	/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window,
	/// who is the real responsible for the child event handling.
	///
	/// You cannot directly instantiate this object, it is created internally by
	/// the control.
	ListViewEvents
}

impl ListViewEvents {
	pub_fn_nfy_ret0_param! { lvn_begin_drag, co::LVN::BEGINDRAG.into(), NMLISTVIEW,
		/// [`LVN_BEGINDRAG`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-begindrag)
		/// notification.
		///
		/// Notifies that a drag-and-drop operation involving the left mouse
		/// button is being initiated.
	}

	pub_fn_nfy_retbool_param! { lvn_begin_label_edit, co::LVN::BEGINLABELEDIT.into(), NMLVDISPINFO,
		/// [`LVN_BEGINLABELEDIT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-beginlabeledit)
		/// notification.
		///
		/// Notifies about the start of label editing for an item.
	}

	pub_fn_nfy_ret0_param! { lvn_begin_r_drag, co::LVN::BEGINRDRAG.into(), NMLISTVIEW,
		/// [`LVN_BEGINRDRAG`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-beginrdrag)
		/// notification.
		///
		/// Notifies that a drag-and-drop operation involving the right mouse
		/// button is being initiated.
	}

	pub_fn_nfy_ret0_param! { lvn_begin_scroll, co::LVN::BEGINSCROLL.into(), NMLVSCROLL,
		/// [`LVN_BEGINSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-beginscroll)
		/// notification.
		///
		/// Notifies when a scrolling operation starts.
	}

	pub_fn_nfy_ret0_param! { lvn_column_click, co::LVN::COLUMNCLICK.into(), NMLISTVIEW,
		/// [`LVN_COLUMNCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-columnclick)
		/// notification.
		///
		/// Notifies that a column header was clicked while the list-view
		/// control was in report mode.
	}

	pub_fn_nfy_ret0_param! { lvn_column_drop_down, co::LVN::COLUMNDROPDOWN.into(), NMLISTVIEW,
		/// [`LVN_COLUMNDROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-columndropdown)
		/// notification.
		///
		/// Sent by a list-view control when the list-view's drop-down button is
		/// pressed.
	}

	pub_fn_nfy_ret0_param! { lvn_column_overflow_click, co::LVN::COLUMNOVERFLOWCLICK.into(), NMLISTVIEW,
		/// [`LVN_COLUMNOVERFLOWCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-columnoverflowclick)
		/// notification.
		///
		/// Sent by a list-view control when its overflow button is clicked.
	}

	pub_fn_nfy_retbool_param! { lvn_delete_all_items, co::LVN::DELETEALLITEMS.into(), NMLISTVIEW,
		/// [`LVN_DELETEALLITEMS`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-deleteallitems)
		/// notification.
		///
		/// Notifies that all items in the control are about to be deleted.
	}

	pub_fn_nfy_ret0_param! { lvn_delete_item, co::LVN::DELETEITEM.into(), NMLISTVIEW,
		/// [`LVN_DELETEITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-deleteitem)
		/// notification.
		///
		/// Notifies that an item is about to be deleted.
		///
		/// # Examples
		///
		/// ```rust,ignore
		/// use winsafe::{gui, NMLISTVIEW};
		///
		/// let list: gui::ListView; // initialized somewhere
		///
		/// list.on().lvn_delete_item(|p: &NMLISTVIEW| {
		///     println!("Item: {}", p.iItem);
		/// });
		/// ```
	}

	pub_fn_nfy_retbool_param! { lvn_end_label_edit, co::LVN::ENDLABELEDIT.into(), NMLVDISPINFO,
		/// [`LVN_ENDLABELEDIT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-endlabeledit)
		/// notification.
		///
		/// Notifies about the end of label editing for an item.
	}

	pub_fn_nfy_ret0_param! { lvn_end_scroll, co::LVN::ENDSCROLL.into(), NMLVSCROLL,
		/// [`LVN_ENDSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-endscroll)
		/// notification.
		///
		/// Notifies when a scrolling operation ends.
	}

	pub_fn_nfy_ret0_param! { lvn_get_disp_info, co::LVN::GETDISPINFO.into(), NMLVDISPINFO,
		/// [`LVN_GETDISPINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-getdispinfo)
		/// notification.
		///
		/// It is a request to provide information needed to display or sort a
		/// list-view item.
	}

	pub_fn_nfy_retbool_mutparam! { lvn_get_empty_markup, co::LVN::GETEMPTYMARKUP.into(), NMLVEMPTYMARKUP,
		/// [`LVN_GETEMPTYMARKUP`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-getemptymarkup)
		/// notification.
		///
		/// Sent by list-view control when the control has no items. The
		/// notification code is a request for the parent window to provide
		/// markup text.
	}

	pub_fn_nfy_ret0_param! { lvn_get_info_tip, co::LVN::GETINFOTIP.into(), NMLVGETINFOTIP,
		/// [`LVN_GETINFOTIP`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-getinfotip)
		/// notification.
		///
		/// Sent by a large icon view list-view control that has the
		/// [`LVS_EX_INFOTIP`](crate::co::LVS_EX::INFOTIP) extended style. This
		/// notification code is sent when the list-view control is requesting
		/// additional text information to be displayed in a tooltip.
	}

	pub_fn_nfy_ret0_param! { lvn_hot_track, co::LVN::HOTTRACK.into(), NMLISTVIEW,
		/// [`LVN_HOTTRACK`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-hottrack)
		/// notification.
		///
		/// Sent by a list-view control when the user moves the mouse over an
		/// item. This notification code is only sent by list-view controls that
		/// have the [`LVS_EX_TRACKSELECT`](crate::co::LVS_EX::TRACKSELECT)
		/// extended list-view style.
	}

	pub_fn_nfy_ret0_param! { lvn_incremental_search, co::LVN::INCREMENTALSEARCH.into(), NMLVFINDITEM,
		/// [`LVN_INCREMENTALSEARCH`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-incrementalsearch)
		/// notification.
		///
		/// Notifies that an incremental search has started.
	}

	pub_fn_nfy_ret0_param! { lvn_insert_item, co::LVN::INSERTITEM.into(), NMLISTVIEW,
		/// [`LVN_INSERTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-insertitem)
		/// notification.
		///
		/// Notifies that a new item was inserted.
	}

	pub_fn_nfy_ret0_param! { lvn_item_activate, co::LVN::ITEMACTIVATE.into(), NMITEMACTIVATE,
		/// [`LVN_ITEMACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-itemactivate)
		/// notification.
		///
		/// Sent by a list-view control when the user activates an item.
	}

	pub_fn_nfy_ret0_param! { lvn_item_changed, co::LVN::ITEMCHANGED.into(), NMLISTVIEW,
		/// [`LVN_ITEMCHANGED`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-itemchanged)
		/// notification.
		///
		/// Notifies that an item has changed.
		///
		/// # Examples
		///
		/// ```rust,ignore
		/// use winsafe::{gui, NMLISTVIEW};
		///
		/// let list: gui::ListView; // initialized somewhere
		///
		/// list.on().lvn_item_changed(|p: &NMLISTVIEW| {
		///     println!("Item: {}", p.iItem);
		/// });
		/// ```
	}

	pub_fn_nfy_retbool_param! { lvn_item_changing, co::LVN::ITEMCHANGING.into(), NMLISTVIEW,
		/// [`LVN_ITEMCHANGING`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-itemchanging)
		/// notification.
		///
		/// Notifies that an item is changing.
	}

	pub_fn_nfy_ret0_param! { lvn_key_down, co::LVN::KEYDOWN.into(), NMLVKEYDOWN,
		/// [`LVN_KEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-keydown)
		/// notification.
		///
		/// Notifies that a key has been pressed.
		///
		/// # Examples
		///
		/// ```rust,ignore
		/// use winsafe::{gui, NMLVKEYDOWN};
		///
		/// let list: gui::ListView; // initialized somewhere
		///
		/// list.on().lvn_delete_item(|p: &NMLVKEYDOWN| {
		///     if p.wVKey == co::VK::DELETE {
		///         println!("DEL key was pressed.");
		///     }
		/// });
		/// ```
	}

	pub_fn_nfy_ret0_param! { lvn_link_click, co::LVN::LINKCLICK.into(), NMLVLINK,
		/// [`LVN_LINKCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-linkclick)
		/// notification.
		///
		/// Notifies that a link has been clicked on.
	}

	pub_fn_nfy_ret0! { lvn_marquee_begin, co::LVN::MARQUEEBEGIN.into(),
		/// [`LVN_MARQUEEBEGIN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-marqueebegin)
		/// notification.
		///
		/// Notifies that a bounding box (marquee) selection has begun.
	}

	pub_fn_nfy_ret0_param! { lvn_od_cache_hint, co::LVN::ODCACHEHINT.into(), NMLVCACHEHINT,
		/// [`LVN_ODCACHEHINT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-odcachehint)
		/// notification.
		///
		/// Sent by a virtual list-view control when the contents of its display
		/// area have changed. For example, a list-view control sends this
		/// notification code when the user scrolls the control's display.
	}

	/// [`LVN_ODFINDITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-odfinditem)
	/// notification.
	///
	/// Sent by a virtual list-view control when it needs the owner to find a
	/// particular callback item. For example, the control will send this
	/// notification code when it receives shortcut keyboard input or when it
	/// receives an [`lvm::FindItem`](crate::msg::lvm::FindItem) message.
	pub fn lvn_od_find_item<F>(&self, func: F)
		where F: Fn(&mut NMLVFINDITEM) -> Option<u32> + 'static,
	{
		self.parent_user_events().add_nfy(self.ctrl_id as _, co::LVN::ODFINDITEM.into(), move |p| {
			Some(match func(unsafe { p.cast_nmhdr_mut::<NMLVFINDITEM>() }) {
				Some(idx) => idx as _,
				None => -1,
			})
		});
	}

	pub_fn_nfy_ret0_param! { lvn_od_state_changed, co::LVN::ODSTATECHANGED.into(), NMLVODSTATECHANGE,
		/// [`LVN_ODSTATECHANGED`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-odstatechanged)
		/// notification.
		///
		/// Sent by a list-view control when the state of an item or range of
		/// items has changed.
	}

	pub_fn_nfy_ret0_param! { lvn_set_disp_info, co::LVN::SETDISPINFO.into(), NMLVDISPINFO,
		/// [`LVN_SETDISPINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/lvn-setdispinfo)
		/// notification.
		///
		/// Notifies that you must update the information it maintains for an
		/// item.
	}

	pub_fn_nfy_ret0_param! { nm_click, co::NM::CLICK, NMITEMACTIVATE,
		/// [`NM_CLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-click-list-view)
		/// notification.
		///
		/// Sent by a list-view control when the user clicks an item with the
		/// left mouse button.
	}

	/// [`NM_CUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-customdraw-list-view)
	/// notification.
	///
	/// Sent by a list-view control to notify about drawing operations.
	pub fn nm_custom_draw<F>(&self, func: F)
		where F: Fn(&NMLVCUSTOMDRAW) -> co::CDRF + 'static,
	{
		self.parent_user_events().add_nfy(self.ctrl_id as _, co::NM::CUSTOMDRAW,
			move |p| Some(func(unsafe { p.cast_nmhdr::<NMLVCUSTOMDRAW>() }).into()));
	}

	pub_fn_nfy_ret0_param! { nm_dbl_clk, co::NM::DBLCLK, NMITEMACTIVATE,
		/// [`NM_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-dblclk-list-view)
		/// notification.
		///
		/// Sent by a list-view control when the user double-clicks an item with
		/// the left mouse button.
	}

	pub_fn_nfy_reti32! { nm_hover, co::NM::HOVER,
		/// [`NM_HOVER`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-hover-list-view)
		/// notification.
		///
		/// Sent by a list-view control when the mouse hovers over an item.
	}

	pub_fn_nfy_ret0! { nm_kill_focus, co::NM::KILLFOCUS,
		/// [`NM_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-killfocus-list-view)
		/// notification.
		///
		/// Notifies that the control has lost the input focus.
	}

	pub_fn_nfy_reti32_param! { nm_r_click, co::NM::RCLICK, NMITEMACTIVATE,
		/// [`NM_RCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-rclick-list-view)
		/// notification.
		///
		/// Sent by a list-view control when the user clicks an item with the right
		/// mouse button.
	}

	pub_fn_nfy_ret0_param! { nm_r_dbl_clk, co::NM::RDBLCLK, NMITEMACTIVATE,
		/// [`NM_RDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-rdblclk-list-view)
		/// notification.
		///
		/// Sent by a list-view control when the user double-clicks an item with
		/// the right mouse button.
	}

	pub_fn_nfy_ret0! { nm_released_capture, co::NM::RELEASEDCAPTURE,
		/// [`NM_RELEASEDCAPTURE`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-releasedcapture-list-view-)
		/// notification.
		///
		/// Notifies that the control is releasing mouse capture.
	}

	pub_fn_nfy_ret0! { nm_return, co::NM::RETURN,
		/// [`NM_RETURN`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-return-list-view-)
		/// notification.
		///
		/// Notifies that the control has the input focus and that the user has
		/// pressed the ENTER key.
	}

	pub_fn_nfy_ret0! { nm_set_focus, co::NM::SETFOCUS,
		/// [`NM_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-setfocus-list-view-)
		/// notification.
		///
		/// Notifies that the control has received the input focus.
	}
}
