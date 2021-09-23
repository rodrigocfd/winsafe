use std::sync::Arc;

use crate::co;
use crate::enums::IdMenu;
use crate::gui::base::Base;
use crate::gui::privs::{multiply_dpi, paint_control_borders};
use crate::gui::raw_base::RawBase;
use crate::handles::{HBRUSH, HCURSOR, HICON};
use crate::structs::{POINT, SIZE, WNDCLASSEX};
use crate::various::WString;

#[derive(Clone)]
pub(in crate::gui) struct RawControl(Arc<Obj>);

struct Obj { // actual fields of RawControl
	base: RawBase,
	opts: WindowControlOpts,
}

impl RawControl {
	pub(in crate::gui) fn new(
		parent_base_ref: &Base, opts: WindowControlOpts) -> RawControl
	{
		let wnd = Self(
			Arc::new(
				Obj {
					base: RawBase::new(Some(parent_base_ref)),
					opts,
				},
			),
		);
		wnd.0.base.ui_thread_message_handler();
		wnd.default_message_handlers(parent_base_ref);
		wnd
	}

	pub(in crate::gui) fn base_ref(&self) -> &Base {
		self.0.base.base_ref()
	}

	pub(in crate::gui) fn run_ui_thread<F: FnOnce()>(&self, func: F) {
		self.0.base.run_ui_thread(func);
	}

	fn default_message_handlers(&self, parent_base_ref: &Base) {
		parent_base_ref.privileged_events_ref().wm(parent_base_ref.creation_wm(), {
			let self2 = self.clone();
			move |_| {
				let opts = &self2.0.opts;

				let mut wcx = WNDCLASSEX::default();
				let mut class_name_buf = WString::default();
				RawBase::fill_wndclassex(self2.base_ref().parent_hinstance()?,
					opts.class_style, opts.class_icon, opts.class_icon,
					opts.class_bg_brush, opts.class_cursor, &mut wcx, &mut class_name_buf)?;
				let atom = self2.0.base.register_class(&mut wcx)?;

				let mut wnd_pos = opts.position;
				let mut wnd_sz = opts.size;
				multiply_dpi(Some(&mut wnd_pos), Some(&mut wnd_sz))?;

				self2.0.base.create_window( // may panic
					atom,
					None,
					IdMenu::Id(opts.ctrl_id),
					wnd_pos, wnd_sz,
					opts.ex_style, opts.style,
				)?;
				Ok(0)
			}
		});

		self.base_ref().user_events_ref().wm_nc_paint({
			let self2 = self.clone();
			move |p| {
				paint_control_borders(*self2.base_ref().hwnd_ref(), p)?;
				Ok(())
			}
		});
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`WindowControl`](crate::gui::WindowControl)
/// programmatically with [`WindowControl::new`](crate::gui::WindowControl::new).
pub struct WindowControlOpts {
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
	/// Defaults to `co::COLOR::WINDOW`.
	pub class_bg_brush: HBRUSH,

	/// Position of control within parent's client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Size of window, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub size: SIZE,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::TABSTOP | WS::GROUP | WS::VISIBLE | WS::CLIPCHILDREN | WS::CLIPSIBLINGS`.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
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
}

impl Default for WindowControlOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: HICON::NULL,
			class_cursor: HCURSOR::NULL,
			class_bg_brush: HBRUSH::from_sys_color(co::COLOR::WINDOW),
			position: POINT { x: 0, y: 0 },
			size: SIZE { cx: 0, cy: 0 },
			style: co::WS::CHILD | co::WS::TABSTOP | co::WS::GROUP | co::WS::VISIBLE | co::WS::CLIPCHILDREN | co::WS::CLIPSIBLINGS,
			ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
		}
	}
}
