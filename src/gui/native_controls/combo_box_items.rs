use std::marker::PhantomData;

use crate::kernel::decl::{WinResult, WString};
use crate::msg::cb;
use crate::prelude::UserHwnd;
use crate::user::decl::HWND;

/// Exposes item methods of a [`ComboBox`](crate::gui::ComboBox) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct ComboBoxItems<'a> {
	pub(in crate::gui::native_controls) hwnd: HWND,
	pub(in crate::gui::native_controls) _owner: PhantomData<&'a ()>,
}

impl<'a> ComboBoxItems<'a> {
	/// Adds new texts by sending [`cb::AddString`](crate::msg::cb::AddString)
	/// messages.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::gui;
	///
	/// let my_combo: gui::ComboBox; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_combo = gui::ComboBox::new(&wnd, gui::ComboBoxOpts::default());
	///
	/// my_combo.items().add(&["John", "Mary"]);
	/// ```
	pub fn add(&self, items: &[impl AsRef<str>]) -> WinResult<()> {
		for text in items.iter() {
			self.hwnd.SendMessage(cb::AddString {
				text: WString::from_str(text.as_ref()),
			})?;
		}
		Ok(())
	}

	/// Retrieves the number of items by sending a
	/// [`cb::GetCount`](crate::msg::cb::GetCount) message.
	#[must_use]
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
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::gui;
	///
	/// let my_combo: gui::ComboBox; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_combo = gui::ComboBox::new(&wnd, gui::ComboBoxOpts::default());
	///
	/// for text in my_combo.items().iter() {
	///     let text = text?;
	///     println!("Text {}", text);
	/// }
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	pub fn iter(&self) -> impl Iterator<Item = WinResult<String>> + 'a {
		ComboBoxItemIter::new(self.hwnd, self.count().unwrap_or(0))
	}

	/// Sets the currently selected index, or clears it, by sending a
	/// [`cb::SetCurSel`](crate::msg::cb::SetCurSel) message.
	pub fn select(&self, index: Option<u32>) {
		self.hwnd.SendMessage(cb::SetCurSel { index });
	}

	/// Retrieves the index of the currently selected item, if any, by sending a
	/// [`cb::GetCurSel`](crate::msg::cb::GetCurSel) message.
	#[must_use]
	pub fn selected_index(&self) -> Option<u32> {
		self.hwnd.SendMessage(cb::GetCurSel {})
	}

	/// Retrieves the currently selected text, if any, by calling
	/// [`selected_index`](crate::gui::spec::ComboBoxItems::selected_index) and
	/// [`text`](crate::gui::spec::ComboBoxItems::text) methods.
	#[must_use]
	pub fn selected_text(&self) -> WinResult<Option<String>> {
		self.text(
			match self.selected_index() {
				None => return Ok(None),
				Some(idx) => idx,
			},
		)
	}

	/// Retrieves the text at the given position, if any, by sending a
	/// [`cb::GetLbText`](crate::msg::cb::GetLbText) message.
	#[must_use]
	pub fn text(&self, index: u32) -> WinResult<Option<String>> {
		let mut buf = WString::new_alloc_buffer(
			match self.hwnd.SendMessage(cb::GetLbTextLen { index }) {
				Err(_) => return Ok(None), // index out of bounds
				Ok(len) => len,
			} as usize + 1,
		);

		self.hwnd.SendMessage(cb::GetLbText { index, text: &mut buf })
			.map(|_| Some(buf.to_string()))
	}
}

//------------------------------------------------------------------------------

struct ComboBoxItemIter<'a> {
	hwnd: HWND,
	count: u32,
	current: u32,
	buffer: WString,
	_owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for ComboBoxItemIter<'a> {
	type Item = WinResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let len = match self.hwnd
			.SendMessage(cb::GetLbTextLen { index: self.current })
		{
			Err(e) => {
				self.current = self.count; // no further iterations will be made
				return Some(Err(e))
			},
			Ok(len) => len,
		};

		self.buffer.realloc_buffer(len as usize + 1);

		match self.hwnd.SendMessage(cb::GetLbText {
			index: self.current,
			text: &mut self.buffer,
		}) {
			Err(e) => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
			Ok(_) => {
				self.current += 1;
				Some(Ok(self.buffer.to_string()))
			},
		}
	}
}

impl<'a> ComboBoxItemIter<'a> {
	fn new(hwnd: HWND, count: u32) -> Self {
		Self {
			hwnd,
			count,
			current: 0,
			buffer: WString::new_alloc_buffer(40), // arbitrary
			_owner: PhantomData,
		}
	}
}
