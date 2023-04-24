use crate::gui::native_controls::status_bar_part::StatusBarPart;
use crate::gui::native_controls::status_bar::StatusBar;
use crate::msg::sb;
use crate::prelude::{GuiWindow, user_Hwnd};

/// Exposes the part methods of a [`StatusBar`](crate::gui::StatusBar) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
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

	/// Retrieves the part at the given zero-based position.
	///
	/// **Note:** This method is cheap – even if `index` is beyond the range of
	/// existing parts, an object will still be returned. However, operations
	/// upon this object will fail.
	#[must_use]
	pub const fn get(&self, index: u8) -> StatusBarPart<'a> {
		StatusBarPart::new(self.owner, index)
	}
}
