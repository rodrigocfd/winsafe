use crate::co;
use crate::decl::*;
use crate::gui::*;
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
	#[must_use]
	pub(in crate::gui) const fn new(owner: &'a TreeView<T>) -> Self {
		Self { owner }
	}

	/// Adds a new root item by sending a
	/// [`tvm::InsertItem`](crate::msg::tvm::InsertItem) message, and returns
	/// the newly added item.
	pub fn add_root(
		&self,
		text: &str,
		icon_index: Option<u32>,
		data: T,
	) -> SysResult<TreeViewItem<'a, T>> {
		self.owner.raw_insert_item(None, text, icon_index, data)
	}

	/// Deletes all items by sending a
	/// [`tvm::DeleteItem`](crate::msg::tvm::DeleteItem) message.
	pub fn delete_all(&self) -> SysResult<()> {
		unsafe {
			self.owner
				.hwnd()
				.SendMessage(tvm::DeleteItem { hitem: &HTREEITEM::NULL })
		}
	}

	/// Retrieves the total number of items by sending a
	/// [`tvm::GetCount`](crate::msg::tvm::GetCount) message.
	#[must_use]
	pub fn count(&self) -> u32 {
		unsafe { self.owner.hwnd().SendMessage(tvm::GetCount {}) }
	}

	/// Retrieves the number of visible items by sending a
	/// [`tvm::GetVisibleCount`](crate::msg::tvm::GetVisibleCount) message.
	#[must_use]
	pub fn count_visible(&self) -> u32 {
		unsafe { self.owner.hwnd().SendMessage(tvm::GetVisibleCount {}) }
	}

	/// Ends the editing of the item's text by sending a
	/// [`tvm::EndEditLabelNow`](crate::msg::tvm::EndEditLabelNow) message.
	pub fn end_edit_label_now(&self, save: bool) -> SysResult<()> {
		unsafe { self.owner.hwnd().SendMessage(tvm::EndEditLabelNow { save }) }
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
	pub fn iter_selected(&self) -> impl Iterator<Item = TreeViewItem<'a, T>> + 'a {
		TreeViewItemIter::new(self.owner, None, co::TVGN::CARET)
	}

	/// Returns an iterator over the root items.
	#[must_use]
	pub fn iter_root(&self) -> impl Iterator<Item = TreeViewItem<'a, T>> + 'a {
		TreeViewChildItemIter::new(self.owner, None)
	}
}

pub(in crate::gui) struct TreeViewItemIter<'a, T: 'static> {
	owner: &'a TreeView<T>,
	current: Option<TreeViewItem<'a, T>>,
	relationship: co::TVGN,
}

impl<'a, T> Iterator for TreeViewItemIter<'a, T> {
	type Item = TreeViewItem<'a, T>;

	fn next(&mut self) -> Option<Self::Item> {
		self.current = unsafe {
			self.owner.hwnd().SendMessage(tvm::GetNextItem {
				relationship: self.relationship,
				hitem: self.current.as_ref().map(|tvi| tvi.htreeitem()),
			})
		}
		.map(|hitem| self.owner.items().get(&hitem));

		self.current
			.as_ref()
			.map(|tvi| TreeViewItem::new(self.owner, unsafe { tvi.htreeitem().raw_copy() }))
	}
}

impl<'a, T> TreeViewItemIter<'a, T> {
	#[must_use]
	pub(in crate::gui) const fn new(
		owner: &'a TreeView<T>,
		current: Option<TreeViewItem<'a, T>>,
		relationship: co::TVGN,
	) -> Self {
		Self { owner, current, relationship }
	}
}

pub(in crate::gui) struct TreeViewChildItemIter<'a, T: 'static> {
	owner: &'a TreeView<T>,
	current: Option<TreeViewItem<'a, T>>,
	first_call: bool,
}

impl<'a, T> Iterator for TreeViewChildItemIter<'a, T> {
	type Item = TreeViewItem<'a, T>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.first_call {
			// Search for the first child.
			self.current = unsafe {
				self.owner.hwnd().SendMessage(tvm::GetNextItem {
					relationship: co::TVGN::CHILD,
					hitem: self.current.as_ref().map(|tvi| tvi.htreeitem()),
				})
			}
			.map(|hitem| self.owner.items().get(&hitem));

			self.first_call = false;
		} else {
			// Search for next siblings.
			self.current = unsafe {
				self.owner.hwnd().SendMessage(tvm::GetNextItem {
					relationship: co::TVGN::NEXT,
					hitem: self.current.as_ref().map(|tvi| tvi.htreeitem()),
				})
			}
			.map(|hitem| self.owner.items().get(&hitem));
		}

		self.current
			.as_ref()
			.map(|tvi| TreeViewItem::new(self.owner, unsafe { tvi.htreeitem().raw_copy() }))
	}
}

impl<'a, T> TreeViewChildItemIter<'a, T> {
	#[must_use]
	pub(in crate::gui) fn new(
		owner: &'a TreeView<T>,
		current: Option<TreeViewItem<'a, T>>,
	) -> Self {
		Self { owner, current, first_call: true }
	}
}
