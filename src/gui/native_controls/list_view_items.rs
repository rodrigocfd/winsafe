use std::cell::Cell;
use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::co;
use crate::handles::HWND;
use crate::msg::lvm;
use crate::structs::{LVFINDINFO, LVHITTESTINFO, LVITEM, RECT};
use crate::various::WString;

/// Exposes item methods of a [`ListView`](crate::gui::ListView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListViewItems {
	hwnd_ptr: Cell<NonNull<HWND>>,
}

impl ListViewItems {
	pub(in crate::gui::native_controls) fn new() -> ListViewItems {
		Self {
			hwnd_ptr: Cell::new(NonNull::from(&HWND::NULL)), // initially invalid
		}
	}

	pub(in crate::gui::native_controls) fn set_hwnd_ref(&self, hwnd_ref: &HWND) {
		self.hwnd_ptr.replace(NonNull::from(hwnd_ref));
	}

	pub(in crate::gui::native_controls) fn hwnd(&self) -> HWND {
		unsafe { *self.hwnd_ptr.get().as_ref() }
	}

	/// Appends a new item by sending an
	/// [`lvm::InsertItem`](crate::msg::lvm::InsertItem) message, and returns
	/// its index.
	///
	/// The texts are relative to each column.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui;
	///
	/// let my_list: gui::ListView; // initialized somewhere
	///
	/// my_list.items().add(
	///     &[
	///         "First column text",
	///         "Second column text",
	///     ],
	///     None, // no icon; requires set_image_list() before
	/// ).unwrap();
	/// ```
	///
	/// # Panics
	///
	/// Panics if `texts` is empty, or if the number of texts is greater than
	/// the number of columns.
	pub fn add<S: AsRef<str>>(&self,
		texts: &[S], icon_index: Option<u32>) -> WinResult<u32>
	{
		if texts.is_empty() {
			panic!("No texts passed when adding a ListView item.");
		}

		let mut lvi = LVITEM::default();
		lvi.mask = co::LVIF::TEXT | co::LVIF::IMAGE;
		lvi.iItem = 0x0fff_ffff; // insert as the last item

		lvi.iImage = match icon_index {
			Some(idx) => idx as _,
			None => -1,
		};

		let mut wtext = WString::from_str(texts[0].as_ref());
		lvi.set_pszText(Some(&mut wtext));

		let new_idx = self.hwnd().SendMessage(lvm::InsertItem { lvitem: &lvi })?;

		for (idx, text) in texts.iter().skip(1).enumerate() { // subsequent columns
			self.set_text(new_idx, idx as u32 + 1, text.as_ref())?;
		}

		Ok(new_idx)
	}

	/// Retrieves the total number of items by sending an
	/// [`lvm::GetItemCount`](crate::msg::lvm::GetItemCount) message.
	pub fn count(&self) -> u32 {
		self.hwnd().SendMessage(lvm::GetItemCount {})
	}

	/// Deletes the items at the given indexes by sending an
	/// [`lvm::DeleteItem`](crate::msg::lvm::DeleteItem) message.
	///
	/// The indexes are iterated backwards, so the last item will be deleted
	/// first.
	pub fn delete(&self, item_indexes: &[u32]) -> WinResult<()> {
		for idx in item_indexes.iter().rev() {
			self.hwnd().SendMessage(lvm::DeleteItem {
				index: *idx,
			})?;
		}
		Ok(())
	}

	/// Deletes all items by sending an
	/// [`lvm::DeleteAllItems`](crate::msg::lvm::DeleteAllItems) message.
	pub fn delete_all(&self) -> WinResult<()> {
		self.hwnd().SendMessage(lvm::DeleteAllItems {})
	}

	/// Deletes the selected items by sending
	/// [`lvm::DeleteAllItems`](crate::msg::lvm::DeleteAllItems) messages.
	pub fn delete_selected(&self) -> WinResult<()> {
		loop {
			match self.hwnd().SendMessage(lvm::GetNextItem {
				initial_index: None,
				relationship: co::LVNI::SELECTED,
			}) {
				Some(index) => self.hwnd().SendMessage(lvm::DeleteItem { index })?,
				None => break,
			};
		}
		Ok(())
	}

	/// Scrolls the list by sending an
	/// [`lvm::EnsureVisible`](crate::msg::lvm::EnsureVisible) message so that
	/// an item is visible in the list.
	pub fn ensure_visible(&self, item_index: u32) -> WinResult<()> {
		self.hwnd().SendMessage(lvm::EnsureVisible {
			index: item_index,
			entirely_visible: true,
		})
	}

	/// Searches for an item with the given text, case-insensitive, by sending
	/// an [`lvm::FindItem`](crate::msg::lvm::FindItem) message.
	pub fn find(&self, text: &str) -> Option<u32> {
		let mut buf = WString::from_str(text);

		let mut lvfi = LVFINDINFO::default();
		lvfi.flags = co::LVFI::STRING;
		lvfi.set_psz(Some(&mut buf));

		self.hwnd().SendMessage(lvm::FindItem {
			start_index: None,
			lvfindinfo: &mut lvfi,
		})
	}

	/// Retrieves the index of the focused item by sending an
	/// [`lvm::GetNextItem`](crate::msg::lvm::GetNextItem) message.
	pub fn focused(&self) -> Option<u32> {
		self.hwnd().SendMessage(lvm::GetNextItem {
			initial_index: None,
			relationship: co::LVNI::FOCUSED,
		})
	}

	/// Retrieves the item at the specified position by sending an
	/// [`lvm::HitTest`](crate::msg::lvm::HitTest) message
	pub fn hit_test(&self, info: &mut LVHITTESTINFO) -> Option<u32> {
		self.hwnd().SendMessage(lvm::HitTest { info })
	}

	/// Tells if the item is the focused one by sending an
	/// [`lvm::GetItemState`](crate::msg::lvm::GetItemState) message.
	pub fn is_focused(&self, item_index: u32) -> bool {
		self.hwnd().SendMessage(lvm::GetItemState {
			index: item_index,
			mask: co::LVIS::FOCUSED,
		}).has(co::LVIS::FOCUSED)
	}

	/// Tells if the item is selected by sending an
	/// [`lvm::GetItemState`](crate::msg::lvm::GetItemState) message.
	pub fn is_selected(&self, item_index: u32) -> bool {
		self.hwnd().SendMessage(lvm::GetItemState {
			index: item_index,
			mask: co::LVIS::SELECTED,
		}).has(co::LVIS::SELECTED)
	}

	/// Tells if the item is currently visible by sending an
	/// [`lvm::IsItemVisible`](crate::msg::lvm::IsItemVisible) message.
	pub fn is_visible(&self, item_index: u32) -> bool {
		self.hwnd().SendMessage(lvm::IsItemVisible { index: item_index })
	}

	/// Retrieves the actual index of the unique ID by sending an
	/// [`lvm::MapIdToIndex`](crate::msg::lvm::MapIdToIndex) message.
	pub fn map_id_to_index(&self, item_id: u32) -> Option<u32> {
		self.hwnd().SendMessage(lvm::MapIdToIndex { id: item_id })
	}

	/// Retrieves an unique ID for the given index by sending an
	/// [`lvm::MapIndexToId`](crate::msg::lvm::MapIndexToId) message.
	pub fn map_index_to_id(&self, item_index: u32) -> Option<u32> {
		self.hwnd().SendMessage(lvm::MapIndexToId { index: item_index })
	}

	/// Retrieves the bound rectangle of item by sending an
	/// [`lvm::GetItemRect`](crate::msg::lvm::GetItemRect) message.
	pub fn rect(&self, item_index: u32, portion: co::LVIR) -> WinResult<RECT> {
		let mut rc = RECT::default();
		self.hwnd().SendMessage(lvm::GetItemRect {
			index: item_index,
			rect: &mut rc,
			portion,
		})?;
		Ok(rc)
	}

	/// Retrieves the indexes of the selected items by sending
	/// [`lvm::GetNextItem`](crate::msg::lvm::GetNextItem) messages.
	pub fn selected(&self) -> Vec<u32> {
		let mut items = Vec::with_capacity(self.selected_count() as _);
		let mut idx = None; // will start at first item

		loop {
			idx = match self.hwnd().SendMessage(lvm::GetNextItem {
				initial_index: idx,
				relationship: co::LVNI::SELECTED,
			}) {
				Some(idx) => {
					items.push(idx);
					Some(idx) // update start item
				},
				None => break,
			};
		}
		items
	}

	/// Retrieves the number of selected items by sending an
	/// [`lvm::GetSelectedCount`](crate::msg::lvm::GetSelectedCount) message.
	pub fn selected_count(&self) -> u32 {
		self.hwnd().SendMessage(lvm::GetSelectedCount {})
	}

	/// Sets the focused item by sending an
	/// [`lvm:SetItemState`](crate::msg::lvm::SetItemState) message.
	pub fn set_focused(&self, item_index: u32) -> WinResult<()> {
		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::FOCUSED;
		lvi.state = co::LVIS::FOCUSED;

		self.hwnd().SendMessage(lvm::SetItemState {
			index: Some(item_index),
			lvitem: &lvi,
		})
	}

	/// Sets or remove the selection from the given item indexes by sending
	/// [`lvm::SetItemState`](crate::msg::lvm::SetItemState) messages.
	pub fn set_selected(&self,
		set: bool, item_indexes: &[u32]) -> WinResult<()>
	{
		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::SELECTED;
		if set { lvi.state = co::LVIS::SELECTED; }

		for idx in item_indexes.iter() {
			self.hwnd().SendMessage(lvm::SetItemState {
				index: Some(*idx),
				lvitem: &lvi,
			})?;
		}
		Ok(())
	}

	/// Sets or remove the selection for all items by sending an
	/// [`lvm::SetItemState`](crate::msg::lvm::SetItemState) message.
	pub fn set_selected_all(&self, set: bool) -> WinResult<()> {
		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::SELECTED;
		if set { lvi.state = co::LVIS::SELECTED; }

		self.hwnd().SendMessage(lvm::SetItemState {
			index: None,
			lvitem: &lvi,
		})
	}

	/// Sets the text of an item under a column by sending an
	/// [`lvm::SetItemText`](crate::msg::lvm::SetItemText) message.
	pub fn set_text(&self,
		item_index: u32, column_index: u32, text: &str) -> WinResult<()>
	{
		let mut lvi = LVITEM::default();
		lvi.iSubItem = column_index as _;

		let mut wtext = WString::from_str(text);
		lvi.set_pszText(Some(&mut wtext));

		self.hwnd().SendMessage(lvm::SetItemText {
			index: item_index,
			lvitem: &lvi,
		})
	}

	/// Retrieves the text of an item under a column by sending an
	/// [`lvm::GetItemText`](crate::msg::lvm::GetItemText) message.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui;
	///
	/// let my_list: gui::ListView; // initialized somewhere
	///
	/// println!("Text: {}", my_list.items().text(0, 2)); // 1st item, 3rd column
	/// ```
	pub fn text(&self, item_index: u32, column_index: u32) -> String {
		let mut buf = WString::default();
		Self::text_retrieve(self.hwnd(), item_index, column_index, &mut buf);
		buf.to_string()
	}

	pub(in crate::gui::native_controls) fn text_retrieve(
		hwnd: HWND, item_index: u32, column_index: u32, buf: &mut WString)
	{
		// Static method because it's also used by ListViewColumns.

		// https://forums.codeguru.com/showthread.php?351972-Getting-listView-item-text-length
		const BLOCK: usize = 64; // arbitrary
		let mut buf_sz = BLOCK;

		let mut buf = buf;

		loop {
			let mut lvi = LVITEM::default();
			lvi.iSubItem = column_index as _;

			buf.realloc_buffer(buf_sz);
			lvi.set_pszText(Some(&mut buf));

			let nchars = hwnd.SendMessage(lvm::GetItemText { // char count without terminating null
				index: item_index,
				lvitem: &mut lvi,
			});

			if (nchars as usize) + 1 < buf_sz { // to break, must have at least 1 char gap
				break;
			}

			buf_sz += BLOCK; // increase buffer size to try again
		}
	}
}
