use std::marker::PhantomData;

use crate::aliases::WinResult;
use crate::co;
use crate::handles::HWND;
use crate::msg::lvm;
use crate::structs::{LVFINDINFO, LVHITTESTINFO, LVITEM, POINT, RECT};
use crate::various::WString;

/// Exposes item methods of a [`ListView`](crate::gui::ListView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListViewItems<'a> {
	pub(in crate::gui::native_controls) hwnd: HWND,
	pub(in crate::gui::native_controls) owner: PhantomData<&'a ()>,
}

impl<'a> ListViewItems<'a> {
	/// Appends a new item by sending an
	/// [`lvm::InsertItem`](crate::msg::lvm::InsertItem) message, and returns
	/// the newly added item.
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
	/// let new_item = my_list.items().add(
	///     &[
	///         "First column text",
	///         "Second column text",
	///     ],
	///     None, // no icon; requires a previous set_image_list()
	/// )?;
	/// ```
	///
	/// # Panics
	///
	/// Panics if `texts` is empty, or if the number of texts is greater than
	/// the number of columns.
	pub fn add<S: AsRef<str>>(&self,
		texts: &[S],
		icon_index: Option<u32>) -> WinResult<ListViewItem<'a>>
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

		let new_item = self.get(
			self.hwnd.SendMessage(lvm::InsertItem { lvitem: &lvi })?,
		);

		for (idx, text) in texts.iter().skip(1).enumerate() { // subsequent columns
			new_item.set_text(idx as u32 + 1, text.as_ref())?;
		}

		Ok(new_item)
	}

	/// Retrieves the total number of items by sending an
	/// [`lvm::GetItemCount`](crate::msg::lvm::GetItemCount) message.
	pub fn count(&self) -> u32 {
		self.hwnd.SendMessage(lvm::GetItemCount {})
	}

	/// Deletes all items by sending an
	/// [`lvm::DeleteAllItems`](crate::msg::lvm::DeleteAllItems) message.
	pub fn delete_all(&self) -> WinResult<()> {
		self.hwnd.SendMessage(lvm::DeleteAllItems {})
	}

	/// Deletes the selected items by sending
	/// [`lvm::DeleteAllItems`](crate::msg::lvm::DeleteAllItems) messages.
	pub fn delete_selected(&self) -> WinResult<()> {
		loop {
			match self.hwnd.SendMessage(lvm::GetNextItem {
				initial_index: None,
				relationship: co::LVNI::SELECTED,
			}) {
				Some(index) => self.hwnd.SendMessage(lvm::DeleteItem { index })?,
				None => break,
			};
		}
		Ok(())
	}

	/// Searches for an item with the given text, case-insensitive, by sending
	/// an [`lvm::FindItem`](crate::msg::lvm::FindItem) message.
	pub fn find(&self, text: &str) -> Option<ListViewItem<'a>> {
		let mut buf = WString::from_str(text);

		let mut lvfi = LVFINDINFO::default();
		lvfi.flags = co::LVFI::STRING;
		lvfi.set_psz(Some(&mut buf));

		self.hwnd.SendMessage(lvm::FindItem {
			start_index: None,
			lvfindinfo: &mut lvfi,
		}).map(|idx| self.get(idx))
	}

	/// Retrieves the index of the focused item by sending an
	/// [`lvm::GetNextItem`](crate::msg::lvm::GetNextItem) message.
	pub fn focused(&self) -> Option<ListViewItem<'a>> {
		self.hwnd.SendMessage(lvm::GetNextItem {
			initial_index: None,
			relationship: co::LVNI::FOCUSED,
		}).map(|idx| self.get(idx))
	}

	/// Retrieves the item at the given zero-based position.
	///
	/// **Note:** This method is cheap – even if `index` is beyond the range of
	/// existing items, an object will still be returned. However, operations
	/// upon this object will fail.
	pub const fn get(&self, index: u32) -> ListViewItem<'a> {
		ListViewItem {
			hwnd: self.hwnd,
			index,
			owner: PhantomData,
		}
	}

	/// Retrieves the item at the specified position by sending an
	/// [`lvm::HitTest`](crate::msg::lvm::HitTest) message
	///
	/// `coords` must be relative to the list view.
	pub fn hit_test(&self, coords: POINT) -> Option<ListViewItem<'a>> {
		let mut lvhti = LVHITTESTINFO::default();
		lvhti.pt = coords;

		self.hwnd.SendMessage(lvm::HitTest { info: &mut lvhti })
			.map(|index| self.get(index))
	}

	/// Returns an iterator over all items.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui::ListView;
	///
	/// let my_list: ListView; // initialized somewhere
	///
	/// for item in my_list.items().iter() {
	///     println!("Item {}, text of the first column: {}",
	///         item.index(), item.text(0));
	/// }
	/// ```
	pub fn iter(&self) -> impl Iterator<Item = ListViewItem<'a>> {
		ListViewItemIter {
			hwnd: self.hwnd,
			current: None,
			relationship: co::LVNI::ALL,
			owner: PhantomData,
		}
	}

	/// Returns an iterator over the selected items.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui;
	///
	/// let my_list: gui::ListView; // initialized somewhere
	///
	/// for item in my_list.items().iter_selected() {
	///     println!("Selected item {}, text of the first column: {}",
	///         item.index(), item.text(0));
	/// }
	/// ```
	pub fn iter_selected(&self) -> impl Iterator<Item = ListViewItem<'a>> {
		ListViewItemIter {
			hwnd: self.hwnd,
			current: None,
			relationship: co::LVNI::ALL | co::LVNI::SELECTED,
			owner: PhantomData,
		}
	}

	/// Retrieves the item of the unique ID by sending an
	/// [`lvm::MapIdToIndex`](crate::msg::lvm::MapIdToIndex) message.
	///
	/// If the item of the given unique ID doesn't exist anymore, returns
	/// `None`.
	pub fn map_id_to_index(&self, item_id: u32) -> Option<ListViewItem<'a>> {
		self.hwnd.SendMessage(lvm::MapIdToIndex { id: item_id })
			.map(|idx| self.get(idx))
	}

	/// Sets or remove the selection for all items by sending an
	/// [`lvm::SetItemState`](crate::msg::lvm::SetItemState) message.
	pub fn select_all(&self, set: bool) -> WinResult<()> {
		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::SELECTED;
		if set { lvi.state = co::LVIS::SELECTED; }

		self.hwnd.SendMessage(lvm::SetItemState {
			index: None,
			lvitem: &lvi,
		})
	}

	/// Retrieves the number of selected items by sending an
	/// [`lvm::GetSelectedCount`](crate::msg::lvm::GetSelectedCount) message.
	pub fn selected_count(&self) -> u32 {
		self.hwnd.SendMessage(lvm::GetSelectedCount {})
	}
}

//------------------------------------------------------------------------------

/// Represents a single item of a [`ListView`](crate::gui::ListView) control.
///
/// **Note:** Each object keeps the zero-based index of an item. If new items
/// are added/removed from the list view control, the object may then point to a
/// different item.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[derive(Clone, Copy)]
pub struct ListViewItem<'a> {
	hwnd: HWND,
	index: u32,
	owner: PhantomData<&'a ()>,
}

impl<'a> ListViewItem<'a> {
	/// Deletes the item by sending an
	/// [`lvm::DeleteItem`](crate::msg::lvm::DeleteItem) message.
	pub fn delete(&self) -> WinResult<()> {
		self.hwnd.SendMessage(lvm::DeleteItem {
			index: self.index,
		})
	}

	/// Scrolls the list by sending an
	/// [`lvm::EnsureVisible`](crate::msg::lvm::EnsureVisible) message so that
	/// the item is visible in the list.
	pub fn ensure_visible(&self) -> WinResult<()> {
		self.hwnd.SendMessage(lvm::EnsureVisible {
			index: self.index,
			entirely_visible: true,
		})
	}

	/// Sets the item as the focused one sending an
	/// [`lvm:SetItemState`](crate::msg::lvm::SetItemState) message.
	pub fn focus(&self) -> WinResult<()> {
		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::FOCUSED;
		lvi.state = co::LVIS::FOCUSED;

		self.hwnd.SendMessage(lvm::SetItemState {
			index: Some(self.index),
			lvitem: &lvi,
		})
	}

	/// Retrieves the icon index of the item by sending an
	/// [`lvm::GetItem`](crate::msg::lvm::GetItem) message.
	pub fn icon_index(&self) -> WinResult<Option<u32>> {
		let mut lvi = LVITEM::default();
		lvi.iItem = self.index as _;
		lvi.mask = co::LVIF::IMAGE;

		self.hwnd.SendMessage(lvm::SetItem { lvitem: &mut lvi })?;

		Ok(match lvi.iImage {
			-1 => None,
			idx => Some(idx as _),
		})
	}

	/// Returns the zero-based index of the item.
	pub const fn index(&self) -> u32 {
		self.index
	}

	/// Tells if the item is the focused one by sending an
	/// [`lvm::GetItemState`](crate::msg::lvm::GetItemState) message.
	pub fn is_focused(&self) -> bool {
		self.hwnd.SendMessage(lvm::GetItemState {
			index: self.index,
			mask: co::LVIS::FOCUSED,
		}).has(co::LVIS::FOCUSED)
	}

	/// Tells if the item is selected by sending an
	/// [`lvm::GetItemState`](crate::msg::lvm::GetItemState) message.
	pub fn is_selected(&self) -> bool {
		self.hwnd.SendMessage(lvm::GetItemState {
			index: self.index,
			mask: co::LVIS::SELECTED,
		}).has(co::LVIS::SELECTED)
	}

	/// Tells if the item is currently visible by sending an
	/// [`lvm::IsItemVisible`](crate::msg::lvm::IsItemVisible) message.
	pub fn is_visible(&self) -> bool {
		self.hwnd.SendMessage(lvm::IsItemVisible { index: self.index })
	}

	/// Retrieves the user-defined value by sending an
	/// [`lvm::GetItem`](crate::msg::lvm::GetItem) message.
	pub fn lparam(&self) -> WinResult<isize> {
		let mut lvi = LVITEM::default();
		lvi.iItem = self.index as _;
		lvi.mask = co::LVIF::PARAM;

		self.hwnd.SendMessage(lvm::GetItem { lvitem: &mut lvi })?;
		Ok(lvi.lParam)
	}

	/// Retrieves the unique ID for the item index by sending an
	/// [`lvm::MapIndexToId`](crate::msg::lvm::MapIndexToId) message.
	///
	/// If the item index has became invalid, returns `None`.
	pub fn map_index_to_id(&self) -> Option<u32> {
		self.hwnd.SendMessage(lvm::MapIndexToId { index: self.index })
	}

	/// Retrieves the bound rectangle of item by sending an
	/// [`lvm::GetItemRect`](crate::msg::lvm::GetItemRect) message.
	pub fn rect(&self, portion: co::LVIR) -> WinResult<RECT> {
		let mut rc = RECT::default();
		self.hwnd.SendMessage(lvm::GetItemRect {
			index: self.index,
			rect: &mut rc,
			portion,
		}).map(|_| rc)
	}

	/// Sets or removes the selection from the item by sending an
	/// [`lvm::SetItemState`](crate::msg::lvm::SetItemState) message.
	pub fn select(&self, set: bool) -> WinResult<()> {
		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::SELECTED;
		if set { lvi.state = co::LVIS::SELECTED; }

			self.hwnd.SendMessage(lvm::SetItemState {
				index: Some(self.index),
				lvitem: &lvi,
			})?;

		Ok(())
	}

	/// Sets the icon index of the item by sending an
	/// [`lvm::SetItem`](crate::msg::lvm::SetItem) message.
	pub fn set_icon_index(&self, icon_index: Option<u32>) -> WinResult<()> {
		let mut lvi = LVITEM::default();
		lvi.iItem = self.index as _;
		lvi.mask = co::LVIF::IMAGE;
		lvi.iImage = icon_index.map_or(-1, |idx| idx as _);

		self.hwnd.SendMessage(lvm::SetItem { lvitem: &mut lvi })
	}

	/// Sets the user-defined value by sending an
	/// [`lvm::SetItem`](crate::msg::lvm::SetItem) message.
	pub fn set_lparam(&self, lparam: isize) -> WinResult<()> {
		let mut lvi = LVITEM::default();
		lvi.iItem = self.index as _;
		lvi.mask = co::LVIF::PARAM;
		lvi.lParam = lparam;

		self.hwnd.SendMessage(lvm::SetItem { lvitem: &mut lvi })
	}

	/// Sets the text of the item under a column by sending an
	/// [`lvm::SetItemText`](crate::msg::lvm::SetItemText) message.
	pub fn set_text(&self, column_index: u32, text: &str) -> WinResult<()> {
		let mut lvi = LVITEM::default();
		lvi.iSubItem = column_index as _;

		let mut wtext = WString::from_str(text);
		lvi.set_pszText(Some(&mut wtext));

		self.hwnd.SendMessage(lvm::SetItemText {
			index: self.index,
			lvitem: &lvi,
		})
	}

	/// Retrieves the text of an item under a column by sending an
	/// [`lvm::GetItemText`](crate::msg::lvm::GetItemText) message.
	pub fn text(&self, column_index: u32) -> String {
		// https://forums.codeguru.com/showthread.php?351972-Getting-listView-item-text-length
		const BLOCK: usize = 64; // arbitrary
		let mut buf_sz = BLOCK;
		let mut buf = WString::default();

		loop {
			let mut lvi = LVITEM::default();
			lvi.iSubItem = column_index as _;

			buf.realloc_buffer(buf_sz);
			lvi.set_pszText(Some(&mut buf));

			let nchars = self.hwnd.SendMessage(lvm::GetItemText { // char count without terminating null
				index: self.index,
				lvitem: &mut lvi,
			});

			if (nchars as usize) + 1 < buf_sz { // to break, must have at least 1 char gap
				break;
			}

			buf_sz += BLOCK; // increase buffer size to try again
		}

		buf.to_string()
	}
}

//------------------------------------------------------------------------------

struct ListViewItemIter<'a> {
	hwnd: HWND,
	current: Option<ListViewItem<'a>>,
	relationship: co::LVNI,
	owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for ListViewItemIter<'a> {
	type Item = ListViewItem<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.current = self.hwnd.SendMessage(lvm::GetNextItem {
			initial_index: self.current.map(|item| item.index()),
			relationship: self.relationship,
		}).map(|index| ListViewItem {
			hwnd: self.hwnd,
			index,
			owner: PhantomData,
		});

		self.current
	}
}
