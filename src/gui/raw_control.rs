use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::gui::base::Base;
use crate::gui::events::WindowEventsAll;
use crate::gui::layout_arranger::{Horz, Vert};
use crate::gui::privs::{multiply_dpi_or_dtu, paint_control_borders};
use crate::gui::raw_base::{Brush, Cursor, Icon, RawBase};
use crate::kernel::decl::{ErrResult, WString};
use crate::prelude:: GuiEvents;
use crate::user::decl::{HWND, IdMenu, POINT, SIZE, WNDCLASSEX};

struct Obj { // actual fields of RawControl
	raw_base: RawBase,
	opts: WindowControlOpts,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// An ordinary custom control window.
#[derive(Clone)]
pub(in crate::gui) struct RawControl(Pin<Arc<Obj>>);

impl RawControl {
	pub(in crate::gui) fn new(
		parent: &Base,
		opts: WindowControlOpts) -> Self
	{
		let (horz, vert) = (opts.horz_resize, opts.vert_resize);
		let new_self = Self(
			Arc::pin(
				Obj {
					raw_base: RawBase::new(Some(parent)),
					opts,
					_pin: PhantomPinned,
				},
			),
		);
		new_self.default_message_handlers(parent, horz, vert);
		new_self
	}

	pub(in crate::gui) unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		self.0.raw_base.as_base()
	}

	pub(in crate::gui) fn hwnd(&self) -> HWND {
		self.0.raw_base.hwnd()
	}

	pub(in crate::gui) fn ctrl_id(&self) -> u16 {
		self.0.opts.ctrl_id
	}

	pub(in crate::gui) fn on(&self) -> &WindowEventsAll {
		self.0.raw_base.on()
	}

	pub(in crate::gui) fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()> + Send + 'static,
	{
		self.0.raw_base.spawn_new_thread(func);
	}

	pub(in crate::gui) fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()> + Send + 'static
	{
		self.0.raw_base.run_ui_thread(func);
	}

	fn default_message_handlers(&self, parent: &Base, horz: Horz, vert: Vert) {
		let self2 = self.clone();
		self.0.raw_base.parent().unwrap().privileged_on().wm(parent.creation_msg(), move |_| {
			let opts = &self2.0.opts;

			let mut wcx = WNDCLASSEX::default();
			let mut class_name_buf = WString::default();
			RawBase::fill_wndclassex(
				self2.0.raw_base.parent_hinstance(),
				opts.class_style, &opts.class_icon, &opts.class_icon,
				&opts.class_bg_brush, &opts.class_cursor, &mut wcx,
				&mut class_name_buf);
			let atom = self2.0.raw_base.register_class(&mut wcx);

			let mut wnd_pos = opts.position;
			let mut wnd_sz = opts.size;
			multiply_dpi_or_dtu(self2.0.raw_base.parent().unwrap(),
				Some(&mut wnd_pos), Some(&mut wnd_sz));

			self2.0.raw_base.create_window(
				atom,
				None,
				IdMenu::Id(opts.ctrl_id),
				wnd_pos, wnd_sz,
				opts.ex_style, opts.style,
			);

			self2.0.raw_base.parent().unwrap()
				.add_to_layout_arranger(self2.hwnd(), horz, vert);
			Ok(None) // not meaningful
		});

		let self2 = self.clone();
		self.on().wm_nc_paint(move |p| {
			paint_control_borders(self2.hwnd(), p);
			Ok(())
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
	/// Defaults to `Icon::None`.
	pub class_icon: Icon,
	/// Window cursor to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `Cursor::Idc(co::IDC::ARROW)`.
	pub class_cursor: Cursor,
	/// Window background brush to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `Brush::Color(co::COLOR::WINDOW)`.
	pub class_bg_brush: Brush,

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
			class_icon: Icon::None,
			class_cursor: Cursor::Idc(co::IDC::ARROW),
			class_bg_brush: Brush::Color(co::COLOR::WINDOW),
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
