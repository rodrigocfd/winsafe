use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::marker::{PhantomData, PhantomPinned};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::gui::{collections::*, events::*, privs::*, *};
use crate::msg::*;
use crate::prelude::*;

struct TreeViewObj<T> {
	base: BaseCtrl,
	events: TreeViewEvents,
	_pin: PhantomPinned,
	_data: PhantomData<T>,
}

native_ctrl! { TreeView: TreeViewObj<T>, T => TreeViewEvents;
	/// Native
	/// [tree view](https://learn.microsoft.com/en-us/windows/win32/controls/tree-view-controls)
	/// control.
	///
	/// The generic parameter specifies the type of the object that will be
	/// embedded on each item – if you don't want to store anything, just use
	/// `()` as the type. Internally, this storage is implemented with pointers
	/// in the item's `LPARAM` fields.
}

impl<T> TreeView<T> {
	/// Instantiates a new `TreeView` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `TreeView` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: TreeViewOpts) -> Self {
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(Arc::pin(TreeViewObj {
			base: BaseCtrl::new(ctrl_id),
			events: TreeViewEvents::new(parent, ctrl_id),
			_pin: PhantomPinned,
			_data: PhantomData,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent
			.as_ref()
			.before_on()
			.wm(parent.as_ref().wnd_ty().creation_msg(), move |_| {
				self2.0.base.create_window(
					opts.window_ex_style,
					"SysTreeView32",
					None,
					opts.window_style | opts.control_style.into(),
					opts.position.into(),
					opts.size.into(),
					&parent2,
				)?;
				parent2
					.as_ref()
					.add_to_layout(self2.hwnd(), opts.resize_behavior)?;
				Ok(0) // ignored
			});

		new_self.default_message_handlers(parent);
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
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self {
		let new_self = Self(Arc::pin(TreeViewObj {
			base: BaseCtrl::new(ctrl_id),
			events: TreeViewEvents::new(parent, ctrl_id),
			_pin: PhantomPinned,
			_data: PhantomData,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent.as_ref().before_on().wm_init_dialog(move |_| {
			self2.0.base.assign_dlg(&parent2)?;
			parent2
				.as_ref()
				.add_to_layout(self2.hwnd(), resize_behavior)?;
			Ok(true) // ignored
		});

		new_self.default_message_handlers(parent);
		new_self
	}

	fn default_message_handlers(&self, parent: &impl AsRef<BaseWnd>) {
		let self2 = self.clone();
		parent
			.as_ref()
			.after_on()
			.wm_notify(self.ctrl_id(), co::TVN::DELETEITEM, move |p| {
				let nmtv = unsafe { p.cast_nmhdr::<NMTREEVIEW>() };
				self2
					.items()
					.get(&nmtv.itemOld.hItem)
					.data_lparam()?
					.map(|pdata| {
						let _ = unsafe { Rc::from_raw(pdata) }; // free allocated LPARAM, if any
					});
				Ok(0) // ignored
			});

		let self2 = self.clone();
		parent.as_ref().after_on().wm_destroy(move || {
			[co::TVSIL::NORMAL, co::TVSIL::STATE]
				.into_iter()
				.for_each(|kind| unsafe {
					self2
						.hwnd()
						.SendMessage(tvm::GetImageList { kind })
						.map(|h| {
							self2
								.hwnd()
								.SendMessage(tvm::SetImageList { himagelist: None, kind }); // remove from control
							let _ = ImageListDestroyGuard::new(h); // destroy
						});
				});
			Ok(())
		});
	}

	pub(in crate::gui) fn raw_insert_item(
		&self,
		hparent: Option<&HTREEITEM>,
		text: &str,
		icon_index: Option<u32>,
		data: T,
	) -> SysResult<TreeViewItem<'_, T>> {
		let mut tvix = TVITEMEX::default();
		tvix.mask = co::TVIF::TEXT;

		let mut buf = WString::from_str(text);
		tvix.set_pszText(Some(&mut buf));

		if let Some(icon_index) = icon_index {
			tvix.mask |= co::TVIF::IMAGE;
			tvix.iImage = icon_index as _;
		}

		// User defined an actual type?
		if TypeId::of::<T>() != TypeId::of::<()>() {
			tvix.mask |= co::TVIF::PARAM;
			let rc_data = Rc::new(RefCell::new(data));
			tvix.lParam = Rc::into_raw(rc_data) as _;
		}

		let mut tvis = TVINSERTSTRUCT::default();
		if let Some(hparent) = hparent {
			tvis.hParent = unsafe { hparent.raw_copy() };
		}

		tvis.set_hInsertAfter(TreeitemTvi::Tvi(co::TVI::LAST));
		tvis.itemex = tvix;

		let new_hitem = unsafe {
			self.hwnd()
				.SendMessage(tvm::InsertItem { item: &mut tvis })?
		};

		Ok(TreeViewItem::new(self, new_hitem))
	}

	/// Retrieves one of the associated image lists by sending a
	/// [`tvm::GetImageList`](crate::msg::tvm::GetImageList) message.
	///
	/// Image lists are lazy-initialized: the first time you call this method
	/// for a given image list, it will be created and assigned with
	/// [`tvm::SetImageList`](crate::msg::tvm::SetImageList).
	///
	/// The image list is owned by the control.
	#[must_use]
	pub fn image_list(&self, kind: co::TVSIL) -> HrResult<HIMAGELIST> {
		match unsafe { self.hwnd().SendMessage(tvm::GetImageList { kind }) } {
			Some(h) => Ok(h), // already created
			None => {
				// Not created yet. Create a new image list and assign it to the list view.
				let h = HIMAGELIST::Create(SIZE::new(16, 16), co::ILC::COLOR32, 1, 1)?.leak();
				unsafe {
					self.hwnd()
						.SendMessage(tvm::SetImageList { himagelist: Some(h.raw_copy()), kind });
				}
				Ok(h)
			},
		}
	}

	/// Exposes the item methods.
	#[must_use]
	pub const fn items(&self) -> TreeViewItems<'_, T> {
		TreeViewItems::new(self)
	}

	/// Sets or unsets the given extended list view styles by sending a
	/// [`tvm::SetExtendedStyle`](crate::msg::tvm::SetExtendedStyle) message.
	pub fn set_extended_style(&self, set: bool, ex_style: co::TVS_EX) -> HrResult<()> {
		unsafe {
			self.hwnd().SendMessage(tvm::SetExtendedStyle {
				mask: ex_style,
				style: if set { ex_style } else { co::TVS_EX::NoValue },
			})
		}
	}
}

/// Options to create a [`TreeView`](crate::gui::TreeView) programmatically with
/// [`TreeView::new`](crate::gui::TreeView::new).
pub struct TreeViewOpts {
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
	/// Tree view styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TVS::HASLINES | TVS::LINESATROOT | TVS::SHOWSELALWAYS | TVS::HASBUTTONS`.
	pub control_style: co::TVS,
	/// Extended tree view styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TVS_EX::NoValue`.
	pub contorl_ex_style: co::TVS_EX,
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
}

impl Default for TreeViewOpts {
	fn default() -> Self {
		Self {
			position: dpi(0, 0),
			size: dpi(120, 120),
			control_style: co::TVS::HASLINES
				| co::TVS::LINESATROOT
				| co::TVS::SHOWSELALWAYS
				| co::TVS::HASBUTTONS,
			contorl_ex_style: co::TVS_EX::NoValue,
			window_style: co::WS::CHILD | co::WS::GROUP | co::WS::TABSTOP | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
		}
	}
}
