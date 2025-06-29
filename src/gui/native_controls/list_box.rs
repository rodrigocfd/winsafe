use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::gui::{collections::*, events::*, privs::*, *};
use crate::msg::*;
use crate::prelude::*;

struct ListBoxObj {
	base: BaseCtrl,
	events: ListBoxEvents,
	_pin: PhantomPinned,
}

native_ctrl! { ListBox: ListBoxObj => ListBoxEvents;
	/// Native
	/// [list box](https://learn.microsoft.com/en-us/windows/win32/controls/about-list-boxes)
	/// control. Not to be confused with the more complex
	/// [list view](crate::gui::ListView) control.
}

impl ListBox {
	/// Instantiates a new `ListBox` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `ListBox` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: ListBoxOpts) -> Self {
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(Arc::pin(ListBoxObj {
			base: BaseCtrl::new(ctrl_id),
			events: ListBoxEvents::new(parent, ctrl_id),
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent
			.as_ref()
			.before_on()
			.wm(parent.as_ref().wnd_ty().creation_msg(), move |_| {
				self2.0.base.create_window(
					opts.window_ex_style,
					"ListBox",
					None,
					opts.window_style | opts.control_style.into(),
					opts.position.into(),
					opts.size.into(),
					&parent2,
				);
				ui_font::set(self2.hwnd());
				self2.items().add(&opts.items)?;
				parent2
					.as_ref()
					.add_to_layout(self2.hwnd(), opts.resize_behavior);
				Ok(0) // ignored
			});

		new_self
	}

	/// Instantiates a new `ListBox` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `ListBox` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self {
		let new_self = Self(Arc::pin(ListBoxObj {
			base: BaseCtrl::new(ctrl_id),
			events: ListBoxEvents::new(parent, ctrl_id),
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

	/// Item methods.
	#[must_use]
	pub const fn items(&self) -> ListBoxItems<'_> {
		ListBoxItems::new(self)
	}

	/// Sets the scrollable width by sending an
	/// [`lb::SetHorizontalExtent`](crate::msg::lb::SetHorizontalExtent) message.
	pub fn set_horizontal_extend(&self, pixels: i32) {
		unsafe {
			self.hwnd()
				.SendMessage(lb::SetHorizontalExtent { width: pixels as _ });
		}
	}
}

/// Options to create a [`ListBox`](crate::gui::ListBox) programmatically with
/// [`ListBox::new`](crate::gui::ListBox::new).
pub struct ListBoxOpts {
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Width and height of control to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(120, 120)`.
	pub size: (i32, i32),
	/// List box styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// To allow multiple selection, add `LBS::MULTIPLESEL`.
	///
	/// Defaults to `LBS::NOTIFY`.
	pub control_style: co::LBS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::GROUP | WS::TABSTOP | WS::VISIBLE`.
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

	/// Items to be added.
	///
	/// Defaults to none.
	pub items: Vec<String>,
}

impl Default for ListBoxOpts {
	fn default() -> Self {
		Self {
			position: dpi(0, 0),
			size: dpi(120, 120),
			control_style: co::LBS::NOTIFY,
			window_style: co::WS::CHILD | co::WS::GROUP | co::WS::TABSTOP | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			items: Vec::<String>::new(),
		}
	}
}
