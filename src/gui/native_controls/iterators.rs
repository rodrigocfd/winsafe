use crate::co;
use crate::decl::*;
use crate::gui::{*, spec::*};
use crate::msg::*;
use crate::prelude::*;

pub(in crate::gui) struct ComboBoxItemIter<'a> {
	owner: &'a ComboBox,
	count: u32,
	current: u32,
	buffer: WString,
}

impl<'a> Iterator for ComboBoxItemIter<'a> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let num_chars = unsafe {
			self.owner.hwnd()
				.SendMessage(cb::GetLbTextLen { index: self.current })
		}.unwrap();

		self.buffer = WString::new_alloc_buf(num_chars as usize + 1);

		unsafe {
			self.owner.hwnd()
				.SendMessage(cb::GetLbText {
					index: self.current,
					text: &mut self.buffer,
				})
		}.unwrap();

		self.current += 1;
		Some(self.buffer.to_string())
	}
}

impl<'a> ComboBoxItemIter<'a> {
	pub(in crate::gui) fn new(owner: &'a ComboBox) -> Self {
		Self {
			owner,
			count: owner.items().count(),
			current: 0,
			buffer: WString::default(),
		}
	}
}

//------------------------------------------------------------------------------

pub(in crate::gui) struct EditLineIter<'a> {
	owner: &'a Edit,
	count: u32,
	current: u32,
	buffer: WString,
}

impl<'a> Iterator for EditLineIter<'a> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		unsafe {
			self.owner.hwnd()
				.SendMessage(em::GetLine {
					index: self.current as _,
					buffer: &mut self.buffer,
				})
		}.unwrap();

		self.current += 1;
		Some(self.buffer.to_string())
	}
}

impl<'a> EditLineIter<'a> {
	pub(in crate::gui) fn new(owner: &'a Edit) -> Self {
		Self {
			owner,
			count: unsafe { owner.hwnd().SendMessage(em::GetLineCount {}) },
			current: 0,
			buffer: WString::new_alloc_buf(
				owner.hwnd().GetWindowTextLength().unwrap() as usize + 1,
			),
		}
	}
}

//------------------------------------------------------------------------------

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
	pub(in crate::gui) fn new(owner: &'a Header) -> Self {
		Self {
			owner,
			count: owner.items().count(),
			current: 0,
		}
	}
}

//------------------------------------------------------------------------------

pub(in crate::gui) struct ListBoxItemIter<'a> {
	owner: &'a ListBox,
	count: u32,
	current: u32,
	buffer: WString,
}

impl<'a> Iterator for ListBoxItemIter<'a> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let num_chars = unsafe {
			self.owner.hwnd()
				.SendMessage(lb::GetTextLen { index: self.current })
		}.unwrap();

		self.buffer = WString::new_alloc_buf(num_chars as usize + 1);

		unsafe {
			self.owner.hwnd()
				.SendMessage(lb::GetText {
					index: self.current,
					text: &mut self.buffer,
				})
		}.unwrap();

		self.current += 1;
		Some(self.buffer.to_string())
	}
}

impl<'a> ListBoxItemIter<'a> {
	pub(in crate::gui) fn new(owner: &'a ListBox) -> Self {
		Self {
			owner,
			count: owner.items().count(),
			current: 0,
			buffer: WString::default(),
		}
	}
}

//------------------------------------------------------------------------------

pub(in crate::gui) struct ListBoxSelItemIter<'a> {
	owner: &'a ListBox,
	indexes: Vec<u32>,
	current: u32,
	buffer: WString,
}

impl<'a> Iterator for ListBoxSelItemIter<'a> {
	type Item = (u32, String);

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.indexes.len() as u32 {
			return None;
		}

		let cur_sel_index = self.indexes[self.current as usize];

		let num_chars = unsafe {
			self.owner.hwnd()
				.SendMessage(lb::GetTextLen { index: cur_sel_index })
		}.unwrap();

		self.buffer = WString::new_alloc_buf(num_chars as usize + 1);

		unsafe {
			self.owner.hwnd()
				.SendMessage(lb::GetText {
					index: cur_sel_index,
					text: &mut self.buffer,
				})
		}.unwrap();

		self.current += 1;
		Some((cur_sel_index, self.buffer.to_string()))
	}
}

impl<'a> ListBoxSelItemIter<'a> {
	pub(in crate::gui) fn new(owner: &'a ListBox) -> Self {
		let styles = unsafe {
			co::LBS::from_raw(
				owner.hwnd().GetWindowLongPtr(co::GWLP::STYLE) as _,
			)
		};

		let indexes = if styles.has(co::LBS::EXTENDEDSEL) { // multiple selection?
			let num_indexes = unsafe {
				owner.hwnd()
					.SendMessage(lb::GetSelCount {})
			}.unwrap();

			let mut indexes = vec![0; num_indexes as _];
			unsafe {
				owner.hwnd()
					.SendMessage(lb::GetSelItems { buffer: &mut indexes })
			}.unwrap();
			indexes

		} else {
			match unsafe { owner.hwnd().SendMessage(lb::GetCurSel {}) } {
				Some(index) => vec![index], // single selection: at max 1
				None => Vec::<u32>::default(),
			}
		};

		Self {
			owner,
			indexes,
			current: 0,
			buffer: WString::default(),
		}
	}
}

//------------------------------------------------------------------------------

pub(in crate::gui) struct ListViewColumnIter<'a> {
	owner: &'a ListView,
	count: u32,
	current: u32,
}

impl<'a> Iterator for ListViewColumnIter<'a> {
	type Item = ListViewColumn<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let item = self.owner.columns().get(self.current);
		self.current += 1;
		Some(item)
	}
}

impl<'a> ListViewColumnIter<'a> {
	pub(in crate::gui) fn new(owner: &'a ListView) -> Self {
		Self {
			owner,
			count: owner.items().count(),
			current: 0,
		}
	}
}

//------------------------------------------------------------------------------

pub(in crate::gui) struct ListViewItemIter<'a> {
	owner: &'a ListView,
	current: Option<ListViewItem<'a>>,
	relationship: co::LVNI,
}

impl<'a> Iterator for ListViewItemIter<'a> {
	type Item = ListViewItem<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.current = unsafe {
			self.owner.hwnd()
				.SendMessage(lvm::GetNextItem {
					initial_index: self.current.map(|item| item.index()),
					relationship: self.relationship,
				})
		}.map(|index| self.owner.items().get(index));

		self.current
	}
}

impl<'a> ListViewItemIter<'a> {
	pub(in crate::gui) const fn new(
		owner: &'a ListView,
		relationship: co::LVNI,
	) -> Self
	{
		Self {
			owner,
			current: None,
			relationship,
		}
	}
}

//------------------------------------------------------------------------------

pub(in crate::gui) struct TreeViewItemIter<'a> {
	owner: &'a TreeView,
	current: Option<TreeViewItem<'a>>,
	relationship: co::TVGN,
}

impl<'a> Iterator for TreeViewItemIter<'a> {
	type Item = TreeViewItem<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.current = unsafe {
			self.owner.hwnd()
				.SendMessage(tvm::GetNextItem {
					relationship: self.relationship,
					hitem: self.current.as_ref().map(|tvi| tvi.htreeitem()),
				})
		}.map(|hitem| self.owner.items().get(hitem));

		self.current.as_ref()
			.map(|tvi| TreeViewItem::new(
				self.owner,
				unsafe { tvi.htreeitem().raw_copy() },
			))
	}
}

impl<'a> TreeViewItemIter<'a> {
	pub(in crate::gui) const fn new(
		owner: &'a TreeView,
		current: Option<TreeViewItem<'a>>,
		relationship: co::TVGN,
	) -> Self
	{
		Self { owner, current, relationship }
	}
}

//------------------------------------------------------------------------------

pub(in crate::gui) struct TreeViewChildItemIter<'a> {
	owner: &'a TreeView,
	current: Option<TreeViewItem<'a>>,
	first_call: bool,
}

impl<'a> Iterator for TreeViewChildItemIter<'a> {
	type Item = TreeViewItem<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.first_call { // search for the first child
			self.current = unsafe {
				self.owner.hwnd()
					.SendMessage(tvm::GetNextItem {
						relationship: co::TVGN::CHILD,
						hitem: self.current.as_ref().map(|tvi| tvi.htreeitem()),
					})
			}.map(|hitem| self.owner.items().get(hitem));

			self.first_call = false;

		} else { // search for next siblings
			self.current = unsafe {
				self.owner.hwnd()
					.SendMessage(tvm::GetNextItem {
						relationship: co::TVGN::NEXT,
						hitem: self.current.as_ref().map(|tvi| tvi.htreeitem()),
					})
			}.map(|hitem| self.owner.items().get(hitem));
		}

		self.current.as_ref()
			.map(|tvi| TreeViewItem::new(
				self.owner,
				unsafe { tvi.htreeitem().raw_copy() },
			))
	}
}

impl<'a> TreeViewChildItemIter<'a> {
	pub(in crate::gui) fn new(
		owner: &'a TreeView,
		current: Option<TreeViewItem<'a>>,
	) -> Self
	{
		Self {
			owner,
			current,
			first_call: true,
		}
	}
}
