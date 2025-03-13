use crate::co;
use crate::decl::*;
use crate::gui::*;
use crate::msg::*;
use crate::prelude::*;

/// A single column of a [`ListView`](crate::gui::ListView) control.
///
/// **Note:** Each object keeps the zero-based index of a column. If new columns
/// are added/removed from the list view control, the object may then point to a
/// different item.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListViewCol<'a, T: 'static = ()> {
	owner: &'a ListView<T>,
	index: u32,
}

impl<'a, T> Clone for ListViewCol<'a, T> {
	// https://stackoverflow.com/q/39415052/6923555
	fn clone(&self) -> Self {
		Self { owner: self.owner, index: self.index }
	}
}
impl<'a, T> Copy for ListViewCol<'a, T> {}

impl<'a, T> ListViewCol<'a, T> {
	#[must_use]
	pub(in crate::gui) const fn new(owner: &'a ListView<T>, index: u32) -> Self {
		Self { owner, index }
	}

	/// Returns the zero-based index of the column.
	#[must_use]
	pub const fn index(&self) -> u32 {
		self.index
	}

	/// Sets the title of the column by sending an
	/// [`lvm::SetColumn`](crate::msg::lvm::SetColumn) message.
	///
	/// Returns the same column, so further operations can be chained.
	pub fn set_title(&self, text: &str) -> SysResult<ListViewCol<'a, T>> {
		let mut lvc = LVCOLUMN::default();
		lvc.iSubItem = self.index as _;
		lvc.mask = co::LVCF::TEXT;

		let mut buf = WString::from_str(text);
		lvc.set_pszText(Some(&mut buf));

		unsafe {
			self.owner
				.hwnd()
				.SendMessage(lvm::SetColumn { index: self.index, lvcolumn: &mut lvc })?;
		}

		Ok(*self)
	}

	/// Sets the width of the column by sending an
	/// [`lvm::SetColumnWidth`](crate::msg::lvm::SetColumnWidth) message.
	///
	/// Returns the same column, so further operations can be chained.
	pub fn set_width(&self, width: i32) -> SysResult<ListViewCol<'a, T>> {
		unsafe {
			self.owner
				.hwnd()
				.SendMessage(lvm::SetColumnWidth { index: self.index, width: width as _ })?;
		}

		Ok(*self)
	}

	/// Sets the width of the column by sending an
	/// [`lvm::SetColumnWidth`](crate::msg::lvm::SetColumnWidth) message. The
	/// width will be calculated to fill the remaining space.
	///
	/// Returns the same column, so further operations can be chained.
	pub fn set_width_to_fill(&self) -> SysResult<ListViewCol<'a, T>> {
		let num_cols = self.owner.cols().count()?;
		if num_cols > 0 {
			let mut cx_used = 0;

			for i in 0..num_cols {
				if i != self.index {
					cx_used += self.owner.cols().get(i).width()?; // retrieve cx of each column, but us
				}
			}

			let rc = self.owner.hwnd().GetClientRect()?; // list view client area

			unsafe {
				self.owner.hwnd().SendMessage(lvm::SetColumnWidth {
					index: self.index,
					width: rc.right as u32 - cx_used,
				})?;
			}
		}
		Ok(*self)
	}

	/// Retrieves the title of the column by sending an
	/// [`lvm::GetColumn`](crate::msg::lvm::GetColumn) message.
	#[must_use]
	pub fn title(&self) -> SysResult<String> {
		let mut lvc = LVCOLUMN::default();
		lvc.iSubItem = self.index as _;
		lvc.mask = co::LVCF::TEXT;

		let mut buf = WString::new_alloc_buf(128); // arbitrary
		lvc.set_pszText(Some(&mut buf));

		unsafe {
			self.owner
				.hwnd()
				.SendMessage(lvm::GetColumn { index: self.index, lvcolumn: &mut lvc })?;
		}

		Ok(buf.to_string())
	}

	/// Retrieves the width of the column by sending an
	/// [`lvm::GetColumnWidth`](crate::msg::lvm::GetColumnWidth) message.
	#[must_use]
	pub fn width(&self) -> SysResult<u32> {
		unsafe {
			self.owner
				.hwnd()
				.SendMessage(lvm::GetColumnWidth { index: self.index })
		}
	}
}
