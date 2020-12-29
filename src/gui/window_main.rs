use std::cell::UnsafeCell;
use std::error::Error;
use std::sync::Arc;

use crate::co;
use crate::enums::{IdIdcStr, IdMenu};
use crate::funcs as f;
use crate::gui::control_util::multiply_dpi;
use crate::gui::events::Events;
use crate::gui::globals::{create_ui_font, delete_ui_font};
use crate::gui::main_loop::run_loop;
use crate::gui::parent::Parent;
use crate::gui::window_base::WindowBase;
use crate::handles::{HACCEL, HBRUSH, HCURSOR, HICON, HINSTANCE, HMENU, HWND};
use crate::priv_funcs::str_dyn_error;
use crate::structs::{POINT, RECT, SIZE, WNDCLASSEX};
use crate::WString;

struct Obj {
	base: WindowBase,
	opts: WindowMainOpts,
	hchild_prev_focus: Option<HWND>, // WM_ACTIVATE woes
}

//------------------------------------------------------------------------------

/// Main application window.
#[derive(Clone)]
pub struct WindowMain {
	obj: Arc<UnsafeCell<Obj>>,
}

unsafe impl Send for WindowMain {}
unsafe impl Sync for WindowMain {}

impl Parent for WindowMain {
	fn on(&self) -> Events {
		let self2 = unsafe { &*self.obj.get() };
		self2.base.on()
	}
}

impl WindowMain {
	/// Creates a new `WindowMain` object.
	pub fn new(opts: WindowMainOpts) -> WindowMain {
		let wnd = Self {
			obj: Arc::new(UnsafeCell::new(
				Obj {
					base: WindowBase::new(),
					opts,
					hchild_prev_focus: None,
				},
			)),
		};
		wnd.default_message_handlers();
		wnd
	}

	/// Returns the underlying handle for this window.
	pub fn hwnd(&self) -> HWND {
		let self2 = unsafe { &*self.obj.get() };
		self2.base.hwnd()
	}
/*
	/// Exposes the events that can be handled with a closure.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Closures must be attached to
	/// events before window creation.
	pub fn on(&self) -> Events {
		self.obj.as_ref().base.on()
	}*/

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

		let self2 = unsafe { &mut *self.obj.get() };

		let hinst = HINSTANCE::GetModuleHandle(None)
			.map_err(|e| Box::new(e))?;
		let mut wcx = WNDCLASSEX::default();
		let mut class_name_buf = WString::new();
		self2.opts.generate_wndclassex(hinst, &mut wcx, &mut class_name_buf)?;
		self2.base.register_class(&mut wcx)?;

		multiply_dpi(None, Some(&mut self2.opts.size))?;

		let screen_sz = SIZE {
			cx: f::GetSystemMetrics(co::SM::CXSCREEN),
			cy: f::GetSystemMetrics(co::SM::CYSCREEN),
		};

		let wnd_pos = POINT {
			x: screen_sz.cx / 2 - self2.opts.size.cx / 2, // center on screen
			y: screen_sz.cy / 2 - self2.opts.size.cy / 2,
		};

		let mut wnd_rc = RECT { // client area, will be adjusted to size with title bar and borders
			left: wnd_pos.x,
			top: wnd_pos.y,
			right: wnd_pos.x + self2.opts.size.cx,
			bottom: wnd_pos.y + self2.opts.size.cy,
		};
		f::AdjustWindowRectEx(&mut wnd_rc, self2.opts.style,
			self2.opts.menu.is_some(), self2.opts.ex_style)?;

		let our_hwnd = self2.base.create_window(
			hinst,
			None,
			&class_name_buf.to_string(),
			Some(&self2.opts.title),
			IdMenu::None,
			POINT { x: wnd_rc.left, y: wnd_rc.top },
			SIZE { cx: wnd_rc.right - wnd_rc.left, cy: wnd_rc.bottom - wnd_rc.top },
			self2.opts.ex_style,
			self2.opts.style,
		)?;

		our_hwnd.ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));
		our_hwnd.UpdateWindow()
			.map_err(|_| str_dyn_error("UpdateWindow failed."))?;

		let res = run_loop(our_hwnd, self2.opts.accel_table)?; // blocks until window is closed
		delete_ui_font();
		Ok(res)
	}

	/// Adds the default event processing.
	fn default_message_handlers(&self) {
		self.on().wm_activate({
			let cloned = self.clone();
			move |p| {
				let self2 = unsafe { &mut *cloned.obj.get() };
				if !p.is_minimized {
					if let Some(hwnd_cur_focus) = HWND::GetFocus() {
						if self2.base.hwnd().IsChild(hwnd_cur_focus) {
							self2.hchild_prev_focus = Some(hwnd_cur_focus); // save previously focused control
						}
					} else if let Some(hwnd_prev_focus) = self2.hchild_prev_focus {
						hwnd_prev_focus.SetFocus(); // put focus back
					}
				}
			}
		});

		self.on().wm_set_focus({
			let our_hwnd = self.hwnd();
			move |_| {
				let hwnd_cur_focus = HWND::GetFocus()
					.unwrap_or(unsafe { HWND::null_handle() });
				if our_hwnd == hwnd_cur_focus {
					// If window receives focus, delegate to first child.
					if let Ok(hchild) = our_hwnd.GetWindow(co::GW::CHILD) {
						if let Some(hchild) = hchild {
							hchild.SetFocus();
						}
					}
				}
			}
		});

		self.on().wm_nc_destroy(|| {
			f::PostQuitMessage(0);
		});
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
	/// Defaults to `co::WS::CAPTION | co::WS::SYSMENU | co::WS::CLIPCHILDREN | co::WS::BORDER | co::WS::VISIBLE`.
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
			style: co::WS::CAPTION | co::WS::SYSMENU | co::WS::CLIPCHILDREN | co::WS::BORDER | co::WS::VISIBLE,
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
		class_name_buf: &'a mut WString) -> Result<(), co::ERROR>
	{
		wcx.hInstance = hinst;
		wcx.style = self.class_style;
		wcx.hIcon = self.class_icon.unwrap_or(unsafe { HICON::null_handle() });
		wcx.hIconSm = wcx.hIcon;

		wcx.hbrBackground = self.class_bg_brush.as_opt()
			.unwrap_or_else(|| HBRUSH::from_sys_color(co::COLOR::BTNFACE));

		wcx.hCursor = match self.class_cursor.as_opt() {
			Some(h) => h,
			None => HINSTANCE::oem().LoadCursor(IdIdcStr::Idc(co::IDC::ARROW))?,
		};

		if wcx.lpszClassName().is_empty() {
			*class_name_buf = WindowBase::generate_wcx_class_name_hash(&wcx);
			wcx.set_lpszClassName(class_name_buf);
		}

		Ok(())
	}
}