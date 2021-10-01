use std::marker::PhantomData;

use crate::aliases::WinResult;
use crate::co;
use crate::handles::HWND;
use crate::msg::sb;
use crate::various::WString;

/// Exposes the part methods of a [`StatusBar`](crate::gui::StatusBar) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct StatusBarParts<'a> {
	pub(in crate::gui::native_controls) hwnd: HWND,
	pub(in crate::gui::native_controls) owner: PhantomData<&'a ()>,
}

impl<'a> StatusBarParts<'a> {
	/// Retrieves the number of parts by sending an
	/// [`sb::GetParts`](crate::msg::sb::GetParts) message.
	pub fn count(&self) -> u8 {
		self.hwnd.SendMessage(sb::GetParts { right_edges: None })
	}

	/// Sets the text of a part by sending an
	/// [`sb::SetText`](crate::msg::sb::SetText) message.
	pub fn set_text(&self, part_index: u8, text: &str) -> WinResult<()> {
		self.hwnd.SendMessage(sb::SetText {
			part_index,
			draw_operation: co::SBT::BORDER,
			text: WString::from_str(text),
		})
	}

	/// Retrieves the text of the item by sending a
	/// [`sb::GetText`](crate::msg::sb::GetText) message.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui;
	///
	/// let my_sb: gui::StatusBar; // initialized somewhere
	///
	/// println!("Text: {}", my_sb.parts().text(0));
	/// ```
	pub fn text(&self, part_index: u8) -> String {
		let (len, _) = self.hwnd.SendMessage(sb::GetTextLength { part_index });
		let mut buf = WString::new_alloc_buffer(len as usize + 1);
		self.hwnd.SendMessage(sb::GetText {
			part_index,
			text: &mut buf,
		});
		buf.to_string()
	}
}
