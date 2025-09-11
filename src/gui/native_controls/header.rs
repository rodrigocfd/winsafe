use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::gui::{collections::*, privs::*, *};
use crate::msg::*;
use crate::prelude::*;

struct HeaderObj {
	base: BaseCtrl,
	events: BaseCtrlEvents,
	_pin: PhantomPinned,
}

native_ctrl! { Header: HeaderObj => GuiEventsHeader;
	/// Native
	/// [header](https://learn.microsoft.com/en-us/windows/win32/controls/header-controls)
	/// control.
}

impl Header {
	/// Instantiates a new `Header` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `Header` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: HeaderOpts) -> Self {
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(Arc::pin(HeaderObj {
			base: BaseCtrl::new(ctrl_id),
			events: BaseCtrlEvents::new(parent, ctrl_id),
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
					"SysHeader32",
					None,
					opts.window_style | opts.control_style.into(),
					opts.position.into(),
					SIZE::with(opts.width, opts.height),
					&parent2,
				);
				ui_font::set(self2.hwnd());
				for (text, width) in opts.items.iter() {
					self2.items().add(text, *width)?;
				}
				parent2
					.as_ref()
					.add_to_layout(self2.hwnd(), opts.resize_behavior);
				Ok(0) // ignored
			});

		new_self.default_message_handlers(parent);
		new_self
	}

	/// Instantiates a new `Header` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `Header` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self {
		let new_self = Self(Arc::pin(HeaderObj {
			base: BaseCtrl::new(ctrl_id),
			events: BaseCtrlEvents::new(parent, ctrl_id),
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

		new_self.default_message_handlers(parent);
		new_self
	}

	/// For the nested Header inside ListView.
	#[must_use]
	pub(in crate::gui) fn from_list_view(parent: &(impl GuiParent + 'static)) -> Self {
		let ctrl_id = auto_id::next();
		let new_self = Self(Arc::pin(HeaderObj {
			base: BaseCtrl::new(ctrl_id),
			events: BaseCtrlEvents::new(parent, ctrl_id),
			_pin: PhantomPinned,
		}));

		new_self.default_message_handlers(parent);
		new_self
	}

	/// For the nested Header inside ListView.
	#[must_use]
	pub(in crate::gui) fn init_nested(&self, hlist: &HWND) -> bool {
		if let Ok(hheader) = unsafe { hlist.SendMessage(lvm::GetHeader {}) } {
			unsafe {
				hheader.SetWindowLongPtr(co::GWLP::ID, self.ctrl_id() as _); // give the header an ID; initially zero
			}
			self.0.base.set_hwnd(hheader);
			true // header object initialized
		} else {
			false // not inialized
		}
	}

	fn default_message_handlers(&self, parent: &impl AsRef<BaseWnd>) {
		let self2 = self.clone();
		parent.as_ref().after_on().wm_destroy(move || {
			[co::HDSIL::NORMAL, co::HDSIL::STATE]
				.into_iter()
				.for_each(|kind| unsafe {
					self2
						.hwnd()
						.SendMessage(hdm::GetImageList { kind })
						.map(|h| {
							self2
								.hwnd()
								.SendMessage(hdm::SetImageList { himagelist: None, kind }); // remove from control
							let _ = ImageListDestroyGuard::new(h); // destroy
						});
				});
			Ok(())
		});
	}

	/// Retrieves one of the associated image lists by sending an
	/// [`hdm::GetImageList`](crate::msg::hdm::GetImageList) message.
	///
	/// Image lists are lazy-initialized: the first time you call this method
	/// for a given image list, it will be created and assigned with
	/// [`hdm::SetImageList`](crate::msg::hdm::SetImageList).
	///
	/// The image list is owned by the control.
	#[must_use]
	pub fn image_list(&self, kind: co::HDSIL) -> HrResult<HIMAGELIST> {
		match unsafe { self.hwnd().SendMessage(hdm::GetImageList { kind }) } {
			Some(h) => Ok(h), // already created
			None => {
				// Not created yet. Create a new image list and assign it to the list view.
				let h = HIMAGELIST::Create(SIZE::with(16, 16), co::ILC::COLOR32, 1, 1)?.leak();
				unsafe {
					self.hwnd()
						.SendMessage(hdm::SetImageList { himagelist: Some(h.raw_copy()), kind });
				}
				Ok(h)
			},
		}
	}

	/// Item methods.
	#[must_use]
	pub const fn items(&self) -> HeaderItems<'_> {
		HeaderItems::new(self)
	}
}

/// Options to create a [`Header`](crate::gui::Header) programmatically with
/// [`Header::new`](crate::gui::Header::new).
pub struct HeaderOpts {
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Control width to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi_x(100)`.
	pub width: i32,
	/// Control height to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi_y(23)`.
	pub height: i32,
	/// Header styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `HDS::BUTTONS | HDS::HORZ`.
	pub control_style: co::HDS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::BORDER | WS::CHILD | WS::GROUP | WS::TABSTOP | WS::VISIBLE`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
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

	/// Items to be added to the control. Each item is composed by the text and
	/// the width.
	///
	/// Defaults to none.
	pub items: Vec<(String, i32)>,
}

impl Default for HeaderOpts {
	fn default() -> Self {
		Self {
			position: dpi(0, 0),
			width: dpi_x(100),
			height: dpi_y(23),
			control_style: co::HDS::BUTTONS | co::HDS::HORZ,
			window_style: co::WS::BORDER
				| co::WS::CHILD
				| co::WS::GROUP
				| co::WS::TABSTOP
				| co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			items: Vec::new(),
		}
	}
}
