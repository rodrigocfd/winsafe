use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::PostQuitMessage;
use crate::gui::events::TreeViewEvents;
use crate::gui::native_controls::base_native_control::{BaseNativeControl, OptsId};
use crate::gui::native_controls::tree_view_items::TreeViewItems;
use crate::gui::privs::{auto_ctrl_id, multiply_dpi};
use crate::gui::traits::{baseref_from_parent, Parent};
use crate::handles::HWND;
use crate::msg::tvm;
use crate::structs::{POINT, SIZE};

/// Native
/// [tree view](https://docs.microsoft.com/en-us/windows/win32/controls/tree-view-controls)
/// control.
///
/// Implements [`Child`](crate::gui::Child) trait.
#[derive(Clone)]
pub struct TreeView(Arc<Obj>);

struct Obj { // actual fields of TreeView
	base: BaseNativeControl,
	opts_id: OptsId<TreeViewOpts>,
	events: TreeViewEvents,
	items: TreeViewItems,
}

impl_send_sync_child!(TreeView);

impl TreeView {
	/// Instantiates a new `TreeView` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: TreeViewOpts) -> TreeView {
		let parent_base_ref = baseref_from_parent(parent);
		let opts = TreeViewOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Wnd(opts),
					events: TreeViewEvents::new(parent_base_ref, ctrl_id),
					items: TreeViewItems::new(parent_base_ref.hwnd_ref()), // wrong HWND, just to construct the object
				},
			),
		);
		new_self.0.items.set_hwnd_ref(new_self.0.base.hwnd_ref()); // correct HWND

		parent_base_ref.privileged_events_ref().wm(parent_base_ref.creation_wm(), {
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});

		new_self
	}

	/// Instantiates a new `TreeView` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> TreeView {
		let parent_base_ref = baseref_from_parent(parent);

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Dlg(ctrl_id),
					events: TreeViewEvents::new(parent_base_ref, ctrl_id),
					items: TreeViewItems::new(parent_base_ref.hwnd_ref()), // wrong HWND, just to construct the object
				},
			),
		);
		new_self.0.items.set_hwnd_ref(new_self.0.base.hwnd_ref()); // correct HWND

		parent_base_ref.privileged_events_ref().wm_init_dialog({
			let me = new_self.clone();
			move |_| { me.create(); true }
		});

		new_self
	}

	fn create(&self) {
		|| -> WinResult<()> {
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

					if opts.tree_view_ex_style != co::TVS_EX::NONE {
						self.toggle_extended_style(true, opts.tree_view_ex_style)?;
					}
					Ok(())
				},
				OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

	pub_fn_ctrlid_hwnd_on_onsubclass!(TreeViewEvents);

	/// Exposes the item methods.
	pub fn items(&self) -> &TreeViewItems {
		&self.0.items
	}

	/// Toggles the given extended list view styles by sending an
	/// [`TVM_SETEXTENDEDSTYLE`](crate::msg::tvm::SetExtendedStyle)
	/// message.
	pub fn toggle_extended_style(&self,
		set: bool, ex_style: co::TVS_EX) -> WinResult<()>
	{
		self.hwnd().SendMessage(tvm::SetExtendedStyle {
			mask: ex_style,
			style: if set { ex_style } else { co::TVS_EX::NONE },
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
	/// Defaults to `TVS_EX::NONE`.
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
}

impl Default for TreeViewOpts {
	fn default() -> Self {
		Self {
			position: POINT::new(0, 0),
			size: SIZE::new(50, 50),
			tree_view_style: co::TVS::HASLINES | co::TVS::LINESATROOT | co::TVS::SHOWSELALWAYS | co::TVS::HASBUTTONS,
			tree_view_ex_style: co::TVS_EX::NONE,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
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
