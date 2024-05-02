use std::any::Any;
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

struct Obj<T> { // atual fields of ListView
	base: BaseNativeControl,
	events: ListViewEvents,
	context_menu: Option<HMENU>,
	_pin: PhantomPinned,
	_data: PhantomData<T>,
}

//------------------------------------------------------------------------------

/// Native
/// [list view](https://learn.microsoft.com/en-us/windows/win32/controls/list-view-controls-overview)
/// control. Not to be confused with the simpler [list box](crate::gui::ListBox)
/// control.
///
/// The generic parameter specifies the type of the object that will be embedded
/// on each item – if you don't want to store anything, just use `()` as the
/// type. Internally, this storage is implemented with pointers in the item's
/// `LPARAM` fields.
///
/// You can have access to the internal header of the list view by creating a
/// [`Header`](crate::gui::Header) object.
pub struct ListView<T: 'static = ()>(Pin<Arc<Obj<T>>>);

impl<T> Clone for ListView<T> { // https://stackoverflow.com/q/39415052/6923555
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

unsafe impl<T> Send for ListView<T> {}

impl<T> AsRef<BaseNativeControl> for ListView<T> {
	fn as_ref(&self) -> &BaseNativeControl {
		&self.0.base
	}
}

impl<T> GuiWindow for ListView<T> {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl<T> GuiChild for ListView<T> {
	fn ctrl_id(&self) -> u16 {
		self.0.base.ctrl_id()
	}
}

impl<T> GuiChildFocus for ListView<T> {}

impl<T> GuiNativeControl for ListView<T> {}

impl<T> GuiNativeControlEvents<ListViewEvents> for ListView<T> {
	fn on(&self) -> &ListViewEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl<T> ListView<T> {
	/// Instantiates a new `ListView` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `ListView` in an event closure.
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: ListViewOpts) -> Self {
		let opts = auto_ctrl_id_if_zero(opts);
		let ctrl_id = opts.ctrl_id;
		let context_menu = opts.context_menu.as_ref().map(|h| unsafe { h.raw_copy() });

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent, ctrl_id),
					events: ListViewEvents::new(parent, ctrl_id),
					context_menu,
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

	/// Instantiates a new `ListView` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// Since the image list is always managed by the control itself,
	/// [`LVS::SHAREIMAGELISTS`](crate::co::LVS::SHAREIMAGELISTS) style will
	/// always be added.
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `ListView` in an event closure.
	///
	/// Panics if the context menu, when specified, does not exist.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
		context_menu_id: Option<u16>,
	) -> Self
	{
		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent, ctrl_id),
					events: ListViewEvents::new(parent, ctrl_id),
					context_menu: context_menu_id.map(
						|id| HINSTANCE::NULL.LoadMenu(IdStr::Id(id)).unwrap()
							.GetSubMenu(0).unwrap(), // usually this is how it's set in the resources
					),
					_pin: PhantomPinned,
					_data: PhantomData,
				},
			),
		);

		let self2 = new_self.clone();
		parent.as_ref().before_user_on().wm(co::WM::INITDIALOG, move |_, _| {
			self2.create(OptsResz::Dlg(resize_behavior))?;
			Ok(WmRet::NotHandled)
		});

		new_self.default_message_handlers(parent.as_ref(), ctrl_id);
		new_self
	}

	fn create(&self, opts_resz: OptsResz<&ListViewOpts>) -> SysResult<()> {
		match opts_resz {
			OptsResz::Wnd(opts) => {
				let mut pos = POINT::new(opts.position.0, opts.position.1);
				let mut sz = SIZE::new(opts.size.0 as _, opts.size.1 as _);
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), Some(&mut sz))?;

				self.0.base.create_window(
					"SysListView32", None, pos, sz,
					opts.window_ex_style,
					opts.window_style | opts.list_view_style.into(),
				)?;

				if opts.list_view_ex_style != co::LVS_EX::NoValue {
					self.set_extended_style(true, opts.list_view_ex_style);
				}

				self.columns().add(&opts.columns);
			},
			OptsResz::Dlg(_) => self.0.base.create_dlg()?,
		}

		let style: co::LVS = self.hwnd().style().into();
		self.hwnd().set_style(style | co::LVS::SHAREIMAGELISTS);

		self.0.base.parent()
			.add_to_layout_arranger(self.hwnd(), opts_resz.resize_behavior())
	}

	fn default_message_handlers(&self, parent: &Base, ctrl_id: u16) {
		let self2 = self.clone();
		self.on_subclass().wm_get_dlg_code(move |p| {
			if !p.is_query && p.vkey_code == co::VK::RETURN {
				let mut nmlvkd = NMLVKEYDOWN::default();
				nmlvkd.hdr.hwndFrom = unsafe { self2.hwnd().raw_copy() };
				nmlvkd.hdr.set_idFrom(self2.ctrl_id());
				nmlvkd.hdr.code = co::LVN::KEYDOWN.into();
				nmlvkd.wVKey = co::VK::RETURN;

				unsafe {
					self2.hwnd()
						.GetAncestor(co::GA::PARENT)
						.unwrap()
						.SendMessage(wm::Notify { nmhdr: &mut nmlvkd.hdr }); // send Enter key to parent
				}
			}
			let dlgc_system = unsafe {
				self2.hwnd().DefSubclassProc::<wm::GetDlgCode>(p.into())
			};
			Ok(dlgc_system)
		});

		let self2 = self.clone();
		self.on_subclass().wm(co::WM::NOTIFY, move |p| {
			let wm_nfy = wm::Notify::from_generic_wm(p);
			if wm_nfy.nmhdr.code >= co::HDN::GETDISPINFO.into()
					&& wm_nfy.nmhdr.code <= co::HDN::BEGINDRAG.into() {
				unsafe {
					self2.hwnd()
						.GetAncestor(co::GA::PARENT)
						.unwrap()
						.SendMessage(wm_nfy); // forward HDN messages to parent
				}
			}
			Ok(WmRet::NotHandled) // HDN notifications still need to be processed by parent list view
		});

		let self2 = self.clone();
		parent.before_user_on().wm_notify(ctrl_id, co::LVN::KEYDOWN, move |p| {
			let lvnk = unsafe { p.cast_nmhdr::<NMLVKEYDOWN>() };
			let has_ctrl = GetAsyncKeyState(co::VK::CONTROL);
			let has_shift = GetAsyncKeyState(co::VK::SHIFT);

			if has_ctrl && lvnk.wVKey == co::VK::CHAR_A { // Ctrl+A
				self2.items().select_all(true);
			} else if lvnk.wVKey == co::VK::APPS { // context menu key
				self2.show_context_menu(false, has_ctrl, has_shift);
			}
			Ok(WmRet::HandledOk)
		});

		let self2 = self.clone();
		parent.before_user_on().wm_notify(ctrl_id, co::NM::RCLICK, move |p| {
			let nmia = unsafe { p.cast_nmhdr::<NMITEMACTIVATE>() };
			let has_ctrl = nmia.uKeyFlags.has(co::LVKF::CONTROL);
			let has_shift = nmia.uKeyFlags.has(co::LVKF::SHIFT);

			self2.show_context_menu(true, has_ctrl, has_shift);
			Ok(WmRet::HandledOk)
		});

		let self2 = self.clone();
		parent.after_user_on().wm_notify(ctrl_id, co::LVN::DELETEITEM, move |p| {
			let nmlv = unsafe { p.cast_nmhdr::<NMLISTVIEW>() };
			self2.items()
				.get(nmlv.iItem as _)
				.data_lparam()
				.map(|pdata| {
					let _ = unsafe { Rc::from_raw(pdata) }; // free allocated LPARAM, if any
				});
			Ok(WmRet::HandledOk)
		});

		let self2 = self.clone();
		parent.after_user_on().wm(co::WM::DESTROY, move |_, _| {
			[co::LVSIL::NORMAL, co::LVSIL::SMALL, co::LVSIL::STATE, co::LVSIL::GROUPHEADER]
				.iter()
				.for_each(|lvsil| {
					self2.image_list(*lvsil).map(|hil| { // destroy each image list, if any
						let _ = unsafe { ImageListDestroyGuard::new(hil.raw_copy()) };
					});
				});
			Ok(WmRet::NotHandled)
		});
	}

	/// Exposes the column methods.
	#[must_use]
	pub const fn columns(&self) -> ListViewColumns<'_, T> {
		ListViewColumns::new(self)
	}

	/// Returns the context menu attached to this list view, if any.
	///
	/// The context menu is attached when the list view is created, either by
	/// calling [`ListView::new`](crate::gui::ListView::new) or
	/// [`ListView::new_dlg`](crate::gui::ListView::new_dlg).
	#[must_use]
	pub fn context_menu(&self) -> Option<&HMENU> {
		self.0.context_menu.as_ref()
	}

	/// Retrieves a reference to one of the associated image lists by sending an
	/// [`lvm::GetImageList`](crate::msg::lvm::GetImageList) message.
	///
	/// The image list is owned by the control.
	#[must_use]
	pub fn image_list(&self, kind: co::LVSIL) -> Option<&HIMAGELIST> {
		unsafe {
			self.hwnd()
				.SendMessage(lvm::GetImageList { kind })
		}.map(|hil| {
			let hil_ptr = &hil as *const HIMAGELIST;
			unsafe { &*hil_ptr }
		})
	}

	/// Exposes the item methods.
	#[must_use]
	pub const fn items(&self) -> ListViewItems<'_, T> {
		ListViewItems::new(self)
	}

	/// Retrieves the current view by sending an
	/// [`lvm::GetView`](crate::msg::lvm::GetView) message.
	#[must_use]
	pub fn current_view(&self) -> co::LV_VIEW {
		unsafe { self.hwnd().SendMessage(lvm::GetView {}) }
	}

	/// Sets the current view by sending an
	/// [`lvm::SetView`](crate::msg::lvm::SetView) message.
	pub fn set_current_view(&self, view: co::LV_VIEW) {
		unsafe {
			self.hwnd()
			.SendMessage(lvm::SetView { view })
		}.unwrap();
	}

	/// Sets or unsets the given extended list view styles by sending an
	/// [`lvm::SetExtendedListViewStyle`](crate::msg::lvm::SetExtendedListViewStyle)
	/// message.
	pub fn set_extended_style(&self, set: bool, ex_style: co::LVS_EX) {
		unsafe {
			self.hwnd().SendMessage(lvm::SetExtendedListViewStyle {
				mask: ex_style,
				style: if set { ex_style } else { co::LVS_EX::NoValue },
			});
		}
	}

	/// Sets the one of the associated image lists by sending an
	/// [`lvm::SetImageList`](crate::msg::lvm::SetImageList) message.
	///
	/// The image list will be owned by the control. Returns the previous one,
	/// if any.
	pub fn set_image_list(&self,
		kind: co::LVSIL,
		himagelist: ImageListDestroyGuard,
	) -> Option<ImageListDestroyGuard>
	{
		let mut himagelist = himagelist;
		let hil = himagelist.leak();

		unsafe {
			self.hwnd()
				.SendMessage(lvm::SetImageList { kind, himagelist: Some(hil) })
				.map(|prev_hil| ImageListDestroyGuard::new(prev_hil))
		}
	}

	/// Allows or disallows the redrawing of the control by sending a
	/// [`wm::SetRedraw`](crate::msg::wm::SetRedraw) message.
	pub fn set_redraw(&self, can_redraw: bool) {
		unsafe {
			self.hwnd()
				.SendMessage(wm::SetRedraw { can_redraw });
		}
	}

	fn show_context_menu(&self,
		follow_cursor: bool,
		has_ctrl: bool,
		has_shift: bool,
	) {
		let hmenu = match self.0.context_menu.as_ref() {
			Some(h) => h,
			None => return, // no menu, nothing to do
		};

		let menu_pos = if follow_cursor { // usually when fired by a right-click
			let mut menu_pos = GetCursorPos().unwrap(); // relative to screen
			self.hwnd().ScreenToClient(&mut menu_pos).unwrap(); // now relative to list view

			match self.items().hit_test(menu_pos) {
				Some(item_over) => {
					if !has_ctrl && !has_shift {
						item_over.select(true); // if not yet
						item_over.focus();
					}
				},
				None => self.items().select_all(false), // no item was right-clicked
			}

			self.hwnd().SetFocus(); // because a right-click won't set the focus by itself
			menu_pos

		} else { // usually fired by the context meny key
			let focused_opt = self.items().focused();

			if focused_opt.is_some() && focused_opt.unwrap().is_visible() {
				let focused = focused_opt.unwrap();
				let rc_item = focused.rect(co::LVIR::BOUNDS);
				POINT::new(rc_item.left + 16,
					rc_item.top + (rc_item.bottom - rc_item.top) / 2)

			} else { // no item is focused and visible
				POINT::new(6, 10) // arbitrary coordinates
			}
		};

		hmenu.track_popup_menu_at_point(
			menu_pos, &self.hwnd().GetParent().unwrap(), self.hwnd())
			.unwrap();
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`ListView`](crate::gui::ListView) programmatically with
/// [`ListView::new`](crate::gui::ListView::new).
pub struct ListViewOpts {
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
	/// List view styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Since the image list is always managed by the control itself,
	/// [`LVS::SHAREIMAGELISTS`](crate::co::LVS::SHAREIMAGELISTS) style will
	/// always be added.
	///
	/// Defaults to `LVS::REPORT | LVS::NOSORTHEADER | LVS::SHOWSELALWAYS`.
	pub list_view_style: co::LVS,
	/// Extended list view styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `LVS_EX::NoValue`.
	pub list_view_ex_style: co::LVS_EX,
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

	/// Context popup menu.
	///
	/// This menu is shared: it must be destroyed manually after the control is
	/// destroyed. But note that menus loaded from resources don't need to be
	/// destroyed.
	///
	/// Defaults to `None`.
	pub context_menu: Option<HMENU>,
	/// Text and width of columns to be added right away. The columns only show
	/// in report mode.
	///
	/// Defaults to none.
	pub columns: Vec<(String, u32)>,
}

impl Default for ListViewOpts {
	fn default() -> Self {
		Self {
			position: (0, 0),
			size: (50, 50),
			list_view_style: co::LVS::REPORT | co::LVS::NOSORTHEADER | co::LVS::SHOWSELALWAYS,
			list_view_ex_style: co::LVS_EX::NoValue,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			context_menu: None,
			columns: Vec::<(String, u32)>::new(),
		}
	}
}

impl ResizeBehavior for &ListViewOpts {
	fn resize_behavior(&self) -> (Horz, Vert) {
		self.resize_behavior
	}
}

impl AutoCtrlId for ListViewOpts {
	fn ctrl_id_mut(&mut self) -> &mut u16 {
		&mut self.ctrl_id
	}
}
