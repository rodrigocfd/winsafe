use std::marker::PhantomData;

use crate::aliases::WinResult;
use crate::handles::HWND;
use crate::msg::cb;
use crate::various::WString;

/// Exposes item methods of a [`ComboBox`](crate::gui::ComboBox) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ComboBoxItems<'a> {
	pub(in crate::gui::native_controls) hwnd: HWND,
	pub(in crate::gui::native_controls) owner: PhantomData<&'a ()>,
}

impl<'a> ComboBoxItems<'a> {
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
			self.hwnd.SendMessage(cb::AddString {
				text: WString::from_str(text.as_ref()),
			})?;
		}
		Ok(())
	}

	/// Retrieves the number of items by sending a
	/// [`cb::GetCount`](crate::msg::cb::GetCount) message.
	pub fn count(&self) -> WinResult<u32> {
		self.hwnd.SendMessage(cb::GetCount {})
	}

	/// Deletes the item at the given index by sending a
	/// [`cb::DeleteString`](crate::msg::cb::DeleteString) message.
	pub fn delete(&self, index: u32) -> WinResult<()> {
		self.hwnd.SendMessage(cb::DeleteString { index })
			.map(|_| ())
	}

	/// Deletes all items by sending a
	/// [`cb::ResetContent`](crate::msg::cb::ResetContent) message.
	pub fn delete_all(&self) {
		self.hwnd.SendMessage(cb::ResetContent {})
	}

	/// Returns an iterator over the texts.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui;
	///
	/// let my_combo: gui::ComboBox; // initialized somewhere
	///
	/// for text in my_combo.items().iter() {
	///     println!("Text {}", text);
	/// }
	/// ```
	pub fn iter(&self) -> impl Iterator<Item = String> {
		ComboBoxItemIter::new(self.hwnd, self.count().unwrap_or(0))
	}

	/// Sets the currently selected index, or clears it, by sending a
	/// [`cb::SetCurSel`](crate::msg::cb::SetCurSel) message.
	pub fn select(&self, index: Option<u32>) {
		self.hwnd.SendMessage(cb::SetCurSel { index });
	}

	/// Retrieves the index of the currently selected item, if any, by sending a
	/// [`cb::GetCurSel`](crate::msg::cb::GetCurSel) message.
	pub fn selected_index(&self) -> Option<u32> {
		self.hwnd.SendMessage(cb::GetCurSel {})
	}

	/// Retrieves the currently selected text, if any, by calling
	/// [`selected_index`](crate::gui::spec::ComboBoxItems::selected_index) and
	/// [`text`](crate::gui::spec::ComboBoxItems::text) methods.
	pub fn selected_text(&self) -> Option<String> {
		self.selected_index()
			.and_then(|idx| self.text(idx))
	}

	/// Retrieves the text at the given position, if any, by sending a
	/// [`cb::GetLbText`](crate::msg::cb::GetLbText) message.
	pub fn text(&self, index: u32) -> Option<String> {
		self.iter().nth(index as _)
	}
}

//------------------------------------------------------------------------------

struct ComboBoxItemIter<'a> {
	hwnd: HWND,
	current: Option<u32>,
	total: u32,
	buf: WString,
	owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for ComboBoxItemIter<'a> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		self.current.and_then(|index| {
			self.hwnd.SendMessage(cb::GetLbTextLen { index })
				.ok()
				.and_then(|len| {
					self.buf.realloc_buffer(len as usize + 1);
					self.hwnd.SendMessage(cb::GetLbText{
						index,
						text: &mut self.buf,
					}).ok()
						.map(|_| {
							self.current = if index + 1 == self.total {
								None // iteration is over
							} else {
								Some(index + 1)
							};
							self.buf.to_string()
						})
				})
		})
	}
}

impl<'a> ComboBoxItemIter<'a> {
	fn new(hwnd: HWND, total: u32) -> Self {
		Self {
			hwnd,
			current: Some(0),
			total,
			buf: WString::default(),
			owner: PhantomData,
		}
	}
}
