use crate::co;
use crate::decl::*;
use crate::gui::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;

/// A single item of a [`Header`](crate::gui::Header) control.
///
/// **Note:** Each object keeps the zero-based index of an item. If new items
/// are added/removed from the list view control, the object may then point to a
/// different item.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[derive(Clone, Copy)]
pub struct HeaderItem<'a> {
	owner: &'a Header,
	index: u32,
}

impl<'a> HeaderItem<'a> {
	#[must_use]
	pub(in crate::gui) const fn new(owner: &'a Header, index: u32) -> Self {
		Self { owner, index }
	}

	/// Deletes the item by sending a
	/// [`hdm::DeleteItem`](crate::msg::hdm::DeleteItem) message.
	pub fn delete(&self) {
		unsafe {
			self.owner.hwnd()
				.SendMessage(hdm::DeleteItem { index: self.index, })
		}.unwrap();
	}

	/// Sets the item as the focused one sending an
	/// [`hdm:SetFocusedItem`](crate::msg::hdm::SetFocusedItem) message.
	pub fn focus(&self) {
		unsafe {
			self.owner.hwnd()
				.SendMessage(hdm::SetFocusedItem { index: self.index })
		}.unwrap();
	}

	/// Return the format flags of the item by sending a
	/// [`hdm::GetItem`](crate::msg::hdm::GetItem) message.
	#[must_use]
	pub fn format(&self) -> co::HDF {
		let mut hdi = HDITEM::default();
		hdi.mask = co::HDI::FORMAT;

		unsafe {
			self.owner.hwnd()
				.SendMessage(hdm::GetItem {
					index: self.index,
					hditem: &mut hdi,
				});
		}

		hdi.fmt
	}

	/// Returns the zero-based index of the item.
	#[must_use]
	pub const fn index(&self) -> u32 {
		self.index
	}

	/// Retrieves the user-defined value by sending a
	/// [`hdm::GetItem`](crate::msg::hdm::GetItem) message.
	#[must_use]
	pub fn lparam(&self) -> isize {
		let mut hdi = HDITEM::default();
		hdi.mask = co::HDI::LPARAM;

		unsafe {
			self.owner.hwnd()
				.SendMessage(hdm::GetItem {
					index: self.index,
					hditem: &mut hdi,
				});
		}

		hdi.lParam
	}

	/// Retrieves the order of the item by sending a
	/// [`hdm::GetItem`](crate::msg::hdm::GetItem) message.
	#[must_use]
	pub fn order(&self) -> u32 {
		let mut hdi = HDITEM::default();
		hdi.mask = co::HDI::ORDER;

		unsafe {
			self.owner.hwnd()
				.SendMessage(hdm::GetItem {
					index: self.index,
					hditem: &mut hdi,
				});
		}

		hdi.iOrder as _
	}

	/// Sets the arrow state of the item by sending a
	/// [`hdm::SetItem`](crate::msg::hdm::SetItem) message.
	pub fn set_arrow(&self, arrow_state: HeaderArrow) {
		let mut hdi = HDITEM::default();
		hdi.mask = co::HDI::FORMAT;

		hdi.fmt = self.format();
		hdi.fmt &= !(co::HDF::SORTUP | co::HDF::SORTDOWN); // remove both
		hdi.fmt |= arrow_state.into();

		unsafe {
			self.owner.hwnd()
				.SendMessage(hdm::SetItem {
					index: self.index,
					hditem: &mut hdi,
				});
		}
	}

	/// Sets the text justification of the column by sending a
	/// [`hdm::SetItem`](crate::msg::hdm::SetItem) message.
	pub fn set_justify(&self, text_justification: HeaderJustify) {
		let mut hdi = HDITEM::default();
		hdi.mask = co::HDI::FORMAT;

		hdi.fmt = self.format();
		hdi.fmt &= !(co::HDF::LEFT | co::HDF::CENTER | co::HDF::RIGHT); // remove all
		hdi.fmt |= text_justification.into();

		unsafe {
			self.owner.hwnd()
				.SendMessage(hdm::SetItem {
					index: self.index,
					hditem: &mut hdi,
				});
		}
	}

	/// Sets the user-defined value of the item by sending a
	/// [`hdm::SetItem`](crate::msg::hdm::SetItem) message.
	pub fn set_lparam(&self, lparam: isize) {
		let mut hdi = HDITEM::default();
		hdi.mask = co::HDI::LPARAM;
		hdi.lParam = lparam;

		unsafe {
			self.owner.hwnd()
				.SendMessage(hdm::SetItem {
					index: self.index,
					hditem: &hdi,
				});
		}
	}

	/// Sets the order of the item by sending a
	/// [`hdm::SetItem`](crate::msg::hdm::SetItem) message.
	pub fn set_order(&self, order: u32) {
		let mut hdi = HDITEM::default();
		hdi.mask = co::HDI::ORDER;
		hdi.iOrder = order as _;

		unsafe {
			self.owner.hwnd()
				.SendMessage(hdm::SetItem {
					index: self.index,
					hditem: &hdi,
				});
		}
	}

	/// Sets the text of the item by sending a
	/// [`hdm::SetItem`](crate::msg::hdm::SetItem) message.
	pub fn set_text(&self, text: &str) {
		let mut hdi = HDITEM::default();
		hdi.mask = co::HDI::TEXT;

		let mut wtext = WString::from_str(text);
		hdi.set_pszText(Some(&mut wtext));

		unsafe {
			self.owner.hwnd()
				.SendMessage(hdm::SetItem {
					index: self.index,
					hditem: &hdi,
				});
		}
	}

	/// Retrieves the text of the item by sending a
	/// [`hdm::GetItem`](crate::msg::hdm::GetItem) message.
	#[must_use]
	pub fn text(&self) -> String {
		let mut hdi = HDITEM::default();
		hdi.mask = co::HDI::TEXT;

		let mut buf = WString::new_alloc_buf(MAX_PATH + 1); // arbitrary
		hdi.set_pszText(Some(&mut buf));

		unsafe {
			self.owner.hwnd()
				.SendMessage(hdm::GetItem {
					index: self.index,
					hditem: &mut hdi,
				});
		}

		let (psz, _) = hdi.raw_pszText();
		unsafe { WString::from_wchars_nullt(psz) }.to_string()
	}
}
