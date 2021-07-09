use std::cell::Cell;
use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::co;
use crate::gui::native_controls::list_view_items::ListViewItems;
use crate::gui::privs::multiply_dpi;
use crate::handles::HWND;
use crate::msg::{hdm, lvm};
use crate::structs::{LVCOLUMN, SIZE};
use crate::various::WString;

/// Exposes column methods of a [`ListView`](crate::gui::ListView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListViewColumns {
	hwnd_ptr: Cell<NonNull<HWND>>,
}

impl ListViewColumns {
	pub(in crate::gui::native_controls) fn new() -> ListViewColumns {
		Self {
			hwnd_ptr: Cell::new(NonNull::from(&HWND::NULL)), // initially invalid
		}
	}

	pub(in crate::gui::native_controls) fn set_hwnd_ref(&self, hwnd_ref: &HWND) {
		self.hwnd_ptr.replace(NonNull::from(hwnd_ref));
	}

	pub(in crate::gui::native_controls) fn hwnd(&self) -> HWND {
		unsafe { *self.hwnd_ptr.get().as_ref() }
	}

	/// Adds many columns at once by sending an
	/// [`LVM_INSERTCOLUMN`](crate::msg::lvm::InsertColumn) message.
	///
	/// Widths will be adjusted to match current system DPI.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui;
	///
	/// let my_list: gui::ListView; // initialized somewhere
	///
	/// my_list.columns().add(&[
	///     ("Name", 300),
	///     ("Address", 500),
	/// ]).unwrap();
	/// ```
	pub fn add<S: AsRef<str>>(&self,
		texts_and_widths: &[(S, u32)]) -> WinResult<()>
	{
		for (text, width) in texts_and_widths.iter() {
			let mut col_cx = SIZE::new(*width as _, 0);
			multiply_dpi(None, Some(&mut col_cx))?;

			let mut lvc = LVCOLUMN::default();
			lvc.mask = co::LVCF::TEXT | co::LVCF::WIDTH;
			lvc.cx = col_cx.cx;

			let mut wtext = WString::from_str(text.as_ref());
			lvc.set_pszText(Some(&mut wtext));

			self.hwnd().SendMessage(lvm::InsertColumn {
				index: 0xffff,
				lvcolumn: &lvc,
			})?;
		}

		Ok(())
	}

	/// Retrieves the texts of all items at the given column.
	pub fn all_texts(&self, column_index: u32) -> Vec<String> {
		let count = self.hwnd().SendMessage(lvm::GetItemCount {});
		let mut texts = Vec::with_capacity(count as _);
		let mut buf = WString::default();

		for idx in 0..count {
			ListViewItems::text_retrieve(self.hwnd(), idx, column_index, &mut buf);
			texts.push(buf.to_string());
		}

		texts
	}

	/// Retrieves the number of columns by sending an
	/// [`HDM_GETITEMCOUNT`](crate::msg::hdm::GetItemCount) message to the
	/// handle returned by [`LVM_GETHEADER`](crate::msg::lvm::GetHeader).
	pub fn count(&self) -> WinResult<u32> {
		self.hwnd().SendMessage(lvm::GetHeader {})?
			.SendMessage(hdm::GetItemCount {})
	}

	/// Retrieves information about the column by sending an
	/// [`LVM_GETCOLUMN`](crate::msg::lvm::GetColumn) message.
	pub fn info(&self, column_index: u32, lvc: &mut LVCOLUMN) -> WinResult<()> {
		self.hwnd().SendMessage(lvm::GetColumn {
			index: column_index,
			lvcolumn: lvc,
		})
	}

	/// Retrieves the texts of the selected items at the given column.
	pub fn selected_texts(&self, column_index: u32) -> Vec<String> {
		let sel_count = self.hwnd().SendMessage(lvm::GetSelectedCount {});
		let mut texts = Vec::with_capacity(sel_count as _);
		let mut buf = WString::default();

		let mut idx = None;
		loop {
			idx = match self.hwnd().SendMessage(lvm::GetNextItem {
				initial_index: idx,
				relationship: co::LVNI::SELECTED,
			}) {
				Some(idx) => {
					ListViewItems::text_retrieve(
						self.hwnd(), idx, column_index, &mut buf);
					texts.push(buf.to_string());
					Some(idx)
				},
				None => break,
			};
		}

		texts
	}

	/// Sets information of the column by sending an
	/// [`LVM_SETCOLUMN`](crate::msg::lvm::SetColumn) message.
	pub fn set_info(&self, column_index: u32, lvc: &LVCOLUMN) -> WinResult<()> {
		self.hwnd().SendMessage(lvm::SetColumn {
			index: column_index,
			lvcolumn: lvc,
		})
	}

	/// Sets the title of the column by calling
	/// [`set_info`](crate::gui::ListViewColumns::set_info).
	pub fn set_title(&self, column_index: u32, text: &str) -> WinResult<()> {
		let mut lvc = LVCOLUMN::default();
		lvc.iSubItem = column_index as _;
		lvc.mask = co::LVCF::TEXT;

		let mut buf = WString::from_str(text);
		lvc.set_pszText(Some(&mut buf));

		self.set_info(column_index, &lvc)
	}

	/// Sets the width of the column by sending an
	/// [`LVM_SETCOLUMNWIDTH`](crate::msg::lvm::SetColumnWidth) message.
	///
	/// Width will be adjusted to match current system DPI.
	pub fn set_width(&self, column_index: u32, width: u32) -> WinResult<()> {
		let mut col_cx = SIZE::new(width as _, 0);
		multiply_dpi(None, Some(&mut col_cx))?;

		self.hwnd().SendMessage(lvm::SetColumnWidth {
			index: column_index,
			width: col_cx.cx as _,
		})
	}

	/// Sets the width of the column by sending an
	/// [`LVM_SETCOLUMNWIDTH`](crate::msg::lvm::SetColumnWidth) message. The
	/// width will be calculated to fill the remaining space.
	pub fn set_width_to_fill(&self, column_index: u32) -> WinResult<()> {
		let num_cols = self.count()?;
		let mut cx_used = 0;

		for i in 0..num_cols {
			if i != column_index {
				cx_used += self.width(i)?; // retrieve cx of each column, but us
			}
		}

		let rc = self.hwnd().GetClientRect()?; // list view client area
		self.hwnd().SendMessage(lvm::SetColumnWidth {
			index: column_index,
			width: rc.right as u32 - cx_used,
		})
	}

	/// Retrieves the title of the column by calling
	/// [`info`](crate::gui::ListViewColumns::info).
	///
	/// The passed buffer will be automatically allocated.
	///
	/// This method can be more performant than
	/// [`title_str`](crate::gui::ListViewColumns::title_str) because the buffer
	/// can be reused, avoiding multiple allocations. However, it has the
	/// inconvenient of the manual conversion from [`WString`](crate::WString)
	/// to `String`.
	pub fn title(&self,
		column_index: u32, buf: &mut WString) -> WinResult<String>
	{
		let mut lvc = LVCOLUMN::default();
		lvc.iSubItem = column_index as _;
		lvc.mask = co::LVCF::TEXT;

		let mut buf = buf;
		buf.realloc_buffer(128); // arbitrary
		lvc.set_pszText(Some(&mut buf));

		self.info(column_index, &mut lvc)?;
		Ok(buf.to_string())
	}

	/// A more convenient [`text`](crate::gui::ListViewColumns::title), which
	/// directly returns a `String` instead of requiring an external buffer.
	pub fn title_str(&self, column_index: u32) -> WinResult<String> {
		let mut buf = WString::default();
		self.title(column_index, &mut buf)?;
		Ok(buf.to_string())
	}

	/// Retrieves the width of the column by sending an
	/// [`LVM_GETCOLUMNWIDTH`](crate::msg::lvm::GetColumnWidth) message.
	pub fn width(&self, column_index: u32) -> WinResult<u32> {
		self.hwnd().SendMessage(lvm::GetColumnWidth { index: column_index })
	}
}
