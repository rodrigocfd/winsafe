use std::cell::UnsafeCell;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::prelude::*;

struct RawModalObj {
	raw_base: RawBase,
	opts: WindowModalOpts,
	hchild_prev_focus_parent: UnsafeCell<HWND>,
	_pin: PhantomPinned,
}

/// An ordinary modal window.
#[derive(Clone)]
pub(in crate::gui) struct RawModal(Pin<Arc<RawModalObj>>);

impl RawModal {
	#[must_use]
	pub(in crate::gui) fn new(opts: WindowModalOpts) -> Self {
		let new_self = Self(Arc::pin(RawModalObj {
			raw_base: RawBase::new(),
			opts,
			hchild_prev_focus_parent: UnsafeCell::new(HWND::NULL),
			_pin: PhantomPinned,
		}));
		new_self.default_message_handlers();
		new_self
	}

	fn default_message_handlers(&self) {
		let self2 = self.clone();
		self.0.raw_base.base().before_on().wm_set_focus(move |_| {
			self2.0.raw_base.delegate_focus_to_first_child();
			Ok(())
		});

		let self2 = self.clone();
		self.0.raw_base.base().on().wm_close(move || {
			if let Ok(hparent) = self2.0.raw_base.base().hwnd().GetWindow(co::GW::OWNER) {
				hparent.EnableWindow(true); // re-enable parent
				self2.0.raw_base.base().hwnd().DestroyWindow()?; // then destroy modal
				let hchild_prev_focus_parent =
					unsafe { &mut *self2.0.hchild_prev_focus_parent.get() };
				if *hchild_prev_focus_parent != HWND::NULL {
					hchild_prev_focus_parent.SetFocus(); // this focus could be set on WM_DESTROY as well
				}
			}
			Ok(())
		});
	}

	#[must_use]
	pub(in crate::gui) fn raw_base(&self) -> &RawBase {
		&self.0.raw_base
	}

	pub(in crate::gui) fn show_modal(&self, parent: &impl GuiParent) -> AnyResult<()> {
		let hinst = parent.hwnd().hinstance();
		let opts = &self.0.opts;
		let atom = self.0.raw_base.register_class(
			&hinst,
			&opts.class_name,
			opts.class_style,
			&opts.class_icon,
			&opts.class_bg_brush,
			&opts.class_cursor,
		)?;

		*unsafe { &mut *self.0.hchild_prev_focus_parent.get() } =
			HWND::GetFocus().unwrap_or(HWND::NULL);
		parent.hwnd().EnableWindow(false); // https://devblogs.microsoft.com/oldnewthing/20040227-00/?p=40463

		let mut rc_wnd = RECT {
			left: 0, // client area, will be adjusted to size with title bar and borders
			top: 0,
			right: opts.size.0,
			bottom: opts.size.1,
		};
		rc_wnd = AdjustWindowRectEx(rc_wnd, opts.style, false, opts.ex_style)?;

		let rc_parent = parent.hwnd().GetWindowRect()?; // relative to screen
		let wnd_pos = POINT::new(
			rc_parent.left + (rc_parent.right - rc_parent.left) / 2
				- (rc_wnd.right - rc_wnd.left) / 2, // center on parent
			rc_parent.top + (rc_parent.bottom - rc_parent.top) / 2
				- (rc_wnd.bottom - rc_wnd.top) / 2,
		);

		self.0.raw_base.create_window(
			opts.ex_style,
			atom,
			Some(&opts.title),
			opts.style,
			POINT::new(wnd_pos.x, wnd_pos.y),
			SIZE::new(rc_wnd.right - rc_wnd.left, rc_wnd.bottom - rc_wnd.top),
			Some(parent.hwnd()),
			IdMenu::None,
			&hinst,
		)?;

		self.0
			.raw_base
			.base()
			.run_modal_loop(opts.process_dlg_msgs)?; // blocks until window is closed
		Ok(())
	}
}
