use std::cell::Cell;
use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::co;
use crate::handles::HWND;
use crate::msg::sb;
use crate::WString;

/// Exposes the part methods of a [`StatusBar`](crate::gui::StatusBar) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct StatusBarParts {
	hwnd_ptr: Cell<NonNull<HWND>>,
}

impl StatusBarParts {
	pub(crate) fn new(hwnd_ref: &HWND) -> StatusBarParts {
		Self {
			hwnd_ptr: Cell::new(NonNull::from(hwnd_ref)), // ref implicitly converted to pointer
		}
	}

	pub(crate) fn set_hwnd_ref(&self, hwnd_ref: &HWND) {
		self.hwnd_ptr.replace(NonNull::from(hwnd_ref)); // ref implicitly converted to pointer
	}

	pub(crate) fn hwnd(&self) -> HWND {
		unsafe { *self.hwnd_ptr.get().as_ref() }
	}

	/// Retrieves the number of parts by sending an
	/// [`SB_GETPARTS`](crate::msg::sb::GetParts) message.
	pub fn count(&self) -> u8 {
		self.hwnd().SendMessage(sb::GetParts { right_edges: None })
	}

	/// Sets the text of a part by sending an
	/// [`SB_SETTEXT`](crate::msg::sb::SetText) message.
	pub fn set_text(&self, part_index: u8, text: &str) -> WinResult<()> {
		self.hwnd().SendMessage(sb::SetText {
			part_index,
			drawing_operation: co::SBT::NONE,
			text,
		})
	}

	/// Retrieves the text of the item by sending a
	/// [`SB_GETTEXT`](crate::msg::sb::GetText) message.
	///
	/// The passed buffer will be automatically allocated.
	///
	/// This method can be more performant than
	/// [`text_str`](crate::gui::StatusBarParts::text_str) because the buffer
	/// can be reused, avoiding multiple allocations. However, it has the
	/// inconvenient of the manual conversion from [`WString`](crate::WString)
	/// to `String`.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{gui, WString};
	///
	/// let my_sb: gui::StatusBar; // initialized somewhere
	///
	/// let mut buf = WString::default();
	/// my_sb.parts().text(0, &mut buf).unwrap();
	///
	/// println!("Text: {}", buf.to_string());
	/// ```
	pub fn text(&self, part_index: u8, mut buf: &mut WString) {
		let (len, _) = self.hwnd().SendMessage(sb::GetTextLength { part_index });
		buf.realloc_buffer(len as usize + 1);

		self.hwnd().SendMessage(sb::GetText {
			part_index,
			text: &mut buf,
		});
	}

	/// A more convenient [`text`](crate::gui::StatusBarParts::text), which
	/// directly returns a `String` instead of requiring an external buffer.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui;
	///
	/// let my_sb: gui::StatusBar; // initialized somewhere
	///
	/// println!("Text: {}", my_sb.parts().text(0).unwrap());
	/// ```
	pub fn text_str(&self, part_index: u8) -> String {
		let mut buf = WString::default();
		self.text(part_index, &mut buf);
		buf.to_string()
	}
}
