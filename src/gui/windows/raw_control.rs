use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::prelude::*;

struct RawControlObj {
	raw_base: RawBase,
	ctrl_id: u16,
	_pin: PhantomPinned,
}

/// An ordinary custom control window.
///
/// Hierarchy: `BaseWnd` -> `RawBase` -> `RawControl`.
#[derive(Clone)]
pub(in crate::gui) struct RawControl(Pin<Arc<RawControlObj>>);

impl RawControl {
	#[must_use]
	pub(in crate::gui) fn new(
		parent: &(impl GuiParent + 'static),
		opts: WindowControlOpts,
	) -> Self {
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(Arc::pin(RawControlObj {
			raw_base: RawBase::new(),
			ctrl_id,
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		let opts2: WindowControlOptsObj = opts.into();
		parent
			.as_ref()
			.before_on()
			.wm(parent.as_ref().wnd_ty().creation_msg(), move |_| {
				let hinst = parent2.hwnd().hinstance();
				let atom = self2.0.raw_base.register_class(
					&hinst,
					&opts2.class_name,
					opts2.class_style,
					&Icon::None,
					&opts2.class_bg_brush,
					&opts2.class_cursor,
				);
				self2.0.raw_base.create_window(
					opts2.ex_style,
					atom,
					None,
					opts2.style,
					opts2.position.into(),
					opts2.size.into(),
					Some(parent2.hwnd()),
					IdMenu::Id(ctrl_id),
					&hinst,
				);

				parent2
					.as_ref()
					.add_to_layout(self2.0.raw_base.base().hwnd(), opts2.resize_behavior);
				Ok(0) // ignored
			});

		new_self.default_message_handlers();
		new_self
	}

	fn default_message_handlers(&self) {
		let self2 = self.clone();
		self.0.raw_base.base().before_on().wm_nc_paint(move |p| {
			paint_control_borders(self2.0.raw_base.base().hwnd(), p);
			Ok(())
		});
	}

	#[must_use]
	pub(in crate::gui) fn raw_base(&self) -> &RawBase {
		&self.0.raw_base
	}

	#[must_use]
	pub(in crate::gui) fn ctrl_id(&self) -> u16 {
		self.0.ctrl_id
	}
}

/// Options to create a [`WindowControl`](crate::gui::WindowControl)
/// programmatically with [`WindowControl::new`](crate::gui::WindowControl::new).
pub struct WindowControlOpts<'a> {
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
	/// Window cursor to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Cursor::Idc(co::IDC::ARROW)`.
	pub class_cursor: Cursor,
	/// Window background brush to be
	/// [registered](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `gui::Brush::Color(co::COLOR::WINDOW)`.
	pub class_bg_brush: Brush,

	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Width and height of window to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(100, 80)`.
	pub size: (i32, i32),
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::TABSTOP | WS::GROUP | WS::VISIBLE | WS::CLIPCHILDREN | WS::CLIPSIBLINGS`.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
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
	/// Horizontal and vertical behavior of the control when the parent window
	/// is resized.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),
}

impl<'a> Default for WindowControlOpts<'a> {
	fn default() -> Self {
		Self {
			class_name: "",
			class_style: co::CS::DBLCLKS,
			class_cursor: Cursor::Idc(co::IDC::ARROW),
			class_bg_brush: Brush::Color(co::COLOR::WINDOW),
			position: dpi(0, 0),
			size: dpi(100, 80),
			style: co::WS::CHILD
				| co::WS::TABSTOP
				| co::WS::GROUP
				| co::WS::VISIBLE
				| co::WS::CLIPCHILDREN
				| co::WS::CLIPSIBLINGS,
			ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
		}
	}
}

impl<'a> Into<WindowControlOptsObj> for WindowControlOpts<'a> {
	fn into(self) -> WindowControlOptsObj {
		WindowControlOptsObj {
			class_name: self.class_name.to_owned(),
			class_style: self.class_style,
			class_cursor: self.class_cursor,
			class_bg_brush: self.class_bg_brush,
			position: self.position,
			size: self.size,
			style: self.style,
			ex_style: self.ex_style,
			resize_behavior: self.resize_behavior,
		}
	}
}

/// To be stored inside the object.
struct WindowControlOptsObj {
	class_name: String,
	class_style: co::CS,
	class_cursor: Cursor,
	class_bg_brush: Brush,
	position: (i32, i32),
	size: (i32, i32),
	style: co::WS,
	ex_style: co::WS_EX,
	resize_behavior: (Horz, Vert),
}
