use crate::gui::{*, iterators::*, spec::*};
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

	/// Retrieves the total number of items by sending a
	/// [`hdm::GetItemCount`](crate::msg::hdm::GetItemCount) message.
	#[must_use]
	pub fn count(&self) -> u32 {
		unsafe {
			self.owner.hwnd()
				.SendMessage(hdm::GetItemCount {})
		}.unwrap()
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
	pub fn iter(&self) -> impl Iterator<Item = HeaderItem<'a>> + 'a {
		HeaderItemIter::new(self.owner)
	}
}
