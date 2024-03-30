use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, native_controls::iterators::*, spec::*};
use crate::msg::*;
use crate::prelude::*;

/// Exposes item methods of a [`TreeView`](crate::gui::TreeView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct TreeViewItems<'a, T: 'static> {
	owner: &'a TreeView<T>,
}

impl<'a, T> TreeViewItems<'a, T> {
	pub(in crate::gui) const fn new(owner: &'a TreeView<T>) -> Self {
		Self { owner }
	}

	/// Adds a new root item by sending a
	/// [`tvm::InsertItem`](crate::msg::tvm::InsertItem) message, and returns
	/// the newly added item.
	pub fn add_root(&self,
		text: &str,
		icon_index: Option<u32>,
		data: T,
	) -> TreeViewItem<'a, T>
	{
		let mut buf = WString::from_str(text);

		let mut tvix = TVITEMEX::default();
		tvix.mask = co::TVIF::TEXT;
		tvix.set_pszText(Some(&mut buf));

		if let Some(icon_index) = icon_index {
			tvix.mask |= co::TVIF::IMAGE;
			tvix.iImage = icon_index as _;
		}

		if TypeId::of::<T>() != TypeId::of::<()>() { // user defined an actual type?
			tvix.mask |= co::TVIF::PARAM;
			let rc_data = Rc::new(RefCell::new(data));
			tvix.lParam = Rc::into_raw(rc_data) as _;
		}

		let mut tvis = TVINSERTSTRUCT::default();
		tvis.set_hInsertAfter(TreeitemTvi::Tvi(co::TVI::LAST));
		tvis.itemex = tvix;

		let new_hitem = unsafe {
			self.owner.hwnd()
				.SendMessage(tvm::InsertItem { item: &mut tvis })
		}.unwrap();

		self.get(&new_hitem)
	}

	/// Deletes all items by sending a
	/// [`tvm::DeleteItem`](crate::msg::tvm::DeleteItem) message.
	pub fn delete_all(&self) {
		unsafe {
			self.owner.hwnd()
				.SendMessage(tvm::DeleteItem { hitem: &HTREEITEM::NULL })
		}.unwrap();
	}

	/// Retrieves the total number of items by sending a
	/// [`tvm::GetCount`](crate::msg::tvm::GetCount) message.
	#[must_use]
	pub fn count(&self) -> u32 {
		unsafe {
			self.owner.hwnd()
				.SendMessage(tvm::GetCount {})
		}
	}

	/// Retrieves the number of visible items by sending a
	/// [`tvm::GetVisibleCount`](crate::msg::tvm::GetVisibleCount) message.
	#[must_use]
	pub fn count_visible(&self) -> u32 {
		unsafe {
			self.owner.hwnd()
				.SendMessage(tvm::GetVisibleCount {})
		}
	}

	/// Ends the editing of the item's text by sending a
	/// [`tvm::EndEditLabelNow`](crate::msg::tvm::EndEditLabelNow) message.
	pub fn end_edit_label_now(&self, save: bool) {
		unsafe {
			self.owner.hwnd()
				.SendMessage(tvm::EndEditLabelNow { save })
		}.unwrap();
	}

	/// Retrieves the item of the given handle.
	///
	/// **Note:** This method is cheap – even if `htreeitem` is invalid, an
	/// object will still be returned. However, operations upon this object will
	/// produce no effect.
	#[must_use]
	pub fn get(&self, hitem: &HTREEITEM) -> TreeViewItem<'a, T> {
		TreeViewItem::new(self.owner, unsafe { hitem.raw_copy() })
	}

	/// Returns an iterator over the selected items.
	#[must_use]
	pub fn iter_selected(&self,
	) -> impl Iterator<Item = TreeViewItem<'a, T>> + 'a
	{
		TreeViewItemIter::new(self.owner, None, co::TVGN::CARET)
	}

	/// Returns an iterator over the root items.
	#[must_use]
	pub fn iter_root(&self) -> impl Iterator<Item = TreeViewItem<'a, T>> + 'a {
		TreeViewChildItemIter::new(self.owner, None)
	}
}
