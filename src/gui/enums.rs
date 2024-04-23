use crate::co;
use crate::decl::*;
use crate::prelude::*;

/// The class background brush to be loaded for
/// [`WindowMainOpts`](crate::gui::WindowMainOpts),
/// [`WindowModalOpts`](crate::gui::WindowModalOpts) or
/// [`WindowControlOpts`](crate::gui::WindowControlOpts).
pub enum Brush {
	/// A solid [system color](co::COLOR).
	Color(co::COLOR),
	/// A brush handle, previously created by you.
	Handle(HBRUSH),
	/// No brush.
	None,
}

impl Brush {
	/// Converts the contents of `Brush` to `HBRUSH`.
	#[must_use]
	pub fn as_hbrush(&self) -> HBRUSH {
		match self {
			Brush::Color(c) => HBRUSH::from_sys_color(*c),
			Brush::Handle(h) => unsafe { h.raw_copy() },
			Brush::None => HBRUSH::NULL,
		}
	}
}

/// Possible states of a [`CheckBox`](crate::gui::CheckBox) control.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckState {
	/// CheckBox is checked.
	Checked,
	/// CheckBox is grayed, indicating an indeterminate state. Applicable only
	/// if the CheckBox was created with [`BS::R3STATE`](crate::co::BS::R3STATE)
	/// or [`BS::AUTO3STATE`](crate::co::BS::AUTO3STATE) styles.
	Indeterminate,
	/// CheckBox is cleared.
	Unchecked,
}

/// The class cursor to be loaded for
/// [`WindowMainOpts`](crate::gui::WindowMainOpts),
/// [`WindowModalOpts`](crate::gui::WindowModalOpts) or
/// [`WindowControlOpts`](crate::gui::WindowControlOpts).
pub enum Cursor {
	/// A cursor handle, previously loaded by you.
	Handle(HCURSOR),
	/// A resource ID.
	Id(u16),
	/// A [`co::IDC`](crate::co::IDC) constant for a stock system cursor.
	Idc(co::IDC),
	/// No cursor.
	None,
	/// A resource string identifier.
	Str(WString),
}

impl Cursor {
	/// Converts the contents of `Cursor` to `HCURSOR`.
	#[must_use]
	pub fn as_hcursor(&self, hinst: &HINSTANCE) -> SysResult<HCURSOR> {
		unsafe {
			Ok(match self {
				Cursor::Handle(h) => h.raw_copy(),
				Cursor::Id(id) => hinst.LoadCursor(IdIdcStr::Id(*id))?.leak(),
				Cursor::Idc(idc) => HINSTANCE::NULL.LoadCursor(IdIdcStr::Idc(*idc))?.leak(),
				Cursor::None => HCURSOR::NULL,
				Cursor::Str(s) => hinst.LoadCursor(IdIdcStr::Str(s.clone()))?.leak(),
			})
		}
	}
}

/// Possible states of the arrow in a
/// [`HeaderItem`](crate::gui::spec::HeaderItem).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum HeaderArrow {
	/// No arrow.
	None,
	/// An arrow pointing up, indicating sorting in ascending order.
	Asc,
	/// An arrow pointing down, indicating sorting in descending order.
	Desc,
}

/// Specifies the horizontal behavior of the control when the parent window is
/// resized.
///
/// The values are analog to [`gui::Vert`](crate::gui::Vert).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Horz {
	/// Nothing will be done when parent window is resized.
	None,
	/// When parent window resizes, the control will move anchored at right.
	/// Size of the control will remain fixed.
	Repos,
	/// When parent window resizes, the control width will stretch/shrink
	/// accordingly. Position will remain fixed.
	Resize,
}

/// The class icon to be loaded for
/// [`WindowMainOpts`](crate::gui::WindowMainOpts),
/// [`WindowModalOpts`](crate::gui::WindowModalOpts) or
/// [`WindowControlOpts`](crate::gui::WindowControlOpts).
pub enum Icon {
	/// An icon handle, previously loaded by you.
	Handle(HICON),
	/// A resource ID.
	Id(u16),
	/// A [`co::IDC`](crate::co::IDC) constant for a stock system icon.
	Idi(co::IDI),
	/// No icon.
	None,
	/// A resource string identifier.
	Str(WString),
}

impl Icon {
	/// Converts the contents of `Icon` to `HICON`.
	#[must_use]
	pub fn as_hicon(&self, hinst: &HINSTANCE) -> SysResult<HICON> {
		unsafe {
			Ok(match self {
				Icon::Handle(h) => h.raw_copy(),
				Icon::Id(id) => hinst.LoadIcon(IdIdiStr::Id(*id))?.leak(),
				Icon::Idi(idi) => HINSTANCE::NULL.LoadIcon(IdIdiStr::Idi(*idi))?.leak(),
				Icon::None => HICON::NULL,
				Icon::Str(s) => hinst.LoadIcon(IdIdiStr::Str(s.clone()))?.leak(),
			})
		}
	}
}

/// Used when adding the parts in
/// [`StatusBar::new`](crate::gui::StatusBar::new).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SbPart {
	/// A part that has a fixed size, in pixels.
	///
	/// Will be adjusted to match current system DPI.
	Fixed(u32),
	/// A part that will resize when the parent window resizes, filling the
	/// space left by the fixed-size parts. Has the resizing proportion.
	///
	/// How proportion works:
	///
	/// 1. Suppose you have 3 parts, respectively with proportions of 1, 1 and 2.
	/// 2. If available client area width is 400px, respective part widths will be 100, 100 and 200px.
	/// 3. If parent is resized to have a client area of 600px, parts will then have 200, 200 and 400px.
	///
	/// If you're uncertain, just give all resizable parts the proportion 1.
	Proportional(u8),
}

/// Specifies the vertical behavior of the control when the parent window is
/// resized.
///
/// The values are analog to [`gui::Horz`](crate::gui::Horz).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vert {
	/// Nothing will be done when parent window is resized.
	None,
	/// When parent window resizes, the control will move anchored at bottom.
	/// Size of the control will remain fixed.
	Repos,
	/// When parent window resizes, the control height will stretch/shrink
	/// accordingly. Position will remain fixed.
	Resize,
}

/// The result of processing a raw [`WM`](crate::gui::events::WindowEvents::wm),
/// [`WM_COMMAND`](crate::gui::events::WindowEvents::wm_command) or
/// [`WM_NOTIFY`](crate::gui::events::WindowEvents::wm_notify) message.
pub enum WmRet {
	/// Behave as if the message was not handled, that means returning:
	///
	/// * [`DefWindowProc`](crate::prelude::user_Hwnd::DefWindowProc) for non-dialog windows;
	/// * `FALSE` for dialog windows.
	///
	/// This type of return should be rare. It means you handled the message,
	/// but you want the OS to behave like you didn't.
	NotHandled,
	/// The message was handled, but the window procedure may return the default
	/// value:
	///
	/// * `0` for non-dialog windows;
	/// * `TRUE` for dialog windows.
	///
	/// This is the most common type of return.
	HandledOk,
	/// The message was handled, and the specific value must be returned by the
	/// window procedure.
	HandledWithRet(isize),
}
