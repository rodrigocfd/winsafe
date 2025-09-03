use std::any::TypeId;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use crate::co;
use crate::decl::*;
use crate::gui::{iterators::*, *};
use crate::msg::*;
use crate::prelude::*;

/// Exposes item methods of a [`ListView`](crate::gui::ListView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListViewItems<'a, T: 'static> {
	owner: &'a ListView<T>,
}

impl<'a, T> ListViewItems<'a, T> {
	#[must_use]
	pub(in crate::gui) const fn new(owner: &'a ListView<T>) -> Self {
		Self { owner }
	}

	/// Appends a new item by sending an
	/// [`lvm::InsertItem`](crate::msg::lvm::InsertItem) message, returning it.
	///
	/// The texts are relative to each column.
	///
	/// # Panics
	///
	/// Panics if `texts` is empty, or if the number of texts is greater than
	/// the number of columns.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_list: gui::ListView; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_list = gui::ListView::new(&wnd, gui::ListViewOpts::default());
	///
	/// let new_item = my_list.items().add(
	///     &[
	///         "First column text",
	///         "Second column text",
	///     ],
	///     None, // no icon; requires icons to be added to the image list
	///     (),   // no object data; requires specifying the generic `ListView` type
	/// );
	/// ```
	pub fn add(
		&self,
		texts: &[impl AsRef<str>],
		icon_index: Option<u32>,
		data: T,
	) -> SysResult<ListViewItem<'a, T>> {
		if texts.is_empty() {
			panic!("No texts passed when adding a ListView item.");
		} else if texts.len() > self.owner.cols().count()? as _ {
			panic!(
				"Cannot set {} text(s) to {} column(s).",
				texts.len(),
				self.owner.cols().count()?
			);
		}

		let mut lvi = LVITEM::default();
		lvi.iItem = 0x0fff_ffff; // insert as the last item
		lvi.mask = co::LVIF::TEXT | co::LVIF::IMAGE;

		let mut wtext = WString::from_str(texts[0].as_ref()); // text of 1st column
		lvi.set_pszText(Some(&mut wtext));

		lvi.iImage = match icon_index {
			Some(i) => i as _,
			None => -1,
		};

		if TypeId::of::<T>() != TypeId::of::<()>() {
			// user defined the generic type
			lvi.mask |= co::LVIF::PARAM;
			let rc_data = Rc::new(RefCell::new(data));
			lvi.lParam = Rc::into_raw(rc_data) as _;
		}

		let new_idx = unsafe {
			self.owner
				.hwnd()
				.SendMessage(lvm::InsertItem { item: &lvi })
		}?;
		let new_item = self.get(new_idx);

		texts
			.iter()
			.enumerate()
			.skip(1) // iterate over subsequent columns
			.try_for_each(|(idx, text)| {
				new_item.set_text(idx as _, text.as_ref())?; // set the text ordinarily
				SysResult::Ok(())
			})?;

		Ok(new_item)
	}

	/// Retrieves the total number of items by sending an
	/// [`lvm::GetItemCount`](crate::msg::lvm::GetItemCount) message.
	#[must_use]
	pub fn count(&self) -> u32 {
		unsafe { self.owner.hwnd().SendMessage(lvm::GetItemCount {}) }
	}

	/// Deletes all items by sending an
	/// [`lvm::DeleteAllItems`](crate::msg::lvm::DeleteAllItems) message.
	pub fn delete_all(&self) -> SysResult<()> {
		unsafe { self.owner.hwnd().SendMessage(lvm::DeleteAllItems {}) }
	}

	/// Deletes all selected items by sending
	/// [`lvm::DeleteItem`](crate::msg::lvm::DeleteItem) messages.
	pub fn delete_selected(&self) -> SysResult<()> {
		loop {
			let next_idx = unsafe {
				self.owner.hwnd().SendMessage(lvm::GetNextItem {
					initial_index: None,
					relationship: co::LVNI::SELECTED,
				})
			};
			match next_idx {
				Some(next_idx) => self.get(next_idx).delete()?,
				None => break,
			}
		}
		Ok(())
	}

	/// Searches for an item with the given text, case-insensitive, by sending
	/// an [`lvm::FindItem`](crate::msg::lvm::FindItem) message.
	#[must_use]
	pub fn find(&self, text: &str) -> Option<ListViewItem<'a, T>> {
		let mut buf = WString::from_str(text);

		let mut lvfi = LVFINDINFO::default();
		lvfi.flags = co::LVFI::STRING;
		lvfi.set_psz(Some(&mut buf));

		unsafe {
			self.owner
				.hwnd()
				.SendMessage(lvm::FindItem { start_index: None, lvfindinfo: &mut lvfi })
		}
		.map(|idx| self.get(idx))
	}

	/// Retrieves the focused item by sending an
	/// [`lvm::GetNextItem`](crate::msg::lvm::GetNextItem) message.
	#[must_use]
	pub fn focused(&self) -> Option<ListViewItem<'a, T>> {
		unsafe {
			self.owner.hwnd().SendMessage(lvm::GetNextItem {
				initial_index: None,
				relationship: co::LVNI::FOCUSED,
			})
		}
		.map(|idx| self.get(idx))
	}

	/// Retrieves the item at the given zero-based position.
	///
	/// **Note:** This method is cheap – even if `index` is beyond the range of
	/// existing items, an object will still be returned. However, operations
	/// upon this object will produce no effect.
	#[must_use]
	pub const fn get(&self, index: u32) -> ListViewItem<'a, T> {
		ListViewItem::new(self.owner, index)
	}

	/// Retrieves the item at the specified position by sending an
	/// [`lvm::HitTest`](crate::msg::lvm::HitTest) message.
	///
	/// `coords` must be relative to the list view.
	#[must_use]
	pub fn hit_test(&self, coords: POINT) -> Option<ListViewItem<'a, T>> {
		let mut lvhti = LVHITTESTINFO::default();
		lvhti.pt = coords;

		unsafe {
			self.owner
				.hwnd()
				.SendMessage(lvm::HitTest { info: &mut lvhti })
		}
		.map(|index| self.get(index))
	}

	/// Returns an iterator over all items.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_list: gui::ListView; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_list = gui::ListView::<()>::new(&wnd, gui::ListViewOpts::default());
	///
	/// for item in my_list.items().iter() {
	///     println!("Item {}, text of the first column: {}",
	///         item.index(), item.text(0));
	/// }
	///
	/// let vec_items = my_list.items()
	///     .iter()
	///     .collect::<Vec<_>>();
	/// ```
	#[must_use]
	pub fn iter(&self) -> impl DoubleEndedIterator<Item = ListViewItem<'a, T>> + 'a {
		ListViewItemIter::new(self.owner, false)
	}

	/// Returns an iterator over the selected items.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_list: gui::ListView; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_list = gui::ListView::<()>::new(&wnd, gui::ListViewOpts::default());
	///
	/// for item in my_list.items().iter_selected() {
	///     println!("Selected item {}, text of the first column: {}",
	///         item.index(), item.text(0));
	/// }
	///
	/// let vec_items = my_list.items()
	///     .iter_selected()
	///     .collect::<Vec<_>>();
	/// ```
	#[must_use]
	pub fn iter_selected(&self) -> impl DoubleEndedIterator<Item = ListViewItem<'a, T>> + 'a {
		ListViewItemIter::new(self.owner, true)
	}

	/// Retrieves the item of the unique ID by sending an
	/// [`lvm::MapIdToIndex`](crate::msg::lvm::MapIdToIndex) message.
	///
	/// If the item of the given unique ID doesn't exist anymore, returns
	/// `None`.
	#[must_use]
	pub fn get_by_uid(&self, uid: u32) -> Option<ListViewItem<'a, T>> {
		unsafe { self.owner.hwnd().SendMessage(lvm::MapIdToIndex { id: uid }) }
			.map(|idx| self.get(idx))
	}

	/// Returns the last item, if any.
	pub fn last(&self) -> Option<ListViewItem<'a, T>> {
		let count = self.count();
		if count > 0 { Some(self.get(count - 1)) } else { None }
	}

	/// Returns the last selected item, if any.
	pub fn last_selected(&self) -> Option<ListViewItem<'a, T>> {
		let count = self.selected_count();
		if count > 0 { Some(self.get(count - 1)) } else { None }
	}

	/// Sets or remove the selection for all items by sending an
	/// [`lvm::SetItemState`](crate::msg::lvm::SetItemState) message.
	pub fn select_all(&self, set: bool) -> SysResult<()> {
		let styles: co::LVS = self.owner.hwnd().style().into();
		if styles.has(co::LVS::SINGLESEL) {
			return Ok(()); // LVM_SETITEMSTATE fails for all items in single-sel list views
		}

		let mut lvi = LVITEM::default();
		lvi.stateMask = co::LVIS::SELECTED;
		if set {
			lvi.state = co::LVIS::SELECTED;
		}

		unsafe {
			self.owner
				.hwnd()
				.SendMessage(lvm::SetItemState { index: None, lvitem: &lvi })
		}
	}

	/// Retrieves the number of selected items by sending an
	/// [`lvm::GetSelectedCount`](crate::msg::lvm::GetSelectedCount) message.
	#[must_use]
	pub fn selected_count(&self) -> u32 {
		unsafe { self.owner.hwnd().SendMessage(lvm::GetSelectedCount {}) }
	}

	/// Sets the number of items in a virtual list view – that is, a list view
	/// created with [`LVS::OWNERDATA`](crate::co::LVS::OWNERDATA) style – by
	/// sending an [`lvm::SetItemCount`](crate::msg::lvm::SetItemCount) message.
	pub fn set_count(&self, count: u32, behavior: Option<co::LVSICF>) -> SysResult<()> {
		unsafe {
			self.owner
				.hwnd()
				.SendMessage(lvm::SetItemCount { count, behavior })
		}
	}

	/// Sorts the items according to a callback by sending an
	/// [`lvm::SortItemsEx`](crate::msg::lvm::SortItemsEx) message.
	///
	/// The callback receives the two items to be compared.
	///
	/// # Examples
	///
	/// Sorting by the text of the first column:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_list: gui::ListView; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_list = gui::ListView::<()>::new(&wnd, gui::ListViewOpts::default());
	///
	/// my_list.items().sort(|itemA, itemB| -> std::cmp::Ordering {
	///     itemA.text(0).cmp( &itemB.text(0) )
	/// });
	/// ```
	pub fn sort<F>(&self, func: F) -> SysResult<()>
	where
		F: FnMut(ListViewItem, ListViewItem) -> Ordering,
	{
		let mut func = func;
		let data = (self.owner, &mut func);

		unsafe {
			self.owner.hwnd().SendMessage(lvm::SortItemsEx {
				param: &data as *const _ as _,
				callback: Self::list_view_item_sort::<F>,
			})
		}
	}

	pub(in crate::gui) extern "system" fn list_view_item_sort<F>(
		lparam1: isize,
		lparam2: isize,
		lparam_sort: isize,
	) -> i32
	where
		F: FnMut(ListViewItem, ListViewItem) -> Ordering,
	{
		let data = unsafe { &mut *(lparam_sort as *mut (&ListView, &mut F)) };
		let item1 = data.0.items().get(lparam1 as _);
		let item2 = data.0.items().get(lparam2 as _);
		data.1(item1, item2) as _
	}
}
