use std::ptr::NonNull;
use std::sync::Arc;

use crate::co;
use crate::gui::base::Base;
use crate::gui::privs::{multiply_dpi_or_dtu, paint_control_borders};
use crate::gui::raw_base::RawBase;
use crate::gui::resizer::{Horz, Vert};
use crate::kernel::decl::WString;
use crate::prelude::{GdiHbrush, GuiEventsView, Handle, UserHwnd};
use crate::user::decl::{
	HBRUSH, HCURSOR, HICON, IdMenu, POINT, SIZE, WNDCLASSEX,
};

/// A WindowControl with a raw window.
#[derive(Clone)]
pub(in crate::gui) struct RawControl(pub(in crate::gui) Arc<Obj>);

pub(in crate::gui) struct Obj { // actual fields of RawControl
	pub(in crate::gui) raw_base: RawBase,
	pub(in crate::gui) opts: WindowControlOpts,
}

impl RawControl {
	pub(in crate::gui) fn new(
		parent_base: &Base,
		opts: WindowControlOpts) -> Self
	{
		let (horz, vert) = (opts.horz_resize, opts.vert_resize);
		let new_self = Self(Arc::new(
			Obj {
				raw_base: RawBase::new(Some(parent_base)),
				opts,
			},
		));
		new_self.default_message_handlers(parent_base, horz, vert);
		new_self
	}

	fn default_message_handlers(&self,
		parent_base: &Base, horz: Horz, vert: Vert)
	{
		parent_base.privileged_on().wm(parent_base.wmcreate_or_wminitdialog(), {
			let self2 = self.clone();
			let parent_base_ptr = NonNull::from(parent_base);
			move |_| {
				let opts = &self2.0.opts;

				let mut wcx = WNDCLASSEX::default();
				let mut class_name_buf = WString::default();
				RawBase::fill_wndclassex(
					self2.0.raw_base.base.parent_base().unwrap().hwnd().hinstance(),
					opts.class_style, opts.class_icon, opts.class_icon,
					opts.class_bg_brush, opts.class_cursor, &mut wcx, &mut class_name_buf)?;
				let atom = self2.0.raw_base.register_class(&mut wcx)?;

				let mut wnd_pos = opts.position;
				let mut wnd_sz = opts.size;
				multiply_dpi_or_dtu(self2.0.raw_base.base.parent_base().unwrap(),
					Some(&mut wnd_pos), Some(&mut wnd_sz))?;

				self2.0.raw_base.create_window(
					atom,
					None,
					IdMenu::Id(opts.ctrl_id),
					wnd_pos, wnd_sz,
					opts.ex_style, opts.style,
				)?;

				unsafe {
					parent_base_ptr.as_ref().add_to_resizer(
						self2.0.raw_base.base.hwnd(), horz, vert)?;
				}

				Ok(0)
			}
		});

		self.0.raw_base.base.on().wm_nc_paint({
			let self2 = self.clone();
			move |p| paint_control_borders(self2.0.raw_base.base.hwnd(), p)
		});
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`WindowControl`](crate::gui::WindowControl)
/// programmatically with [`WindowControl::new`](crate::gui::WindowControl::new).
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
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
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Size of window, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
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
	/// Horizontal behavior when the parent is resized.
	///
	/// Defaults to `Horz::None`.
	pub horz_resize: Horz,
	/// Vertical behavior when the parent is resized.
	///
	/// Defaults to `Vert::None`.
	pub vert_resize: Vert,
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
			horz_resize: Horz::None,
			vert_resize: Vert::None,
		}
	}
}
