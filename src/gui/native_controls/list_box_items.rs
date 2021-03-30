use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::handles::HWND;
use crate::msg::lb;

/// Exposes item methods of a [`ListBox`](crate::gui::ListBox) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListBoxItems {
	hwnd_ptr: VeryUnsafeCell<NonNull<HWND>>,
}

impl ListBoxItems {
	pub(crate) fn new(hwnd_ref: &HWND) -> ListBoxItems {
		Self {
			hwnd_ptr: VeryUnsafeCell::new(NonNull::from(hwnd_ref)), // ref implicitly converted to pointer
		}
	}

	pub(crate) fn set_hwnd_ref(&self, hwnd_ref: &HWND) {
		*self.hwnd_ptr.as_mut() = NonNull::from(hwnd_ref); // ref implicitly converted to pointer
	}

	pub(crate) fn hwnd(&self) -> HWND {
		unsafe { *self.hwnd_ptr.as_ref() }
	}

	/// Adds new texts by sending [`LB_ADDSTRING`](crate::msg::lb::AddString)
	/// messages.
	pub fn add(&self, items: &[&str]) -> WinResult<()> {
		for text in items.iter() {
			self.hwnd().SendMessage(lb::AddString { text })?;
		}
		Ok(())
	}

	/// Retrieves the number of items by sending an
	/// [`LB_GETCOUNT`](crate::msg::lb::GetCount) message.
	pub fn count(&self) -> WinResult<u32> {
		self.hwnd().SendMessage(lb::GetCount {})
	}

	/// Deletes the item at the given index by sending an
	/// [`LB_DELETESTRING`](crate::msg::lb::DeleteString) message.
	pub fn delete(&self, index: u32) -> WinResult<()> {
		self.hwnd().SendMessage(lb::DeleteString { index })
			.map(|_| ())
	}

	/// Deletes all items by sending an
	/// [`LB_RESETCONTENT`](crate::msg::lb::ResetContent) message.
	pub fn delete_all(&self) {
		self.hwnd().SendMessage(lb::ResetContent {})
	}
}
