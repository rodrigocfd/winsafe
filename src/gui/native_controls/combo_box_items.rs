use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::funcs::PostQuitMessage;
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::handles::HWND;
use crate::msg::cb;
use crate::WString;

/// Exposes item methods of a [`ComboBox`](crate::gui::ComboBox) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ComboBoxItems {
	hwnd_ptr: VeryUnsafeCell<NonNull<HWND>>,
}

impl ComboBoxItems {
	pub(crate) fn new(hwnd_ref: &HWND) -> ComboBoxItems {
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

	/// Adds new texts by sending [`CB_ADDSTRING`](crate::msg::cb::AddString)
	/// messages.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::ComboBox;
	///
	/// let cmb_names: ComboBox; // initialize it somewhere...
	///
	/// cmb_names.items().add(&["John", "Mary"]);
	/// ```
	pub fn add(&self, items: &[&str]) -> WinResult<()> {
		for text in items.iter() {
			self.hwnd().SendMessage(cb::AddString { text })?;
		}
		Ok(())
	}

	/// Retrieves the number of items by sending a
	/// [`CB_GETCOUNT`](crate::msg::cb::GetCount) message.
	pub fn count(&self) -> WinResult<u32> {
		self.hwnd().SendMessage(cb::GetCount {})
	}

	/// Deletes the item at the given index by sending a
	/// [`CB_DELETESTRING`](crate::msg::cb::DeleteString) message.
	pub fn delete(&self, index: u32) -> WinResult<()> {
		self.hwnd().SendMessage(cb::DeleteString { index })
			.map(|_| ())
	}

	/// Deletes all items by sending a
	/// [`CB_RESETCONTENT`](crate::msg::cb::ResetContent) message.
	pub fn delete_all(&self) {
		self.hwnd().SendMessage(cb::ResetContent {})
	}

	/// Retrieves the index of the currently selected item, if any, by sending a
	/// [`CB_GETCURSEL`](crate::msg::cb::GetCurSel) message.
	pub fn selected_index(&self) -> Option<u32> {
		self.hwnd().SendMessage(cb::GetCurSel {})
	}

	/// Retrieves the currently selected text, if any, by calling
	/// [`selected_index`](crate::gui::ComboBoxItems::selected_index) and
	/// [`text`](crate::gui::ComboBoxItems::text) methods.
	pub fn selected_text(&self) -> Option<String> {
		self.selected_index()
			.and_then(|idx| self.text(idx))
	}

	/// Sets the currently selected index, or clears it, by sending a
	/// [`CB_SETCURSEL`](crate::msg::cb::SetCurSel) message.
	pub fn set_selected(&self, index: Option<u32>) {
		self.hwnd().SendMessage(cb::SetCurSel { index });
	}

	/// Retrieves the text at the given position, if any, by sending a
	/// [`CB_GETLBTEXT`](crate::msg::cb::GetLbText) message.
	pub fn text(&self, index: u32) -> Option<String> {
		match self.hwnd().SendMessage(cb::GetLbTextLen { index }) {
			Err(err) => {
				PostQuitMessage(err);
				None
			},
			Ok(len) => {
				let mut buf = WString::new_alloc_buffer(len as usize + 1);
				self.hwnd().SendMessage(cb::GetLbText{
					index,
					text: &mut buf,
				}).ok().map(|_| buf.to_string())
			},
		}
	}
}
