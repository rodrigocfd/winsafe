use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::co;
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::handles::HWND;
use crate::msg::lvm;
use crate::structs::{LVITEM};
use crate::WString;

/// Exposes item methods of a [`ListView`](crate::gui::ListView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListViewItems {
	hwnd_ptr: VeryUnsafeCell<NonNull<HWND>>,
}

impl ListViewItems {
	pub(crate) fn new(hwnd_ref: &HWND) -> ListViewItems {
		Self {
			hwnd_ptr: VeryUnsafeCell::new(NonNull::from(hwnd_ref)), // ref implicitly converted to pointer
		}
	}

	pub(crate) fn set_hwnd_ref(&self, hwnd_ref: &HWND) {
		*self.hwnd_ptr.as_mut() = NonNull::from(hwnd_ref); // ref implicitly converted to pointer
	}

	pub(crate) fn hwnd(&self) -> HWND {
		unsafe { *self.hwnd_ptr.as_ref() }
	}

	/// Appends a new item by sending an
	/// [`LVM_INSERTITEM`](crate::msg::lvm::InsertItem) message, and returns its
	/// index.
	pub fn add(&self, text: &str, icon_index: Option<u32>) -> WinResult<u32> {
		let mut lvi = LVITEM::default();
		lvi.mask = co::LVIF::TEXT | co::LVIF::IMAGE;
		lvi.iItem = 0x0fff_ffff; // insert as the last one

		lvi.iImage = match icon_index {
			Some(idx) => idx as i32,
			None => -1,
		};

		let mut wtext = WString::from_str(text);
		lvi.set_pszText(&mut wtext);

		self.hwnd().SendMessage(lvm::InsertItem { lvitem: &lvi })
	}

	/// Retrieves the total number of items by sending an
	/// [`LVM_GETITEMCOUNT`](crate::msg::lvm::GetItemCount) message.
	pub fn count(&self) -> u32 {
		self.hwnd().SendMessage(lvm::GetItemCount {})
	}

	/// Deletes the items at the given indexes by sending an
	/// [`LVM_DELETEITEM`](crate::msg::lvm::DeleteItem) message
	pub fn delete(&self, item_indexes: &[u32]) -> WinResult<()> {
		for idx in item_indexes.iter() {
			self.hwnd().SendMessage(lvm::DeleteItem {
				index: *idx as i32,
			})?;
		}
		Ok(())
	}

	/// Deletes all items by sending an
	/// [`LVM_DELETEALLITEMS`](crate::msg::lvm::DeleteAllItems) message.
	pub fn delete_all(&self) -> WinResult<()> {
		self.hwnd().SendMessage(lvm::DeleteAllItems {})
	}

	/// Scrolls the list by sending an
	/// [`LVM_ENSUREVISIBLE`](crate::msg::lvm::EnsureVisible) message so that an
	/// item is visible in the list.
	pub fn ensure_visible(&self, item_index: u32) -> WinResult<()> {
		self.hwnd().SendMessage(lvm::EnsureVisible {
			index: item_index as i32,
			entirely_visible: true,
		})
	}

	/// Retrieves the index of the focused item by sending an
	/// [`LVM_GETNEXTITEM`](crate::msg::lvm::GetNextItem) message
	pub fn focused(&self) -> Option<u32> {
		self.hwnd().SendMessage(lvm::GetNextItem {
			initial_index: -1,
			relationship: co::LVNI::FOCUSED,
		})
	}

	/// Tells if the item is the focused one by sending an
	/// [`LVM_GETITEMSTATE`](crate::msg::lvm::GetItemState) message.
	pub fn is_focused(&self, item_index: u32) -> bool {
		self.hwnd().SendMessage(lvm::GetItemState {
			index: item_index as i32,
			mask: co::LVIS::FOCUSED,
		}).has(co::LVIS::FOCUSED)
	}

	/// Tells if the item is selected by sending an
	/// [`LVM_GETITEMSTATE`](crate::msg::lvm::GetItemState) message.
	pub fn is_selected(&self, item_index: u32) -> bool {
		self.hwnd().SendMessage(lvm::GetItemState {
			index: item_index as i32,
			mask: co::LVIS::SELECTED,
		}).has(co::LVIS::SELECTED)
	}

	/// Tells if the item is currently visible by sending an
	/// [`LVM_ISITEMVISIBLE`](crate::msg::lvm::IsItemVisible) message.
	pub fn is_visible(&self, item_index: u32) -> bool {
		self.hwnd().SendMessage(lvm::IsItemVisible { index: item_index as i32 })
	}

	/// Retrieves the indexes of the selected items by sending
	/// [`LVM_GETNEXTITEM`](crate::msg::lvm::GetNextItem) messages.
	pub fn selected(&self) -> Vec<u32> {
		let mut items = Vec::with_capacity(self.selected_count() as usize);
		let mut idx = -1;

		loop {
			idx = match self.hwnd().SendMessage(lvm::GetNextItem {
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

	/// Retrieves the number of selected items by sending an
	/// [`LVM_GETSELECTEDCOUNT`](crate::msg::lvm::GetSelectedCount) message.
	pub fn selected_count(&self) -> u32 {
		self.hwnd().SendMessage(lvm::GetSelectedCount {})
	}

	/// Sets the focused item by sending an
	/// [`LVM_SETITEMSTATE`](crate::msg::lvm::SetItemState) message.
	pub fn set_focused(&self, item_index: u32) -> WinResult<()> {
		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::FOCUSED;
		lvi.state = co::LVIS::FOCUSED;

		self.hwnd().SendMessage(lvm::SetItemState {
			index: item_index as i32,
			lvitem: &lvi,
		})
	}

	/// Sets or remove the selection from the given item indexes by sending
	/// [`LVM_SETITEMSTATE`](crate::msg::lvm::SetItemState) messages.
	pub fn set_selected(&self,
		set: bool, item_indexes: &[u32]) -> WinResult<()>
	{
		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::SELECTED;
		if set { lvi.state = co::LVIS::SELECTED; }

		for idx in item_indexes.iter() {
			self.hwnd().SendMessage(lvm::SetItemState {
				index: *idx as i32,
				lvitem: &lvi,
			})?;
		}
		Ok(())
	}

	/// Sets or remove the selection for all items by sending an
	/// [`LVM_SETITEMSTATE`](crate::msg::lvm::SetItemState) message.
	pub fn set_selected_all(&self, set: bool) -> WinResult<()> {
		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::SELECTED;
		if set { lvi.state = co::LVIS::SELECTED; }

		self.hwnd().SendMessage(lvm::SetItemState {
			index: -1,
			lvitem: &lvi,
		})
	}

	/// Sets the text of an item under a column by sending an
	/// [`LVM_SETITEMTEXT`](crate::msg::lvm::SetItemText) message.
	pub fn set_text(&self,
		item_index: u32, column_index: u32, text: &str) -> WinResult<()>
	{
		let mut lvi = LVITEM::default();
		lvi.iSubItem = column_index as i32;

		let mut wtext = WString::from_str(text);
		lvi.set_pszText(&mut wtext);

		self.hwnd().SendMessage(lvm::SetItemText {
			index: item_index as i32,
			lvitem: &lvi,
		})
	}

	/// Retrieves the text of an item under a column by sending an
	/// [`LVM_GETITEMTEXT`](crate::msg::lvm::GetItemText) message.
	pub fn text(&self, item_index: u32, column_index: u32) -> String {
		// https://forums.codeguru.com/showthread.php?351972-Getting-listView-item-text-length
		const BLOCK: usize = 64; // arbitrary
		let mut buf_sz = BLOCK;

		loop {
			let mut lvi = LVITEM::default();
			lvi.iSubItem = column_index as i32;

			let mut buf = WString::new_alloc_buffer(buf_sz);
			lvi.set_pszText(&mut buf);

			let nchars = self.hwnd().SendMessage(lvm::GetItemText {
				index: item_index as i32,
				lvitem: &mut lvi,
			});

			if (nchars as usize) < buf_sz { // to break, must have at least 1 char gap
				return buf.to_string();
			}

			buf_sz += BLOCK; // increase buffer size to try again
		}
	}
}
