use crate::co;
use crate::decl::*;
use crate::gui::*;
use crate::msg;
use crate::prelude::*;

/// A single part of a [`StatusBar`](crate::gui::StatusBar) control.
///
/// **Note:** Each object keeps the zero-based index of a part. If new parts are
/// added/removed from the status bar control, the object may then point to a
/// different item.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[derive(Clone, Copy)]
pub struct StatusBarPart<'a> {
	owner: &'a StatusBar,
	index: u32,
}

impl<'a> StatusBarPart<'a> {
	#[must_use]
	pub(in crate::gui) const fn new(owner: &'a StatusBar, index: u32) -> Self {
		Self { owner, index }
	}

	/// Returns the zero-based index of the part.
	#[must_use]
	pub const fn index(&self) -> u32 {
		self.index
	}

	/// Sets the icon of a part by sending an
	/// [`SbSetIcon`](crate::msg::SbSetIcon) message.
	///
	/// Returns the same part, so further operations can be chained.
	pub fn set_icon(&self, hicon: Option<&HICON>) -> SysResult<Self> {
		unsafe {
			self.owner
				.hwnd()
				.SendMessage(msg::SbSetIcon { part_index: self.index as _, hicon })?;
		}
		Ok(*self)
	}

	/// Sets the text of a part by sending an
	/// [`SbSetText`](crate::msg::SbSetText) message.
	///
	/// Returns the same part, so further operations can be chained.
	pub fn set_text(&self, text: &str) -> SysResult<Self> {
		unsafe {
			self.owner.hwnd().SendMessage(msg::SbSetText {
				part_index: self.index as _,
				draw_operation: co::SBT::BORDER,
				text: WString::from_str(text),
			})?;
		}
		Ok(*self)
	}

	/// Retrieves the text of the item by sending a
	/// [`SbGetText`](crate::msg::SbGetText) message.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_sb: gui::StatusBar; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_sb = gui::StatusBar::new(&wnd, &[]);
	///
	/// println!("Text: {}", my_sb.parts().get(0).text());
	/// ```
	#[must_use]
	pub fn text(&self) -> String {
		let (num_chars, _) = unsafe {
			self.owner
				.hwnd()
				.SendMessage(msg::SbGetTextLength { part_index: self.index as _ })
		};

		let mut buf = WString::new_alloc_buf(num_chars as usize + 1);
		unsafe {
			self.owner.hwnd().SendMessage(msg::SbGetText {
				part_index: self.index as _,
				text: &mut buf,
			});
		}
		buf.to_string()
	}
}
