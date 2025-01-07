use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::marker::{PhantomData, PhantomPinned};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::gui::{*, events::*, privs::*, spec::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj<T> { // actual fields of TreeView
	base: BaseNativeControl,
	events: TreeViewEvents,
	_pin: PhantomPinned,
	_data: PhantomData<T>,
}

//------------------------------------------------------------------------------

/// Native
/// [tree view](https://learn.microsoft.com/en-us/windows/win32/controls/tree-view-controls)
/// control.
///
/// The generic parameter specifies the type of the object that will be embedded
/// on each item – if you don't want to store anything, just use `()` as the
/// type. Internally, this storage is implemented with pointers in the item's
/// `LPARAM` fields.
pub struct TreeView<T: 'static = ()>(Pin<Arc<Obj<T>>>);

impl<T> Clone for TreeView<T> { // https://stackoverflow.com/q/39415052/6923555
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

unsafe impl<T> Send for TreeView<T> {}

impl<T> AsRef<BaseNativeControl> for TreeView<T> {
	fn as_ref(&self) -> &BaseNativeControl {
		&self.0.base
	}
}

impl<T> GuiWindow for TreeView<T> {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl<T> GuiChild for TreeView<T> {
	fn ctrl_id(&self) -> u16 {
		self.0.base.ctrl_id()
	}
}

impl<T> GuiChildFocus for TreeView<T> {}

impl<T> GuiNativeControl for TreeView<T> {}

impl<T> GuiNativeControlEvents<TreeViewEvents> for TreeView<T> {
	fn on(&self) -> &TreeViewEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
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
	pub fn new(parent: &impl GuiParent, opts: TreeViewOpts) -> Self {
		let opts = auto_ctrl_id_if_zero(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent, ctrl_id),
					events: TreeViewEvents::new(parent, ctrl_id),
					_pin: PhantomPinned,
					_data: PhantomData,
				},
			),
		);

		let self2 = new_self.clone();
		parent.as_ref().before_user_on().wm_create_or_initdialog(move |_, _| {
			self2.create(OptsResz::Wnd(&opts))?;
			Ok(WmRet::NotHandled)
		});

		new_self.default_message_handlers(parent.as_ref(), ctrl_id);
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
		resize_behavior: (Horz, Vert),
	) -> Self
	{
		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent, ctrl_id),
					events: TreeViewEvents::new(parent, ctrl_id),
					_pin: PhantomPinned,
					_data: PhantomData,
				},
			),
		);

		let self2 = new_self.clone();
		parent.as_ref().before_user_on().wm_init_dialog(move |_| {
			self2.create(OptsResz::Dlg(resize_behavior))?;
			Ok(false) // this return value is discarded
		});

		new_self.default_message_handlers(parent.as_ref(), ctrl_id);
		new_self
	}

	fn create(&self, opts_resz: OptsResz<&TreeViewOpts>) -> SysResult<()> {
		match opts_resz {
			OptsResz::Wnd(opts) => {
				let mut pos = POINT::new(opts.position.0, opts.position.1);
				let mut sz = SIZE::new(opts.size.0 as _, opts.size.1 as _);
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), Some(&mut sz))?;

				self.0.base.create_window( // may panic
					"SysTreeView32", None, pos, sz,
					opts.window_ex_style,
					opts.window_style | opts.tree_view_style.into(),
				)?;

				if opts.tree_view_ex_style != co::TVS_EX::NoValue {
					self.set_extended_style(true, opts.tree_view_ex_style);
				}
			},
			OptsResz::Dlg(_) => self.0.base.create_dlg()?,
		}

		self.0.base.parent()
			.add_to_layout_arranger(self.hwnd(), opts_resz.resize_behavior())
	}

	fn default_message_handlers(&self, parent: &Base, ctrl_id: u16) {
		let self2 = self.clone();
		parent.after_user_on().wm_notify(ctrl_id, co::TVN::DELETEITEM, move |p| {
			let nmtv = unsafe { p.cast_nmhdr::<NMTREEVIEW>() };
			self2.items()
				.get(&nmtv.itemOld.hItem)
				.data_lparam()
				.map(|pdata| {
					let _ = unsafe { Rc::from_raw(pdata) }; // free allocated LPARAM, if any
				});
			Ok(WmRet::HandledOk)
		});

		let self2 = self.clone();
		parent.after_user_on().wm_destroy(move || {
			[co::TVSIL::NORMAL, co::TVSIL::STATE]
				.iter()
				.for_each(|tvsil| {
					self2.image_list(*tvsil).map(|hil| { // destroy each image list, if any
						let _ = unsafe { ImageListDestroyGuard::new(hil.raw_copy()) };
					});
				});
			Ok(())
		});
	}

	pub(in crate::gui) fn raw_insert_item(&self,
		hparent: Option<&HTREEITEM>,
		text: &str,
		icon_index: Option<u32>,
		data: T,
	) -> TreeViewItem<'_, T>
	{
		let mut tvix = TVITEMEX::default();
		tvix.mask = co::TVIF::TEXT;

		let mut buf = WString::from_str(text);
		tvix.set_pszText(Some(&mut buf));

		if let Some(icon_index) = icon_index {
			tvix.mask |= co::TVIF::IMAGE;
			tvix.iImage = icon_index as _;
		}

		if TypeId::of::<T>() != TypeId::of::<()>() { // user defined an actual type?
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
				.SendMessage(tvm::InsertItem { item: &mut tvis })
		}.unwrap();

		TreeViewItem::new(self, new_hitem)
	}

	/// Retrieves a reference to one of the associated image lists by sending a
	/// [`tvm::GetImageList`](crate::msg::tvm::GetImageList) message.
	///
	/// The image list is owned by the control.
	#[must_use]
	pub fn image_list(&self, kind: co::TVSIL) -> Option<&HIMAGELIST> {
		unsafe {
			self.hwnd()
				.SendMessage(tvm::GetImageList { kind })
		}.map(|hil| {
			let hil_ptr = &hil as *const HIMAGELIST;
			unsafe { &*hil_ptr }
		})
	}

	/// Exposes the item methods.
	#[must_use]
	pub const fn items(&self) -> TreeViewItems<'_, T> {
		TreeViewItems::new(self)
	}

	/// Sets or unsets the given extended list view styles by sending a
	/// [`tvm::SetExtendedStyle`](crate::msg::tvm::SetExtendedStyle) message.
	pub fn set_extended_style(&self, set: bool, ex_style: co::TVS_EX) {
		unsafe {
			self.hwnd()
				.SendMessage(tvm::SetExtendedStyle {
					mask: ex_style,
					style: if set { ex_style } else { co::TVS_EX::NoValue },
				})
		}.unwrap();
	}

	/// Sets the one of the associated image lists by sending a
	/// [`tvm::SetImageList`](crate::msg::tvm::SetImageList) message.
	///
	/// The image list will be owned by the control. Returns the previous one,
	/// if any.
	pub fn set_image_list(&self,
		kind: co::TVSIL,
		himagelist: ImageListDestroyGuard,
	) -> Option<ImageListDestroyGuard>
	{
		let mut himagelist = himagelist;
		let hil = himagelist.leak();

		unsafe {
			self.hwnd()
				.SendMessage(tvm::SetImageList { kind, himagelist: Some(hil) })
				.map(|prev_hil| ImageListDestroyGuard::new(prev_hil))
		}
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`TreeView`](crate::gui::TreeView) programmatically with
/// [`TreeView::new`](crate::gui::TreeView::new).
pub struct TreeViewOpts {
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
	/// Horizontal and vertical behavior of the control when the parent window
	/// is resized.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),
}

impl Default for TreeViewOpts {
	fn default() -> Self {
		Self {
			position: (0, 0),
			size: (50, 50),
			tree_view_style: co::TVS::HASLINES | co::TVS::LINESATROOT | co::TVS::SHOWSELALWAYS | co::TVS::HASBUTTONS,
			tree_view_ex_style: co::TVS_EX::NoValue,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
		}
	}
}

impl ResizeBehavior for &TreeViewOpts {
	fn resize_behavior(&self) -> (Horz, Vert) {
		self.resize_behavior
	}
}

impl AutoCtrlId for TreeViewOpts {
	fn ctrl_id_mut(&mut self) -> &mut u16 {
		&mut self.ctrl_id
	}
}
