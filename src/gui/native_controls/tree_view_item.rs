use std::any::TypeId;
use std::cell::RefCell;
use std::mem::ManuallyDrop;
use std::rc::Rc;

use crate::co;
use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::kernel::privs::*;
use crate::msg;
use crate::prelude::*;

/// A single item of a [`TreeView`](crate::gui::TreeView) control.
///
/// Each object keeps an unique [`HTREEITEM`](crate::HTREEITEM) handle.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct TreeViewItem<'a, T: 'static = ()> {
	owner: &'a TreeView<T>,
	hitem: HTREEITEM,
}

impl<'a, T> Clone for TreeViewItem<'a, T> {
	fn clone(&self) -> Self {
		Self {
			owner: self.owner,
			hitem: unsafe { self.hitem.raw_copy() },
		}
	}
}

impl<'a, T> TreeViewItem<'a, T> {
	#[must_use]
	pub(in crate::gui) const fn new(owner: &'a TreeView<T>, hitem: HTREEITEM) -> Self {
		Self { owner, hitem }
	}

	/// Adds a new child item by sending a
	/// [`TvmInsertItem`](crate::msg::TvmInsertItem) message, and returns the
	/// newly added item.
	pub fn add_child(&self, text: &str, icon_index: Option<u32>, data: T) -> SysResult<Self> {
		self.owner
			.raw_insert_item(Some(&self.hitem), text, icon_index, data)
	}

	/// Returns a [`Rc`](std::rc::Rc)/[`RefCell`](std::cell::RefCell) with the
	/// stored data by sending an [`TvmGetItem`](crate::msg::TvmGetItem)
	/// message.
	///
	/// # Panics
	///
	/// Panics if the `TreeView` doesn't have an actual type, that is, if it was
	/// declared as `TreeView<()>`.
	///
	/// Panics if the item index is invalid.
	#[must_use]
	pub fn data(&self) -> Rc<RefCell<T>> {
		if TypeId::of::<T>() == TypeId::of::<()>() {
			panic!("TreeView<()> will hold no data."); // user didn't define the generic type
		}

		match self.data_lparam() {
			None => panic!("TreeViewItem with invalid index, no data."),
			Some(rc_ptr) => {
				let rc_obj = ManuallyDrop::new(unsafe { Rc::from_raw(rc_ptr) });
				Rc::clone(&rc_obj)
			},
		}
	}

	#[must_use]
	pub(in crate::gui) fn data_lparam(&self) -> Option<*mut RefCell<T>> {
		let mut tvix = TVITEMEX::default();
		tvix.hItem = unsafe { self.hitem.raw_copy() };
		tvix.mask = co::TVIF::PARAM;

		unsafe {
			self.owner
				.hwnd()
				.SendMessage(msg::TvmGetItem { tvitem: &mut tvix })
				.expect("TreeViewItem with invalid index, no data.");
		}

		match tvix.lParam {
			0 => None,
			lp => Some(lp as _), // return the stored Rc pointer
		}
	}

	/// Deletes the item by sending a
	/// [`TvmDeleteItem`](crate::msg::TvmDeleteItem) message.
	pub fn delete(&self) -> SysResult<()> {
		unsafe {
			self.owner
				.hwnd()
				.SendMessage(msg::TvmDeleteItem { hitem: &self.hitem })
		}
	}

	/// Begins in-place editing of the item's text by sending a
	/// [`TvmEditLabel`](crate::msg::TvmEditLabel) message.
	///
	/// Returns a handle to the edit control.
	pub fn edit_label(&self) -> SysResult<HWND> {
		unsafe {
			self.owner
				.hwnd()
				.SendMessage(msg::TvmEditLabel { hitem: &self.hitem })
		}
	}

	/// Ensures that a tree-view item is visible, expanding the parent item or
	/// scrolling the tree-view control, if necessary, by sending a
	/// [`TvmEnsureVisible`](crate::msg::TvmEnsureVisible) message.
	///
	/// Returns whether a scroll occurred and no items were expanded.
	pub fn ensure_visible(&self) -> bool {
		unsafe {
			self.owner
				.hwnd()
				.SendMessage(msg::TvmEnsureVisible { hitem: &self.hitem })
				!= 0
		}
	}

	/// Expands or collapse the item by sending a
	/// [`TvmExpand`](crate::msg::TvmExpand) message.
	pub fn expand(&self, expand: bool) -> SysResult<()> {
		unsafe {
			self.owner.hwnd().SendMessage(msg::TvmExpand {
				hitem: &self.hitem,
				action: if expand { co::TVE::EXPAND } else { co::TVE::COLLAPSE },
			})
		}
	}

	/// Returns the underlying handle of the item.
	#[must_use]
	pub const fn htreeitem(&self) -> &HTREEITEM {
		&self.hitem
	}

	/// Tells if the item is expanded by sending a
	/// [`TvmGetItemState`](crate::msg::TvmGetItemState) message.
	#[must_use]
	pub fn is_expanded(&self) -> bool {
		unsafe {
			self.owner.hwnd().SendMessage(msg::TvmGetItemState {
				hitem: &self.hitem,
				mask: co::TVIS::EXPANDED,
			})
		}
		.has(co::TVIS::EXPANDED)
	}

	/// Tells if the item is a root by sending a
	/// [`TvmGetNextItem`](crate::msg::TvmGetNextItem) message.
	#[must_use]
	pub fn is_root(&self) -> bool {
		self.parent().is_none()
	}

	/// Returns an iterator over the child items.
	#[must_use]
	pub fn iter_children(&self) -> impl Iterator<Item = TreeViewItem<'a, T>> + 'a {
		TreeViewChildItemIter::new(self.owner, Some(self.clone()))
	}

	/// Returns an iterator over the next sibling items.
	#[must_use]
	pub fn iter_next_siblings(&self) -> impl Iterator<Item = TreeViewItem<'a, T>> + 'a {
		TreeViewItemIter::new(self.owner, Some(self.clone()), co::TVGN::NEXT)
	}

	/// Returns an iterator over the previous sibling items.
	#[must_use]
	pub fn iter_prev_siblings(&self) -> impl Iterator<Item = TreeViewItem<'a, T>> + 'a {
		TreeViewItemIter::new(self.owner, Some(self.clone()), co::TVGN::PREVIOUS)
	}

	/// Retrieves the parent of the item by sending a
	/// [`TvmGetNextItem`](crate::msg::TvmGetNextItem) message.
	#[must_use]
	pub fn parent(&self) -> Option<Self> {
		unsafe {
			self.owner.hwnd().SendMessage(msg::TvmGetNextItem {
				relationship: co::TVGN::PARENT,
				hitem: Some(&self.hitem),
			})
		}
		.map(|hitem| TreeViewItem::new(self.owner, hitem))
	}

	/// Sets the text of the item by sending a
	/// [`TvmSetItem`](crate::msg::TvmSetItem) message.
	pub fn set_text(&self, text: &str) -> SysResult<()> {
		let mut buf = WString::from_str(text);

		let mut tvix = TVITEMEX::default();
		tvix.hItem = unsafe { self.hitem.raw_copy() };
		tvix.mask = co::TVIF::TEXT;
		tvix.set_pszText(Some(&mut buf));

		unsafe {
			self.owner
				.hwnd()
				.SendMessage(msg::TvmSetItem { tvitem: &tvix })
		}
	}

	/// Retrieves the text of the item by sending a
	/// [`TvmGetItem`](crate::msg::TvmGetItem) message.
	#[must_use]
	pub fn text(&self) -> SysResult<String> {
		let mut tvix = TVITEMEX::default();
		tvix.hItem = unsafe { self.hitem.raw_copy() };
		tvix.mask = co::TVIF::TEXT;

		let mut buf = WString::new_alloc_buf(MAX_PATH + 1); // arbitrary
		tvix.set_pszText(Some(&mut buf));

		unsafe {
			self.owner
				.hwnd()
				.SendMessage(msg::TvmGetItem { tvitem: &mut tvix })?;
		}

		Ok(buf.to_string())
	}
}
