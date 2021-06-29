use std::cell::Cell;
use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::handles::HWND;
use crate::msg::lb;

/// Exposes item methods of a [`ListBox`](crate::gui::ListBox) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListBoxItems {
	hwnd_ptr: Cell<NonNull<HWND>>,
}

impl ListBoxItems {
	pub(in crate::gui::native_controls) fn new() -> ListBoxItems {
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

	/// Adds new texts by sending [`LB_ADDSTRING`](crate::msg::lb::AddString)
	/// messages.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::ListBox;
	///
	/// let lst_names: ListBox; // initialized somewhere
	///
	/// lst_names.items().add(&["John", "Mary"]);
	/// ```
	pub fn add<S: AsRef<str>>(&self, items: &[S]) -> WinResult<()> {
		for text in items.iter() {
			self.hwnd().SendMessage(lb::AddString { text: text.as_ref() })?;
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
