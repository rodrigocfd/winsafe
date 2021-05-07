use std::any::Any;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::{GetAsyncKeyState, PostQuitMessage};
use crate::gui::base::Base;
use crate::gui::events::{ListViewEvents, WindowEvents};
use crate::gui::native_controls::list_view_columns::ListViewColumns;
use crate::gui::native_controls::list_view_items::ListViewItems;
use crate::gui::native_controls::native_control_base::{NativeControlBase, OptsId};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi};
use crate::gui::traits::{baseref_from_parent, Child, Parent};
use crate::handles::HWND;
use crate::msg::lvm;
use crate::structs::{NMLVKEYDOWN, POINT, SIZE};

/// Native
/// [list view](https://docs.microsoft.com/en-us/windows/win32/controls/list-view-controls-overview)
/// control. Not to be confused with the simpler [list box](crate::gui::ListBox)
/// control.
///
/// Implements [`Child`](crate::gui::Child) trait.
#[derive(Clone)]
pub struct ListView(Arc<Obj>);

struct Obj { // actual fields of ListView
	base: NativeControlBase,
	opts_id: OptsId<ListViewOpts>,
	events: ListViewEvents,
	columns: ListViewColumns,
	items: ListViewItems,
}

unsafe impl Send for ListView {}
unsafe impl Sync for ListView {}

impl Child for ListView {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl ListView {
	/// Instantiates a new `ListView` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: ListViewOpts) -> ListView {
		let parent_ref = baseref_from_parent(parent);
		let opts = ListViewOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(parent_ref),
					opts_id: OptsId::Wnd(opts),
					events: ListViewEvents::new(parent_ref, ctrl_id),
					columns: ListViewColumns::new(parent_ref.hwnd_ref()), // wrong HWND, just to construct the object
					items: ListViewItems::new(parent_ref.hwnd_ref()),
				},
			),
		);
		new_self.0.columns.set_hwnd_ref(new_self.0.base.hwnd_ref()); // correct HWND
		new_self.0.items.set_hwnd_ref(new_self.0.base.hwnd_ref());

		parent_ref.privileged_events_ref().wm(parent_ref.create_wm(), {
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});

		new_self.handled_events(parent_ref, ctrl_id);
		new_self
	}

	/// Instantiates a new `ListView` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: i32) -> ListView {
		let parent_ref = baseref_from_parent(parent);

		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(parent_ref),
					opts_id: OptsId::Dlg(ctrl_id),
					events: ListViewEvents::new(parent_ref, ctrl_id),
					columns: ListViewColumns::new(parent_ref.hwnd_ref()), // wrong HWND, just to construct the object
					items: ListViewItems::new(parent_ref.hwnd_ref()),
				},
			),
		);
		new_self.0.columns.set_hwnd_ref(new_self.0.base.hwnd_ref()); // correct HWND
		new_self.0.items.set_hwnd_ref(new_self.0.base.hwnd_ref());

		parent_ref.privileged_events_ref().wm_init_dialog({
			let me = new_self.clone();
			move |_| { me.create(); true }
		});

		new_self.handled_events(parent_ref, ctrl_id);
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
						"SysListView32", None, pos, sz,
						opts.ctrl_id,
						opts.ex_window_style,
						opts.window_style | opts.list_view_style.into(),
					)?;

					if opts.ex_list_view_style != co::LVS_EX::NONE {
						self.toggle_extended_style(true, opts.ex_list_view_style);
					}
					Ok(())
				},
				OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

	fn handled_events(&self, parent_ref: &Base, ctrl_id: i32) {
		parent_ref.privileged_events_ref().add_nfy(ctrl_id as _, co::LVN::KEYDOWN.into(), {
			let me = self.clone();
			move |p| {
				let lvnk = unsafe { p.cast_nmhdr::<NMLVKEYDOWN>() };

				if lvnk.wVKey == co::VK('A' as _)
					&& GetAsyncKeyState(co::VK::CONTROL) // Ctrl+A pressed?
				{
					me.items().set_selected_all(true)
						.unwrap_or_else(|err| PostQuitMessage(err));
				}
				None
			}
		});
	}

	ctrlid_hwnd_on_onsubclass!(ListViewEvents);

	/// Column methods.
	pub fn columns(&self) -> &ListViewColumns {
		&self.0.columns
	}

	/// Item methods.
	pub fn items(&self) -> &ListViewItems {
		&self.0.items
	}

	/// Retrieves the current view by sending an
	/// [`LVM_GETVIEW`](crate::msg::lvm::GetView) message.
	pub fn current_view(&self) -> co::LV_VIEW {
		self.hwnd().SendMessage(lvm::GetView {})
	}

	/// Sets the current view by sending an
	/// [`LVM_SETVIEW`](crate::msg::lvm::SetView) message.
	pub fn set_current_view(&self, view: co::LV_VIEW) -> WinResult<()> {
		self.hwnd().SendMessage(lvm::SetView { view })
	}

	/// Toggles the given extended list view styles by sending an
	/// [`LVM_SETEXTENDEDLISTVIEWSTYLE`](crate::msg::lvm::SetExtendedListViewStyle)
	/// message.
	pub fn toggle_extended_style(&self, set: bool, ex_style: co::LVS_EX) {
		self.hwnd().SendMessage(lvm::SetExtendedListViewStyle {
			mask: ex_style,
			style: if set { ex_style } else { co::LVS_EX::NONE },
		});
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`ListView`](crate::gui::ListView) programmatically with
/// [`ListView::new`](crate::gui::ListView::new).
pub struct ListViewOpts {
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
	/// List view styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `LVS::REPORT | LVS::NOSORTHEADER | LVS::SHOWSELALWAYS | LVS::SHAREIMAGELISTS`.
	pub list_view_style: co::LVS,
	/// Extended list view styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `LVS_EX::NONE`.
	pub ex_list_view_style: co::LVS_EX,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::CLIENTEDGE`.
	pub ex_window_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: i32,
}

impl Default for ListViewOpts {
	fn default() -> Self {
		Self {
			position: POINT::new(0, 0),
			size: SIZE::new(50, 50),
			list_view_style: co::LVS::REPORT | co::LVS::NOSORTHEADER | co::LVS::SHOWSELALWAYS | co::LVS::SHAREIMAGELISTS,
			ex_list_view_style: co::LVS_EX::NONE,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			ex_window_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
		}
	}
}

impl ListViewOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
