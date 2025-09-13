use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::prelude::*;

struct RawModelessObj {
	raw_base: RawBase,
	_pin: PhantomPinned,
}

/// An ordinary modeless window.
///
/// Hierarchy: `BaseWnd` -> `RawBase` -> `RawModeless`.
#[derive(Clone)]
pub(in crate::gui) struct RawModeless(Pin<Arc<RawModelessObj>>);

impl RawModeless {
	#[must_use]
	pub(in crate::gui) fn new(
		parent: &(impl GuiParent + 'static),
		opts: WindowModelessOpts,
	) -> Self {
		let new_self = Self(Arc::pin(RawModelessObj {
			raw_base: RawBase::new(),
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		let opts2: WindowModelessOptsObj = opts.into();
		parent
			.as_ref()
			.before_on()
			.wm(parent.as_ref().wnd_ty().creation_msg(), move |_| {
				let hinst = parent2.hwnd().hinstance();
				let atom = self2.0.raw_base.register_class(
					&hinst,
					&opts2.class_name,
					opts2.class_style,
					&opts2.class_icon,
					&opts2.class_bg_brush,
					&opts2.class_cursor,
				);

				let rc_parent = parent2
					.hwnd()
					.ClientToScreenRc(parent2.hwnd().GetClientRect().expect(DONTFAIL))
					.expect(DONTFAIL);

				self2.0.raw_base.create_window(
					opts2.ex_style,
					atom,
					Some(&opts2.title),
					opts2.style,
					POINT::with(
						opts2.position.0 + rc_parent.left,
						opts2.position.1 + rc_parent.top,
					),
					opts2.size.into(),
					Some(parent2.hwnd()),
					IdMenu::None,
					&hinst,
				);

				Ok(0) // ignored
			});

		new_self
	}

	#[must_use]
	pub(in crate::gui) fn raw_base(&self) -> &RawBase {
		&self.0.raw_base
	}
}

/// Options to create a [`WindowModeless`](crate::gui::WindowModeless)
/// programmatically with
/// [`WindowModeless::new`](crate::gui::WindowModeless::new).
pub struct WindowModelessOpts<'a> {
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
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Width and height of window client area, in pixels, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	/// Does not include title bar or borders.
	///
	/// Defaults to `gui::dpi(220, 150)`.
	pub size: (i32, i32),
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CAPTION | WS::SYSMENU | WS::CLIPCHILDREN | WS::BORDER | WS::VISIBLE`.
	///
	/// Suggestions:
	/// * `WS::SIZEBOX` to make the window resizable.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::TOOLWINDOW`.
	pub ex_style: co::WS_EX,
}

impl<'a> Default for WindowModelessOpts<'a> {
	fn default() -> Self {
		Self {
			class_name: "",
			class_style: co::CS::DBLCLKS,
			class_icon: Icon::None,
			class_cursor: Cursor::Idc(co::IDC::ARROW),
			class_bg_brush: Brush::Color(co::COLOR::BTNFACE),
			title: "",
			position: dpi(0, 0),
			size: dpi(220, 150),
			style: co::WS::CAPTION
				| co::WS::SYSMENU
				| co::WS::CLIPCHILDREN
				| co::WS::BORDER
				| co::WS::VISIBLE,
			ex_style: co::WS_EX::LEFT | co::WS_EX::TOOLWINDOW,
		}
	}
}

impl<'a> Into<WindowModelessOptsObj> for WindowModelessOpts<'a> {
	fn into(self) -> WindowModelessOptsObj {
		WindowModelessOptsObj {
			class_name: self.class_name.to_owned(),
			class_style: self.class_style,
			class_icon: self.class_icon,
			class_cursor: self.class_cursor,
			class_bg_brush: self.class_bg_brush,
			title: self.title.to_owned(),
			position: self.position,
			size: self.size,
			style: self.style,
			ex_style: self.ex_style,
		}
	}
}

/// To be stored inside the object.
struct WindowModelessOptsObj {
	class_name: String,
	class_style: co::CS,
	class_icon: Icon,
	class_cursor: Cursor,
	class_bg_brush: Brush,
	title: String,
	position: (i32, i32),
	size: (i32, i32),
	style: co::WS,
	ex_style: co::WS_EX,
}
