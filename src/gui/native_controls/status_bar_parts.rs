use crate::co;
use crate::gui::native_controls::status_bar::StatusBar;
use crate::kernel::decl::WString;
use crate::msg::sb;
use crate::prelude::{GuiWindow, user_Hwnd};

/// Exposes the part methods of a [`StatusBar`](crate::gui::StatusBar) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct StatusBarParts<'a> {
	owner: &'a StatusBar,
}

impl<'a> StatusBarParts<'a> {
	pub(in crate::gui) const fn new(owner: &'a StatusBar) -> Self {
		Self { owner }
	}

	/// Retrieves the number of parts by sending an
	/// [`sb::GetParts`](crate::msg::sb::GetParts) message.
	#[must_use]
	pub fn count(&self) -> u8 {
		self.owner.hwnd()
			.SendMessage(sb::GetParts { right_edges: None })
	}

	/// Sets the text of a part by sending an
	/// [`sb::SetText`](crate::msg::sb::SetText) message.
	pub fn set_text(&self, part_index: u8, text: &str) {
		self.owner.hwnd()
			.SendMessage(sb::SetText {
				part_index,
				draw_operation: co::SBT::BORDER,
				text: WString::from_str(text),
			})
			.unwrap();
	}

	/// Retrieves the text of the item by sending a
	/// [`sb::GetText`](crate::msg::sb::GetText) message.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::gui;
	///
	/// let my_sb: gui::StatusBar; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_sb = gui::StatusBar::new(&wnd, &[]);
	///
	/// println!("Text: {}", my_sb.parts().text(0));
	/// ```
	#[must_use]
	pub fn text(&self, part_index: u8) -> String {
		let (num_chars, _) = self.owner.hwnd()
			.SendMessage(sb::GetTextLength { part_index });

		let mut buf = WString::new_alloc_buf(num_chars as usize + 1);
		self.owner.hwnd()
			.SendMessage(sb::GetText {
				part_index,
				text: &mut buf,
			});
		buf.to_string()
	}
}
