use std::error::Error;

use crate::co;
use crate::enums::{IdIdcStr, IdMenu};
use crate::funcs as f;
use crate::gui::control_util::multiply_dpi;
use crate::gui::events::Events;
use crate::gui::globals::{create_ui_font, delete_ui_font};
use crate::gui::main_loop::run_loop;
use crate::gui::Parent;
use crate::gui::window_base::WindowBase;
use crate::handles::{HACCEL, HBRUSH, HCURSOR, HICON, HINSTANCE, HMENU, HWND};
use crate::internal_defs::str_dyn_error;
use crate::structs::{POINT, RECT, SIZE, WNDCLASSEX};
use crate::Utf16;

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
		&mut self, cmd_show: Option<co::SW>) -> Result<i32, Box<dyn Error>>
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
		let mut wcx = WNDCLASSEX::default();
		let mut class_name_buf = Utf16::new();
		self.opts.generate_wndclassex(hinst, &mut wcx, &mut class_name_buf)?;
		self.base.register_class(&mut wcx)?;

		multiply_dpi(None, Some(&mut self.opts.size))?;

		let screen_sz = SIZE {
			cx: f::GetSystemMetrics(co::SM::CXSCREEN),
			cy: f::GetSystemMetrics(co::SM::CYSCREEN),
		};

		let wnd_pos = POINT {
			x: screen_sz.cx / 2 - self.opts.size.cx / 2, // center on screen
			y: screen_sz.cx / 2 - self.opts.size.cy / 2,
		};

		let mut wnd_rc = RECT { // client area, will be adjusted to size with title bar and borders
			left: wnd_pos.x,
			top: wnd_pos.y,
			right: wnd_pos.x + self.opts.size.cx,
			bottom: wnd_pos.y + self.opts.size.cy,
		};
		f::AdjustWindowRectEx(
			&mut wnd_rc, self.opts.style, false, self.opts.ex_style)?;

		let our_hwnd = self.base.create_window(hinst, None,
			&class_name_buf.to_string(), Some(&self.opts.title), IdMenu::None,
			POINT { x: wnd_rc.left, y: wnd_rc.top },
			SIZE { cx: wnd_rc.right - wnd_rc.left, cy: wnd_rc.bottom - wnd_rc.top },
			self.opts.ex_style, self.opts.style)?;

		our_hwnd.ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));
		our_hwnd.UpdateWindow()
			.map_err(|_| str_dyn_error("UpdateWindow failed."))?;

		let res = run_loop(our_hwnd, self.opts.accel_table)?; // blocks until window is closed
		delete_ui_font();
		Ok(res)
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
	/// Defaults to no `None`.
	pub class_icon: Option<HICON>,
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
	/// Defaults to `None`.
	pub menu: Option<HMENU>,
	/// Main accelerator table of the window to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `None`.
	pub accel_table: Option<HACCEL>,
}

impl Default for WindowMainOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: None,
			class_cursor: unsafe { HCURSOR::null_handle() },
			class_bg_brush: unsafe { HBRUSH::null_handle() },
			title: "".to_owned(),
			size: SIZE { cx: 600, cy: 500 },
			style: co::WS::CAPTION | co::WS::SYSMENU | co::WS::CLIPCHILDREN | co::WS::BORDER,
			ex_style: co::WS_EX::LEFT,
			menu: None,
			accel_table: None,
		}
	}
}

impl WindowMainOpts {
	fn generate_wndclassex<'a, 'b>( // https://stackoverflow.com/q/65481548/6923555
		&self,
		hinst: HINSTANCE,
		wcx: &mut WNDCLASSEX<'_, 'a>,
		class_name_buf: &'a mut Utf16) -> Result<(), co::ERROR>
	{
		wcx.hInstance = hinst;
		wcx.style = self.class_style;
		wcx.hIcon = self.class_icon.unwrap_or(unsafe { HICON::null_handle() });
		wcx.hIconSm = wcx.hIcon;
		wcx.hbrBackground = self.class_bg_brush;

		if wcx.hCursor.is_null() {
			wcx.hCursor = HINSTANCE::oem()
				.LoadCursor(IdIdcStr::Idc(co::IDC::ARROW))?;
		}

		if wcx.lpszClassName().is_empty() {
			*class_name_buf = Utf16::from_str(
				&WindowBase::generate_wcx_class_name_hash(&wcx),
			);
			wcx.set_lpszClassName(class_name_buf);
		}

		Ok(())
	}
}