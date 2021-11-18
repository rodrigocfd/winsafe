use std::marker::PhantomData;

use crate::aliases::WinResult;
use crate::co;
use crate::gui::privs::multiply_dpi;
use crate::handles::HWND;
use crate::msg::{hdm, lvm};
use crate::structs::{LVCOLUMN, SIZE};
use crate::various::WString;

/// Exposes column methods of a [`ListView`](crate::gui::ListView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListViewColumns<'a> {
	pub(in crate::gui::native_controls) hwnd: HWND,
	pub(in crate::gui::native_controls) owner: PhantomData<&'a ()>,
}

impl<'a> ListViewColumns<'a> {
	/// Adds many columns at once by sending an
	/// [`lvm::InsertColumn`](crate::msg::lvm::InsertColumn) message.
	///
	/// Widths will be adjusted to match current system DPI.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::gui;
	///
	/// let my_list: gui::ListView; // initialized somewhere
	///
	/// my_list.columns().add(&[
	///     ("Name", 300),
	///     ("Address", 500),
	/// ])?;
	/// ```
	pub fn add(&self,
		texts_and_widths: &[(impl AsRef<str>, u32)]) -> WinResult<()>
	{
		for (text, width) in texts_and_widths.iter() {
			let mut col_cx = SIZE::new(*width as _, 0);
			multiply_dpi(None, Some(&mut col_cx))?;

			let mut lvc = LVCOLUMN::default();
			lvc.mask = co::LVCF::TEXT | co::LVCF::WIDTH;
			lvc.cx = col_cx.cx;

			let mut wtext = WString::from_str(text.as_ref());
			lvc.set_pszText(Some(&mut wtext));

			self.hwnd.SendMessage(lvm::InsertColumn {
				index: 0xffff, // insert as the last columns
				lvcolumn: &lvc,
			})?;
		}

		Ok(())
	}

	/// Retrieves the number of columns by sending an
	/// [`hdm::GetItemCount`](crate::msg::hdm::GetItemCount) message to the
	/// handle returned by [`lvm::GetHeader`](crate::msg::lvm::GetHeader).
	pub fn count(&self) -> WinResult<u32> {
		self.hwnd.SendMessage(lvm::GetHeader {})?
			.SendMessage(hdm::GetItemCount {})
	}

	/// Retrieves information about the column by sending an
	/// [`lvm::GetColumn`](crate::msg::lvm::GetColumn) message.
	pub fn info(&self, column_index: u32, lvc: &mut LVCOLUMN) -> WinResult<()> {
		self.hwnd.SendMessage(lvm::GetColumn {
			index: column_index,
			lvcolumn: lvc,
		})
	}

	/// Sets information of the column by sending an
	/// [`lvm::SetColumn`](crate::msg::lvm::SetColumn) message.
	pub fn set_info(&self, column_index: u32, lvc: &LVCOLUMN) -> WinResult<()> {
		self.hwnd.SendMessage(lvm::SetColumn {
			index: column_index,
			lvcolumn: lvc,
		})
	}

	/// Sets the title of the column by calling
	/// [`set_info`](crate::gui::spec::ListViewColumns::set_info).
	pub fn set_title(&self, column_index: u32, text: &str) -> WinResult<()> {
		let mut lvc = LVCOLUMN::default();
		lvc.iSubItem = column_index as _;
		lvc.mask = co::LVCF::TEXT;

		let mut buf = WString::from_str(text);
		lvc.set_pszText(Some(&mut buf));

		self.set_info(column_index, &lvc)
	}

	/// Sets the width of the column by sending an
	/// [`lvm::SetColumnWidth`](crate::msg::lvm::SetColumnWidth) message.
	///
	/// Width will be adjusted to match current system DPI.
	pub fn set_width(&self, column_index: u32, width: u32) -> WinResult<()> {
		let mut col_cx = SIZE::new(width as _, 0);
		multiply_dpi(None, Some(&mut col_cx))?;

		self.hwnd.SendMessage(lvm::SetColumnWidth {
			index: column_index,
			width: col_cx.cx as _,
		})
	}

	/// Sets the width of the column by sending an
	/// [`lvm::SetColumnWidth`](crate::msg::lvm::SetColumnWidth) message. The
	/// width will be calculated to fill the remaining space.
	pub fn set_width_to_fill(&self, column_index: u32) -> WinResult<()> {
		let num_cols = self.count()?;
		let mut cx_used = 0;

		for i in 0..num_cols {
			if i != column_index {
				cx_used += self.width(i)?; // retrieve cx of each column, but us
			}
		}

		let rc = self.hwnd.GetClientRect()?; // list view client area
		self.hwnd.SendMessage(lvm::SetColumnWidth {
			index: column_index,
			width: rc.right as u32 - cx_used,
		})
	}

	/// Retrieves the title of the column by calling
	/// [`info`](crate::gui::spec::ListViewColumns::info).
	pub fn title(&self, column_index: u32) -> WinResult<String> {
		let mut lvc = LVCOLUMN::default();
		lvc.iSubItem = column_index as _;
		lvc.mask = co::LVCF::TEXT;

		let mut buf = WString::new_alloc_buffer(128); // arbitrary
		lvc.set_pszText(Some(&mut buf));

		self.info(column_index, &mut lvc)?;
		Ok(buf.to_string())
	}

	/// Retrieves the width of the column by sending an
	/// [`lvm::GetColumnWidth`](crate::msg::lvm::GetColumnWidth) message.
	pub fn width(&self, column_index: u32) -> WinResult<u32> {
		self.hwnd.SendMessage(lvm::GetColumnWidth { index: column_index })
	}
}
