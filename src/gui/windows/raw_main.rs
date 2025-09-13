use std::cell::UnsafeCell;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::gui::{privs::*, *};
use crate::prelude::*;

struct RawMainObj {
	raw_base: RawBase,
	opts: WindowMainOptsObj,
	hchild_prev_focus: UnsafeCell<HWND>,
	_pin: PhantomPinned,
}

/// An ordinary main window.
///
/// Hierarchy: `BaseWnd` -> `RawBase` -> `RawMain`.
#[derive(Clone)]
pub(in crate::gui) struct RawMain(Pin<Arc<RawMainObj>>);

impl RawMain {
	#[must_use]
	pub(in crate::gui) fn new(opts: WindowMainOpts) -> Self {
		let new_self = Self(Arc::pin(RawMainObj {
			raw_base: RawBase::new(),
			opts: opts.into(),
			hchild_prev_focus: UnsafeCell::new(HWND::NULL),
			_pin: PhantomPinned,
		}));
		new_self.default_message_handlers();
		new_self
	}

	fn default_message_handlers(&self) {
		let self2 = self.clone();
		self.0.raw_base.base().before_on().wm_activate(move |p| {
			if !p.is_minimized {
				let hchild_prev_focus = unsafe { &mut *self2.0.hchild_prev_focus.get() };
				if p.event == co::WA::INACTIVE {
					if let Some(hwnd_cur_focus) = HWND::GetFocus() {
						if self2.0.raw_base.base().hwnd().IsChild(&hwnd_cur_focus) {
							*hchild_prev_focus = hwnd_cur_focus; // save previously focused control
						}
					}
				} else if *hchild_prev_focus != HWND::NULL {
					hchild_prev_focus.SetFocus(); // put focus back
				}
			}
			Ok(())
		});

		let self2 = self.clone();
		self.0.raw_base.base().before_on().wm_set_focus(move |_| {
			self2.0.raw_base.delegate_focus_to_first_child();
			Ok(())
		});

		self.0.raw_base.base().on().wm_nc_destroy(move || {
			PostQuitMessage(0);
			Ok(())
		});
	}

	#[must_use]
	pub(in crate::gui) fn raw_base(&self) -> &RawBase {
		&self.0.raw_base
	}

	pub(in crate::gui) fn run_main(
		&self,
		hinst: &HINSTANCE,
		cmd_show: Option<co::SW>,
	) -> AnyResult<i32> {
		let opts = &self.0.opts;
		let atom = self.0.raw_base.register_class(
			hinst,
			&opts.class_name,
			opts.class_style,
			&opts.class_icon,
			&opts.class_bg_brush,
			&opts.class_cursor,
		);

		let sz_screen =
			SIZE::with(GetSystemMetrics(co::SM::CXSCREEN), GetSystemMetrics(co::SM::CYSCREEN));

		let pt_wnd = POINT::with(
			sz_screen.cx / 2 - opts.size.0 / 2, // center on screen
			sz_screen.cy / 2 - opts.size.1 / 2,
		);

		let mut rc_wnd = RECT {
			left: pt_wnd.x, // client area, will be adjusted to size with title bar and borders
			top: pt_wnd.y,
			right: pt_wnd.x + opts.size.0 as i32,
			bottom: pt_wnd.y + opts.size.1 as i32,
		};
		rc_wnd = AdjustWindowRectEx(rc_wnd, opts.style, opts.menu != HMENU::NULL, opts.ex_style)
			.expect(DONTFAIL);

		self.0.raw_base.create_window(
			opts.ex_style,
			atom,
			Some(&opts.title),
			opts.style,
			POINT::with(rc_wnd.left, rc_wnd.top),
			SIZE::with(rc_wnd.right - rc_wnd.left, rc_wnd.bottom - rc_wnd.top),
			None,
			if opts.menu == HMENU::NULL { IdMenu::None } else { IdMenu::Menu(&opts.menu) },
			hinst,
		);

		let hwnd = self.0.raw_base.base().hwnd();
		hwnd.ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));
		hwnd.UpdateWindow().expect(DONTFAIL);
		BaseWnd::run_main_loop(opts.accel_table.as_deref(), opts.process_dlg_msgs) // blocks until window is closed
	}
}

/// Options to create a [`WindowMain`](crate::gui::WindowMain) programmatically
/// with [`WindowMain::new`](crate::gui::WindowMain::new).
pub struct WindowMainOpts<'a> {
	/// Window class name to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to an auto-generated string.
	pub class_name: &'a str,
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
	pub title: &'a str,
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
	/// [`HACCEL::CreateAcceleratorTable`](crate::HACCEL::CreateAcceleratorTable)
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

impl<'a> Default for WindowMainOpts<'a> {
	fn default() -> Self {
		Self {
			class_name: "",
			class_style: co::CS::DBLCLKS,
			class_icon: Icon::None,
			class_cursor: Cursor::Idc(co::IDC::ARROW),
			class_bg_brush: Brush::Color(co::COLOR::BTNFACE),
			title: "",
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

impl<'a> Into<WindowMainOptsObj> for WindowMainOpts<'a> {
	fn into(self) -> WindowMainOptsObj {
		WindowMainOptsObj {
			class_name: self.class_name.to_owned(),
			class_style: self.class_style,
			class_icon: self.class_icon,
			class_cursor: self.class_cursor,
			class_bg_brush: self.class_bg_brush,
			title: self.title.to_owned(),
			size: self.size,
			style: self.style,
			ex_style: self.ex_style,
			menu: self.menu,
			accel_table: self.accel_table,
			process_dlg_msgs: self.process_dlg_msgs,
		}
	}
}

/// To be stored inside the object.
struct WindowMainOptsObj {
	class_name: String,
	class_style: co::CS,
	class_icon: Icon,
	class_cursor: Cursor,
	class_bg_brush: Brush,
	title: String,
	size: (i32, i32),
	style: co::WS,
	ex_style: co::WS_EX,
	menu: HMENU,
	accel_table: Option<DestroyAcceleratorTableGuard>,
	process_dlg_msgs: bool,
}
