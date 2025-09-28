use crate::co;
use crate::decl::*;
use crate::gui::*;
use crate::kernel::privs::*;
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
	/// my_list.items().add(&["John", "Mary"])?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn add(&self, items: &[impl AsRef<str>]) -> SysResult<()> {
		items.iter().try_for_each(|text| unsafe {
			self.owner
				.hwnd()
				.SendMessage(lb::AddString { text: WString::from_str(text.as_ref()) })
				.map(|_| ())
		})
	}

	/// Retrieves the number of items by sending an
	/// [`lb::GetCount`](crate::msg::lb::GetCount) message.
	#[must_use]
	pub fn count(&self) -> SysResult<u32> {
		unsafe { self.owner.hwnd().SendMessage(lb::GetCount {}) }
	}

	/// Deletes the item at the given index by sending an
	/// [`lb::DeleteString`](crate::msg::lb::DeleteString) message.
	pub fn delete(&self, index: u32) -> SysResult<()> {
		unsafe {
			self.owner.hwnd().SendMessage(lb::DeleteString { index })?;
		}
		Ok(())
	}

	/// Deletes all items by sending an
	/// [`lb::ResetContent`](crate::msg::lb::ResetContent) message.
	pub fn delete_all(&self) {
		unsafe {
			self.owner.hwnd().SendMessage(lb::ResetContent {});
		}
	}

	/// Ensures that the specified item in a list box is visible by sending an
	/// [`lb::SetTopIndex`](crate::msg::lb::SetTopIndex) message.
	pub fn ensure_visible(&self, index: u32) -> SysResult<()> {
		unsafe { self.owner.hwnd().SendMessage(lb::SetTopIndex { index }) }
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
	/// for text in my_list.items().iter()? {
	///     println!("Text {}", text?);
	/// }
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn iter(&self) -> SysResult<impl DoubleEndedIterator<Item = SysResult<String>> + 'a> {
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
	/// for idx_text in my_list.items().iter_selected()? {
	///     let (idx, text) = idx_text?;
	///     println!("Text {idx}: {text}");
	/// }
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn iter_selected(
		&self,
	) -> SysResult<impl DoubleEndedIterator<Item = SysResult<(u32, String)>> + 'a> {
		ListBoxSelItemIter::new(self.owner)
	}

	/// Retrieves the number of selected items by sending an
	/// [`lb::GetSelCount`](crate::msg::lb::GetSelCount) message.
	#[must_use]
	pub fn selected_count(&self) -> SysResult<u32> {
		unsafe { self.owner.hwnd().SendMessage(lb::GetSelCount {}) }
	}

	/// Retrieves the text at the given position, if any, by sending a
	/// [`lb::GetText`](crate::msg::lb::GetText) message.
	#[must_use]
	pub fn text(&self, index: u32) -> SysResult<String> {
		let num_chars = unsafe { self.owner.hwnd().SendMessage(lb::GetTextLen { index }) }?;

		let mut buf = WString::new_alloc_buf(num_chars as usize + 1);
		unsafe {
			self.owner
				.hwnd()
				.SendMessage(lb::GetText { index, text: &mut buf })?;
		}

		Ok(buf.to_string())
	}
}

struct ListBoxItemIter<'a> {
	owner: &'a ListBox,
	double_idx: DoubleIterIndex,
	buffer: WString,
}

impl<'a> Iterator for ListBoxItemIter<'a> {
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a> DoubleEndedIterator for ListBoxItemIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a> ListBoxItemIter<'a> {
	#[must_use]
	fn new(owner: &'a ListBox) -> SysResult<Self> {
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
					.SendMessage(lb::GetTextLen { index: cur_idx })
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
					.SendMessage(lb::GetText { index: cur_idx, text: &mut self.buffer })
			} {
				Err(e) => DoubleIter::YieldLast(Err(e)), // failed
				Ok(_) => DoubleIter::Yield(Ok(self.buffer.to_string())),
			}
		})
	}
}

struct ListBoxSelItemIter<'a> {
	owner: &'a ListBox,
	double_idx: DoubleIterIndex,
	indexes: Vec<u32>,
	buffer: WString,
}

impl<'a> Iterator for ListBoxSelItemIter<'a> {
	type Item = SysResult<(u32, String)>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a> DoubleEndedIterator for ListBoxSelItemIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a> ListBoxSelItemIter<'a> {
	#[must_use]
	fn new(owner: &'a ListBox) -> SysResult<Self> {
		let style: co::LBS = owner.hwnd().style().into();
		let allow_multiple = style.has(co::LBS::EXTENDEDSEL) || style.has(co::LBS::MULTIPLESEL);
		let indexes = if allow_multiple {
			let num_indexes = unsafe { owner.hwnd().SendMessage(lb::GetSelCount {}) }?;

			let mut indexes = vec![0; num_indexes as _];
			unsafe {
				owner
					.hwnd()
					.SendMessage(lb::GetSelItems { buffer: &mut indexes })
			}?;
			indexes
		} else {
			match unsafe { owner.hwnd().SendMessage(lb::GetCurSel {}) } {
				Some(index) => vec![index], // single selection: at max 1
				None => Vec::<u32>::new(),
			}
		}; // cache all the selected indexes right away

		Ok(Self {
			owner,
			double_idx: DoubleIterIndex::new(indexes.len() as _),
			indexes,
			buffer: WString::new(),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<SysResult<(u32, String)>> {
		self.double_idx.grab(is_front, |cur_idx| {
			let cur_sel_idx = self.indexes[cur_idx as usize];

			// First, get number of chars, without terminating null.
			let num_chars = match unsafe {
				self.owner
					.hwnd()
					.SendMessage(lb::GetTextLen { index: cur_sel_idx })
			} {
				Err(e) => {
					return DoubleIter::YieldLast(Err(e)); // failed
				},
				Ok(n) => n as usize,
			};

			// Then allocate the buffer and get the chars.
			self.buffer = WString::new_alloc_buf(num_chars + 1);
			match unsafe {
				self.owner.hwnd().SendMessage(lb::GetText {
					index: cur_sel_idx,
					text: &mut self.buffer,
				})
			} {
				Err(e) => DoubleIter::YieldLast(Err(e)), // failed
				Ok(_) => DoubleIter::Yield(Ok((cur_sel_idx, self.buffer.to_string()))),
			}
		})
	}
}
