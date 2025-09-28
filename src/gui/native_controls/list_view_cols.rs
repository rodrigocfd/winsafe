use crate::co;
use crate::decl::*;
use crate::gui::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;

/// Exposes column methods of a [`ListView`](crate::gui::ListView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListViewCols<'a, T: 'static> {
	owner: &'a ListView<T>,
}

impl<'a, T> ListViewCols<'a, T> {
	#[must_use]
	pub(in crate::gui) const fn new(owner: &'a ListView<T>) -> Self {
		Self { owner }
	}

	/// Sends a [`lvm::InsertColumn`](crate::msg::lvm::InsertColumn) to add a
	/// new column with its width, returning the new column.
	pub fn add(&self, text: &str, width: i32) -> SysResult<ListViewCol<'a, T>> {
		let mut lvc = LVCOLUMN::default();
		lvc.mask = co::LVCF::TEXT | co::LVCF::WIDTH;
		lvc.cx = width as _;

		let mut wtext = WString::from_str(text);
		lvc.set_pszText(Some(&mut wtext));

		let idx = unsafe {
			self.owner.hwnd().SendMessage(lvm::InsertColumn {
				index: 0xffff, // insert as the last column
				column: &lvc,
			})?
		};
		Ok(self.owner.cols().get(idx))
	}

	/// Retrieves the number of columns by sending an
	/// [`hdm::GetItemCount`](crate::msg::hdm::GetItemCount) message to the
	/// embedded [`Header`](crate::gui::Header).
	#[must_use]
	pub fn count(&self) -> SysResult<u32> {
		match self.owner.header() {
			Some(header) => header.items().count(),
			None => Err(co::ERROR::INVALID_HANDLE),
		}
	}

	/// Retrieves the column at the given zero-based position.
	///
	/// **Note:** This method is cheap – even if `index` is beyond the range of
	/// existing columns, an object will still be returned. However, operations
	/// upon this object will produce no effect.
	#[must_use]
	pub const fn get(&self, index: u32) -> ListViewCol<'a, T> {
		ListViewCol::new(self.owner, index)
	}

	/// Returns an iterator over all columns.
	#[must_use]
	pub fn iter(&self) -> SysResult<impl DoubleEndedIterator<Item = ListViewCol<'a, T>> + 'a> {
		ListViewColIter::new(self.owner)
	}

	/// Returns the last column, if any.
	pub fn last(&self) -> SysResult<Option<ListViewCol<'a, T>>> {
		let count = self.count()?;
		Ok(if count > 0 { Some(self.get(count - 1)) } else { None })
	}
}

struct ListViewColIter<'a, T: 'static> {
	owner: &'a ListView<T>,
	double_idx: DoubleIterIndex,
}

impl<'a, T> Iterator for ListViewColIter<'a, T> {
	type Item = ListViewCol<'a, T>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a, T> DoubleEndedIterator for ListViewColIter<'a, T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a, T> ListViewColIter<'a, T> {
	#[must_use]
	fn new(owner: &'a ListView<T>) -> SysResult<Self> {
		Ok(Self {
			owner,
			double_idx: DoubleIterIndex::new(owner.cols().count()?),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<ListViewCol<'a, T>> {
		self.double_idx.grab(is_front, |cur_idx| {
			let item = self.owner.cols().get(cur_idx);
			DoubleIter::Yield(item)
		})
	}
}
