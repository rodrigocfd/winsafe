use std::marker::PhantomData;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::{GetAsyncKeyState, GetCursorPos};
use crate::gui::base::Base;
use crate::gui::events::{
	EventsView,
	ListViewEvents,
	sealed_events_wm_nfy::SealedEventsWmNfy,
};
use crate::gui::native_controls::base_native_control::{BaseNativeControl, OptsId};
use crate::gui::native_controls::list_view_columns::ListViewColumns;
use crate::gui::native_controls::list_view_items::ListViewItems;
use crate::gui::privs::{auto_ctrl_id, multiply_dpi};
use crate::gui::resizer::{Horz, Vert};
use crate::gui::traits::{baseref_from_parent, Child, Parent, Window};
use crate::handles::{HIMAGELIST, HMENU};
use crate::msg::{lvm, wm};
use crate::structs::{NMITEMACTIVATE, NMLVKEYDOWN, POINT, SIZE};

struct Obj { // actual fields of ListView
	base: BaseNativeControl,
	opts_id: OptsId<ListViewOpts>,
	events: ListViewEvents,
	context_menu: Option<HMENU>,
}

impl_obj_window!(Obj);
impl_obj_child!(Obj);
impl_obj_nativecontrol!(Obj);

//------------------------------------------------------------------------------

/// Native
/// [list view](https://docs.microsoft.com/en-us/windows/win32/controls/list-view-controls-overview)
/// control. Not to be confused with the simpler [list box](crate::gui::ListBox)
/// control.
#[derive(Clone)]
pub struct ListView(Arc<Obj>);

impl_send_sync!(ListView);
impl_ctl_debug!(ListView);

impl_ctl_window!(ListView);
impl_ctl_aswindow!(ListView);
impl_ctl_child!(ListView);
impl_ctl_nativecontrol!(ListView);
impl_ctl_asnativecontrol!(ListView);
impl_ctl_nativecontrolevents!(ListView, ListViewEvents);
impl_ctl_focus!(ListView);

impl ListView {
	/// Instantiates a new `ListView` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &impl Parent, opts: ListViewOpts) -> ListView {
		let parent_base_ref = baseref_from_parent(parent);
		let opts = ListViewOpts::define_ctrl_id(opts);
		let (ctrl_id, horz, vert) = (opts.ctrl_id, opts.horz_resize, opts.vert_resize);
		let context_menu = opts.context_menu;

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Wnd(opts),
					events: ListViewEvents::new(parent_base_ref, ctrl_id),
					context_menu,
				},
			),
		);

		parent_base_ref.privileged_events_ref().wm(parent_base_ref.create_or_initdlg(), {
			let self2 = new_self.clone();
			move |_| { self2.create(horz, vert)?; Ok(0) }
		});

		new_self.handled_events(parent_base_ref, ctrl_id);
		new_self
	}

	/// Instantiates a new `ListView` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// **Note:** The optional `context_menu` is shared: it must be destroyed
	/// manually after the control is destroyed. But note that menus loaded from
	/// resources don't need to be destroyed.
	pub fn new_dlg(
		parent: &impl Parent, ctrl_id: u16,
		horz_resize: Horz, vert_resize: Vert,
		context_menu: Option<HMENU>) -> ListView
	{
		let parent_base_ref = baseref_from_parent(parent);

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Dlg(ctrl_id),
					events: ListViewEvents::new(parent_base_ref, ctrl_id),
					context_menu,
				},
			),
		);

		parent_base_ref.privileged_events_ref().wm_init_dialog({
			let self2 = new_self.clone();
			move |_| { self2.create(horz_resize, vert_resize)?; Ok(true) }
		});

		new_self.handled_events(parent_base_ref, ctrl_id);
		new_self
	}

	fn create(&self, horz: Horz, vert: Vert) -> WinResult<()> {
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

				if opts.list_view_ex_style != co::LVS_EX::NoValue {
					self.set_extended_style(true, opts.list_view_ex_style);
				}

				self.columns().add(&opts.columns)?;
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ())?, // may panic
		}

		self.0.base.parent_base_ref().resizer_add(
			self.0.base.parent_base_ref(), self.0.base.hwnd_ref(), horz, vert)
	}

	fn handled_events(&self, parent_base_ref: &Base, ctrl_id: u16) {
		parent_base_ref.privileged_events_ref().add_nfy(ctrl_id, co::LVN::KEYDOWN.into(), {
			let self2 = self.clone();
			move |p| {
				let lvnk = unsafe { p.cast_nmhdr::<NMLVKEYDOWN>() };
				let has_ctrl = GetAsyncKeyState(co::VK::CONTROL);
				let has_shift = GetAsyncKeyState(co::VK::SHIFT);

				if has_ctrl && lvnk.wVKey == co::VK('A' as _) { // Ctrl+A
					self2.items().select_all(true)?;
				} else if lvnk.wVKey == co::VK::APPS { // context menu key
					self2.show_context_menu(false, has_ctrl, has_shift)?;
				}
				Ok(None)
			}
		});

		parent_base_ref.privileged_events_ref().add_nfy(ctrl_id, co::NM::RCLICK.into(), {
			let self2 = self.clone();
			move |p| {
				let nmia = unsafe { p.cast_nmhdr::<NMITEMACTIVATE>() };
				let has_ctrl = nmia.uKeyFlags.has(co::LVKF::CONTROL);
				let has_shift = nmia.uKeyFlags.has(co::LVKF::SHIFT);

				self2.show_context_menu(true, has_ctrl, has_shift)?;
				Ok(None)
			}
		});
	}

	/// Exposes the column methods.
	pub fn columns<'a>(&'a self) -> ListViewColumns<'a> {
		ListViewColumns {
			hwnd: self.hwnd(),
			owner: PhantomData,
		}
	}

	/// Returns the context menu attached to this list view, if any.
	///
	/// The context menu is attached when the list view is created, either by
	/// calling [`ListView::new`](crate::gui::ListView::new) or
	/// [`ListView::new_dlg`](crate::gui::ListView::new_dlg).
	pub fn context_menu(&self) -> Option<HMENU> {
		self.0.context_menu
	}

	/// Retrieves one of the associated image lists by sending an
	/// [`lvm::GetImageList`](crate::msg::lvm::GetImageList) message.
	pub fn image_list(&self, kind: co::LVSIL) -> Option<HIMAGELIST> {
		self.hwnd().SendMessage(lvm::GetImageList { kind })
	}

	/// Exposes the item methods.
	pub fn items<'a>(&'a self) -> ListViewItems<'a> {
		ListViewItems {
			hwnd: self.hwnd(),
			owner: PhantomData,
		}
	}

	/// Retrieves the current view by sending an
	/// [`lvm::GetView`](crate::msg::lvm::GetView) message.
	pub fn current_view(&self) -> co::LV_VIEW {
		self.hwnd().SendMessage(lvm::GetView {})
	}

	/// Sets the current view by sending an
	/// [`lvm::SetView`](crate::msg::lvm::SetView) message.
	pub fn set_current_view(&self, view: co::LV_VIEW) -> WinResult<()> {
		self.hwnd().SendMessage(lvm::SetView { view })
	}

	/// Sets or unsets the given extended list view styles by sending an
	/// [`lvm::SetExtendedListViewStyle`](crate::msg::lvm::SetExtendedListViewStyle)
	/// message.
	pub fn set_extended_style(&self, set: bool, ex_style: co::LVS_EX) {
		self.hwnd().SendMessage(lvm::SetExtendedListViewStyle {
			mask: ex_style,
			style: if set { ex_style } else { co::LVS_EX::NoValue },
		});
	}

	/// Sets the one of the associated image lists by sending an
	/// [`lvm::SetImageList`](crate::msg::lvm::SetImageList) message.
	///
	/// Returns the previous image list, if any.
	pub fn set_image_list(&self,
		kind: co::LVSIL, himagelist: HIMAGELIST) -> Option<HIMAGELIST>
	{
		self.hwnd().SendMessage(lvm::SetImageList { kind, himagelist })
	}

	/// Allows or disallows the redrawing of the control by sending a
	/// [`wm::SetRedraw`](crate::msg::wm::SetRedraw) message.
	pub fn set_redraw(&self, can_redraw: bool) {
		self.hwnd().SendMessage(wm::SetRedraw { can_redraw });
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

			match self.items().hit_test(menu_pos) {
				Some(item_over) => {
					if !has_ctrl && !has_shift {
						item_over.select(true)?; // if not yet
						item_over.focus()?;
					}
				},
				None => { // no item was right-clicked
					self.items().select_all(false)?;
				},
			}

			self.hwnd().SetFocus(); // because a right-click won't set the focus by itself
			menu_pos

		} else { // usually fired by the context meny key
			let focused_opt = self.items().focused();

			if focused_opt.is_some() && focused_opt.unwrap().is_visible() {
				let focused = focused_opt.unwrap();
				let rc_item = focused.rect(co::LVIR::BOUNDS)?;
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
	/// Defaults to `LVS_EX::NoValue`.
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
	pub ctrl_id: u16,
	/// Horizontal behavior when the parent is resized.
	///
	/// Defaults to `Horz::None`.
	pub horz_resize: Horz,
	/// Vertical behavior when the parent is resized.
	///
	/// Defaults to `Vert::None`.
	pub vert_resize: Vert,

	/// Context popup menu.
	///
	/// This menu is shared: it must be destroyed manually after the control is
	/// destroyed. But note that menus loaded from resources don't need to be
	/// destroyed.
	///
	/// Defaults to `None`.
	pub context_menu: Option<HMENU>,
	/// Text and width of columns to be added right away. The columns only show
	/// in report mode.
	///
	/// Defaults to none.
	pub columns: Vec<(String, u32)>,
}

impl Default for ListViewOpts {
	fn default() -> Self {
		Self {
			position: POINT::new(0, 0),
			size: SIZE::new(50, 50),
			list_view_style: co::LVS::REPORT | co::LVS::NOSORTHEADER | co::LVS::SHOWSELALWAYS | co::LVS::SHAREIMAGELISTS,
			list_view_ex_style: co::LVS_EX::NoValue,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
			horz_resize: Horz::None,
			vert_resize: Vert::None,
			context_menu: None,
			columns: Vec::default(),
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
