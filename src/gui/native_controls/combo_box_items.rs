use crate::decl::*;
use crate::gui::*;
use crate::kernel::privs::*;
use crate::msg;
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

	/// Adds new texts by sending [`CbAddString`](crate::msg::CbAddString)
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
					.SendMessage(msg::CbAddString { text: WString::from_str(text.as_ref()) })?;
			}
		}
		Ok(())
	}

	/// Retrieves the number of items by sending a
	/// [`CbGetCount`](crate::msg::CbGetCount) message.
	#[must_use]
	pub fn count(&self) -> SysResult<u32> {
		unsafe { self.owner.hwnd().SendMessage(msg::CbGetCount {}) }
	}

	/// Deletes the item at the given index by sending a
	/// [`CbDeleteString`](crate::msg::CbDeleteString) message.
	pub fn delete(&self, index: u32) -> SysResult<()> {
		unsafe {
			self.owner
				.hwnd()
				.SendMessage(msg::CbDeleteString { index })?;
		}
		Ok(())
	}

	/// Deletes all items by sending a
	/// [`CbResetContent`](crate::msg::CbResetContent) message.
	pub fn delete_all(&self) {
		unsafe {
			self.owner.hwnd().SendMessage(msg::CbResetContent {});
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
	/// [`CbSetCurSel`](crate::msg::CbSetCurSel) message.
	pub fn select(&self, index: Option<u32>) {
		unsafe {
			self.owner.hwnd().SendMessage(msg::CbSetCurSel { index });
		}
	}

	/// Retrieves the index of the currently selected item, if any, by sending a
	/// [`CbGetCurSel`](crate::msg::CbGetCurSel) message.
	#[must_use]
	pub fn selected_index(&self) -> Option<u32> {
		unsafe { self.owner.hwnd().SendMessage(msg::CbGetCurSel {}) }
	}

	/// Retrieves the currently selected text, if any, by calling
	/// [`selected_index`](crate::gui::collections::ComboBoxItems::selected_index)
	/// and [`text`](crate::gui::collections::ComboBoxItems::text) methods.
	#[must_use]
	pub fn selected_text(&self) -> SysResult<Option<String>> {
		self.selected_index().map(|idx| self.text(idx)).transpose()
	}

	/// Retrieves the text at the given position, if any, by sending a
	/// [`CbGetLbText`](crate::msg::CbGetLbText) message.
	#[must_use]
	pub fn text(&self, index: u32) -> SysResult<String> {
		let num_chars = unsafe {
			self.owner
				.hwnd()
				.SendMessage(msg::CbGetLbTextLen { index })?
		};

		let mut buf = WString::new_alloc_buf(num_chars as usize + 1);
		unsafe {
			self.owner
				.hwnd()
				.SendMessage(msg::CbGetLbText { index, text: &mut buf })?;
		}
		Ok(buf.to_string())
	}
}

struct ComboBoxItemIter<'a> {
	owner: &'a ComboBox,
	double_idx: DoubleIterIndex,
	buffer: WString,
}

impl<'a> Iterator for ComboBoxItemIter<'a> {
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a> DoubleEndedIterator for ComboBoxItemIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a> ComboBoxItemIter<'a> {
	#[must_use]
	fn new(owner: &'a ComboBox) -> SysResult<Self> {
		Ok(Self {
			owner,
			double_idx: DoubleIterIndex::new(owner.items().count()?),
			buffer: WString::new(),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<SysResult<String>> {
		self.double_idx.grab(is_front, |cur_idx| {
			// First, get number of chars, without terminating null.
			let num_chars = match unsafe {
				self.owner
					.hwnd()
					.SendMessage(msg::CbGetLbTextLen { index: cur_idx })
			} {
				Err(e) => {
					return DoubleIter::YieldLast(Err(e)); // failed
				},
				Ok(n) => n as usize,
			};

			// Then allocate the buffer and get the chars.
			self.buffer = WString::new_alloc_buf(num_chars + 1);
			match unsafe {
				self.owner
					.hwnd()
					.SendMessage(msg::CbGetLbText { index: cur_idx, text: &mut self.buffer })
			} {
				Err(e) => DoubleIter::YieldLast(Err(e)), // failed
				Ok(_) => DoubleIter::Yield(Ok(self.buffer.to_string())),
			}
		})
	}
}
