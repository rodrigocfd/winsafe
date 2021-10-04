use std::marker::PhantomData;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::TreeitemTvi;
use crate::handles::{HTREEITEM, HWND};
use crate::msg::tvm;
use crate::privs::MAX_PATH;
use crate::structs::{TVINSERTSTRUCT, TVITEMEX};
use crate::various::WString;

/// Exposes item methods of a [`TreeView`](crate::gui::TreeView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct TreeViewItems<'a> {
	pub(in crate::gui::native_controls) hwnd: HWND,
	pub(in crate::gui::native_controls) owner: PhantomData<&'a ()>,
}

impl<'a> TreeViewItems<'a> {
	/// Adds a new root item by sending a
	/// [`tvm::InsertItem`](crate::msg::tvm::InsertItem) message, and returns
	/// the newly added item.
	pub fn add_root(&self,
		text: &str, icon_index: Option<u32>) -> WinResult<TreeViewItem<'a>>
	{
		let mut buf = WString::from_str(text);

		let mut tvix = TVITEMEX::default();
		tvix.mask = co::TVIF::TEXT;
		if let Some(icon_index) = icon_index {
			tvix.mask |= co::TVIF::IMAGE;
			tvix.iImage = icon_index as _;
		}
		tvix.set_pszText(Some(&mut buf));

		let mut tvis = TVINSERTSTRUCT::default();
		tvis.set_hInsertAfter(TreeitemTvi::Tvi(co::TVI::LAST));
		tvis.itemex = tvix;

		self.hwnd.SendMessage(tvm::InsertItem { tvinsertstruct: &mut tvis })
			.map(|htreeitem| self.get(htreeitem))
	}

	/// Deletes all items by sending a
	/// [`tvm::DeleteItem`](crate::msg::tvm::DeleteItem) message.
	pub fn delete_all(&self) -> WinResult<()> {
		self.hwnd.SendMessage(tvm::DeleteItem { hitem: HTREEITEM::NULL })
	}

	/// Retrieves the total number of items by sending a
	/// [`tvm::GetCount`](crate::msg::tvm::GetCount) message.
	pub fn count(&self) -> u32 {
		self.hwnd.SendMessage(tvm::GetCount {})
	}

	/// Retrieves the number of visible items by sending a
	/// [`tvm::GetVisibleCount`](crate::msg::tvm::GetVisibleCount) message.
	pub fn count_visible(&self) -> u32 {
		self.hwnd.SendMessage(tvm::GetVisibleCount {})
	}

	/// Ends the editing of the item's text by sending a
	/// [`tvm::EndEditLabelNow`](crate::msg::tvm::EndEditLabelNow) message.
	pub fn end_edit_label_now(&self, save: bool) -> WinResult<()> {
		self.hwnd.SendMessage(tvm::EndEditLabelNow { save })
	}

	/// Retrieves the item of the given handle.
	///
	/// **Note:** This method is cheap – even if `htreeitem` is invalid, an
	/// object will still be returned. However, operations upon this object will
	/// fail.
	pub const fn get(&self, htreeitem: HTREEITEM) -> TreeViewItem<'a> {
		TreeViewItem {
			hwnd: self.hwnd,
			htreeitem,
			owner: PhantomData,
		}
	}

	/// Returns an iterator over the selected items.
	pub fn iter_selected(&self) -> impl Iterator<Item = TreeViewItem<'a>> {
		TreeViewItemIter {
			hwnd: self.hwnd,
			current: None,
			relationship: co::TVGN::CARET,
			owner: PhantomData,
		}
	}

	/// Returns an iterator over the root items.
	pub fn iter_root(&self) -> impl Iterator<Item = TreeViewItem<'a>> {
		TreeViewChildItemIter {
			hwnd: self.hwnd,
			current: None,
			first_call: true,
			owner: PhantomData,
		}
	}
}

//------------------------------------------------------------------------------

/// Represents a single item of a [`TreeView`](crate::gui::TreeView) control.
///
/// Each object keeps an unique [`HTREEITEM`](crate::HTREEITEM) handle.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[derive(Clone, Copy)]
pub struct TreeViewItem<'a> {
	hwnd: HWND,
	htreeitem: HTREEITEM,
	owner: PhantomData<&'a ()>,
}

impl<'a> TreeViewItem<'a> {
	/// Adds a new child item by sending a
	/// [`tvm::InsertItem`](crate::msg::tvm::InsertItem) message, and returns
	/// the newly added item.
	pub fn add_child(&self,
		text: &str,
		icon_index: Option<u32>) -> WinResult<TreeViewItem<'a>>
	{
		let mut buf = WString::from_str(text);

		let mut tvix = TVITEMEX::default();
		tvix.mask = co::TVIF::TEXT;
		if let Some(icon_index) = icon_index {
			tvix.mask |= co::TVIF::IMAGE;
			tvix.iImage = icon_index as _;
		}
		tvix.set_pszText(Some(&mut buf));

		let mut tvis = TVINSERTSTRUCT::default();
		tvis.hParent = self.htreeitem;
		tvis.set_hInsertAfter(TreeitemTvi::Tvi(co::TVI::LAST));
		tvis.itemex = tvix;

		self.hwnd.SendMessage(tvm::InsertItem { tvinsertstruct: &mut tvis })
			.map(|htreeitem| TreeViewItem {
				hwnd: self.hwnd,
				htreeitem,
				owner: PhantomData,
			})
	}

	/// Deletes the item by sending a
	/// [`tvm::DeleteItem`](crate::msg::tvm::DeleteItem) message.
	pub fn delete(&self) -> WinResult<()> {
		self.hwnd.SendMessage(tvm::DeleteItem { hitem: self.htreeitem })
	}

	/// Begins in-place editing of the item's text by sending a
	/// [`tvm::EditLabel`](crate::msg::tvm::EditLabel) message.
	///
	/// Returns a handle to the edit control.
	pub fn edit_label(&self) -> WinResult<HWND> {
		self.hwnd.SendMessage(tvm::EditLabel { hitem: self.htreeitem })
	}

	/// Ensures that a tree-view item is visible, expanding the parent item or
	/// scrolling the tree-view control, if necessary, by sending a
	/// [`tvm::EnsureVisible`](crate::msg::tvm::EnsureVisible) message.
	///
	/// Returns whether a scroll occurred and no items were expanded.
	pub fn ensure_visible(&self) -> bool {
		self.hwnd.SendMessage(tvm::EnsureVisible { hitem: self.htreeitem }) != 0
	}

	/// Expands or collapse the item by sending a
	/// [`tvm::Expand`](crate::msg::tvm::Expand) message.
	pub fn expand(&self, expand: bool) -> WinResult<()> {
		self.hwnd.SendMessage(tvm::Expand {
			hitem: self.htreeitem,
			action: if expand { co::TVE::EXPAND } else { co::TVE::COLLAPSE },
		})
	}

	/// Returns the underlying handle of the item.
	pub const fn htreeitem(&self) -> HTREEITEM {
		self.htreeitem
	}

	/// Tells if the item is expanded by sending a
	/// [`tvm::GetItemState`](crate::msg::tvm::GetItemState) message.
	pub fn is_expanded(&self) -> bool {
		self.hwnd.SendMessage(tvm::GetItemState {
			hitem: self.htreeitem,
			mask: co::TVIS::EXPANDED,
		}).has(co::TVIS::EXPANDED)
	}

	/// Tells if the item is a root by sending a
	/// [`tvm::GetNextItem`](crate::msg::tvm::GetNextItem) message.
	pub fn is_root(&self) -> bool {
		self.parent().is_none()
	}

	/// Returns an iterator over the child items.
	pub fn iter_children(&self) -> impl Iterator<Item = TreeViewItem<'a>> {
		TreeViewChildItemIter {
			hwnd: self.hwnd,
			current: Some(*self),
			first_call: true,
			owner: PhantomData,
		}
	}

	/// Returns an iterator over the next sibling items.
	pub fn iter_next_siblings(&self) -> impl Iterator<Item = TreeViewItem<'a>> {
		TreeViewItemIter {
			hwnd: self.hwnd,
			current: Some(*self),
			relationship: co::TVGN::NEXT,
			owner: PhantomData,
		}
	}

	/// Returns an iterator over the previous sibling items.
	pub fn iter_prev_siblings(&self) -> impl Iterator<Item = TreeViewItem<'a>> {
		TreeViewItemIter {
			hwnd: self.hwnd,
			current: Some(*self),
			relationship: co::TVGN::PREVIOUS,
			owner: PhantomData,
		}
	}

	/// Retrieves the parent of the item by sending a
	/// [`tvm::GetNextItem`](crate::msg::tvm::GetNextItem) message.
	pub fn parent(&self) -> Option<TreeViewItem<'a>> {
		self.hwnd.SendMessage(tvm::GetNextItem {
			relationship: co::TVGN::PARENT,
			hitem: Some(self.htreeitem),
		}).map(|htreeitem| TreeViewItem {
			hwnd: self.hwnd,
			htreeitem,
			owner: PhantomData,
		})
	}

	/// Sets the text of the item by sending a
	/// [`tvm::SetItem`](crate::msg::tvm::SetItem) message.
	pub fn set_text(&self, text: &str) -> WinResult<()> {
		let mut buf = WString::from_str(text);

		let mut tvi = TVITEMEX::default();
		tvi.hItem = self.htreeitem;
		tvi.mask = co::TVIF::TEXT;
		tvi.set_pszText(Some(&mut buf));

		self.hwnd.SendMessage(tvm::SetItem { tvitem: &tvi })
	}

	/// Retrieves the text of the item by sending a
	/// [`tvm::GetItem`](crate::msg::tvm::GetItem) message.
	pub fn text(&self) -> WinResult<String> {
		let mut tvi = TVITEMEX::default();
		tvi.hItem = self.htreeitem;
		tvi.mask = co::TVIF::TEXT;

		let mut buf = WString::new_alloc_buffer(MAX_PATH + 1); // arbitrary
		tvi.set_pszText(Some(&mut buf));

		self.hwnd.SendMessage(tvm::GetItem { tvitem: &mut tvi })?;
		Ok(buf.to_string())
	}
}

//------------------------------------------------------------------------------

struct TreeViewItemIter<'a> {
	hwnd: HWND,
	current: Option<TreeViewItem<'a>>,
	relationship: co::TVGN,
	owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for TreeViewItemIter<'a> {
	type Item = TreeViewItem<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.current = self.hwnd.SendMessage(tvm::GetNextItem {
			relationship: self.relationship,
			hitem: self.current.map(|item| item.htreeitem()),
		}).map(|htreeitem| TreeViewItem {
			hwnd: self.hwnd,
			htreeitem,
			owner: PhantomData,
		});

		self.current
	}
}

struct TreeViewChildItemIter<'a> {
	hwnd: HWND,
	current: Option<TreeViewItem<'a>>,
	first_call: bool,
	owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for TreeViewChildItemIter<'a> {
	type Item = TreeViewItem<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.first_call { // search for the first child
			self.current = self.hwnd.SendMessage(tvm::GetNextItem {
				relationship: co::TVGN::CHILD,
				hitem: self.current.map(|item| item.htreeitem()),
			}).map(|htreeitem| TreeViewItem {
				hwnd: self.hwnd,
				htreeitem,
				owner: PhantomData,
			});
			self.first_call = false;

		} else { // search for next siblings
			self.current = self.current
				.and_then(|item| item.iter_next_siblings().next())
		}

		self.current
	}
}
