use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*, spec::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj { // actual fields of ListBox
	base: BaseNativeControl,
	events: ListBoxEvents,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Native
/// [list box](https://learn.microsoft.com/en-us/windows/win32/controls/about-list-boxes)
/// control. Not to be confused with the more complex
/// [list view](crate::gui::ListView) control.
#[derive(Clone)]
pub struct ListBox(Pin<Arc<Obj>>);

unsafe impl Send for ListBox {}

impl AsRef<BaseNativeControl> for ListBox {
	fn as_ref(&self) -> &BaseNativeControl {
		&self.0.base
	}
}

impl GuiWindow for ListBox {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiChild for ListBox {
	fn ctrl_id(&self) -> u16 {
		self.0.base.ctrl_id()
	}
}

impl GuiChildFocus for ListBox {}

impl GuiNativeControl for ListBox {}

impl GuiNativeControlEvents<ListBoxEvents> for ListBox {
	fn on(&self) -> &ListBoxEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl ListBox {
	/// Instantiates a new `ListBox` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `ListBox` in an event closure.
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: ListBoxOpts) -> Self {
		let opts = auto_ctrl_id_if_zero(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent, ctrl_id),
					events: ListBoxEvents::new(parent, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent.as_ref().before_user_on().wm_create_or_initdialog(move |_, _| {
			self2.create(OptsResz::Wnd(&opts))?;
			Ok(WmRet::NotHandled)
		});

		new_self
	}

	/// Instantiates a new `ListBox` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `ListBox` in an event closure.
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
					events: ListBoxEvents::new(parent, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent.as_ref().before_user_on().wm(co::WM::INITDIALOG, move |_, _| {
			self2.create(OptsResz::Dlg(resize_behavior))?;
			Ok(WmRet::NotHandled)
		});

		new_self
	}

	fn create(&self, opts_resz: OptsResz<&ListBoxOpts>) -> SysResult<()> {
		match opts_resz {
			OptsResz::Wnd(opts) => {
				let mut pos = POINT::new(opts.position.0, opts.position.1);
				let mut sz = SIZE::new(opts.size.0 as _, opts.size.1 as _);
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), Some(&mut sz))?;

				self.0.base.create_window(
					"ListBox", None, pos, sz,
					opts.window_ex_style,
					opts.window_style | opts.list_box_style.into(),
				)?;

				unsafe {
					self.hwnd().SendMessage(wm::SetFont {
						hfont: ui_font(),
						redraw: true,
					});
				}

				self.items().add(&opts.items);
			},
			OptsResz::Dlg(_) => self.0.base.create_dlg()?,
		}

		self.0.base.parent()
			.add_to_layout_arranger(self.hwnd(), opts_resz.resize_behavior())
	}

	/// Item methods.
	#[must_use]
	pub const fn items(&self) -> ListBoxItems<'_> {
		ListBoxItems::new(self)
	}

	/// Sets the scrollable width by sending an
	/// [`lb::SetHorizontalExtent`](crate::msg::lb::SetHorizontalExtent) message.
	pub fn set_horizontal_extend(&self, pixels: u32) {
		unsafe {
			self.hwnd()
				.SendMessage(lb::SetHorizontalExtent { width: pixels });
		}
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`ListBox`](crate::gui::ListBox) programmatically with
/// [`ListBox::new`](crate::gui::ListBox::new).
pub struct ListBoxOpts {
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
	/// Width and height of control to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to `(50, 50)`.
	pub size: (u32, u32),
	/// List box styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `LBS::NOTIFY`.
	pub list_box_style: co::LBS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::CLIENTEDGE`.
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

	/// Items to be added right away to the control.
	///
	/// Defaults to none.
	pub items: Vec<String>,
}

impl Default for ListBoxOpts {
	fn default() -> Self {
		Self {
			position: (0, 0),
			size: (50, 50),
			list_box_style: co::LBS::NOTIFY,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			items: Vec::<String>::new(),
		}
	}
}

impl ResizeBehavior for &ListBoxOpts {
	fn resize_behavior(&self) -> (Horz, Vert) {
		self.resize_behavior
	}
}

impl AutoCtrlId for ListBoxOpts {
	fn ctrl_id_mut(&mut self) -> &mut u16 {
		&mut self.ctrl_id
	}
}
