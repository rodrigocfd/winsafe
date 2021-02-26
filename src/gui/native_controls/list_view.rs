use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::{GetAsyncKeyState, PostQuitMessage};
use crate::gui::events::{ListViewEvents, MsgEvents};
use crate::gui::native_controls::native_control_base::{NativeControlBase, OptsId};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi};
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;
use crate::msg;
use crate::structs::{LVCOLUMN, LVITEM, NMLVKEYDOWN, POINT, SIZE};
use crate::WString;

/// Native
/// [list view](https://docs.microsoft.com/en-us/windows/win32/controls/list-view-controls-overview)
/// control.
///
/// Not to be confused with the simpler [list box](crate::gui::ListBox) control.
#[derive(Clone)]
pub struct ListView(Arc<Obj>);

struct Obj { // actual fields of ListView
	base: NativeControlBase<ListViewEvents>,
	opts_id: OptsId<ListViewOpts>,
}

unsafe impl Send for ListView {}
unsafe impl Sync for ListView {}

impl Child for ListView {
	fn hctrl_ref(&self) -> &HWND {
		self.0.base.hctrl_ref()
	}
}

impl ListView {
	/// Instantiates a new `ListView` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: ListViewOpts) -> ListView {
		let opts = ListViewOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;
		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(
						parent,
						ListViewEvents::new(parent, opts.ctrl_id),
					),
					opts_id: OptsId::Wnd(opts),
				},
			),
		);
		parent.privileged_events_ref().wm_create({
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});
		new_self.handled_events(parent, ctrl_id);
		new_self
	}

	/// Instantiates a new `ListView` object, to be loaded from a dialog resource
	/// with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> ListView {
		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(
						parent,
						ListViewEvents::new(parent, ctrl_id),
					),
					opts_id: OptsId::Dlg(ctrl_id),
				},
			),
		);
		parent.privileged_events_ref().wm_init_dialog({
			let me = new_self.clone();
			move |_| { me.create(); true }
		});
		new_self.handled_events(parent, ctrl_id);
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
						opts.ex_window_style,
						opts.window_style | opts.list_view_style.into(),
					)?;

					if opts.ex_list_view_style != co::LVS_EX::NONE {
						self.toggle_extended_style(true, opts.ex_list_view_style);
					}
					Ok(())
				},
				OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

	fn handled_events(&self, parent: &dyn Parent, ctrl_id: u16) {
		parent.privileged_events_ref().add_nfy(ctrl_id, co::LVN::KEYDOWN.into(), {
			let me = self.clone();
			move |p| {
				let lvnk = unsafe { p.cast_nmhdr::<NMLVKEYDOWN>() };

				if lvnk.wVKey == co::VK('A' as u16)
					&& GetAsyncKeyState(co::VK::CONTROL) // Ctrl+A pressed?
				{
					me.set_selected_all_items(true)
						.unwrap_or_else(|err| PostQuitMessage(err));
				}
				None
			}
		});
	}

	hwnd_ctrlid_on_onsubclass!(ListViewEvents);

	/// Adds many columns at once.
	///
	/// Widths will be adjusted to match current system DPI.
	pub fn add_columns(&self,
		texts_and_widths: &[(&str, u32)]) -> WinResult<()>
	{
		for (text, width) in texts_and_widths.iter() {
			let mut col_cx = SIZE::new(*width as i32, 0);
			multiply_dpi(None, Some(&mut col_cx))?;

			let mut lvc = LVCOLUMN::default();
			lvc.mask = co::LVCF::TEXT | co::LVCF::WIDTH;
			lvc.cx = col_cx.cx;

			let mut wtext = WString::from_str(text);
			lvc.set_pszText(&mut wtext);

			self.hwnd().SendMessage(msg::LvmInsertColumn {
				index: 0xffff,
				lvcolumn: &lvc,
			})?;
		}

		Ok(())
	}

	/// Appends a new item, returning its index.
	pub fn add_item(&self,
		text: &str, icon_index: Option<u32>) -> WinResult<u32>
	{
		let mut lvi = LVITEM::default();
		lvi.mask = co::LVIF::TEXT | co::LVIF::IMAGE;
		lvi.iItem = 0x0fff_ffff; // insert as the last one

		lvi.iImage = match icon_index {
			Some(idx) => idx as i32,
			None => -1,
		};

		let mut wtext = WString::from_str(text);
		lvi.set_pszText(&mut wtext);

		self.hwnd().SendMessage(msg::LvmInsertItem { lvitem: &lvi })
	}

	/// Retrieves the number of columns.
	pub fn column_count(&self) -> WinResult<u32> {
		self.hwnd().SendMessage(msg::LvmGetHeader {})?
			.SendMessage(msg::HdmGetItemCount {})
	}

	/// Retrieves the current view.
	pub fn current_view(&self) -> co::LV_VIEW {
		self.hwnd().SendMessage(msg::LvmGetView {})
	}

	/// Deletes all items.
	pub fn delete_all_items(&self) -> WinResult<()> {
		self.hwnd().SendMessage(msg::LvmDeleteAllItems {})
	}

	/// Deletes the items at the given indexes.
	pub fn delete_items(&self, indexes: &[u32]) -> WinResult<()> {
		for idx in indexes.iter() {
			self.hwnd().SendMessage(msg::LvmDeleteItem {
				index: *idx as i32,
			})?;
		}
		Ok(())
	}

	/// Ensures that an item is visible in the list.
	pub fn ensure_item_visible(&self, index: u32) -> WinResult<()> {
		self.hwnd().SendMessage(msg::LvmEnsureVisible {
			index: index as i32,
			entirely_visible: true,
		})
	}

	/// Retrieves the index of the focused item.
	pub fn focused_item(&self) -> Option<u32> {
		self.hwnd().SendMessage(msg::LvmGetNextItem {
			initial_index: -1,
			relationship: co::LVNI::FOCUSED,
		})
	}

	/// Tells if the item is the focused one.
	pub fn is_item_focused(&self, index: u32) -> bool {
		self.hwnd().SendMessage(msg::LvmGetItemState {
			index: index as i32,
			mask: co::LVIS::FOCUSED,
		}).has(co::LVIS::FOCUSED)
	}

	/// Tells if the item is selected.
	pub fn is_item_selected(&self, index: u32) -> bool {
		self.hwnd().SendMessage(msg::LvmGetItemState {
			index: index as i32,
			mask: co::LVIS::SELECTED,
		}).has(co::LVIS::SELECTED)
	}

	/// Tells if the item is currently visible.
	pub fn is_item_visible(&self, index: u32) -> bool {
		self.hwnd().SendMessage(msg::LvmIsItemVisible { index: index as i32 })
	}

	/// Retrieves the total number of items.
	pub fn item_count(&self) -> u32 {
		self.hwnd().SendMessage(msg::LvmGetItemCount {})
	}

	/// Retrieves the text of an item under any column.
	pub fn item_text(&self, item_index: u32, column_index: u32) -> String {
		// https://forums.codeguru.com/showthread.php?351972-Getting-listView-item-text-length
		const BLOCK: usize = 64; // arbitrary
		let mut buf_sz = BLOCK;

		loop {
			let mut lvi = LVITEM::default();
			lvi.iSubItem = column_index as i32;

			let mut buf = WString::new_alloc_buffer(buf_sz);
			lvi.set_pszText(&mut buf);

			let nchars = self.hwnd().SendMessage(msg::LvmGetItemText {
				index: item_index as i32,
				lvitem: &mut lvi,
			});

			if (nchars as usize) < buf_sz { // to break, must have at least 1 char gap
				return buf.to_string();
			}

			buf_sz += BLOCK; // increase buffer size to try again
		}
	}

	/// Sets the current view.
	pub fn set_current_view(&self, view: co::LV_VIEW) -> WinResult<()> {
		self.hwnd().SendMessage(msg::LvmSetView { view })
	}

	/// Retrieves the number of selected items.
	pub fn selected_item_count(&self) -> u32 {
		self.hwnd().SendMessage(msg::LvmGetSelectedCount {})
	}

	/// Retrieves the indexes of the selected items.
	pub fn selected_items(&self) -> Vec<u32> {
		let mut items = Vec::with_capacity(self.selected_item_count() as usize);
		let mut idx = -1;

		loop {
			idx = match self.hwnd().SendMessage(msg::LvmGetNextItem {
				initial_index: idx,
				relationship: co::LVNI::SELECTED,
			}) {
				Some(idx) => idx as i32,
				None => break,
			};
			items.push(idx as u32);
		}
		items
	}

	/// Sets the focused item.
	pub fn set_focused_item(&self, index: u32) -> WinResult<()> {
		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::FOCUSED;
		lvi.state = co::LVIS::FOCUSED;

		self.hwnd().SendMessage(msg::LvmSetItemState {
			index: index as i32,
			lvitem: &lvi,
		})
	}

	/// Sets the text of an item under any column.
	pub fn set_item_text(&self,
		item_index: u32, column_index: u32, text: &str) -> WinResult<()>
	{
		let mut lvi = LVITEM::default();
		lvi.iSubItem = column_index as i32;

		let mut wtext = WString::from_str(text);
		lvi.set_pszText(&mut wtext);

		self.hwnd().SendMessage(msg::LvmSetItemText {
			index: item_index as i32,
			lvitem: &lvi,
		})
	}

	/// Sets or remove the selection for all items.
	pub fn set_selected_all_items(&self, set: bool) -> WinResult<()> {
		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::SELECTED;
		if set { lvi.state = co::LVIS::SELECTED; }

		self.hwnd().SendMessage(msg::LvmSetItemState {
			index: -1,
			lvitem: &lvi,
		})
	}

	/// Sets or remove the selection from the given item indexes.
	pub fn set_selected_items(&self,
		set: bool, indexes: &[u32]) -> WinResult<()>
	{
		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::SELECTED;
		if set { lvi.state = co::LVIS::SELECTED; }

		for idx in indexes.iter() {
			self.hwnd().SendMessage(msg::LvmSetItemState {
				index: *idx as i32,
				lvitem: &lvi,
			})?;
		}
		Ok(())
	}

	/// Toggles the given extended list view styles.
	pub fn toggle_extended_style(&self, set: bool, ex_style: co::LVS_EX) {
		self.hwnd().SendMessage(msg::LvmSetExtendedListViewStyle {
			mask: ex_style,
			style: if set { ex_style } else { co::LVS_EX::NONE },
		});
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`ListView`](crate::gui::ListView) programatically with
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
	pub ex_list_view_style: co::LVS_EX,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::CLIENTEDGE`.
	pub ex_window_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
}

impl Default for ListViewOpts {
	fn default() -> Self {
		Self {
			position: POINT::new(0, 0),
			size: SIZE::new(50, 50),
			list_view_style: co::LVS::REPORT | co::LVS::NOSORTHEADER | co::LVS::SHOWSELALWAYS | co::LVS::SHAREIMAGELISTS,
			ex_list_view_style: co::LVS_EX::NONE,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			ex_window_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
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
