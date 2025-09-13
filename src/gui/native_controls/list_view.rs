use std::any::Any;
use std::cell::UnsafeCell;
use std::marker::{PhantomData, PhantomPinned};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::gui::{collections::*, privs::*, *};
use crate::msg::*;
use crate::prelude::*;

struct ListViewObj<T> {
	base: BaseCtrl,
	events: BaseCtrlEvents,
	context_menu: Option<DestroyMenuGuard>, // the context menu itself is the 1st submenu
	header: UnsafeCell<Option<Header>>, // if doesn't exist, will be set to None on WM_CREATE and WM_INITDIALOG
	_pin: PhantomPinned,
	_data: PhantomData<T>,
}

native_ctrl! { ListView: ListViewObj<T>, T => GuiEventsListView;
	/// Native
	/// [list view](https://learn.microsoft.com/en-us/windows/win32/controls/list-view-controls-overview)
	/// control. Not to be confused with the simpler
	/// [list box](crate::gui::ListBox) control.
	///
	/// The generic parameter specifies the type of the object that will be
	/// embedded on each item – if you don't want to store anything, just use
	/// `()` as the type. Internally, this storage is implemented with pointers
	/// in the item's `LPARAM` fields.
	///
	/// You can have access to the internal header of the list view by creating
	/// a [`Header`](crate::gui::Header) object.
}

impl<T> ListView<T> {
	/// Instantiates a new `ListView` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `ListView` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: ListViewOpts) -> Self {
		let mut opts = opts;
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let context_menu = opts.context_menu.take();

		let new_self = Self(Arc::pin(ListViewObj {
			base: BaseCtrl::new(ctrl_id),
			events: BaseCtrlEvents::new(parent, ctrl_id),
			context_menu,
			header: UnsafeCell::new(Some(Header::from_list_view(parent))), // initially does exist
			_pin: PhantomPinned,
			_data: PhantomData,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		let columns2 = opts.owned_columns();
		parent
			.as_ref()
			.before_on()
			.wm(parent.as_ref().wnd_ty().creation_msg(), move |_| {
				self2.0.base.create_window(
					opts.window_ex_style,
					"SysListView32",
					None,
					opts.window_style | (opts.control_style & !co::LVS::SHAREIMAGELISTS).into(),
					opts.position.into(),
					opts.size.into(),
					&parent2,
				);
				if opts.control_ex_style != co::LVS_EX::NoValue {
					self2.set_extended_style(true, opts.control_ex_style);
				}
				if !unsafe { &*self2.0.header.get() }
					.as_ref()
					.unwrap()
					.init_nested(self2.hwnd())
				{
					*unsafe { &mut *self2.0.header.get() } = None; // no header, delete it
				}
				for (text, cx) in columns2.iter() {
					self2.cols().add(text, *cx)?;
				}
				parent2
					.as_ref()
					.add_to_layout(self2.hwnd(), opts.resize_behavior);
				Ok(0) // ignored
			});

		new_self.default_message_handlers(parent);
		new_self
	}

	/// Instantiates a new `ListView` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// The `context_menu_id` must point to the root menu whose first submenu
	/// will be effectively displayed as the context menu.
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
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
		context_menu_id: Option<u16>,
	) -> Self {
		let new_self = Self(Arc::pin(ListViewObj {
			base: BaseCtrl::new(ctrl_id),
			events: BaseCtrlEvents::new(parent, ctrl_id),
			context_menu: context_menu_id.map(|id| {
				parent
					.hwnd()
					.hinstance()
					.LoadMenu(IdStr::Id(id))
					.expect("Invalid ListView context menu ID")
			}),
			header: UnsafeCell::new(Some(Header::from_list_view(parent))), // initially does exist
			_pin: PhantomPinned,
			_data: PhantomData,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent.as_ref().before_on().wm_init_dialog(move |_| {
			self2.0.base.assign_dlg(&parent2);
			self2
				.hwnd()
				.set_style(co::LVS::from(self2.hwnd().style()) & !co::LVS::SHAREIMAGELISTS);
			if !unsafe { &*self2.0.header.get() }
				.as_ref()
				.unwrap()
				.init_nested(self2.hwnd())
			{
				*unsafe { &mut *self2.0.header.get() } = None; // no header, delete it
			}
			parent2
				.as_ref()
				.add_to_layout(self2.hwnd(), resize_behavior);
			Ok(true) // ignored
		});

		new_self.default_message_handlers(parent);
		new_self
	}

	fn default_message_handlers(&self, parent: &impl AsRef<BaseWnd>) {
		let self2 = self.clone();
		self.on_subclass().wm_get_dlg_code(move |p| {
			if !p.is_query && p.vkey_code == co::VK::RETURN {
				let mut nmlvkd = NMLVKEYDOWN::default();
				nmlvkd.hdr.hwndFrom = unsafe { self2.hwnd().raw_copy() };
				nmlvkd.hdr.set_idFrom(self2.ctrl_id());
				nmlvkd.hdr.code = co::LVN::KEYDOWN.into();
				nmlvkd.wVKey = co::VK::RETURN;

				let hparent = self2.hwnd().GetAncestor(co::GA::PARENT).unwrap();
				unsafe {
					hparent.SendMessage(wm::Notify { nmhdr: &mut nmlvkd.hdr }); // forward Enter key to parent
				}
			}
			let dlgc_system = unsafe { self2.hwnd().DefSubclassProc::<wm::GetDlgCode>(p.into()) };
			Ok(dlgc_system)
		});

		let self2 = self.clone();
		parent
			.as_ref()
			.before_on()
			.wm_notify(self.ctrl_id(), co::LVN::KEYDOWN, move |p| {
				let lvnk = unsafe { p.cast_nmhdr::<NMLVKEYDOWN>() };
				let has_ctrl = GetAsyncKeyState(co::VK::CONTROL);
				let has_shift = GetAsyncKeyState(co::VK::SHIFT);

				if has_ctrl && lvnk.wVKey == co::VK::CHAR_A {
					self2.items().select_all(true).expect(DONTFAIL); // Ctrl+A
				} else if lvnk.wVKey == co::VK::APPS {
					self2.show_context_menu(false, has_ctrl, has_shift); // context menu key
				}
				Ok(0) // ignored
			});

		let self2 = self.clone();
		parent
			.as_ref()
			.before_on()
			.wm_notify(self.ctrl_id(), co::NM::RCLICK, move |p| {
				let nmia = unsafe { p.cast_nmhdr::<NMITEMACTIVATE>() };
				let has_ctrl = nmia.uKeyFlags.has(co::LVKF::CONTROL);
				let has_shift = nmia.uKeyFlags.has(co::LVKF::SHIFT);

				self2.show_context_menu(true, has_ctrl, has_shift);
				Ok(0) // ignored
			});

		let self2 = self.clone();
		parent
			.as_ref()
			.after_on()
			.wm_notify(self.ctrl_id(), co::LVN::DELETEITEM, move |p| {
				let nmlv = unsafe { p.cast_nmhdr::<NMLISTVIEW>() };
				let rc_ptr = self2
					.items()
					.get(nmlv.iItem as _)
					.data_lparam()
					.expect(DONTFAIL);

				if !rc_ptr.is_null() {
					let _ = unsafe { Rc::from_raw(rc_ptr) }; // free allocated LPARAM
				}
				Ok(0) // ignored
			});
	}

	fn show_context_menu(&self, follow_cursor: bool, has_ctrl: bool, has_shift: bool) {
		let hmenu = match self.context_menu() {
			Some(h) => h,
			None => return, // no menu, nothing to do
		};

		let menu_pos = if follow_cursor {
			// Usually when fired by a right-click.
			let menu_pos = self
				.hwnd()
				.ScreenToClient(GetCursorPos().expect(DONTFAIL)) // relative to screen
				.expect(DONTFAIL); // now relative to list view

			match self.items().hit_test(menu_pos) {
				Some(item_over) => {
					if !has_ctrl && !has_shift {
						item_over.select(true).expect(DONTFAIL); // if not yet
						item_over.focus().expect(DONTFAIL);
					}
				},
				None => self.items().select_all(false).expect(DONTFAIL), // no item was right-clicked
			}

			self.focus().expect(DONTFAIL); // because a right-click won't set the focus by itself
			menu_pos
		} else {
			// Usually fired by the context menu key.
			let focused_opt = self.items().focused();

			if focused_opt.is_some() && focused_opt.unwrap().is_visible() {
				let focused = focused_opt.unwrap();
				let rc_item = focused.rect(co::LVIR::BOUNDS).expect(DONTFAIL);
				POINT::with(rc_item.left + 16, rc_item.top + (rc_item.bottom - rc_item.top) / 2)
			} else {
				// No item is focused and visible.
				POINT::with(6, 10) // arbitrary coordinates
			}
		};

		hmenu
			.track_popup_menu_at_point(
				menu_pos,
				&self.hwnd().GetParent().expect(DONTFAIL),
				self.hwnd(),
			)
			.expect(DONTFAIL);
	}

	/// Column methods.
	#[must_use]
	pub const fn cols(&self) -> ListViewCols<'_, T> {
		ListViewCols::new(self)
	}

	/// Returns a handle to the first submenu of the context menu owned by the
	/// list view, if any.
	///
	/// The first submenu is the one to be effectively displayed by the control.
	#[must_use]
	pub fn context_menu(&self) -> Option<HMENU> {
		self.0
			.context_menu
			.as_ref()
			.map(|hmenu| hmenu.GetSubMenu(0).unwrap())
	}

	/// Retrieves the current view by sending an
	/// [`lvm::GetView`](crate::msg::lvm::GetView) message.
	#[must_use]
	pub fn current_view(&self) -> co::LV_VIEW {
		unsafe { self.hwnd().SendMessage(lvm::GetView {}) }
	}

	/// Returns the embedded [`Header`](crate::gui::Header) of the list view, if
	/// any.
	///
	/// The `Header` is tried to be initialized during the internal
	/// [`wm::Create`](crate::msg::wm::Create) and
	/// [`wm::InitDialog`](crate::msg::wm::InitDialog) processing. If the list
	/// view doesn't happen to have a header – only report view lists have
	/// headers –, the internal `Header` is removed, and this method will
	/// forever return `None`.
	///
	/// This means you may call `header()` to add events to the internal
	/// `Header`, but if the list view has no header, these events will simply be
	/// discarded.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_list: gui::ListView; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_list = gui::ListView::<()>::new(&wnd, gui::ListViewOpts::default());
	///
	/// let my_list2 = my_list.clone();
	/// my_list.header().unwrap().on().hdn_item_click(move |p| {
	///     let h = my_list2.header().unwrap().items().get(p.iItem as _);
	///     println!("Col: {}", h.text());
	///     Ok(())
	/// });
	/// ```
	#[must_use]
	pub fn header(&self) -> Option<&Header> {
		unsafe { &*self.0.header.get() }.as_ref()
	}

	/// Retrieves one of the associated image lists by sending an
	/// [`lvm::GetImageList`](crate::msg::lvm::GetImageList) message.
	///
	/// Image lists are lazy-initialized: the first time you call this method
	/// for a given image list, it will be created and assigned with
	/// [`lvm::SetImageList`](crate::msg::lvm::SetImageList).
	///
	/// The image list is owned by the control.
	#[must_use]
	pub fn image_list(&self, kind: co::LVSIL) -> HrResult<HIMAGELIST> {
		match unsafe { self.hwnd().SendMessage(lvm::GetImageList { kind }) } {
			Some(h) => Ok(h), // already created
			None => {
				// Not created yet. Create a new image list and assign it to the list view.
				let sz = match kind {
					co::LVSIL::NORMAL => SIZE::with(32, 32),
					_ => SIZE::with(16, 16),
				};
				let h = HIMAGELIST::Create(sz, co::ILC::COLOR32, 1, 1)?.leak();
				unsafe {
					self.hwnd()
						.SendMessage(lvm::SetImageList { himagelist: Some(h.raw_copy()), kind });
				}
				Ok(h)
			},
		}
	}

	/// Item methods.
	#[must_use]
	pub const fn items(&self) -> ListViewItems<'_, T> {
		ListViewItems::new(self)
	}

	/// Sets the current view by sending an
	/// [`lvm::SetView`](crate::msg::lvm::SetView) message.
	pub fn set_current_view(&self, view: co::LV_VIEW) -> SysResult<()> {
		unsafe { self.hwnd().SendMessage(lvm::SetView { view }) }
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

	/// Allows or disallows the redrawing of the control by sending a
	/// [`wm::SetRedraw`](crate::msg::wm::SetRedraw) message.
	pub fn set_redraw(&self, can_redraw: bool) {
		unsafe {
			self.hwnd().SendMessage(wm::SetRedraw { can_redraw });
		}
	}
}

/// Options to create a [`ListView`](crate::gui::ListView) programmatically with
/// [`ListView::new`](crate::gui::ListView::new).
pub struct ListViewOpts<'a> {
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
	/// List view styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Since the image list is always managed by the control itself,
	/// [`LVS::SHAREIMAGELISTS`](crate::co::LVS::SHAREIMAGELISTS) style will
	/// always be unset.
	///
	/// Defaults to `LVS::REPORT | LVS::NOSORTHEADER | LVS::SHOWSELALWAYS`.
	pub control_style: co::LVS,
	/// Extended list view styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `LVS_EX::FULLROWSELECT`.
	pub control_ex_style: co::LVS_EX,
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

	/// Context popup menu.
	///
	/// This menu will be owned by the control, which will automatically destroy
	/// it.
	///
	/// The first submenu is the one which will be effectively displayed as the
	/// context menu.
	///
	/// Defaults to `None`.
	pub context_menu: Option<DestroyMenuGuard>,
	/// Text and width of columns to be added. The columns only show in report
	/// mode.
	///
	/// Defaults to none.
	pub columns: &'a [(&'a str, i32)],
}

impl<'a> Default for ListViewOpts<'a> {
	fn default() -> Self {
		Self {
			position: dpi(0, 0),
			size: dpi(120, 120),
			control_style: co::LVS::REPORT | co::LVS::NOSORTHEADER | co::LVS::SHOWSELALWAYS,
			control_ex_style: co::LVS_EX::FULLROWSELECT,
			window_style: co::WS::CHILD | co::WS::GROUP | co::WS::TABSTOP | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			context_menu: None,
			columns: &[],
		}
	}
}

impl<'a> ListViewOpts<'a> {
	#[must_use]
	fn owned_columns(&self) -> Vec<(String, i32)> {
		self.columns
			.iter()
			.map(|(text, width)| ((*text).to_owned(), *width))
			.collect()
	}
}
