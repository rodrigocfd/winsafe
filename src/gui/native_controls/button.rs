use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::msg::*;
use crate::prelude::*;

struct ButtonObj {
	base: BaseCtrl,
	events: BaseCtrlEvents,
	_pin: PhantomPinned,
}

native_ctrl! { Button: ButtonObj => GuiEventsButton;
	/// Native
	/// [button](https://learn.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#push-buttons)
	/// control.
	///
	/// # Examples
	///
	/// Basic structure of a program with a main window and a button, both
	/// created programmatically:
	///
	/// ```no_run
	/// use winsafe::{self as w, co, gui, prelude::*};
	///
	/// fn main() {
	///     if let Err(err) = Main::create_and_run() {
	///         w::HWND::NULL
	///             .MessageBox(&err.to_string(), "Uncaught error", co::MB::ICONERROR)
	///             .unwrap();
	///     }
	/// }
	///
	/// #[derive(Clone)]
	/// struct Main {
	///     wnd: gui::WindowMain,
	///     btn: gui::Button,
	/// }
	///
	/// impl Main {
	///     #[must_use]
	///     fn create_and_run() -> w::AnyResult<i32> {
	///         let wnd = gui::WindowMain::new(gui::WindowMainOpts {
	///             title: "Main window",
	///             ..Default::default()
	///         });
	///         let btn = gui::Button::new(
	///             &wnd,
	///             gui::ButtonOpts {
	///                 text: "&Click me",
	///                 position: gui::dpi(20, 20),
	///                 ..Default::default()
	///             },
	///         );
	///
	///         let new_self = Self { wnd, btn };
	///         new_self.events();
	///
	///         new_self.wnd.run_main(None)
	///     }
	///
	///     fn events(&self) {
	///         let self2 = self.clone();
	///             self.btn.on().bn_clicked(move || {
	///             self2.wnd.hwnd().SetWindowText("Button clicked")?;
	///             Ok(())
	///         });
	///     }
	/// }
	/// ```
}

impl Button {
	/// Instantiates a new `Button` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `Button` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: ButtonOpts) -> Self {
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(Arc::pin(ButtonObj {
			base: BaseCtrl::new(ctrl_id),
			events: BaseCtrlEvents::new(parent, ctrl_id),
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		let text2 = opts.text.to_owned();
		parent
			.as_ref()
			.before_on()
			.wm(parent.as_ref().wnd_ty().creation_msg(), move |_| {
				self2.0.base.create_window(
					opts.window_ex_style,
					"BUTTON",
					Some(&text2),
					opts.window_style | opts.control_style.into(),
					opts.position.into(),
					SIZE::with(opts.width, opts.height),
					&parent2,
				);
				ui_font::set(self2.hwnd());
				parent2
					.as_ref()
					.add_to_layout(self2.hwnd(), opts.resize_behavior);
				Ok(0) // ignored
			});

		new_self
	}

	/// Instantiates a new `Button` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `Button` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self {
		let new_self = Self(Arc::pin(ButtonObj {
			base: BaseCtrl::new(ctrl_id),
			events: BaseCtrlEvents::new(parent, ctrl_id),
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent.as_ref().before_on().wm_init_dialog(move |_| {
			self2.0.base.assign_dlg(&parent2);
			parent2
				.as_ref()
				.add_to_layout(self2.hwnd(), resize_behavior);
			Ok(true) // ignored
		});

		new_self
	}

	/// Fires the click event for the button by sending a
	/// [`bm::Click`](crate::msg::bm::Click) message.
	pub fn trigger_click(&self) {
		unsafe {
			self.hwnd().SendMessage(bm::Click {});
		}
	}
}

/// Options to create a [`Button`](crate::gui::Button) programmatically with
/// [`Button::new`](crate::gui::Button::new).
pub struct ButtonOpts<'a> {
	/// Text of the control to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: &'a str,
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Control width to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi_x(88)`.
	pub width: i32,
	/// Control height to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi_y(26)`.
	pub height: i32,
	/// Button styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `BS::PUSHBUTTON`.
	///
	/// Suggestions:
	/// * replace with `BS::DEFPUSHBUTTON` for the default button of the window;
	/// * add `BS::NOTIFY` to receive notifications other than the simple click.
	pub control_style: co::BS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::GROUP | WS::TABSTOP | WS::VISIBLE`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	pub window_ex_style: co::WS_EX,

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

impl<'a> Default for ButtonOpts<'a> {
	fn default() -> Self {
		Self {
			text: "",
			position: dpi(0, 0),
			width: dpi_x(88),
			height: dpi_y(26),
			control_style: co::BS::PUSHBUTTON,
			window_style: co::WS::CHILD | co::WS::GROUP | co::WS::TABSTOP | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
		}
	}
}
