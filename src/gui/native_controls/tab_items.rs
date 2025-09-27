use crate::co;
use crate::decl::*;
use crate::gui::*;
use crate::msg::*;
use crate::prelude::*;

/// Exposes the methods of a [`Tab`](crate::gui::Tab) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct TabItems<'a> {
	owner: &'a Tab,
}

impl<'a> TabItems<'a> {
	#[must_use]
	pub(in crate::gui) const fn new(owner: &'a Tab) -> Self {
		Self { owner }
	}

	/// Manually appends a new tab by sending a
	/// [`tcm::InsertItem`](crate::msg::tcm::InsertItem) message, and returns
	/// the newly added item.
	///
	/// # Safety
	///
	/// By adding a tab item manually, you are responsible for all the message
	/// handling. Prefer adding items automatically by filling the
	/// [`TabOpts::pages`](crate::gui::TabOpts::pages) member when calling the
	/// [`Tab::new`](crate::gui::Tab::new) function.
	pub unsafe fn add(&self, title: &str) -> TabItem<'a> {
		let mut wtitle = WString::from_str(title);
		let mut tci = TCITEM::default();
		tci.mask = co::TCIF::TEXT;
		tci.set_pszText(Some(&mut wtitle));

		let idx = unsafe {
			self.owner.hwnd().SendMessage(tcm::InsertItem {
				index: 0x0fff_ffff, // insert as the last item
				item: &tci,
			})
		}
		.unwrap();
		self.get(idx)
	}

	/// Retrieves the total number of items by sending an
	/// [`tcm::GetItemCount`](crate::msg::tcm::GetItemCount) message.
	#[must_use]
	pub fn count(&self) -> SysResult<u32> {
		unsafe { self.owner.hwnd().SendMessage(tcm::GetItemCount {}) }
	}

	/// Deletes all items by sending a
	/// [`tcm::DeleteAllItems`](crate::msg::tcm::DeleteAllItems) message.
	///
	/// # Safety
	///
	/// If you delete a tab automatically created, which has a container window
	/// attached to it, the rendering will be out-of-order.
	pub unsafe fn delete_all(&self) -> SysResult<()> {
		unsafe { self.owner.hwnd().SendMessage(tcm::DeleteAllItems {}) }
	}

	/// Retrieves the item at the given zero-based position.
	///
	/// **Note:** This method is cheap – even if `index` is beyond the range of
	/// existing items, an object will still be returned. However, operations
	/// upon this object will produce no effect.
	#[must_use]
	pub const fn get(&self, index: u32) -> TabItem<'a> {
		TabItem::new(self.owner, index)
	}

	/// Returns an iterator over all items.
	#[must_use]
	pub fn iter(&self) -> SysResult<impl DoubleEndedIterator<Item = TabItem<'a>> + 'a> {
		TabItemIter::new(self.owner)
	}

	/// Returns the focused item by sending a
	/// [`tcm::GetCurFocus`](crate::msg::tcm::GetCurFocus) message.
	#[must_use]
	pub fn focused(&self) -> Option<TabItem<'a>> {
		unsafe { self.owner.hwnd().SendMessage(tcm::GetCurFocus {}) }.map(|i| self.get(i))
	}

	/// Returns the selected item by sending a
	/// [`tcm::GetCurSel`](crate::msg::tcm::GetCurSel) message.
	#[must_use]
	pub fn selected(&self) -> Option<TabItem<'a>> {
		unsafe { self.owner.hwnd().SendMessage(tcm::GetCurSel {}) }.map(|i| self.get(i))
	}
}

struct TabItemIter<'a> {
	owner: &'a Tab,
	front_idx: u32,
	past_back_idx: u32,
}

impl<'a> Iterator for TabItemIter<'a> {
	type Item = TabItem<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a> DoubleEndedIterator for TabItemIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a> TabItemIter<'a> {
	#[must_use]
	fn new(owner: &'a Tab) -> SysResult<Self> {
		Ok(Self {
			owner,
			front_idx: 0,
			past_back_idx: owner.items().count()?,
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<TabItem<'a>> {
		if self.front_idx == self.past_back_idx {
			return None;
		}
		let our_idx = if is_front { self.front_idx } else { self.past_back_idx - 1 };

		let item = self.owner.items().get(our_idx);
		if is_front {
			self.front_idx += 1;
		} else {
			self.past_back_idx -= 1;
		}
		Some(item)
	}
}
