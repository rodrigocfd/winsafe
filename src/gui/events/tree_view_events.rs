use crate::co;
use crate::comctl::decl::{NMMOUSE, NMTREEVIEW, NMTVCUSTOMDRAW};
use crate::gui::base::Base;
use crate::gui::events::base_events_proxy::BaseEventsProxy;
use crate::kernel::decl::ErrResult;

/// Exposes tree view control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-tree-view-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct TreeViewEvents(BaseEventsProxy);

impl TreeViewEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_nfy_withparm_noret! { tvn_delete_item, co::TVN::DELETEITEM, NMTREEVIEW,
		/// [`TVN_DELETEITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/tvn-deleteitem)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { tvn_item_changed, co::TVN::ITEMCHANGED, NMTREEVIEW,
		/// [`TVN_ITEMCHANGED`](https://docs.microsoft.com/en-us/windows/win32/controls/tvn-itemchanged)
		/// notification.
	}

	pub_fn_nfy_withparm_boolret! { tvn_item_changing, co::TVN::ITEMCHANGING, NMTREEVIEW,
		/// [`TVN_ITEMCHANGING`](https://docs.microsoft.com/en-us/windows/win32/controls/tvn-itemchanging)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { tvn_item_expanded, co::TVN::ITEMEXPANDED, NMTREEVIEW,
		/// [`TVN_ITEMEXPANDED`](https://docs.microsoft.com/en-us/windows/win32/controls/tvn-itemexpanded)
		/// notification.
	}

	pub_fn_nfy_withparm_boolret! { tvn_item_expanding, co::TVN::ITEMEXPANDING, NMTREEVIEW,
		/// [`TVN_ITEMEXPANDING`](https://docs.microsoft.com/en-us/windows/win32/controls/tvn-itemexpanding)
		/// notification.
	}

	pub_fn_nfy_withparm_noret! { tvn_sel_changed, co::TVN::SELCHANGED, NMTREEVIEW,
		/// [`TVN_SELCHANGED`](https://docs.microsoft.com/en-us/windows/win32/controls/tvn-selchanged)
		/// notification.
	}

	pub_fn_nfy_withparm_boolret! { tvn_sel_changing, co::TVN::SELCHANGING, NMTREEVIEW,
		/// [`TVN_SELCHANGING`](https://docs.microsoft.com/en-us/windows/win32/controls/tvn-selchanging)
		/// notification.
	}

	pub_fn_nfy_noparm_i32ret! { nm_click, co::NM::CLICK,
		/// [`NM_CLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-click-tree-view)
		/// notification.
	}

	/// [`NM_CUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-customdraw-tree-view)
	/// notification.
	pub fn nm_custom_draw<F>(&self, func: F)
		where F: Fn(&NMTVCUSTOMDRAW) -> ErrResult<co::CDRF> + 'static,
	{
		self.0.wm_notify(co::NM::CUSTOMDRAW,
			move |p| Ok(Some(func(unsafe { p.cast_nmhdr::<NMTVCUSTOMDRAW>() })?.0 as _)));
	}

	pub_fn_nfy_noparm_i32ret! { nm_dbl_clk, co::NM::DBLCLK,
		/// [`NM_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-dblclk-tree-view)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { nm_kill_focus, co::NM::KILLFOCUS,
		/// [`NM_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-killfocus-tree-view)
		/// notification.
	}

	pub_fn_nfy_noparm_i32ret! { nm_r_click, co::NM::RCLICK,
		/// [`NM_RCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-rclick-tree-view)
		/// notification.
	}

	pub_fn_nfy_noparm_i32ret! { nm_r_dbl_clk, co::NM::RDBLCLK,
		/// [`NM_RDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-rdblclk-tree-view)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { nm_return, co::NM::RETURN,
		/// [`NM_RETURN`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-return-tree-view-)
		/// notification.
	}

	pub_fn_nfy_withparm_i32ret! { nm_set_cursor, co::NM::SETCURSOR, NMMOUSE,
		/// [`NM_MOUSE`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-setcursor-tree-view-)
		/// notification.
	}

	pub_fn_nfy_noparm_noret! { nm_set_focus, co::NM::SETFOCUS,
		/// [`NM_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-setfocus-tree-view-)
		/// notification.
	}
}
