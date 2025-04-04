use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::msg::*;
use crate::prelude::*;

/// Exposes item methods of a [`ComboBox`](crate::gui::ComboBox) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ComboBoxItems<'a> {
	owner: &'a ComboBox,
}

impl<'a> ComboBoxItems<'a> {
	#[must_use]
	pub(in crate::gui) const fn new(owner: &'a ComboBox) -> Self {
		Self { owner }
	}

	/// Adds new texts by sending [`cb::AddString`](crate::msg::cb::AddString)
	/// messages.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_combo: gui::ComboBox; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_combo = gui::ComboBox::new(&wnd, gui::ComboBoxOpts::default());
	///
	/// my_combo.items().add(&["John", "Mary"])?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn add(&self, items: &[impl AsRef<str>]) -> SysResult<()> {
		for text in items.iter() {
			unsafe {
				self.owner
					.hwnd()
					.SendMessage(cb::AddString { text: WString::from_str(text.as_ref()) })?;
			}
		}
		Ok(())
	}

	/// Retrieves the number of items by sending a
	/// [`cb::GetCount`](crate::msg::cb::GetCount) message.
	#[must_use]
	pub fn count(&self) -> SysResult<u32> {
		unsafe { self.owner.hwnd().SendMessage(cb::GetCount {}) }
	}

	/// Deletes the item at the given index by sending a
	/// [`cb::DeleteString`](crate::msg::cb::DeleteString) message.
	pub fn delete(&self, index: u32) -> SysResult<()> {
		unsafe {
			self.owner.hwnd().SendMessage(cb::DeleteString { index })?;
		}
		Ok(())
	}

	/// Deletes all items by sending a
	/// [`cb::ResetContent`](crate::msg::cb::ResetContent) message.
	pub fn delete_all(&self) {
		unsafe {
			self.owner.hwnd().SendMessage(cb::ResetContent {});
		}
	}

	/// Returns an iterator over the text items.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_combo: gui::ComboBox; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_combo = gui::ComboBox::new(&wnd, gui::ComboBoxOpts::default());
	///
	/// for text in my_combo.items().iter()? {
	///     println!("Text {}", text?);
	/// }
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn iter(&self) -> SysResult<impl DoubleEndedIterator<Item = SysResult<String>> + 'a> {
		ComboBoxItemIter::new(self.owner)
	}

	/// Sets the currently selected index, or clears it, by sending a
	/// [`cb::SetCurSel`](crate::msg::cb::SetCurSel) message.
	pub fn select(&self, index: Option<u32>) {
		unsafe {
			self.owner.hwnd().SendMessage(cb::SetCurSel { index });
		}
	}

	/// Retrieves the index of the currently selected item, if any, by sending a
	/// [`cb::GetCurSel`](crate::msg::cb::GetCurSel) message.
	#[must_use]
	pub fn selected_index(&self) -> Option<u32> {
		unsafe { self.owner.hwnd().SendMessage(cb::GetCurSel {}) }
	}

	/// Retrieves the currently selected text, if any, by calling
	/// [`selected_index`](crate::gui::collections::ComboBoxItems::selected_index)
	/// and [`text`](crate::gui::collections::ComboBoxItems::text) methods.
	#[must_use]
	pub fn selected_text(&self) -> SysResult<Option<String>> {
		self.selected_index().map(|idx| self.text(idx)).transpose()
	}

	/// Retrieves the text at the given position, if any, by sending a
	/// [`cb::GetLbText`](crate::msg::cb::GetLbText) message.
	#[must_use]
	pub fn text(&self, index: u32) -> SysResult<String> {
		let num_chars = unsafe { self.owner.hwnd().SendMessage(cb::GetLbTextLen { index })? };

		let mut buf = WString::new_alloc_buf(num_chars as usize + 1);
		unsafe {
			self.owner
				.hwnd()
				.SendMessage(cb::GetLbText { index, text: &mut buf })?;
		}
		Ok(buf.to_string())
	}
}
