use std::error::Error;

use crate::co;
use crate::enums::IdIdcStr;
use crate::funcs as f;
use crate::gui::events::Events;
use crate::gui::globals::{create_ui_font, delete_ui_font};
use crate::gui::Parent;
use crate::gui::window_base::WindowBase;
use crate::handles::{HACCEL, HBRUSH, HCURSOR, HICON, HINSTANCE, HMENU, HWND};
use crate::internal_defs::str_dyn_error;
use crate::structs::{SIZE, WNDCLASSEX};

/// Main application window.
#[derive(Clone)]
pub struct WindowMain {
	base: WindowBase,
	opts: WindowMainOpts,
}

impl WindowMain {
	/// Creates a new `WindowMain` object.
	pub fn new(opts: WindowMainOpts) -> WindowMain {
		Self {
			base: WindowBase::new(),
			opts: WindowMainOpts::default(),
		}
	}

	/// Returns the underlying handle for this window.
	pub fn hwnd(&self) -> HWND {
		self.base.hwnd()
	}

	/// Creates the window and runs the main application loop. This function will
	/// block until the window is closed.
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	pub fn run_as_main(
		&self, cmd_show: Option<co::SW>) -> Result<i32, Box<dyn Error>>
	{
		if f::IsWindowsVistaOrGreater()
			.map_err(|e| Box::new(e))?
		{
			f::SetProcessDPIAware()
				.map_err(|_| str_dyn_error("SetProcessDPIAware failed."))?;
		}

		f::InitCommonControls();
		create_ui_font()?;

		let hinst = HINSTANCE::GetModuleHandle(None)
			.map_err(|e| Box::new(e))?;
		let wcx = self.opts.generate_wndclassex(hinst)?;




		delete_ui_font();
		Ok(0)
	}
}

impl Parent for WindowMain {
	fn on(&self) -> Events {
		self.base.on()
	}
}

//------------------------------------------------------------------------------

/// Options for [`WindowMain::new`](crate::gui::WindowMain::new).
#[derive(Clone)]
pub struct WindowMainOpts {
	/// Window class name to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to an auto-generated string.
	pub class_name: String,
	/// Window class styles to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::CS::DBLCLKS`.
	pub class_style: co::CS,
	/// Window main icon to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to no icon.
	pub class_icon: HICON,
	/// Window cursor to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::IDC::ARROW`.
	pub class_cursor: HCURSOR,
	/// Window background brush to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::COLOR::BTNFACE`.
	pub class_bg_brush: HBRUSH,

	/// Window title to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub title: String,
	/// Size of window client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	/// Does not include title bar or borders.
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 600x500.
	pub size: SIZE,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `co::WS::CAPTION | co::WS::SYSMENU | co::WS::CLIPCHILDREN | co::WS::BORDER`.
	///
	/// Suggestions:
	/// * `co::WS::SIZEBOX` to make the window resizable;
	/// * `co::WS::MINIMIZEBOX` to have a minimize button;
	/// * `co::WS::MAXIMIZEBOX` to have a maximize button.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `co::WS_EX::LEFT`.
	pub ex_style: co::WS_EX,
	/// Main menu of the window to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// This menu is not shared, the window will own it, and destroy it when the
	/// window is destroyed.
	///
	/// Defaults to absent.
	pub menu: HMENU,
	/// Main accelerator table of the window to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to absent.
	pub accel_table: HACCEL,
}

impl Default for WindowMainOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: unsafe { HICON::null_handle() },
			class_cursor: unsafe { HCURSOR::null_handle() },
			class_bg_brush: unsafe { HBRUSH::null_handle() },
			title: "".to_owned(),
			size: SIZE { cx: 600, cy: 500 },
			style: co::WS::CAPTION | co::WS::SYSMENU | co::WS::CLIPCHILDREN | co::WS::BORDER,
			ex_style: co::WS_EX::LEFT,
			menu: unsafe { HMENU::null_handle() },
			accel_table: unsafe { HACCEL::null_handle() },
		}
	}
}

impl WindowMainOpts {
	fn generate_wndclassex(
		&self, hinst: HINSTANCE) -> Result<WNDCLASSEX, co::ERROR>
	{
		let mut wcx = WNDCLASSEX::default();
		wcx.hInstance = hinst;
		wcx.style = self.class_style;
		wcx.hIcon = self.class_icon;
		wcx.hIconSm = self.class_icon;
		wcx.hbrBackground = self.class_bg_brush;

		if wcx.hCursor.is_null() {
			wcx.hCursor = HINSTANCE::oem()
				.LoadCursor(IdIdcStr::Idc(co::IDC::ARROW))?;
		}



		Ok(wcx)
	}
}