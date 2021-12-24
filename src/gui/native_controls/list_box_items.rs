use std::marker::PhantomData;

use crate::co;
use crate::kernel::decl::{WinResult, WString};
use crate::msg::lb;
use crate::prelude::{NativeBitflag, UserHwnd};
use crate::user::decl::HWND;

/// Exposes item methods of a [`ListBox`](crate::gui::ListBox) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
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
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::gui;
	///
	/// let my_list: gui::ListBox; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_list = gui::ListBox::new(&wnd, gui::ListBoxOpts::default());
	///
	/// my_list.items().add(&["John", "Mary"]);
	/// ```
	pub fn add(&self, items: &[impl AsRef<str>]) -> WinResult<()> {
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

	/// Returns an iterator over the texts.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::gui;
	///
	/// let my_list: gui::ListBox; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_list = gui::ListBox::new(&wnd, gui::ListBoxOpts::default());
	///
	/// for text in my_list.items().iter() {
	///     let text = text?;
	///     println!("Text {}", text);
	/// }
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	pub fn iter(&self) -> impl Iterator<Item = WinResult<String>> + 'a {
		ListBoxItemIter::new(self.hwnd, self.count().unwrap_or(0))
	}

	/// Returns the currently selected items.
	///
	/// This method works for both single and multiple-selection lists.
	pub fn selected(&self) -> Vec<u32> {
		let styles = co::LBS(self.hwnd.GetWindowLongPtr(co::GWLP::STYLE) as _);

		if styles.has(co::LBS::EXTENDEDSEL) {
			let num_indexes = match self.hwnd.SendMessage(lb::GetSelCount {}) {
				Err(_) => return vec![], // should never happen
				Ok(sel_count) => sel_count as _,
			};
			let mut indexes = vec![0; num_indexes];

			match self.hwnd.SendMessage(lb::GetSelItems {
				buffer: &mut indexes,
			}) {
				Err(_) => vec![], // should never happen
				Ok(_) => indexes,
			}

		} else {
			self.hwnd.SendMessage(lb::GetCurSel {})
				.map_or_else(
					|| Vec::default(),
					|idx| vec![idx],
				)
		}
	}

	/// Retrieves the text at the given position, if any, by sending a
	/// [`lb::GetText`](crate::msg::lb::GetText) message.
	pub fn text(&self, index: u32) -> WinResult<Option<String>> {
		let mut buf = WString::new_alloc_buffer(
			match self.hwnd.SendMessage(lb::GetTextLen { index }) {
				Err(_) => return Ok(None), // index out of bounds
				Ok(len) => len,
			} as usize + 1,
		);

		self.hwnd.SendMessage(lb::GetText { index, text: &mut buf })
			.map(|_| Some(buf.to_string()))
	}
}

//------------------------------------------------------------------------------

struct ListBoxItemIter<'a> {
	hwnd: HWND,
	count: u32,
	current: u32,
	buffer: WString,
	owner_: PhantomData<&'a ()>,
}

impl<'a> Iterator for ListBoxItemIter<'a> {
	type Item = WinResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let len = match self.hwnd
			.SendMessage(lb::GetTextLen { index: self.current })
		{
			Err(e) => {
				self.current = self.count; // no further iterations will be made
				return Some(Err(e))
			},
			Ok(len) => len,
		};

		self.buffer.realloc_buffer(len as usize + 1);

		match self.hwnd.SendMessage(lb::GetText {
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

impl<'a> ListBoxItemIter<'a> {
	fn new(hwnd: HWND, count: u32) -> Self {
		Self {
			hwnd,
			count,
			current: 0,
			buffer: WString::new_alloc_buffer(40), // arbitrary
			owner_: PhantomData,
		}
	}
}
