use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::gui::base::Base;
use crate::gui::events::{TreeViewEvents, WindowEvents};
use crate::gui::layout_arranger::{Horz, Vert};
use crate::gui::native_controls::base_native_control::{
	BaseNativeControl, OptsId,
};
use crate::gui::native_controls::tree_view_items::TreeViewItems;
use crate::gui::privs::{auto_ctrl_id, multiply_dpi_or_dtu};
use crate::kernel::decl::SysResult;
use crate::msg::tvm;
use crate::prelude::{
	GuiChild, GuiChildFocus, GuiEvents, GuiNativeControl,
	GuiNativeControlEvents, GuiParent, GuiWindow, Handle, user_Hwnd,
};
use crate::user::decl::{HWND, POINT, SIZE};

struct Obj { // actual fields of TreeView
	base: BaseNativeControl,
	opts_id: OptsId<TreeViewOpts>,
	events: TreeViewEvents,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Native
/// [tree view](https://learn.microsoft.com/en-us/windows/win32/controls/tree-view-controls)
/// control.
#[derive(Clone)]
pub struct TreeView(Pin<Arc<Obj>>);

unsafe impl Send for TreeView {}

impl GuiWindow for TreeView {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiChild for TreeView {
	fn ctrl_id(&self) -> u16 {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}
}

impl GuiChildFocus for TreeView {}

impl GuiNativeControl for TreeView {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl GuiNativeControlEvents<TreeViewEvents> for TreeView {
	fn on(&self) -> &TreeViewEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl TreeView {
	/// Instantiates a new `TreeView` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `TreeView` in an event closure.
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: TreeViewOpts) -> TreeView {
		let parent_ref = unsafe { Base::from_guiparent(parent) };
		let opts = TreeViewOpts::define_ctrl_id(opts);
		let (ctrl_id, horz, vert) = (opts.ctrl_id, opts.horz_resize, opts.vert_resize);

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_ref),
					opts_id: OptsId::Wnd(opts),
					events: TreeViewEvents::new(parent_ref, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_ref.privileged_on().wm(parent_ref.creation_msg(), move |_| {
			self2.create(horz, vert)?;
			Ok(None) // not meaningful
		});

		new_self
	}

	/// Instantiates a new `TreeView` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `TreeView` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert)) -> TreeView
	{
		let parent_ref = unsafe { Base::from_guiparent(parent) };

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_ref),
					opts_id: OptsId::Dlg(ctrl_id),
					events: TreeViewEvents::new(parent_ref, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_ref.privileged_on().wm_init_dialog(move |_| {
			self2.create(resize_behavior.0, resize_behavior.1)?;
			Ok(true) // not meaningful
		});

		new_self
	}

	fn create(&self, horz: Horz, vert: Vert) -> SysResult<()> {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				let mut sz = opts.size;
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), Some(&mut sz))?;

				self.0.base.create_window( // may panic
					"SysTreeView32", None, pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.tree_view_style.into(),
				)?;

				if opts.tree_view_ex_style != co::TVS_EX::NoValue {
					self.set_extended_style(true, opts.tree_view_ex_style);
				}
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id),
		}

		self.0.base.parent().add_to_layout_arranger(self.hwnd(), horz, vert)
	}

	/// Exposes the item methods.
	#[must_use]
	pub const fn items(&self) -> TreeViewItems {
		TreeViewItems::new(self)
	}

	/// Sets or unsets the given extended list view styles by sending a
	/// [`tvm::SetExtendedStyle`](crate::msg::tvm::SetExtendedStyle) message.
	pub fn set_extended_style(&self, set: bool, ex_style: co::TVS_EX) {
		self.hwnd()
			.SendMessage(tvm::SetExtendedStyle {
				mask: ex_style,
				style: if set { ex_style } else { co::TVS_EX::NoValue },
			})
			.unwrap();
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`TreeView`](crate::gui::TreeView) programmatically with
/// [`TreeView::new`](crate::gui::TreeView::new).
pub struct TreeViewOpts {
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
	/// Defaults to 50 x 50.
	pub size: SIZE,
	/// Tree view styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TVS::HASLINES | TVS::LINESATROOT | TVS::SHOWSELALWAYS | TVS::HASBUTTONS`.
	pub tree_view_style: co::TVS,
	/// Extended tree view styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TVS_EX::NoValue`.
	pub tree_view_ex_style: co::TVS_EX,
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
	/// Horizontal behavior when the parent is resized.
	///
	/// Defaults to `Horz::None`.
	pub horz_resize: Horz,
	/// Vertical behavior when the parent is resized.
	///
	/// Defaults to `Vert::None`.
	pub vert_resize: Vert,
}

impl Default for TreeViewOpts {
	fn default() -> Self {
		Self {
			position: POINT::new(0, 0),
			size: SIZE::new(50, 50),
			tree_view_style: co::TVS::HASLINES | co::TVS::LINESATROOT | co::TVS::SHOWSELALWAYS | co::TVS::HASBUTTONS,
			tree_view_ex_style: co::TVS_EX::NoValue,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
			horz_resize: Horz::None,
			vert_resize: Vert::None,
		}
	}
}

impl TreeViewOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
