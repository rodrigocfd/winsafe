use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::co;
use crate::gui::immut::Immut;
use crate::gui::privs::multiply_dpi;
use crate::handles::HWND;
use crate::msg::{hdm, lvm};
use crate::structs::{LVCOLUMN, SIZE};
use crate::WString;

/// Exposes column methods of a [`ListView`](crate::gui::ListView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListViewColumns {
	hwnd_ptr: Immut<NonNull<HWND>>,
}

impl ListViewColumns {
	pub(crate) fn new(hwnd_ref: &HWND) -> ListViewColumns {
		Self {
			hwnd_ptr: Immut::new(NonNull::from(hwnd_ref)), // ref implicitly converted to pointer
		}
	}

	pub(crate) fn set_hwnd_ref(&self, hwnd_ref: &HWND) {
		*self.hwnd_ptr.as_mut() = NonNull::from(hwnd_ref); // ref implicitly converted to pointer
	}

	pub(crate) fn hwnd(&self) -> HWND {
		unsafe { *self.hwnd_ptr.as_ref() }
	}

	/// Adds many columns at once by sending an
	/// [`LVM_INSERTCOLUMN`](crate::msg::lvm::InsertColumn) message.
	///
	/// Widths will be adjusted to match current system DPI.
	pub fn add(&self, texts_and_widths: &[(&str, u32)]) -> WinResult<()> {
		for (text, width) in texts_and_widths.iter() {
			let mut col_cx = SIZE::new(*width as i32, 0);
			multiply_dpi(None, Some(&mut col_cx))?;

			let mut lvc = LVCOLUMN::default();
			lvc.mask = co::LVCF::TEXT | co::LVCF::WIDTH;
			lvc.cx = col_cx.cx;

			let mut wtext = WString::from_str(text);
			lvc.set_pszText(&mut wtext);

			self.hwnd().SendMessage(lvm::InsertColumn {
				index: 0xffff,
				lvcolumn: &lvc,
			})?;
		}

		Ok(())
	}

	/// Retrieves the number of columns by sending an
	/// [`HDM_GETITEMCOUNT`](crate::msg::hdm::GetItemCount) message to the handle
	/// returned by [`LVM_GETHEADER`](crate::msg::lvm::GetHeader).
	pub fn count(&self) -> WinResult<u32> {
		self.hwnd().SendMessage(lvm::GetHeader {})?
			.SendMessage(hdm::GetItemCount {})
	}
}
