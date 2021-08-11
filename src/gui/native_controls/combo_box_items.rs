use std::cell::Cell;
use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::funcs::PostQuitMessage;
use crate::handles::HWND;
use crate::msg::cb;
use crate::various::WString;

/// Exposes item methods of a [`ComboBox`](crate::gui::ComboBox) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ComboBoxItems {
	hwnd_ptr: Cell<NonNull<HWND>>,
}

impl ComboBoxItems {
	pub(in crate::gui::native_controls) fn new() -> ComboBoxItems {
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

	/// Adds new texts by sending [`cb::AddString`](crate::msg::cb::AddString)
	/// messages.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::ComboBox;
	///
	/// let cmb_names: ComboBox; // initialized somewhere
	///
	/// cmb_names.items().add(&["John", "Mary"]);
	/// ```
	pub fn add<S: AsRef<str>>(&self, items: &[S]) -> WinResult<()> {
		for text in items.iter() {
			self.hwnd().SendMessage(cb::AddString { text: text.as_ref() })?;
		}
		Ok(())
	}

	/// Retrieves the number of items by sending a
	/// [`cb::GetCount`](crate::msg::cb::GetCount) message.
	pub fn count(&self) -> WinResult<u32> {
		self.hwnd().SendMessage(cb::GetCount {})
	}

	/// Deletes the item at the given index by sending a
	/// [`cb::DeleteString`](crate::msg::cb::DeleteString) message.
	pub fn delete(&self, index: u32) -> WinResult<()> {
		self.hwnd().SendMessage(cb::DeleteString { index })
			.map(|_| ())
	}

	/// Deletes all items by sending a
	/// [`cb::ResetContent`](crate::msg::cb::ResetContent) message.
	pub fn delete_all(&self) {
		self.hwnd().SendMessage(cb::ResetContent {})
	}

	/// Retrieves the index of the currently selected item, if any, by sending a
	/// [`cb::GetCurSel`](crate::msg::cb::GetCurSel) message.
	pub fn selected_index(&self) -> Option<u32> {
		self.hwnd().SendMessage(cb::GetCurSel {})
	}

	/// Retrieves the currently selected text, if any, by calling
	/// [`selected_index`](crate::gui::spec::ComboBoxItems::selected_index) and
	/// [`text`](crate::gui::spec::ComboBoxItems::text) methods.
	pub fn selected_text(&self) -> Option<String> {
		self.selected_index()
			.and_then(|idx| self.text(idx))
	}

	/// Sets the currently selected index, or clears it, by sending a
	/// [`cb::SetCurSel`](crate::msg::cb::SetCurSel) message.
	pub fn set_selected(&self, index: Option<u32>) {
		self.hwnd().SendMessage(cb::SetCurSel { index });
	}

	/// Retrieves the text at the given position, if any, by sending a
	/// [`cb::GetLbText`](crate::msg::cb::GetLbText) message.
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
