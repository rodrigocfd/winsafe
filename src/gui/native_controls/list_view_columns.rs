use crate::co;
use crate::comctl::decl::LVCOLUMN;
use crate::gui::native_controls::list_view::ListView;
use crate::gui::privs::multiply_dpi;
use crate::kernel::decl::WString;
use crate::msg::{hdm, lvm};
use crate::prelude::{GuiWindow, user_Hwnd};
use crate::user::decl::SIZE;

/// Exposes column methods of a [`ListView`](crate::gui::ListView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct ListViewColumns<'a> {
	owner: &'a ListView,
}

impl<'a> ListViewColumns<'a> {
	pub(in crate::gui) const fn new(owner: &'a ListView) -> Self {
		Self { owner }
	}

	/// Adds many columns at once by sending an
	/// [`lvm::InsertColumn`](crate::msg::lvm::InsertColumn) message.
	///
	/// Widths will be adjusted to match current system DPI.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::gui;
	///
	/// let my_list: gui::ListView; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_list = gui::ListView::new(&wnd, gui::ListViewOpts::default());
	///
	/// my_list.columns().add(&[
	///     ("Name", 300),
	///     ("Address", 500),
	/// ]);
	/// ```
	pub fn add(&self, texts_and_widths: &[(impl AsRef<str>, u32)]) {
		for (text, width) in texts_and_widths.iter() {
			let mut col_cx = SIZE::new(*width as _, 0);
			multiply_dpi(None, Some(&mut col_cx));

			let mut lvc = LVCOLUMN::default();
			lvc.mask = co::LVCF::TEXT | co::LVCF::WIDTH;
			lvc.cx = col_cx.cx;

			let mut wtext = WString::from_str(text.as_ref());
			lvc.set_pszText(Some(&mut wtext));

			self.owner.hwnd()
				.SendMessage(lvm::InsertColumn {
					index: 0xffff, // insert as the last columns
					lvcolumn: &lvc,
				})
				.unwrap();
		}
	}

	/// Retrieves the number of columns by sending an
	/// [`hdm::GetItemCount`](crate::msg::hdm::GetItemCount) message to the
	/// handle returned by [`lvm::GetHeader`](crate::msg::lvm::GetHeader).
	#[must_use]
	pub fn count(&self) -> u32 {
		self.owner.hwnd()
			.SendMessage(lvm::GetHeader {})
			.unwrap()
			.SendMessage(hdm::GetItemCount {})
			.unwrap()
	}

	/// Retrieves information about the column by sending an
	/// [`lvm::GetColumn`](crate::msg::lvm::GetColumn) message.
	pub fn info(&self, column_index: u32, lvc: &mut LVCOLUMN) {
		self.owner.hwnd()
			.SendMessage(lvm::GetColumn {
				index: column_index,
				lvcolumn: lvc,
			})
			.unwrap();
	}

	/// Sets information of the column by sending an
	/// [`lvm::SetColumn`](crate::msg::lvm::SetColumn) message.
	pub fn set_info(&self, column_index: u32, lvc: &LVCOLUMN) {
		self.owner.hwnd()
			.SendMessage(lvm::SetColumn {
				index: column_index,
				lvcolumn: lvc,
			})
			.unwrap();
	}

	/// Sets the title of the column by calling
	/// [`set_info`](crate::gui::spec::ListViewColumns::set_info).
	pub fn set_title(&self, column_index: u32, text: &str) {
		let mut lvc = LVCOLUMN::default();
		lvc.iSubItem = column_index as _;
		lvc.mask = co::LVCF::TEXT;

		let mut buf = WString::from_str(text);
		lvc.set_pszText(Some(&mut buf));

		self.set_info(column_index, &lvc);
	}

	/// Sets the width of the column by sending an
	/// [`lvm::SetColumnWidth`](crate::msg::lvm::SetColumnWidth) message.
	///
	/// Width will be adjusted to match current system DPI.
	pub fn set_width(&self, column_index: u32, width: u32) {
		let mut col_cx = SIZE::new(width as _, 0);
		multiply_dpi(None, Some(&mut col_cx));

		self.owner.hwnd()
			.SendMessage(lvm::SetColumnWidth {
				index: column_index,
				width: col_cx.cx as _,
			})
			.unwrap();
	}

	/// Sets the width of the column by sending an
	/// [`lvm::SetColumnWidth`](crate::msg::lvm::SetColumnWidth) message. The
	/// width will be calculated to fill the remaining space.
	pub fn set_width_to_fill(&self, column_index: u32) {
		let num_cols = self.count();
		let mut cx_used = 0;

		for i in 0..num_cols {
			if i != column_index {
				cx_used += self.width(i); // retrieve cx of each column, but us
			}
		}

		let rc = self.owner.hwnd().GetClientRect().unwrap(); // list view client area
		self.owner.hwnd()
			.SendMessage(lvm::SetColumnWidth {
				index: column_index,
				width: rc.right as u32 - cx_used,
			})
			.unwrap();
	}

	/// Retrieves the title of the column by calling
	/// [`info`](crate::gui::spec::ListViewColumns::info).
	#[must_use]
	pub fn title(&self, column_index: u32) -> String {
		let mut lvc = LVCOLUMN::default();
		lvc.iSubItem = column_index as _;
		lvc.mask = co::LVCF::TEXT;

		let mut buf = WString::new_alloc_buf(128); // arbitrary
		lvc.set_pszText(Some(&mut buf));

		self.info(column_index, &mut lvc);
		buf.to_string()
	}

	/// Retrieves the width of the column by sending an
	/// [`lvm::GetColumnWidth`](crate::msg::lvm::GetColumnWidth) message.
	#[must_use]
	pub fn width(&self, column_index: u32) -> u32 {
		self.owner.hwnd()
			.SendMessage(lvm::GetColumnWidth { index: column_index })
			.unwrap()
	}
}
