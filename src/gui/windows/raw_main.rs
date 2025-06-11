use std::cell::UnsafeCell;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::prelude::*;

struct RawMainObj {
	raw_base: RawBase,
	opts: WindowMainOpts,
	hchild_prev_focus: UnsafeCell<HWND>,
	_pin: PhantomPinned,
}

/// An ordinary main window.
#[derive(Clone)]
pub(in crate::gui) struct RawMain(Pin<Arc<RawMainObj>>);

impl RawMain {
	#[must_use]
	pub(in crate::gui) fn new(opts: WindowMainOpts) -> Self {
		let new_self = Self(Arc::pin(RawMainObj {
			raw_base: RawBase::new(),
			opts,
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
		)?;

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
		rc_wnd = AdjustWindowRectEx(rc_wnd, opts.style, opts.menu != HMENU::NULL, opts.ex_style)?;

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

		self.0
			.raw_base
			.base()
			.hwnd()
			.ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));
		self.0.raw_base.base().hwnd().UpdateWindow()?;
		BaseWnd::run_main_loop(opts.accel_table.as_deref(), opts.process_dlg_msgs) // blocks until window is closed
	}
}
