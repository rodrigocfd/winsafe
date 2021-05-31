use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::{GetAsyncKeyState, GetCursorPos, PostQuitMessage};
use crate::gui::base::Base;
use crate::gui::events::ListViewEvents;
use crate::gui::native_controls::list_view_columns::ListViewColumns;
use crate::gui::native_controls::list_view_items::ListViewItems;
use crate::gui::native_controls::native_control_base::{NativeControlBase, OptsId};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi};
use crate::gui::traits::{baseref_from_parent, Parent};
use crate::handles::{HMENU, HWND};
use crate::msg::lvm;
use crate::structs::{LVHITTESTINFO, NMITEMACTIVATE, NMLVKEYDOWN, POINT, SIZE};

/// Native
/// [list view](https://docs.microsoft.com/en-us/windows/win32/controls/list-view-controls-overview)
/// control. Not to be confused with the simpler [list box](crate::gui::ListBox)
/// control.
///
/// Implements [`Child`](crate::gui::Child) trait.
#[derive(Clone)]
pub struct ListView(Arc<Obj>);

struct Obj { // actual fields of ListView
	base: NativeControlBase,
	opts_id: OptsId<ListViewOpts>,
	events: ListViewEvents,
	columns: ListViewColumns,
	items: ListViewItems,
	context_menu: Option<HMENU>,
}

impl_send_sync_child!(ListView);

impl ListView {
	/// Instantiates a new `ListView` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: ListViewOpts) -> ListView {
		let parent_base_ref = baseref_from_parent(parent);
		let opts = ListViewOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;
		let context_menu = opts.context_menu;

		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(parent_base_ref),
					opts_id: OptsId::Wnd(opts),
					events: ListViewEvents::new(parent_base_ref, ctrl_id),
					columns: ListViewColumns::new(parent_base_ref.hwnd_ref()), // wrong HWND, just to construct the object
					items: ListViewItems::new(parent_base_ref.hwnd_ref()),
					context_menu,
				},
			),
		);
		new_self.0.columns.set_hwnd_ref(new_self.0.base.hwnd_ref()); // correct HWND
		new_self.0.items.set_hwnd_ref(new_self.0.base.hwnd_ref());

		parent_base_ref.privileged_events_ref().wm(parent_base_ref.creation_wm(), {
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});

		new_self.handled_events(parent_base_ref, ctrl_id);
		new_self
	}

	/// Instantiates a new `ListView` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// **Note:** The optional `context_menu` is shared: it must be destroyed
	/// manually after the control is destroyed. But note that menus loaded from
	/// resources don't need to be destroyed.
	pub fn new_dlg(
		parent: &dyn Parent,
		ctrl_id: i32,
		context_menu: Option<HMENU>) -> ListView
	{
		let parent_base_ref = baseref_from_parent(parent);

		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(parent_base_ref),
					opts_id: OptsId::Dlg(ctrl_id),
					events: ListViewEvents::new(parent_base_ref, ctrl_id),
					columns: ListViewColumns::new(parent_base_ref.hwnd_ref()), // wrong HWND, just to construct the object
					items: ListViewItems::new(parent_base_ref.hwnd_ref()),
					context_menu,
				},
			),
		);
		new_self.0.columns.set_hwnd_ref(new_self.0.base.hwnd_ref()); // correct HWND
		new_self.0.items.set_hwnd_ref(new_self.0.base.hwnd_ref());

		parent_base_ref.privileged_events_ref().wm_init_dialog({
			let me = new_self.clone();
			move |_| { me.create(); true }
		});

		new_self.handled_events(parent_base_ref, ctrl_id);
		new_self
	}

	fn create(&self) {
		|| -> WinResult<()> {
			match &self.0.opts_id {
				OptsId::Wnd(opts) => {
					let mut pos = opts.position;
					let mut sz = opts.size;
					multiply_dpi(Some(&mut pos), Some(&mut sz))?;

					self.0.base.create_window( // may panic
						"SysListView32", None, pos, sz,
						opts.ctrl_id,
						opts.window_ex_style,
						opts.window_style | opts.list_view_style.into(),
					)?;

					if opts.list_view_ex_style != co::LVS_EX::NONE {
						self.toggle_extended_style(true, opts.list_view_ex_style);
					}
					Ok(())
				},
				OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

	fn handled_events(&self, parent_base_ref: &Base, ctrl_id: i32) {
		parent_base_ref.privileged_events_ref().add_nfy(ctrl_id as _, co::LVN::KEYDOWN.into(), {
			let me = self.clone();
			move |p| {
				let lvnk = unsafe { p.cast_nmhdr::<NMLVKEYDOWN>() };
				let has_ctrl = GetAsyncKeyState(co::VK::CONTROL);
				let has_shift = GetAsyncKeyState(co::VK::SHIFT);

				if has_ctrl && lvnk.wVKey == co::VK('A' as _) { // Ctrl+A
					me.items().set_selected_all(true)
						.unwrap_or_else(|err| PostQuitMessage(err));
				} else if lvnk.wVKey == co::VK::APPS { // context menu key
					me.show_context_menu(false, has_ctrl, has_shift).unwrap();
				}
				None
			}
		});

		parent_base_ref.privileged_events_ref().add_nfy(ctrl_id as _, co::NM::RCLICK.into(), {
			let me = self.clone();
			move |p| {
				let nmia = unsafe { p.cast_nmhdr::<NMITEMACTIVATE>() };
				let has_ctrl = nmia.uKeyFlags.has(co::LVKF::CONTROL);
				let has_shift = nmia.uKeyFlags.has(co::LVKF::SHIFT);

				me.show_context_menu(true, has_ctrl, has_shift).unwrap();
				None
			}
		});
	}

	pub_fn_ctrlid_hwnd_on_onsubclass!(ListViewEvents);

	/// Exposes the column methods.
	pub fn columns(&self) -> &ListViewColumns {
		&self.0.columns
	}

	/// Returns the context menu attached to this list view, if any.
	///
	/// The context menu is attached when the list view is created, either by
	/// calling [`new`](crate::gui::ListView::new) or
	/// [`new_dlg`](crate::gui::ListView::new_dlg).
	pub fn context_menu(&self) -> Option<HMENU> {
		self.0.context_menu
	}

	/// Exposes the item methods.
	pub fn items(&self) -> &ListViewItems {
		&self.0.items
	}

	/// Retrieves the current view by sending an
	/// [`LVM_GETVIEW`](crate::msg::lvm::GetView) message.
	pub fn current_view(&self) -> co::LV_VIEW {
		self.hwnd().SendMessage(lvm::GetView {})
	}

	/// Sets the current view by sending an
	/// [`LVM_SETVIEW`](crate::msg::lvm::SetView) message.
	pub fn set_current_view(&self, view: co::LV_VIEW) -> WinResult<()> {
		self.hwnd().SendMessage(lvm::SetView { view })
	}

	/// Toggles the given extended list view styles by sending an
	/// [`LVM_SETEXTENDEDLISTVIEWSTYLE`](crate::msg::lvm::SetExtendedListViewStyle)
	/// message.
	pub fn toggle_extended_style(&self, set: bool, ex_style: co::LVS_EX) {
		self.hwnd().SendMessage(lvm::SetExtendedListViewStyle {
			mask: ex_style,
			style: if set { ex_style } else { co::LVS_EX::NONE },
		});
	}

	fn show_context_menu(&self,
		follow_cursor: bool, has_ctrl: bool, has_shift: bool) -> WinResult<()>
	{
		let hmenu = match self.0.context_menu {
			Some(h) => h,
			None => return Ok(()), // no menu, nothing to do
		};

		let menu_pos = if follow_cursor { // usually when fired by a right-click
			let mut menu_pos = GetCursorPos()?; // relative to screen
			self.hwnd().ScreenToClient(&mut menu_pos)?; // now relative to list view

			let mut lvhti = LVHITTESTINFO::default(); // find item below cursor, if any
			lvhti.pt = menu_pos;

			match self.items().hit_test(&mut lvhti) {
				Some(idx) => { // an item was right-clicked
					if !has_ctrl && !has_shift {
						if !self.items().is_selected(idx) {
							self.items().set_selected_all(false)?;
							self.items().set_selected(true, &[idx])?;
						}
						self.items().set_focused(idx)?;
					}
				},
				None => { // no item was right-clicked
					self.items().set_selected_all(false)?;
				},
			}
			self.hwnd().SetFocus(); // because a right-click won't set the focus by itself
			menu_pos

		} else { // usually fired by the context meny key
			let focused_idx_opt = self.items().focused();

			if focused_idx_opt.is_some() && self.items().is_visible(focused_idx_opt.unwrap()) {
				let focused_idx = focused_idx_opt.unwrap();
				let rc_item = self.items().rect(focused_idx, co::LVIR::BOUNDS)?;
				POINT::new(rc_item.left + 16,
					rc_item.top + (rc_item.bottom - rc_item.top) / 2)

			} else { // no item is focused and visible
				POINT::new(6, 10) // arbitrary
			}
		};

		hmenu.TrackPopupMenuAtPoint(
			menu_pos, self.hwnd().GetParent()?, self.hwnd())
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`ListView`](crate::gui::ListView) programmatically with
/// [`ListView::new`](crate::gui::ListView::new).
pub struct ListViewOpts {
	/// Control position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Control size, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 50 x 50.
	pub size: SIZE,
	/// List view styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `LVS::REPORT | LVS::NOSORTHEADER | LVS::SHOWSELALWAYS | LVS::SHAREIMAGELISTS`.
	pub list_view_style: co::LVS,
	/// Extended list view styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `LVS_EX::NONE`.
	pub list_view_ex_style: co::LVS_EX,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::CLIENTEDGE`.
	pub window_ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: i32,
	/// Context popup menu.
	///
	/// This menu is shared: it must be destroyed manually after the control is
	/// destroyed. But note that menus loaded from resources don't need to be
	/// destroyed.
	///
	/// Defaults to `None`.
	pub context_menu: Option<HMENU>,
}

impl Default for ListViewOpts {
	fn default() -> Self {
		Self {
			position: POINT::new(0, 0),
			size: SIZE::new(50, 50),
			list_view_style: co::LVS::REPORT | co::LVS::NOSORTHEADER | co::LVS::SHOWSELALWAYS | co::LVS::SHAREIMAGELISTS,
			list_view_ex_style: co::LVS_EX::NONE,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
			context_menu: None,
		}
	}
}

impl ListViewOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
