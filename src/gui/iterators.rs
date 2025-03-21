use crate::co;
use crate::decl::*;
use crate::gui::*;
use crate::msg::*;
use crate::prelude::*;

pub(in crate::gui) struct ComboBoxItemIter<'a> {
	owner: &'a ComboBox,
	count: u32,
	current: u32,
	buffer: WString,
}

impl<'a> Iterator for ComboBoxItemIter<'a> {
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let num_chars = match unsafe {
			self.owner
				.hwnd()
				.SendMessage(cb::GetLbTextLen { index: self.current })
		} {
			Err(e) => {
				self.current = self.count; // halt
				return Some(Err(e));
			},
			Ok(n) => n,
		};

		self.buffer = WString::new_alloc_buf(num_chars as usize + 1);
		if let Err(e) = unsafe {
			self.owner.hwnd().SendMessage(cb::GetLbText {
				index: self.current,
				text: &mut self.buffer,
			})
		} {
			self.current = self.count; // halt
			return Some(Err(e));
		}

		self.current += 1;
		Some(Ok(self.buffer.to_string()))
	}
}

impl<'a> ComboBoxItemIter<'a> {
	#[must_use]
	pub(in crate::gui) fn new(owner: &'a ComboBox) -> SysResult<Self> {
		Ok(Self {
			owner,
			count: owner.items().count()?,
			current: 0,
			buffer: WString::new(),
		})
	}
}

pub(in crate::gui) struct HeaderItemIter<'a> {
	owner: &'a Header,
	count: u32,
	current: u32,
}

impl<'a> Iterator for HeaderItemIter<'a> {
	type Item = HeaderItem<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let item = self.owner.items().get(self.current);
		self.current += 1;
		Some(item)
	}
}

impl<'a> HeaderItemIter<'a> {
	#[must_use]
	pub(in crate::gui) fn new(owner: &'a Header) -> SysResult<Self> {
		Ok(Self {
			owner,
			count: owner.items().count()?,
			current: 0,
		})
	}
}

pub(in crate::gui) struct ListBoxItemIter<'a> {
	owner: &'a ListBox,
	count: u32,
	current: u32,
	buffer: WString,
}

impl<'a> Iterator for ListBoxItemIter<'a> {
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let num_chars = match unsafe {
			self.owner
				.hwnd()
				.SendMessage(lb::GetTextLen { index: self.current })
		} {
			Err(e) => {
				self.current = self.count; // halt
				return Some(Err(e));
			},
			Ok(n) => n,
		};

		self.buffer = WString::new_alloc_buf(num_chars as usize + 1);
		if let Err(e) = unsafe {
			self.owner.hwnd().SendMessage(lb::GetText {
				index: self.current,
				text: &mut self.buffer,
			})
		} {
			self.current = self.count; // halt
			return Some(Err(e));
		}

		self.current += 1;
		Some(Ok(self.buffer.to_string()))
	}
}

impl<'a> ListBoxItemIter<'a> {
	#[must_use]
	pub(in crate::gui) fn new(owner: &'a ListBox) -> SysResult<Self> {
		Ok(Self {
			owner,
			count: owner.items().count()?,
			current: 0,
			buffer: WString::new(),
		})
	}
}

pub(in crate::gui) struct ListBoxSelItemIter<'a> {
	owner: &'a ListBox,
	indexes: Vec<u32>,
	current: u32,
	buffer: WString,
}

impl<'a> Iterator for ListBoxSelItemIter<'a> {
	type Item = SysResult<(u32, String)>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.indexes.len() as _ {
			return None;
		}

		let cur_sel_index = self.indexes[self.current as usize];

		let num_chars = match unsafe {
			self.owner
				.hwnd()
				.SendMessage(lb::GetTextLen { index: cur_sel_index })
		} {
			Err(e) => {
				self.current = self.indexes.len() as _; // halt
				return Some(Err(e));
			},
			Ok(n) => n,
		};

		self.buffer = WString::new_alloc_buf(num_chars as usize + 1);
		if let Err(e) = unsafe {
			self.owner.hwnd().SendMessage(lb::GetText {
				index: cur_sel_index,
				text: &mut self.buffer,
			})
		} {
			self.current = self.indexes.len() as _; // halt
			return Some(Err(e));
		}

		self.current += 1;
		Some(Ok((cur_sel_index, self.buffer.to_string())))
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

		Ok(Self {
			owner,
			indexes,
			current: 0,
			buffer: WString::new(),
		})
	}
}

pub(in crate::gui) struct ListViewColIter<'a, T: 'static> {
	owner: &'a ListView<T>,
	count: u32,
	current: u32,
}

impl<'a, T> Iterator for ListViewColIter<'a, T> {
	type Item = ListViewCol<'a, T>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let item = self.owner.cols().get(self.current);
		self.current += 1;
		Some(item)
	}
}

impl<'a, T> ListViewColIter<'a, T> {
	#[must_use]
	pub(in crate::gui) fn new(owner: &'a ListView<T>) -> SysResult<Self> {
		Ok(Self {
			owner,
			count: owner.cols().count()?,
			current: 0,
		})
	}
}

pub(in crate::gui) struct ListViewItemIter<'a, T: 'static> {
	owner: &'a ListView<T>,
	current: Option<ListViewItem<'a, T>>,
	relationship: co::LVNI,
}

impl<'a, T> Iterator for ListViewItemIter<'a, T> {
	type Item = ListViewItem<'a, T>;

	fn next(&mut self) -> Option<Self::Item> {
		self.current = unsafe {
			self.owner.hwnd().SendMessage(lvm::GetNextItem {
				initial_index: self.current.map(|item| item.index()),
				relationship: self.relationship,
			})
		}
		.map(|index| self.owner.items().get(index));

		self.current
	}
}

impl<'a, T> ListViewItemIter<'a, T> {
	#[must_use]
	pub(in crate::gui) const fn new(owner: &'a ListView<T>, relationship: co::LVNI) -> Self {
		Self { owner, current: None, relationship }
	}
}

pub(in crate::gui) struct StatusBarPartIter<'a> {
	owner: &'a StatusBar,
	count: u32,
	current: u32,
}

impl<'a> Iterator for StatusBarPartIter<'a> {
	type Item = StatusBarPart<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let item = self.owner.parts().get(self.current);
		self.current += 1;
		Some(item)
	}
}

impl<'a> StatusBarPartIter<'a> {
	#[must_use]
	pub(in crate::gui) fn new(owner: &'a StatusBar) -> Self {
		Self {
			owner,
			count: owner.parts().count(),
			current: 0,
		}
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
