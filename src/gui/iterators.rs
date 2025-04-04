use crate::co;
use crate::decl::*;
use crate::gui::*;
use crate::msg::*;
use crate::prelude::*;

pub(in crate::gui) struct ComboBoxItemIter<'a> {
	owner: &'a ComboBox,
	front_idx: u32,
	past_back_idx: u32,
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
	pub(in crate::gui) fn new(owner: &'a ComboBox) -> SysResult<Self> {
		Ok(Self {
			owner,
			front_idx: 0,
			past_back_idx: owner.items().count()?,
			buffer: WString::new(),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<SysResult<String>> {
		if self.front_idx == self.past_back_idx {
			return None;
		}
		let our_idx = if is_front { self.front_idx } else { self.past_back_idx - 1 };

		// First, get number of chars, without terminating null.
		let num_chars = match unsafe {
			self.owner
				.hwnd()
				.SendMessage(cb::GetLbTextLen { index: our_idx })
		} {
			Err(e) => {
				(self.front_idx, self.past_back_idx) = (0, 0); // halt
				return Some(Err(e));
			},
			Ok(n) => n as usize,
		};

		// Then allocate the buffer and get the chars.
		self.buffer = WString::new_alloc_buf(num_chars + 1);
		if let Err(e) = unsafe {
			self.owner
				.hwnd()
				.SendMessage(cb::GetLbText { index: our_idx, text: &mut self.buffer })
		} {
			(self.front_idx, self.past_back_idx) = (0, 0); // halt
			return Some(Err(e));
		}

		if is_front {
			self.front_idx += 1;
		} else {
			self.past_back_idx -= 1;
		}

		Some(Ok(self.buffer.to_string()))
	}
}

pub(in crate::gui) struct HeaderItemIter<'a> {
	owner: &'a Header,
	front_idx: u32,
	past_back_idx: u32,
}

impl<'a> Iterator for HeaderItemIter<'a> {
	type Item = HeaderItem<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a> DoubleEndedIterator for HeaderItemIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a> HeaderItemIter<'a> {
	#[must_use]
	pub(in crate::gui) fn new(owner: &'a Header) -> SysResult<Self> {
		Ok(Self {
			owner,
			front_idx: 0,
			past_back_idx: owner.items().count()?,
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<HeaderItem<'a>> {
		if self.front_idx == self.past_back_idx {
			return None;
		}
		let our_idx = if is_front { self.front_idx } else { self.past_back_idx - 1 };

		let item = self.owner.items().get(our_idx);
		if is_front {
			self.front_idx += 1;
		} else {
			self.past_back_idx -= 1;
		}
		Some(item)
	}
}

pub(in crate::gui) struct ListBoxItemIter<'a> {
	owner: &'a ListBox,
	front_idx: u32,
	past_back_idx: u32,
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
	pub(in crate::gui) fn new(owner: &'a ListBox) -> SysResult<Self> {
		Ok(Self {
			owner,
			front_idx: 0,
			past_back_idx: owner.items().count()?,
			buffer: WString::new(),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<SysResult<String>> {
		if self.front_idx == self.past_back_idx {
			return None;
		}
		let our_idx = if is_front { self.front_idx } else { self.past_back_idx - 1 };

		// First, get number of chars, without terminating null.
		let num_chars = match unsafe {
			self.owner
				.hwnd()
				.SendMessage(lb::GetTextLen { index: our_idx })
		} {
			Err(e) => {
				(self.front_idx, self.past_back_idx) = (0, 0); // halt
				return Some(Err(e));
			},
			Ok(n) => n as usize,
		};

		// Then allocate the buffer and get the chars.
		self.buffer = WString::new_alloc_buf(num_chars + 1);
		if let Err(e) = unsafe {
			self.owner
				.hwnd()
				.SendMessage(lb::GetText { index: our_idx, text: &mut self.buffer })
		} {
			(self.front_idx, self.past_back_idx) = (0, 0); // halt
			return Some(Err(e));
		}

		if is_front {
			self.front_idx += 1;
		} else {
			self.past_back_idx -= 1;
		}

		Some(Ok(self.buffer.to_string()))
	}
}

pub(in crate::gui) struct ListBoxSelItemIter<'a> {
	owner: &'a ListBox,
	indexes: Vec<u32>,
	front_idx: u32,
	past_back_idx: u32,
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
	pub(in crate::gui) fn new(owner: &'a ListBox) -> SysResult<Self> {
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
		};
		let count = indexes.len();

		Ok(Self {
			owner,
			indexes,
			front_idx: 0,
			past_back_idx: count as _,
			buffer: WString::new(),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<SysResult<(u32, String)>> {
		if self.front_idx == self.past_back_idx {
			return None;
		}
		let our_idx = if is_front { self.front_idx } else { self.past_back_idx - 1 };
		let cur_sel_index = self.indexes[our_idx as usize];

		// First, get number of chars, without terminating null.
		let num_chars = match unsafe {
			self.owner
				.hwnd()
				.SendMessage(lb::GetTextLen { index: cur_sel_index })
		} {
			Err(e) => {
				(self.front_idx, self.past_back_idx) = (0, 0); // halt
				return Some(Err(e));
			},
			Ok(n) => n as usize,
		};

		// Then allocate the buffer and get the chars.
		self.buffer = WString::new_alloc_buf(num_chars + 1);
		if let Err(e) = unsafe {
			self.owner.hwnd().SendMessage(lb::GetText {
				index: cur_sel_index,
				text: &mut self.buffer,
			})
		} {
			(self.front_idx, self.past_back_idx) = (0, 0); // halt
			return Some(Err(e));
		}

		if is_front {
			self.front_idx += 1;
		} else {
			self.past_back_idx -= 1;
		}

		Some(Ok((cur_sel_index, self.buffer.to_string())))
	}
}

pub(in crate::gui) struct ListViewColIter<'a, T: 'static> {
	owner: &'a ListView<T>,
	front_idx: u32,
	past_back_idx: u32,
}

impl<'a, T> Iterator for ListViewColIter<'a, T> {
	type Item = ListViewCol<'a, T>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a, T> DoubleEndedIterator for ListViewColIter<'a, T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a, T> ListViewColIter<'a, T> {
	#[must_use]
	pub(in crate::gui) fn new(owner: &'a ListView<T>) -> SysResult<Self> {
		Ok(Self {
			owner,
			front_idx: 0,
			past_back_idx: owner.cols().count()?,
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<ListViewCol<'a, T>> {
		if self.front_idx == self.past_back_idx {
			return None;
		}
		let our_idx = if is_front { self.front_idx } else { self.past_back_idx - 1 };

		let item = self.owner.cols().get(our_idx);
		if is_front {
			self.front_idx += 1;
		} else {
			self.past_back_idx -= 1;
		}
		Some(item)
	}
}

pub(in crate::gui) struct ListViewItemIter<'a, T: 'static> {
	owner: &'a ListView<T>,
	front_idx: u32,
	past_back_idx: u32,
	is_sel: bool,
}

impl<'a, T> Iterator for ListViewItemIter<'a, T> {
	type Item = ListViewItem<'a, T>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a, T> DoubleEndedIterator for ListViewItemIter<'a, T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a, T> ListViewItemIter<'a, T> {
	#[must_use]
	pub(in crate::gui) fn new(owner: &'a ListView<T>, is_sel: bool) -> Self {
		Self {
			owner,
			front_idx: 0,
			past_back_idx: owner.items().count(),
			is_sel,
		}
	}

	fn grab(&mut self, is_front: bool) -> Option<ListViewItem<'a, T>> {
		if self.front_idx == self.past_back_idx {
			return None;
		}

		let mut our_idx = if is_front { self.front_idx } else { self.past_back_idx - 1 };
		let mut item = self.owner.items().get(our_idx);

		// LVNI_SELECTED|LVNI_PREVIOUS flags don't seem to work together, so we check each item manually.
		while self.is_sel && !item.is_selected() {
			if is_front {
				self.front_idx += 1;
			} else {
				self.past_back_idx -= 1;
			}
			if self.front_idx == self.past_back_idx {
				return None;
			}
			our_idx = if is_front { self.front_idx } else { self.past_back_idx - 1 };
			item = self.owner.items().get(our_idx);
		}

		if is_front {
			self.front_idx += 1;
		} else {
			self.past_back_idx -= 1;
		}
		Some(item)
	}
}

pub(in crate::gui) struct StatusBarPartIter<'a> {
	owner: &'a StatusBar,
	front_idx: u32,
	past_back_idx: u32,
}

impl<'a> Iterator for StatusBarPartIter<'a> {
	type Item = StatusBarPart<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a> DoubleEndedIterator for StatusBarPartIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a> StatusBarPartIter<'a> {
	#[must_use]
	pub(in crate::gui) fn new(owner: &'a StatusBar) -> Self {
		Self {
			owner,
			front_idx: 0,
			past_back_idx: owner.parts().count(),
		}
	}

	fn grab(&mut self, is_front: bool) -> Option<StatusBarPart<'a>> {
		if self.front_idx == self.past_back_idx {
			return None;
		}
		let our_idx = if is_front { self.front_idx } else { self.past_back_idx - 1 };

		let part = self.owner.parts().get(our_idx);
		if is_front {
			self.front_idx += 1;
		} else {
			self.past_back_idx -= 1;
		}
		Some(part)
	}
}

pub(in crate::gui) struct TreeViewItemIter<'a, T: 'static> {
	owner: &'a TreeView<T>,
	current: Option<TreeViewItem<'a, T>>,
	relationship: co::TVGN,
}

impl<'a, T> Iterator for TreeViewItemIter<'a, T> {
	type Item = TreeViewItem<'a, T>;

	fn next(&mut self) -> Option<Self::Item> {
		self.current = unsafe {
			self.owner.hwnd().SendMessage(tvm::GetNextItem {
				relationship: self.relationship,
				hitem: self.current.as_ref().map(|tvi| tvi.htreeitem()),
			})
		}
		.map(|hitem| self.owner.items().get(&hitem));

		self.current
			.as_ref()
			.map(|tvi| TreeViewItem::new(self.owner, unsafe { tvi.htreeitem().raw_copy() }))
	}
}

impl<'a, T> TreeViewItemIter<'a, T> {
	#[must_use]
	pub(in crate::gui) const fn new(
		owner: &'a TreeView<T>,
		current: Option<TreeViewItem<'a, T>>,
		relationship: co::TVGN,
	) -> Self {
		Self { owner, current, relationship }
	}
}

pub(in crate::gui) struct TreeViewChildItemIter<'a, T: 'static> {
	owner: &'a TreeView<T>,
	current: Option<TreeViewItem<'a, T>>,
	first_call: bool,
}

impl<'a, T> Iterator for TreeViewChildItemIter<'a, T> {
	type Item = TreeViewItem<'a, T>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.first_call {
			// Search for the first child.
			self.current = unsafe {
				self.owner.hwnd().SendMessage(tvm::GetNextItem {
					relationship: co::TVGN::CHILD,
					hitem: self.current.as_ref().map(|tvi| tvi.htreeitem()),
				})
			}
			.map(|hitem| self.owner.items().get(&hitem));

			self.first_call = false;
		} else {
			// Search for next siblings.
			self.current = unsafe {
				self.owner.hwnd().SendMessage(tvm::GetNextItem {
					relationship: co::TVGN::NEXT,
					hitem: self.current.as_ref().map(|tvi| tvi.htreeitem()),
				})
			}
			.map(|hitem| self.owner.items().get(&hitem));
		}

		self.current
			.as_ref()
			.map(|tvi| TreeViewItem::new(self.owner, unsafe { tvi.htreeitem().raw_copy() }))
	}
}

impl<'a, T> TreeViewChildItemIter<'a, T> {
	#[must_use]
	pub(in crate::gui) fn new(
		owner: &'a TreeView<T>,
		current: Option<TreeViewItem<'a, T>>,
	) -> Self {
		Self { owner, current, first_call: true }
	}
}
