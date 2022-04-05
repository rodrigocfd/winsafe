use std::sync::Arc;

use crate::co;
use crate::gui::base::Base;
use crate::gui::privs::multiply_dpi;
use crate::gui::raw_base::RawBase;
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::kernel::decl::{ErrResult, HINSTANCE, WString};
use crate::prelude::{
	GdiHbrush, GuiEventsView, Handle, KernelHinstance, UserHaccel, UserHwnd,
};
use crate::user::decl::{
	AdjustWindowRectEx, GetSystemMetrics, HACCEL, HBRUSH, HCURSOR, HICON, HMENU,
	HWND, IdMenu, POINT, PostQuitMessage, RECT, SIZE, WNDCLASSEX,
};

/// A WindowMain with a raw window.
#[derive(Clone)]
pub(in crate::gui) struct RawMain(pub(in crate::gui) Arc<Obj>);

pub(in crate::gui) struct Obj { // actual fields of RawMain
	pub(in crate::gui) raw_base: RawBase,
	opts: WindowMainOpts,
	hchild_prev_focus: VeryUnsafeCell<HWND>, // WM_ACTIVATE woes
}

impl RawMain {
	pub(in crate::gui) fn new(
		parent_base: Option<&Base>,
		opts: WindowMainOpts) -> Self
	{
		let new_self = Self(Arc::new(
			Obj {
				raw_base: RawBase::new(parent_base),
				opts,
				hchild_prev_focus: VeryUnsafeCell::new(HWND::NULL),
			},
		));
		new_self.default_message_handlers();
		new_self
	}

	pub(in crate::gui) fn run_main(&self,
		cmd_show: Option<co::SW>) -> ErrResult<i32>
	{
		let opts = &self.0.opts;

		let mut wcx = WNDCLASSEX::default();
		let mut class_name_buf = WString::default();
		RawBase::fill_wndclassex(
			HINSTANCE::GetModuleHandle(None)?,
			opts.class_style, opts.class_icon, opts.class_icon,
			opts.class_bg_brush, opts.class_cursor, &mut wcx, &mut class_name_buf)?;
		let atom = self.0.raw_base.register_class(&mut wcx)?;

		let mut wnd_sz = opts.size;
		multiply_dpi(None, Some(&mut wnd_sz))?;

		let screen_sz = SIZE::new(
			GetSystemMetrics(co::SM::CXSCREEN),
			GetSystemMetrics(co::SM::CYSCREEN),
		);

		let wnd_pos = POINT::new(
			screen_sz.cx / 2 - wnd_sz.cx / 2, // center on screen
			screen_sz.cy / 2 - wnd_sz.cy / 2,
		);

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

		self.0.raw_base.create_window(
			atom,
			Some(&opts.title),
			if opts.menu.is_null() { IdMenu::None } else { IdMenu::Menu(opts.menu) },
			POINT::new(wnd_rc.left, wnd_rc.top), wnd_sz,
			opts.ex_style, opts.style,
		)?;

		self.0.raw_base.base.hwnd().ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));
		self.0.raw_base.base.hwnd().UpdateWindow()?;

		let loop_ret = Base::run_main_loop(opts.accel_table.as_opt()); // blocks until window is closed

		if let Some(haccel) = opts.accel_table.as_opt() {
			haccel.DestroyAcceleratorTable();
		}

		loop_ret
	}

	fn default_message_handlers(&self) {
		self.0.raw_base.base.on().wm_activate({
			let self2 = self.clone();
			move |p| {
				if !p.is_minimized {
					if p.event == co::WA::INACTIVE {
						if let Some(hwnd_cur_focus) = HWND::GetFocus() {
							if self2.0.raw_base.base.hwnd().IsChild(hwnd_cur_focus) {
								*self2.0.hchild_prev_focus.as_mut() = hwnd_cur_focus; // save previously focused control
							}
						}
					} else if !self2.0.hchild_prev_focus.is_null() {
						self2.0.hchild_prev_focus.SetFocus(); // put focus back
					}
				}
				Ok(())
			}
		});

		self.0.raw_base.base.on().wm_set_focus({
			let self2 = self.clone();
			move |_| self2.0.raw_base.delegate_focus_to_first_child()
		});

		self.0.raw_base.base.on().wm_nc_destroy(|| {
			PostQuitMessage(0);
			Ok(())
		});
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`WindowMain`](crate::gui::WindowMain) programmatically
/// with [`WindowMain::new`](crate::gui::WindowMain::new).
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
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
	/// This menu is **not** shared: the window will own it, and destroy it when
	/// the window is destroyed.
	///
	/// Defaults to none.
	pub menu: HMENU,
	/// Main accelerator table of the window to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// This accelerator table is **not** shared: the window will own it, and
	/// destroy it when the window is destroyed.
	///
	/// Defaults to none.
	pub accel_table: HACCEL,
}

impl Default for WindowMainOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: HICON::NULL,
			class_cursor: HCURSOR::NULL,
			class_bg_brush: HBRUSH::from_sys_color(co::COLOR::BTNFACE),
			title: "".to_owned(),
			size: SIZE { cx: 600, cy: 500 },
			style: co::WS::CAPTION | co::WS::SYSMENU | co::WS::CLIPCHILDREN | co::WS::BORDER | co::WS::VISIBLE,
			ex_style: co::WS_EX::LEFT,
			menu: HMENU::NULL,
			accel_table: HACCEL::NULL,
		}
	}
}
