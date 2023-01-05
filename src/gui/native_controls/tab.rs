use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::gui::base::Base;
use crate::gui::events::{TabEvents, WindowEvents};
use crate::gui::layout_arranger::{Horz, Vert};
use crate::gui::native_controls::base_native_control::{
	BaseNativeControl, OptsId,
};
use crate::gui::native_controls::tab_items::TabItems;
use crate::gui::privs::{auto_ctrl_id, multiply_dpi_or_dtu};
use crate::msg::tcm;
use crate::prelude::{
	GuiChild, GuiChildFocus, GuiEvents, GuiNativeControl,
	GuiNativeControlEvents, GuiParent, GuiWindow, Handle, user_Hwnd,
};
use crate::user::decl::{HWND, POINT, SIZE};

struct Obj { // actual fields of Tab
	base: BaseNativeControl,
	opts_id: OptsId<TabOpts>,
	events: TabEvents,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Native
/// [tab](https://learn.microsoft.com/en-us/windows/win32/controls/tab-controls)
/// control.
#[derive(Clone)]
pub struct Tab(Pin<Arc<Obj>>);

unsafe impl Send for Tab {}

impl GuiWindow for Tab {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiChild for Tab {
	fn ctrl_id(&self) -> u16 {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}
}

impl GuiChildFocus for Tab {}

impl GuiNativeControl for Tab {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl GuiNativeControlEvents<TabEvents> for Tab {
	fn on(&self) -> &TabEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl Tab {
	/// Instantiates a new `Tab` object, to be created on the parent window with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `TreeView` in an event closure.
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: TabOpts) -> Tab {
		let parent_ref = unsafe { Base::from_guiparent(parent) };
		let opts = TabOpts::define_ctrl_id(opts);
		let (ctrl_id, horz, vert) = (opts.ctrl_id, opts.horz_resize, opts.vert_resize);

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_ref),
					opts_id: OptsId::Wnd(opts),
					events: TabEvents::new(parent_ref, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_ref.privileged_on().wm(parent_ref.creation_msg(), move |_| {
			self2.create(horz, vert);
			Ok(None) // not meaningful
		});

		new_self
	}

	/// Instantiates a new `Tab` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `TreeView` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert)) -> Tab
	{
		let parent_ref = unsafe { Base::from_guiparent(parent) };

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_ref),
					opts_id: OptsId::Dlg(ctrl_id),
					events: TabEvents::new(parent_ref, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_ref.privileged_on().wm_init_dialog(move |_| {
			self2.create(resize_behavior.0, resize_behavior.1);
			Ok(true) // not meaningful
		});

		new_self
	}

	fn create(&self, horz: Horz, vert: Vert) {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				let mut sz = opts.size;
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), Some(&mut sz));

				self.0.base.create_window( // may panic
					"SysTabControl32", None, pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.tab_style.into(),
				);

				if opts.tab_ex_style != co::TCS_EX::NoValue {
					self.set_extended_style(true, opts.tab_ex_style);
				}
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id),
		}

		self.0.base.parent().add_to_layout_arranger(self.hwnd(), horz, vert);
	}

	/// Exposes the item methods.
	#[must_use]
	pub const fn items(&self) -> TabItems {
		TabItems::new(self)
	}

	/// Sets or unsets the given extended list view styles by sending a
	/// [`tcm::SetExtendedStyle`](crate::msg::tcm::SetExtendedStyle) message.
	pub fn set_extended_style(&self, set: bool, ex_style: co::TCS_EX) {
		self.hwnd()
			.SendMessage(tcm::SetExtendedStyle {
				mask: ex_style,
				style: if set { ex_style } else { co::TCS_EX::NoValue },
			});
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`Tab`](crate::gui::Tab) programmatically with
/// [`Tab::new`](crate::gui::Tab::new).
pub struct TabOpts {
	/// Control position within parent client area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Control size, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to 80 x 50.
	pub size: SIZE,
	/// Tab styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TCS::NoValue`.
	pub tab_style: co::TCS,
	/// Extended tab styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TCS_EX::NoValue`.
	pub tab_ex_style: co::TCS_EX,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::NoValue`.
	pub window_ex_style: co::WS_EX,

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

impl Default for TabOpts {
	fn default() -> Self {
		Self {
			position: POINT::new(0, 0),
			size: SIZE::new(80, 50),
			tab_style: co::TCS::NoValue,
			tab_ex_style: co::TCS_EX::NoValue,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::NoValue,
			ctrl_id: 0,
			horz_resize: Horz::None,
			vert_resize: Vert::None,
		}
	}
}

impl TabOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
