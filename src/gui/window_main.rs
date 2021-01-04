use std::cell::UnsafeCell;
use std::error::Error;
use std::sync::Arc;

use crate::co;
use crate::enums::{IdIdcStr, IdMenu};
use crate::funcs as f;
use crate::gui::events::MsgEvents;
use crate::gui::globals::{create_ui_font, delete_ui_font, multiply_dpi};
use crate::gui::main_loop::run_loop;
use crate::gui::parent::Parent;
use crate::gui::window_base::WindowBase;
use crate::handles::{HACCEL, HBRUSH, HCURSOR, HICON, HINSTANCE, HMENU, HWND};
use crate::priv_funcs::str_dyn_error;
use crate::structs::{POINT, RECT, SIZE, WNDCLASSEX};
use crate::WString;

/// Main application window.
#[derive(Clone)]
pub struct WindowMain {
	obj: Arc<UnsafeCell<Obj>>,
}

struct Obj { // actual fields of WindowMain
	base: WindowBase,
	opts: WindowMainOpts,
	hchild_prev_focus: Option<HWND>, // WM_ACTIVATE woes
}

unsafe impl Send for WindowMain {}
unsafe impl Sync for WindowMain {}

cref_mref!(WindowMain);

impl Parent for WindowMain {
	fn hwnd_ref(&self) -> &HWND {
		self.cref().base.hwnd()
	}

	fn events_ref(&self) -> &MsgEvents {
		self.cref().base.on()
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
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the window is created.
	pub fn hwnd(&self) -> HWND {
		*self.cref().base.hwnd()
	}

	/// Exposes the window events.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Events must be set before window
	/// creation.
	///
	/// # Examples
	///
	/// Prints some info right after the window is created:
	///
	/// ```rust,ignore
	/// use winsafe::gui::WindowMain;
	///
	/// let wnd: WindowMain; // initialize it somewhere...
	///
	/// wnd.on().wm_create({
	///   let wnd = wnd.clone(); // pass into the closure
	///   move |parms| {
	///     println!("HWND: {}, client area: {}x{}",
	///       wnd.hwnd(),
	///       parms.createstruct.cx, parms.createstruct.cy);
	///     0
	///   }
	/// });
	/// ```
	pub fn on(&self) -> &MsgEvents {
		self.cref().base.on()
	}

	/// Creates the window by calling
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx), then runs the main
	/// application loop. This method will block until the window is closed.
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	pub fn run_as_main(&self,
		cmd_show: Option<co::SW>) -> Result<i32, Box<dyn Error>>
	{
		if f::IsWindowsVistaOrGreater()
			.map_err(|e| Box::new(e))?
		{
			f::SetProcessDPIAware()
				.map_err(|_| str_dyn_error("SetProcessDPIAware failed."))?;
		}

		f::InitCommonControls();
		create_ui_font()?;

		let opts = &mut self.mref().opts;
		let hinst = HINSTANCE::GetModuleHandle(None)
			.map_err(|e| Box::new(e))?;

		let mut wcx = WNDCLASSEX::default();
		let mut class_name_buf = WString::new();
		opts.generate_wndclassex(hinst, &mut wcx, &mut class_name_buf)?;
		self.cref().base.register_class(&mut wcx)?;

		multiply_dpi(None, Some(&mut opts.size))?;

		let screen_sz = SIZE {
			cx: f::GetSystemMetrics(co::SM::CXSCREEN),
			cy: f::GetSystemMetrics(co::SM::CYSCREEN),
		};

		let wnd_pos = POINT {
			x: screen_sz.cx / 2 - opts.size.cx / 2, // center on screen
			y: screen_sz.cy / 2 - opts.size.cy / 2,
		};

		let mut wnd_rc = RECT { // client area, will be adjusted to size with title bar and borders
			left: wnd_pos.x,
			top: wnd_pos.y,
			right: wnd_pos.x + opts.size.cx,
			bottom: wnd_pos.y + opts.size.cy,
		};
		f::AdjustWindowRectEx(&mut wnd_rc, opts.style,
			!opts.menu.is_null(), opts.ex_style)?;

		let our_hwnd = self.cref().base.create_window( // may panic
			hinst,
			None,
			&class_name_buf.to_string(),
			Some(&opts.title),
			IdMenu::None,
			POINT { x: wnd_rc.left, y: wnd_rc.top },
			SIZE { cx: wnd_rc.right - wnd_rc.left, cy: wnd_rc.bottom - wnd_rc.top },
			opts.ex_style,
			opts.style,
		)?;

		our_hwnd.ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));
		our_hwnd.UpdateWindow()
			.map_err(|_| str_dyn_error("UpdateWindow failed."))?;

		let res = run_loop(our_hwnd, opts.accel_table.as_opt())?; // blocks until window is closed
		delete_ui_font(); // cleanup
		Ok(res)
	}

	/// Adds the default event processing.
	fn default_message_handlers(&self) {
		self.on().wm_activate({
			let self2 = self.clone();
			move |p| {
				if !p.is_minimized {
					if p.event == co::WA::INACTIVE {
						if let Some(hwnd_cur_focus) = HWND::GetFocus() {
							if self2.cref().base.hwnd().IsChild(hwnd_cur_focus) {
								self2.mref().hchild_prev_focus = Some(hwnd_cur_focus); // save previously focused control
							}
						}
					} else if let Some(hwnd_prev_focus) = self2.cref().hchild_prev_focus {
						hwnd_prev_focus.SetFocus(); // put focus back
					}
				}
			}
		});

		self.on().wm_set_focus({
			let self2 = self.clone();
			move |_| {
				if let Some(hwnd_cur_focus) = HWND::GetFocus() {
					if self2.hwnd() == hwnd_cur_focus {
						// If window receives focus, delegate to first child.
						// https://stackoverflow.com/a/2835220/6923555
						if let Some(hchild) = self2.hwnd().GetWindow(co::GW::CHILD).unwrap() {
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
	/// Defaults to none.
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
	/// Defaults to none.
	pub menu: HMENU,
	/// Main accelerator table of the window to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to none.
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
			style: co::WS::CAPTION | co::WS::SYSMENU | co::WS::CLIPCHILDREN | co::WS::BORDER | co::WS::VISIBLE,
			ex_style: co::WS_EX::LEFT,
			menu: unsafe { HMENU::null_handle() },
			accel_table: unsafe { HACCEL::null_handle() },
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
		wcx.hIcon = self.class_icon;
		wcx.hIconSm = self.class_icon;

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
