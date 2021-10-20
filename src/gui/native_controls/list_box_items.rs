use std::marker::PhantomData;

use crate::aliases::WinResult;
use crate::handles::HWND;
use crate::msg::lb;
use crate::various::WString;

/// Exposes item methods of a [`ListBox`](crate::gui::ListBox) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListBoxItems<'a> {
	pub(in crate::gui::native_controls) hwnd: HWND,
	pub(in crate::gui::native_controls) owner: PhantomData<&'a ()>,
}

impl<'a> ListBoxItems<'a> {
	/// Adds new texts by sending [`lb::AddString`](crate::msg::lb::AddString)
	/// messages.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::ListBox;
	///
	/// let lst_names: ListBox; // initialized somewhere
	///
	/// lst_names.items().add(&["John", "Mary"]);
	/// ```
	pub fn add<S: AsRef<str>>(&self, items: &[S]) -> WinResult<()> {
		for text in items.iter() {
			self.hwnd.SendMessage(lb::AddString {
				text: WString::from_str(text.as_ref()),
			})?;
		}
		Ok(())
	}

	/// Retrieves the number of items by sending an
	/// [`lb::GetCount`](crate::msg::lb::GetCount) message.
	pub fn count(&self) -> WinResult<u32> {
		self.hwnd.SendMessage(lb::GetCount {})
	}

	/// Deletes the item at the given index by sending an
	/// [`lb::DeleteString`](crate::msg::lb::DeleteString) message.
	pub fn delete(&self, index: u32) -> WinResult<()> {
		self.hwnd.SendMessage(lb::DeleteString { index })
			.map(|_| ())
	}

	/// Deletes all items by sending an
	/// [`lb::ResetContent`](crate::msg::lb::ResetContent) message.
	pub fn delete_all(&self) {
		self.hwnd.SendMessage(lb::ResetContent {})
	}
}
