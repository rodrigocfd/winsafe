use crate::co;
use crate::decl::*;
use crate::gui::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;

/// Exposes the item methods of a [`Header`](crate::gui::Header) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct HeaderItems<'a> {
	owner: &'a Header,
}

impl<'a> HeaderItems<'a> {
	#[must_use]
	pub(in crate::gui) const fn new(owner: &'a Header) -> Self {
		Self { owner }
	}

	/// Adds a new item by sending an
	/// [`hdm::InsertItem`](crate::msg::hdm::InsertItem) message, returning the
	/// new item.
	pub fn add(&self, text: &str, width: i32) -> SysResult<HeaderItem<'a>> {
		let mut hdi = HDITEM::default();
		hdi.mask = co::HDI::TEXT | co::HDI::WIDTH;
		hdi.cxy = width;

		let mut wtext = WString::from_str(text);
		hdi.set_pszText(Some(&mut wtext));

		let idx = unsafe {
			self.owner
				.hwnd()
				.SendMessage(hdm::InsertItem { index_after: 0xffff, item: &hdi })?
		};
		Ok(self.get(idx))
	}

	/// Retrieves the total number of items by sending a
	/// [`hdm::GetItemCount`](crate::msg::hdm::GetItemCount) message.
	#[must_use]
	pub fn count(&self) -> SysResult<u32> {
		unsafe { self.owner.hwnd().SendMessage(hdm::GetItemCount {}) }
	}

	/// Retrieves the item at the given zero-based position.
	///
	/// **Note:** This method is cheap – even if `index` is beyond the range of
	/// existing items, an object will still be returned. However, operations
	/// upon this object will produce no effect.
	#[must_use]
	pub const fn get(&self, index: u32) -> HeaderItem<'a> {
		HeaderItem::new(self.owner, index)
	}

	/// Returns an iterator over all items.
	#[must_use]
	pub fn iter(&self) -> SysResult<impl DoubleEndedIterator<Item = HeaderItem<'a>> + 'a> {
		HeaderItemIter::new(self.owner)
	}

	/// Returns the last item, if any.
	pub fn last(&self) -> SysResult<Option<HeaderItem<'a>>> {
		let count = self.count()?;
		Ok(if count > 0 { Some(self.get(count - 1)) } else { None })
	}
}

struct HeaderItemIter<'a> {
	owner: &'a Header,
	double_idx: DoubleIterIndex,
}

impl<'a> Iterator for HeaderItemIter<'a> {
	type Item = HeaderItem<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a> DoubleEndedIterator for HeaderItemIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a> HeaderItemIter<'a> {
	#[must_use]
	fn new(owner: &'a Header) -> SysResult<Self> {
		Ok(Self {
			owner,
			double_idx: DoubleIterIndex::new(owner.items().count()?),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<HeaderItem<'a>> {
		self.double_idx.grab(is_front, |cur_idx| {
			let item = self.owner.items().get(cur_idx);
			DoubleIter::Yield(item)
		})
	}
}
