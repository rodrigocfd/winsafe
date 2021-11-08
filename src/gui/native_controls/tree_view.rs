use std::any::Any;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::gui::events::{EventsView, TreeViewEvents, WindowEvents};
use crate::gui::native_controls::base_native_control::{BaseNativeControl, OptsId};
use crate::gui::native_controls::tree_view_items::TreeViewItems;
use crate::gui::privs::{auto_ctrl_id, multiply_dpi};
use crate::gui::resizer::{Horz, Vert};
use crate::gui::traits::{
	AsAny,
	Child,
	EnableControl,
	FocusControl,
	NativeControl,
	NativeControlEvents,
	Parent,
	Window,
};
use crate::handles::{Handle, HWND};
use crate::msg::tvm;
use crate::structs::{POINT, SIZE};

/// Native
/// [tree view](https://docs.microsoft.com/en-us/windows/win32/controls/tree-view-controls)
/// control.
#[derive(Clone)]
pub struct TreeView(Arc<Obj>);

struct Obj { // actual fields of TreeView
	base: BaseNativeControl,
	opts_id: OptsId<TreeViewOpts>,
	events: TreeViewEvents,
}

unsafe impl Send for TreeView {}

impl AsAny for TreeView {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl Window for TreeView {
	fn hwnd(&self) -> HWND {
		self.0.base.hwnd()
	}
}

impl Child for TreeView {
	fn ctrl_id(&self) -> u16 {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}
}

impl NativeControl for TreeView {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl NativeControlEvents<TreeViewEvents> for TreeView {
	fn on(&self) -> &TreeViewEvents {
		if !self.hwnd().is_null() {
			panic!("Cannot add events after the control creation.");
		} else if !self.0.base.parent_base().hwnd().is_null() {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl EnableControl for TreeView {}
impl FocusControl for TreeView {}

impl TreeView {
	/// Instantiates a new `TreeView` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &impl Parent, opts: TreeViewOpts) -> TreeView {
		let opts = TreeViewOpts::define_ctrl_id(opts);
		let (ctrl_id, horz, vert) = (opts.ctrl_id, opts.horz_resize, opts.vert_resize);
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Wnd(opts),
					events: TreeViewEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm(parent.as_base().wmcreate_or_wminitdialog(), {
			let self2 = new_self.clone();
			move |_| { self2.create(horz, vert)?; Ok(0) }
		});
		new_self
	}

	/// Instantiates a new `TreeView` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(
		parent: &impl Parent,
		ctrl_id: u16,
		horz_resize: Horz, vert_resize: Vert) -> TreeView
	{
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Dlg(ctrl_id),
					events: TreeViewEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm_init_dialog({
			let self2 = new_self.clone();
			move |_| { self2.create(horz_resize, vert_resize)?; Ok(true) }
		});
		new_self
	}

	fn create(&self, horz: Horz, vert: Vert) -> WinResult<()> {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				let mut sz = opts.size;
				multiply_dpi(Some(&mut pos), Some(&mut sz))?;

				self.0.base.create_window( // may panic
					"SysTreeView32", None, pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.tree_view_style.into(),
				)?;

				if opts.tree_view_ex_style != co::TVS_EX::NoValue {
					self.set_extended_style(true, opts.tree_view_ex_style)?;
				}
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ())?, // may panic
		}

		self.0.base.parent_base().add_to_resizer(self.hwnd(), horz, vert)
	}

	/// Exposes the item methods.
	pub fn items<'a>(&'a self) -> TreeViewItems<'a> {
		TreeViewItems {
			hwnd: self.hwnd(),
			owner: PhantomData,
		}
	}

	/// Sets or unsets the given extended list view styles by sending a
	/// [`tvm::SetExtendedStyle`](crate::msg::tvm::SetExtendedStyle) message.
	pub fn set_extended_style(&self,
		set: bool, ex_style: co::TVS_EX) -> WinResult<()>
	{
		self.hwnd().SendMessage(tvm::SetExtendedStyle {
			mask: ex_style,
			style: if set { ex_style } else { co::TVS_EX::NoValue },
		})
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`TreeView`](crate::gui::TreeView) programmatically with
/// [`TreeView::new`](crate::gui::TreeView::new).
pub struct TreeViewOpts {
	/// Control position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Control size, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 50 x 50.
	pub size: SIZE,
	/// Tree view styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TVS::HASLINES | TVS::LINESATROOT | TVS::SHOWSELALWAYS | TVS::HASBUTTONS`.
	pub tree_view_style: co::TVS,
	/// Extended tree view styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TVS_EX::NoValue`.
	pub tree_view_ex_style: co::TVS_EX,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
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
