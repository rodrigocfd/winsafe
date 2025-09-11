use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::msg::*;
use crate::prelude::*;

/// An error that occurred within a closure of a window message handling.
/// Usually these errors are thrown by the user closures.
///
/// This error types wraps the actual user error along with the parameters of
/// the message where the error happened.
pub struct MsgError {
	src_msg: WndMsg,
	source: Box<dyn std::error::Error + Send + Sync>,
}

impl std::error::Error for MsgError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		Some(self.source.as_ref())
	}
}

impl std::fmt::Display for MsgError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "WM {} - {}", self.src_msg.msg_id, self.source.to_string())
	}
}
impl std::fmt::Debug for MsgError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self, f)
	}
}

impl MsgError {
	/// Constructs a new `MsgError` by wrapping the given error.
	#[must_use]
	pub const fn new(
		src_msg: WndMsg,
		source: Box<dyn std::error::Error + Send + Sync>,
	) -> MsgError {
		Self { src_msg, source }
	}

	/// The source message information where the error originated from.
	#[must_use]
	pub const fn src_msg(&self) -> WndMsg {
		self.src_msg
	}
}

/// Global horizontal and vertical system DPI factor.
static mut DPI: (i32, i32) = (0, 0);

fn cache_dpi() {
	if unsafe { DPI } == (0, 0) {
		let hdc_screen = HWND::NULL.GetDC().expect(DONTFAIL);
		let x = hdc_screen.GetDeviceCaps(co::GDC::LOGPIXELSX);
		let y = hdc_screen.GetDeviceCaps(co::GDC::LOGPIXELSY);
		unsafe {
			DPI = (x, y);
		}
	}
}

/// Returns the value adjusted according to the current horizontal system DPI,
/// retrieved with [`HDC::GetDeviceCaps`](crate::HDC::GetDeviceCaps).
pub fn dpi_x(x_val: i32) -> i32 {
	cache_dpi();
	MulDiv(x_val.into(), unsafe { DPI }.0, 96)
}

/// Returns the value adjusted according to the current vertical system DPI,
/// retrieved with [`HDC::GetDeviceCaps`](crate::HDC::GetDeviceCaps).
pub fn dpi_y(x_val: i32) -> i32 {
	cache_dpi();
	MulDiv(x_val.into(), unsafe { DPI }.0, 96)
}

/// Returns the values adjusted according to the current horizontal and vertical
/// system DPI, retrieved with
/// [`HDC::GetDeviceCaps`](crate::HDC::GetDeviceCaps).
pub fn dpi(x_val: i32, y_val: i32) -> (i32, i32) {
	(dpi_x(x_val), dpi_y(y_val))
}

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
