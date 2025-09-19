use std::any::Any;

use crate::co;
use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::prelude::*;

/// A page of a [`Tab`](crate::gui::Tab) control, which can handle events. Can
/// be programmatically created or load a dialog resource from a `.res` file.
/// Passed to [`TabOpts`](crate::gui::TabOpts) and
/// [`Tab::new_dlg`](crate::gui::Tab::new_dlg).
#[derive(Clone)]
pub struct TabPage {
	wnd: WindowControl,
}

unsafe impl Send for TabPage {}

impl AsRef<BaseWnd> for TabPage {
	fn as_ref(&self) -> &BaseWnd {
		self.wnd.as_ref()
	}
}

impl GuiWindow for TabPage {
	fn hwnd(&self) -> &HWND {
		self.as_ref().hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiParent for TabPage {}

impl TabPage {
	/// Instantiates a new `TabPage` object, to be created internally with
	/// [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `TabPage` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: TabPageOpts) -> Self {
		Self {
			wnd: WindowControl::new(parent, opts.into()),
		}
	}

	/// Instantiates a new `TabPage` object, to be loaded from a dialog
	/// resource with
	/// [`HINSTANCE::CreateDialogParam`](crate::HINSTANCE::CreateDialogParam).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `TabPage` in an event closure.
	///
	/// Panics if the creation process fails.
	#[must_use]
	pub fn new_dlg(parent: &(impl GuiParent + 'static), dlg_id: u16) -> Self {
		Self {
			wnd: WindowControl::new_dlg(parent, dlg_id, (0, 0), (Horz::None, Vert::None), None),
		}
	}

	/// Exposes methods to handle window messages.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Events must be set before
	/// window creation.
	#[must_use]
	pub fn on(&self) -> &impl GuiEventsParent {
		self.wnd.on()
	}
}

/// Options to create a [`TabPage`](crate::gui::TabPage) programmatically with
/// [`TabPage::new`](crate::gui::TabPage::new).
pub struct TabPageOpts<'a> {
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
}

impl<'a> Default for TabPageOpts<'a> {
	fn default() -> Self {
		Self {
			class_name: "",
			class_style: co::CS::DBLCLKS,
			class_cursor: Cursor::Idc(co::IDC::ARROW),
			class_bg_brush: Brush::Color(co::COLOR::WINDOW),
		}
	}
}

impl<'a> Into<WindowControlOpts<'a>> for TabPageOpts<'a> {
	fn into(self) -> WindowControlOpts<'a> {
		WindowControlOpts {
			class_name: self.class_name,
			class_style: self.class_style,
			class_cursor: self.class_cursor,
			class_bg_brush: self.class_bg_brush,
			position: (0, 0),
			size: (0, 0),
			style: co::WS::CHILD
				| co::WS::TABSTOP
				| co::WS::GROUP
				| co::WS::VISIBLE
				| co::WS::CLIPCHILDREN
				| co::WS::CLIPSIBLINGS,
			ex_style: co::WS_EX::LEFT | co::WS_EX::CONTROLPARENT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
		}
	}
}
