use crate::decl::*;
use crate::gui::{*, iterators::*};
use crate::msg::*;
use crate::prelude::*;

/// Exposes item methods of a [`ListBox`](crate::gui::ListBox) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ListBoxItems<'a> {
	owner: &'a ListBox,
}

impl<'a> ListBoxItems<'a> {
	#[must_use]
	pub(in crate::gui) const fn new(owner: &'a ListBox) -> Self {
		Self { owner }
	}

	/// Adds new texts by sending [`lb::AddString`](crate::msg::lb::AddString)
	/// messages.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_list: gui::ListBox; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_list = gui::ListBox::new(&wnd, gui::ListBoxOpts::default());
	///
	/// my_list.items().add(&["John", "Mary"]);
	/// ```
	pub fn add(&self, items: &[impl AsRef<str>]) {
		for text in items.iter() {
			unsafe {
				self.owner.hwnd()
					.SendMessage(lb::AddString {
						text: WString::from_str(text.as_ref()),
					})
			}.unwrap();
		}
	}

	/// Retrieves the number of items by sending an
	/// [`lb::GetCount`](crate::msg::lb::GetCount) message.
	#[must_use]
	pub fn count(&self) -> u32 {
		unsafe {
			self.owner.hwnd()
				.SendMessage(lb::GetCount {})
		}.unwrap()
	}

	/// Deletes the item at the given index by sending an
	/// [`lb::DeleteString`](crate::msg::lb::DeleteString) message.
	pub fn delete(&self, index: u32) {
		unsafe {
			self.owner.hwnd()
				.SendMessage(lb::DeleteString { index })
		}.unwrap();
	}

	/// Deletes all items by sending an
	/// [`lb::ResetContent`](crate::msg::lb::ResetContent) message.
	pub fn delete_all(&self) {
		unsafe { self.owner.hwnd().SendMessage(lb::ResetContent {}); }
	}

	/// Ensures that the specified item in a list box is visible by sending an
	/// [`lb::SetTopIndex`](crate::msg::lb::SetTopIndex) message.
	pub fn ensure_visible(&self, index: u32) {
		unsafe {
			self.owner.hwnd()
				.SendMessage(lb::SetTopIndex { index })
		}.unwrap();
	}

	/// Returns an iterator over the texts.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_list: gui::ListBox; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_list = gui::ListBox::new(&wnd, gui::ListBoxOpts::default());
	///
	/// for text in my_list.items().iter() {
	///     println!("Text {}", text);
	/// }
	/// ```
	#[must_use]
	pub fn iter(&self) -> impl Iterator<Item = String> + 'a {
		ListBoxItemIter::new(self.owner)
	}

	/// Returns an iterator over the currently selected texts.
	///
	/// This method works for both single and multiple-selection lists.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_list: gui::ListBox; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_list = gui::ListBox::new(&wnd, gui::ListBoxOpts::default());
	///
	/// for (sel_idx, text) in my_list.items().iter_selected() {
	///     println!("Text {}: {}", sel_idx, text);
	/// }
	/// ```
	#[must_use]
	pub fn iter_selected(&self) -> impl Iterator<Item = (u32, String)> + 'a {
		ListBoxSelItemIter::new(self.owner)
	}

	/// Retrieves the number of selected items by sending an
	/// [`lb::GetSelCount`](crate::msg::lb::GetSelCount) message.
	#[must_use]
	pub fn selected_count(&self) -> u32 {
		unsafe {
			self.owner.hwnd()
				.SendMessage(lb::GetSelCount {})
		}.unwrap()
	}

	/// Retrieves the text at the given position, if any, by sending a
	/// [`lb::GetText`](crate::msg::lb::GetText) message.
	#[must_use]
	pub fn text(&self, index: u32) -> String {
		let num_chars = unsafe {
			self.owner.hwnd()
				.SendMessage(lb::GetTextLen { index })
		}.unwrap();

		let mut buf = WString::new_alloc_buf(num_chars as usize + 1);

		unsafe {
			self.owner.hwnd()
				.SendMessage(lb::GetText { index, text: &mut buf })
		}.unwrap();

		buf.to_string()
	}
}
