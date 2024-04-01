use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::gui::{*, events::*, privs::*, spec::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj { // actual fields of Header
	base: BaseNativeControl,
	_pin: PhantomPinned,
	events: HeaderEvents,
}

/// Variant field for creating a `Header` control.
enum OptsReszLv<'a> {
	/// Options for a raw control creation.
	Wnd(&'a HeaderOpts),
	/// Resize behavior for a dialog control creation.
	Dlg((Horz, Vert)),
	/// `BaseNativeControl` of the owner `ListView`.
	Lv(NonNull<BaseNativeControl>),
}

//------------------------------------------------------------------------------

/// Native
/// [header](https://learn.microsoft.com/en-us/windows/win32/controls/header-controls)
/// control.
#[derive(Clone)]
pub struct Header(Pin<Arc<Obj>>);

unsafe impl Send for Header {}

impl AsRef<BaseNativeControl> for Header {
	fn as_ref(&self) -> &BaseNativeControl {
		&self.0.base
	}
}

impl GuiWindow for Header {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiChild for Header {
	fn ctrl_id(&self) -> u16 {
		self.0.base.ctrl_id()
	}
}

impl GuiChildFocus for Header {}

impl GuiNativeControl for Header {}

impl GuiNativeControlEvents<HeaderEvents> for Header {
	fn on(&self) -> &HeaderEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl Header {
	/// Instantiates a new `Header` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `Header` in an event closure.
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: HeaderOpts) -> Self {
		let opts = auto_ctrl_id_if_zero(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent, ctrl_id),
					events: HeaderEvents::new(parent, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent.as_ref().privileged_on().wm_create_or_initdialog(move |_, _| {
			self2.create(OptsReszLv::Wnd(&opts))?;
			Ok(())
		});

		new_self
	}

	/// Instantiates a new `Header` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `Header` in an event closure.
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
					events: HeaderEvents::new(parent, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent.as_ref().privileged_on().wm(co::WM::INITDIALOG, move |_, _| {
			self2.create(OptsReszLv::Dlg(resize_behavior))?;
			Ok(())
		});

		new_self
	}

	/// Instantiates a new `Header` object to be loaded from a
	/// [`ListView`](crate::gui::ListView) control. This will give you access to
	/// the inner `Header` control of that `ListView`.
	#[must_use]
	pub fn from_list_view<T: 'static>(list_view: &ListView<T>) -> Self {
		let lv_base_ref: &BaseNativeControl = list_view.as_ref();
		let parent_base_ref = lv_base_ref.parent();
		let ctrl_id = next_auto_ctrl_id();

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_base_ref, ctrl_id),
					events: HeaderEvents::new(parent_base_ref, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let lv_base_ptr = NonNull::from(lv_base_ref);
		let self2 = new_self.clone();
		parent_base_ref.privileged_on().wm_create_or_initdialog(move |_, _| {
			self2.create(OptsReszLv::Lv(lv_base_ptr))?;
			Ok(())
		});

		new_self
	}

	fn create(&self, opts_resz_lv: OptsReszLv) -> SysResult<()> {
		match &opts_resz_lv {
			OptsReszLv::Wnd(opts) => {
				let mut pos = POINT::new(opts.position.0, opts.position.1);
				let mut sz = SIZE::new(opts.size.0 as _, opts.size.1 as _);
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), Some(&mut sz))?;

				self.0.base.create_window( // may panic
					"SysHeader32", None, pos, sz,
					opts.window_ex_style,
					opts.window_style | opts.header_style.into(),
				)?;

				self.0.base.parent()
					.add_to_layout_arranger(self.hwnd(), opts.resize_behavior)
			},
			OptsReszLv::Dlg(resize_behavior) => {
				self.0.base.create_dlg()?;
				self.0.base.parent()
					.add_to_layout_arranger(self.hwnd(), *resize_behavior)
			},
			OptsReszLv::Lv(lv_base_ptr) => {
				let hheader = unsafe {
					let lv_base_ref = lv_base_ptr.as_ref();
					let hheader = lv_base_ref.hwnd().SendMessage(lvm::GetHeader {})?;
					hheader.SetWindowLongPtr(co::GWLP::ID, self.ctrl_id() as _); // give the header its new ID
					hheader
				};
				self.0.base.assign_hctrl(hheader);
				Ok(())
			},
		}
	}

	/// Retrieves a reference to one of the associated image lists by sending an
	/// [`hdm::GetImageList`](crate::msg::hdm::GetImageList) message.
	///
	/// The image list is owned by the control.
	#[must_use]
	pub fn image_list(&self, kind: co::HDSIL) -> Option<&HIMAGELIST> {
		unsafe {
			self.hwnd()
				.SendMessage(hdm::GetImageList { kind })
		}.map(|hil| {
			let hil_ptr = &hil as *const HIMAGELIST;
			unsafe { &*hil_ptr }
		})
	}

	/// Exposes the item methods.
	#[must_use]
	pub const fn items(&self) -> HeaderItems<'_> {
		HeaderItems::new(self)
	}

	/// Sets the one of the associated image lists by sending an
	/// [`hdm::SetImageList`](crate::msg::hdm::SetImageList) message.
	///
	/// The image list will be owned by the control. Returns the previous one,
	/// if any.
	pub fn set_image_list(&self,
		kind: co::HDSIL,
		himagelist: ImageListDestroyGuard,
	) -> Option<ImageListDestroyGuard>
	{
		let mut himagelist = himagelist;
		let hil = himagelist.leak();

		unsafe {
			self.hwnd()
				.SendMessage(hdm::SetImageList { kind, himagelist: Some(hil) })
				.map(|prev_hil| ImageListDestroyGuard::new(prev_hil))
		}
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`Header`](crate::gui::Header) programmatically with
/// [`Header::new`](crate::gui::Header::new).
pub struct HeaderOpts {
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
	/// Defaults to `(0, 0)`.
	pub size: (u32, u32),
	/// Header styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `HDS::BUTTONS | HDS::HORZ`.
	pub header_style: co::HDS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::BORDER`.
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
	/// Horizontal and vertical behavior of the control when the parent window
	/// is resized.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),
}

impl Default for HeaderOpts {
	fn default() -> Self {
		Self {
			position: (0, 0),
			size: (0, 0),
			header_style: co::HDS::BUTTONS | co::HDS::HORZ,
			window_style: co::WS::CHILD | co::WS::BORDER,
			window_ex_style: co::WS_EX::NoValue,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
		}
	}
}

impl ResizeBehavior for &HeaderOpts {
	fn resize_behavior(&self) -> (Horz, Vert) {
		self.resize_behavior
	}
}

impl AutoCtrlId for HeaderOpts {
	fn ctrl_id_mut(&mut self) -> &mut u16 {
		&mut self.ctrl_id
	}
}
