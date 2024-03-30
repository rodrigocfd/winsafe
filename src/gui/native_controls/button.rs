use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj { // actual fields of Button
	base: BaseNativeControl,
	events: ButtonEvents,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Native
/// [button](https://learn.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#push-buttons)
/// control.
#[derive(Clone)]
pub struct Button(Pin<Arc<Obj>>);

unsafe impl Send for Button {}

impl AsRef<BaseNativeControl> for Button {
	fn as_ref(&self) -> &BaseNativeControl {
		&self.0.base
	}
}

impl GuiWindow for Button {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiWindowText for Button {}

impl GuiChild for Button {
	fn ctrl_id(&self) -> u16 {
		self.0.base.ctrl_id()
	}
}

impl GuiChildFocus for Button {}

impl GuiNativeControl for Button {}

impl GuiNativeControlEvents<ButtonEvents> for Button {
	fn on(&self) -> &ButtonEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl Button {
	/// Instantiates a new `Button` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `Button` in an event closure.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// let btn = gui::Button::new(
	///     &wnd,
	///     gui::ButtonOpts {
	///         position: (10, 10),
	///         text: "&Click me".to_owned(),
	///         ..Default::default()
	///     },
	/// );
	/// ```
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: ButtonOpts) -> Self {
		let opts = auto_ctrl_id_if_zero(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent, ctrl_id),
					events: ButtonEvents::new(parent, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent.as_ref().privileged_on().wm_create_or_initdialog(move |_, _| {
 			self2.create(OptsResz::Wnd(&opts))?;
			Ok(())
		});

		new_self
	}

	/// Instantiates a new `Button` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `Button` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self
	{
		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent, ctrl_id),
					events: ButtonEvents::new(parent, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent.as_ref().privileged_on().wm(co::WM::INITDIALOG, move |_, _| {
			self2.create(OptsResz::Dlg(resize_behavior))?;
			Ok(())
		});

		new_self
	}

	fn create(&self, opts_resz: OptsResz<&ButtonOpts>) -> SysResult<()> {
		match &opts_resz {
			OptsResz::Wnd(opts) => {
				let mut pos = POINT::new(opts.position.0, opts.position.1);
				let mut sz = SIZE::new(opts.width as _, opts.height as _);
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), Some(&mut sz))?;

				self.0.base.create_window(
					"BUTTON", Some(&opts.text), pos, sz,
					opts.window_ex_style,
					opts.window_style | opts.button_style.into(),
				)?;

				unsafe {
					self.hwnd().SendMessage(wm::SetFont {
						hfont: ui_font(),
						redraw: true,
					});
				}
			},
			OptsResz::Dlg(_) => self.0.base.create_dlg()?,
		}

		self.0.base.parent()
			.add_to_layout_arranger(self.hwnd(), opts_resz.resize_behavior())
	}

	/// Fires the click event for the button by sending a
	/// [`bm::Click`](crate::msg::bm::Click) message.
	pub fn trigger_click(&self) {
		unsafe { self.hwnd().SendMessage(bm::Click {}); }
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`Button`](crate::gui::Button) programmatically with
/// [`Button::new`](crate::gui::Button::new).
pub struct ButtonOpts {
	/// Text of the control to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to `(0, 0)`.
	pub position: (i32, i32),
	/// Control width to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the value is in Dialog Template Units;
	/// otherwise in pixels, which will be multiplied to match current system
	/// DPI.
	///
	/// Defaults to `88`.
	pub width: u32,
	/// Control height to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the value is in Dialog Template Units;
	/// otherwise in pixels, which will be multiplied to match current system
	/// DPI.
	///
	/// Defaults to `26`.
	pub height: u32,
	/// Button styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `BS::PUSHBUTTON`.
	///
	/// Suggestions:
	/// * replace with `BS::DEFPUSHBUTTON` for the default button of the window;
	/// * add `BS::NOTIFY` to receive notifications other than the simple click.
	pub button_style: co::BS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
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

impl Default for ButtonOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: (0, 0),
			width: 88,
			height: 26,
			button_style: co::BS::PUSHBUTTON,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
		}
	}
}

impl ResizeBehavior for &ButtonOpts {
	fn resize_behavior(&self) -> (Horz, Vert) {
		self.resize_behavior
	}
}

impl AutoCtrlId for ButtonOpts {
	fn ctrl_id_mut(&mut self) -> &mut u16 {
		&mut self.ctrl_id
	}
}
