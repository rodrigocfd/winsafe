use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::gui::*;
use crate::prelude::*;

/// Options to create a [`WindowControl`](crate::gui::WindowControl)
/// programmatically with [`WindowControl::new`](crate::gui::WindowControl::new).
pub struct WindowControlOpts {
	/// Window class name to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to an auto-generated string.
	pub class_name: String,
	/// Window class styles to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::CS::DBLCLKS`.
	pub class_style: co::CS,
	/// Window main icon to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Icon::None`.
	pub class_icon: Icon,
	/// Window cursor to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Cursor::Idc(co::IDC::ARROW)`.
	pub class_cursor: Cursor,
	/// Window background brush to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Brush::Color(co::COLOR::WINDOW)`.
	pub class_bg_brush: Brush,

	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Width and height of window to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(100, 80)`.
	pub size: (i32, i32),
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::TABSTOP | WS::GROUP | WS::VISIBLE | WS::CLIPCHILDREN | WS::CLIPSIBLINGS`.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	///
	/// Suggestion:
	/// * `WS_EX::CLIENTEDGE` to have a border.
	pub ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
	/// Horizontal and vertical behavior of the control when the parent window
	/// is resized.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),
}

impl Default for WindowControlOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: Icon::None,
			class_cursor: Cursor::Idc(co::IDC::ARROW),
			class_bg_brush: Brush::Color(co::COLOR::WINDOW),
			position: dpi(0, 0),
			size: dpi(100, 80),
			style: co::WS::CHILD
				| co::WS::TABSTOP
				| co::WS::GROUP
				| co::WS::VISIBLE
				| co::WS::CLIPCHILDREN
				| co::WS::CLIPSIBLINGS,
			ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
		}
	}
}

/// Options to create a [`WindowMain`](crate::gui::WindowMain) programmatically
/// with [`WindowMain::new`](crate::gui::WindowMain::new).
pub struct WindowMainOpts {
	/// Window class name to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to an auto-generated string.
	pub class_name: String,
	/// Window class styles to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::CS::DBLCLKS`.
	pub class_style: co::CS,
	/// Window main icon to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Icon::None`.
	pub class_icon: Icon,
	/// Window cursor to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Cursor::Idc(co::IDC::ARROW)`.
	pub class_cursor: Cursor,
	/// Window background brush to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Brush::Color(co::COLOR::BTNFACE)`.
	pub class_bg_brush: Brush,

	/// Window title to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub title: String,
	/// Width and height of window client area, in pixels, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	/// Does not include title bar or borders.
	///
	/// Defaults to `gui::dpi(600, 400)`.
	pub size: (i32, i32),
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CAPTION | WS::SYSMENU | WS::CLIPCHILDREN | WS::BORDER | WS::VISIBLE`.
	///
	/// Suggestions:
	/// * `WS::SIZEBOX` to make the window resizable;
	/// * `WS::MINIMIZEBOX` to have a minimize button;
	/// * `WS::MAXIMIZEBOX` to have a maximize button.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	pub ex_style: co::WS_EX,
	/// Main menu of the window to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// This menu is **not** shared: the window will own it, and destroy it when
	/// the window is destroyed.
	///
	/// Defaults to none.
	pub menu: HMENU,
	/// Main accelerator table of the window to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	/// Use
	/// [`HACCEL::CreateAcceleratorTable`](crate::prelude::user_Haccel::CreateAcceleratorTable)
	/// to create one.
	///
	/// Defaults to `None`.
	pub accel_table: Option<DestroyAcceleratorTableGuard>,
	/// In most applications, the window loop calls
	/// [`IsDialogMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isdialogmessagew)
	/// so child control messages will properly work. However, this has the side
	/// effect of inhibiting
	/// [`WM_CHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-char)
	/// messages from being sent to the window procedure. So, applications which
	/// do not have child controls and deal directly with character processing –
	/// like text editors – will never be able to receive `WM_CHAR`.
	///
	/// This flag, when `true`, will enable the normal `IsDialogMessage` call in
	/// the window loop. When `false`, the call will be suppressed.
	///
	/// Defaults to `true`.
	pub process_dlg_msgs: bool,
}

impl Default for WindowMainOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: Icon::None,
			class_cursor: Cursor::Idc(co::IDC::ARROW),
			class_bg_brush: Brush::Color(co::COLOR::BTNFACE),
			title: "".to_owned(),
			size: dpi(600, 400),
			style: co::WS::CAPTION
				| co::WS::SYSMENU
				| co::WS::CLIPCHILDREN
				| co::WS::BORDER
				| co::WS::VISIBLE,
			ex_style: co::WS_EX::LEFT,
			menu: HMENU::NULL,
			accel_table: None,
			process_dlg_msgs: true,
		}
	}
}

/// Options to create a [`WindowModal`](crate::gui::WindowModal)
/// programmatically with [`WindowModal::new`](crate::gui::WindowModal::new).
pub struct WindowModalOpts {
	/// Window class name to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to an auto-generated string.
	pub class_name: String,
	/// Window class styles to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::CS::DBLCLKS`.
	pub class_style: co::CS,
	/// Window main icon to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Icon::None`.
	pub class_icon: Icon,
	/// Window cursor to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Cursor::Idc(co::IDC::ARROW)`.
	pub class_cursor: Cursor,
	/// Window background brush to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Brush::Color(co::COLOR::BTNFACE)`.
	pub class_bg_brush: Brush,

	/// Window title to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub title: String,
	/// Width and height of window client area, in pixels, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	/// Does not include title bar or borders.
	///
	/// Defaults to `gui::dpi(500, 400)`.
	pub size: (i32, i32),
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CAPTION | WS::SYSMENU | WS::CLIPCHILDREN | WS::BORDER | WS::VISIBLE`.
	///
	/// Suggestions:
	/// * `WS::SIZEBOX` to make the window resizable;
	/// * `WS::MAXIMIZEBOX` to have a maximize button.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::DLGMODALFRAME`.
	pub ex_style: co::WS_EX,
	/// In most applications, the window loop calls
	/// [`IsDialogMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isdialogmessagew)
	/// so child control messages will properly work. However, this has the side
	/// effect of inhibiting
	/// [`WM_CHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-char)
	/// messages from being sent to the window procedure. So, applications which
	/// do not have child controls and deal directly with character processing –
	/// like text editors – will never be able to receive `WM_CHAR`.
	///
	/// This flag, when `true`, will enable the normal `IsDialogMessage` call in
	/// the window loop. When `false`, the call will be suppressed.
	///
	/// Defaults to `true`.
	pub process_dlg_msgs: bool,
}

impl Default for WindowModalOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: Icon::None,
			class_cursor: Cursor::Idc(co::IDC::ARROW),
			class_bg_brush: Brush::Color(co::COLOR::BTNFACE),
			title: "".to_owned(),
			size: dpi(500, 400),
			style: co::WS::CAPTION
				| co::WS::SYSMENU
				| co::WS::CLIPCHILDREN
				| co::WS::BORDER
				| co::WS::VISIBLE,
			ex_style: co::WS_EX::LEFT | co::WS_EX::DLGMODALFRAME,
			process_dlg_msgs: true,
		}
	}
}

/// Options to create a [`WindowModeless`](crate::gui::WindowModeless)
/// programmatically with
/// [`WindowModeless::new`](crate::gui::WindowModeless::new).
pub struct WindowModelessOpts {
	/// Window class name to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to an auto-generated string.
	pub class_name: String,
	/// Window class styles to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::CS::DBLCLKS`.
	pub class_style: co::CS,
	/// Window main icon to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Icon::None`.
	pub class_icon: Icon,
	/// Window cursor to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Cursor::Idc(co::IDC::ARROW)`.
	pub class_cursor: Cursor,
	/// Window background brush to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Brush::Color(co::COLOR::BTNFACE)`.
	pub class_bg_brush: Brush,

	/// Window title to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub title: String,
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Width and height of window client area, in pixels, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	/// Does not include title bar or borders.
	///
	/// Defaults to `gui::dpi(220, 150)`.
	pub size: (i32, i32),
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CAPTION | WS::SYSMENU | WS::CLIPCHILDREN | WS::BORDER | WS::VISIBLE`.
	///
	/// Suggestions:
	/// * `WS::SIZEBOX` to make the window resizable.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::TOOLWINDOW`.
	pub ex_style: co::WS_EX,
}

impl Default for WindowModelessOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: Icon::None,
			class_cursor: Cursor::Idc(co::IDC::ARROW),
			class_bg_brush: Brush::Color(co::COLOR::BTNFACE),
			title: "".to_owned(),
			position: dpi(0, 0),
			size: dpi(220, 150),
			style: co::WS::CAPTION
				| co::WS::SYSMENU
				| co::WS::CLIPCHILDREN
				| co::WS::BORDER
				| co::WS::VISIBLE,
			ex_style: co::WS_EX::LEFT | co::WS_EX::TOOLWINDOW,
		}
	}
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
