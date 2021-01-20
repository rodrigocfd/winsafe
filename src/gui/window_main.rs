use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::{IdIdcStr, IdMenu};
use crate::funcs::{AdjustWindowRectEx, GetSystemMetrics, PostQuitMessage};
use crate::gui::events::MsgEvents;
use crate::gui::immut::Immut;
use crate::gui::main_loop::run_loop;
use crate::gui::privs::multiply_dpi;
use crate::gui::traits::Parent;
use crate::gui::window_base::WindowBase;
use crate::handles::{HACCEL, HBRUSH, HCURSOR, HICON, HINSTANCE, HMENU, HWND};
use crate::structs::{POINT, RECT, SIZE, WNDCLASSEX};
use crate::WString;

#[derive(Clone)]
pub struct WindowMain(Arc<Immut<Obj>>);

struct Obj { // actual fields of WindowMain
	base: WindowBase,
	opts: CustomMainOpts,
	hchild_prev_focus: Option<HWND>, // WM_ACTIVATE woes
}

impl Parent for WindowMain {
	fn hwnd_ref(&self) -> &HWND {
		self.0.base.hwnd_ref()
	}

	fn user_events_ref(&self) -> &MsgEvents {
		self.0.base.user_events_ref()
	}

	fn privileged_events_ref(&self) -> &MsgEvents {
		self.0.base.privileged_events_ref()
	}
}

impl WindowMain {
	pub fn new(opts: CustomMainOpts) -> WindowMain {
		let wnd = Self(
			Arc::new(Immut::new(
				Obj {
					base: WindowBase::new(None), // no parent
					opts,
					hchild_prev_focus: None,
				},
			)),
		);
		wnd.default_message_handlers();
		wnd
	}

	pub fn run_main(&self, cmd_show: Option<co::SW>) -> WinResult<i32> {
		let opts = &self.0.opts;

		let mut wcx = WNDCLASSEX::default();
		let mut class_name_buf = WString::new();
		opts.generate_wndclassex(
			self.0.base.parent_hinstance()?, &mut wcx, &mut class_name_buf)?;
		self.0.base.register_class(&mut wcx)?;

		let mut wnd_sz = opts.size;
		multiply_dpi(None, Some(&mut wnd_sz))?;

		let screen_sz = SIZE {
			cx: GetSystemMetrics(co::SM::CXSCREEN),
			cy: GetSystemMetrics(co::SM::CYSCREEN),
		};

		let wnd_pos = POINT {
			x: screen_sz.cx / 2 - wnd_sz.cx / 2, // center on screen
			y: screen_sz.cy / 2 - wnd_sz.cy / 2,
		};

		let mut wnd_rc = RECT { // client area, will be adjusted to size with title bar and borders
			left: wnd_pos.x,
			top: wnd_pos.y,
			right: wnd_pos.x + wnd_sz.cx,
			bottom: wnd_pos.y + wnd_sz.cy,
		};
		AdjustWindowRectEx(&mut wnd_rc, opts.style,
			!opts.menu.is_null(), opts.ex_style)?;
		wnd_sz.cx = wnd_rc.right - wnd_rc.left;
		wnd_sz.cy = wnd_rc.bottom - wnd_rc.top;

		self.0.base.create_window( // may panic
			&class_name_buf.to_string(),
			Some(&opts.title),
			IdMenu::None,
			POINT::new(wnd_rc.left, wnd_rc.top), wnd_sz,
			opts.ex_style, opts.style,
		)?;

		self.hwnd_ref().ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));
		self.hwnd_ref().UpdateWindow()?;

		run_loop(self.hwnd_ref(), opts.accel_table.as_opt()) // blocks until window is closed
	}

	fn default_message_handlers(&self) {
		self.user_events_ref().wm_activate({
			let self2 = self.clone();
			move |p| {
				if !p.is_minimized {
					if p.event == co::WA::INACTIVE {
						if let Some(hwnd_cur_focus) = HWND::GetFocus() {
							if self2.0.base.hwnd_ref().IsChild(hwnd_cur_focus) {
								self2.0.as_mut().hchild_prev_focus = Some(hwnd_cur_focus); // save previously focused control
							}
						}
					} else if let Some(hwnd_prev_focus) = self2.0.hchild_prev_focus {
						hwnd_prev_focus.SetFocus(); // put focus back
					}
				}
			}
		});

		self.user_events_ref().wm_set_focus({
			let self2 = self.clone();
			move |_| {
				if let Some(hwnd_cur_focus) = HWND::GetFocus() {
					if *self2.hwnd_ref() == hwnd_cur_focus {
						self2.0.base.focus_first_child(); // if window receives focus, delegate to first child
					}
				}
			}
		});

		self.user_events_ref().wm_nc_destroy(|| {
			PostQuitMessage(0);
		});
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`CustomMain`](crate::gui::CustomMain) programatically
/// with [`CustomMain::new`](crate::gui::CustomMain::new).
pub struct CustomMainOpts {
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
	/// Defaults to 600 x 500.
	pub size: SIZE,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CAPTION | WS::SYSMENU | WS::CLIPCHILDREN | WS::BORDER | WS::VISIBLE`.
	///
	/// Suggestions:
	/// * `WS::SIZEBOX` to make the window resizable;
	/// * `WS::MINIMIZEBOX` to have a minimize button;
	/// * `WS::MAXIMIZEBOX` to have a maximize button.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
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

impl Default for CustomMainOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: unsafe { HICON::null_handle() },
			class_cursor: unsafe { HCURSOR::null_handle() },
			class_bg_brush: HBRUSH::from_sys_color(co::COLOR::BTNFACE),
			title: "".to_owned(),
			size: SIZE { cx: 600, cy: 500 },
			style: co::WS::CAPTION | co::WS::SYSMENU | co::WS::CLIPCHILDREN | co::WS::BORDER | co::WS::VISIBLE,
			ex_style: co::WS_EX::LEFT,
			menu: unsafe { HMENU::null_handle() },
			accel_table: unsafe { HACCEL::null_handle() },
		}
	}
}

impl CustomMainOpts {
	fn generate_wndclassex<'a, 'b>( // https://stackoverflow.com/q/65481548/6923555
		&self,
		hinst: HINSTANCE,
		wcx: &mut WNDCLASSEX<'_, 'a>,
		class_name_buf: &'a mut WString) -> WinResult<()>
	{
		wcx.hInstance = hinst;
		wcx.style = self.class_style;
		wcx.hIcon = self.class_icon;
		wcx.hIconSm = self.class_icon;
		wcx.hbrBackground = self.class_bg_brush;

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
