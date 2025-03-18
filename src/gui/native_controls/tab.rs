use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::gui::{collections::*, events::*, privs::*, *};
use crate::msg::*;
use crate::prelude::*;

struct TabObj {
	base: BaseCtrl,
	events: TabEvents,
	children: Vec<(String, Box<dyn AsRef<WindowControl>>)>, // title + content
	_pin: PhantomPinned,
}

native_ctrl! { Tab: TabObj => TabEvents;
	/// Native
	/// [tab](https://learn.microsoft.com/en-us/windows/win32/controls/tab-controls)
	/// control.
}

impl Tab {
	/// Instantiates a new `Tab` object, to be created on the parent window with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `Tab` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: TabOpts) -> Self {
		let mut opts = opts;
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let children = opts.items.drain(..).collect::<Vec<_>>();

		let new_self = Self(Arc::pin(TabObj {
			base: BaseCtrl::new(ctrl_id),
			events: TabEvents::new(parent, ctrl_id),
			children,
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent
			.as_ref()
			.before_on()
			.wm(parent.as_ref().is_dlg().create_msg(), move |_| {
				self2.0.base.create_window(
					opts.window_ex_style,
					"SysTabControl32",
					None,
					opts.window_style | opts.control_style.into(),
					opts.position.into(),
					opts.size.into(),
					&parent2,
				)?;
				ui_font::set(self2.hwnd())?;
				if opts.control_ex_style != co::TCS_EX::NoValue {
					self2.set_extended_style(true, opts.control_ex_style);
				}
				self2.0.children.iter().for_each(|(text, _)| unsafe {
					self2.items().add(text); // add the tabs
				});
				self2.display_tab(0)?; // 1st tab selected by default
				parent2
					.as_ref()
					.add_to_layout(self2.hwnd(), opts.resize_behavior)?;
				Ok(0) // ignored
			});

		new_self.default_message_handlers(parent);
		new_self
	}

	/// Instantiates a new `Tab` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `Tab` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
		items: Vec<(String, Box<dyn AsRef<WindowControl>>)>,
	) -> Self {
		let new_self = Self(Arc::pin(TabObj {
			base: BaseCtrl::new(ctrl_id),
			events: TabEvents::new(parent, ctrl_id),
			children: items,
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent.as_ref().before_on().wm_init_dialog(move |_| {
			self2.0.base.assign_dlg(&parent2)?;
			self2.0.children.iter().for_each(|(text, _)| unsafe {
				self2.items().add(text); // add the tabs
			});
			self2.display_tab(0)?; // 1st tab selected by default
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
			.before_on()
			.wm_notify(self.ctrl_id(), co::TCN::SELCHANGE, move |_| {
				if let Some(sel_item) = self2.items().selected() {
					self2.display_tab(sel_item.index())?;
				}
				Ok(0) // ignored
			});

		let self2 = self.clone();
		parent.as_ref().after_on().wm_destroy(move || {
			self2.image_list().map(|hil| {
				let _ = unsafe { ImageListDestroyGuard::new(hil.raw_copy()) }; // destroy the image list, if any
			});
			Ok(())
		});
	}

	fn display_tab(&self, index: u32) -> SysResult<()> {
		self.0
			.children
			.iter()
			.enumerate()
			.filter(|(i, _)| *i != index as usize)
			.for_each(|(_, (_, item))| {
				item.as_ref().as_ref().hwnd().ShowWindow(co::SW::HIDE); // hide all others
			});

		if let Some((_, item)) = self.0.children.get(index as usize) {
			let mut rc = self
				.hwnd()
				.GetParent()?
				.ScreenToClientRc(self.hwnd().GetWindowRect()?)?;
			unsafe {
				self.hwnd().SendMessage(tcm::AdjustRect {
					display_rect: false,
					rect: &mut rc, // ideal size of the child
				});
			}
			item.as_ref().as_ref().hwnd().SetWindowPos(
				HwndPlace::None,
				POINT::new(rc.left, rc.top),
				SIZE::new(rc.right - rc.left, rc.bottom - rc.top),
				co::SWP::NOZORDER | co::SWP::SHOWWINDOW,
			)?;
		}

		Ok(())
	}

	/// Retrieves a reference to the associated image list by sending a
	/// [`tcm::GetImageList`](crate::msg::tcm::GetImageList) message.
	///
	/// The image list is owned by the control.
	#[must_use]
	pub fn image_list(&self) -> Option<&HIMAGELIST> {
		unsafe { self.hwnd().SendMessage(tcm::GetImageList {}) }.map(|hil| {
			let hil_ptr = &hil as *const HIMAGELIST;
			unsafe { &*hil_ptr }
		})
	}

	/// Item methods.
	#[must_use]
	pub const fn items(&self) -> TabItems {
		TabItems::new(self)
	}

	/// Sets or unsets the given extended list view styles by sending a
	/// [`tcm::SetExtendedStyle`](crate::msg::tcm::SetExtendedStyle) message.
	pub fn set_extended_style(&self, set: bool, ex_style: co::TCS_EX) {
		unsafe {
			self.hwnd().SendMessage(tcm::SetExtendedStyle {
				mask: ex_style,
				style: if set { ex_style } else { co::TCS_EX::NoValue },
			});
		}
	}

	/// Sets the associated image list by sending a
	/// [`tcm::SetImageList`](crate::msg::tcm::SetImageList) message.
	///
	/// The image list will be owned by the control. Returns the previous one,
	/// if any.
	pub fn set_image_list(
		&self,
		himagelist: ImageListDestroyGuard,
	) -> Option<ImageListDestroyGuard> {
		let mut himagelist = himagelist;
		let hil = himagelist.leak();

		unsafe {
			self.hwnd()
				.SendMessage(tcm::SetImageList { himagelist: Some(hil) })
				.map(|prev_hil| ImageListDestroyGuard::new(prev_hil))
		}
	}
}

/// Options to create a [`Tab`](crate::gui::Tab) programmatically with
/// [`Tab::new`](crate::gui::Tab::new).
pub struct TabOpts {
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Width and height of control to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(80, 50)`.
	pub size: (i32, i32),
	/// Tab styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TCS::NoValue`.
	pub control_style: co::TCS,
	/// Extended tab styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TCS_EX::NoValue`.
	pub control_ex_style: co::TCS_EX,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::GROUP | WS::TABSTOP | WS::VISIBLE`.
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

	/// Items to be added as soon as the control is created. The tuple contains
	/// the title of the tab and the window to be rendered inside of it.
	///
	/// Note that, in o order to make the focus rotation work properly, the
	/// child windows must be created with the
	/// [`co::WS_EX::CONTROLPARENT`](crate::co::WS_EX::CONTROLPARENT) extended
	/// style.
	///
	/// Defaults to none.
	pub items: Vec<(String, Box<dyn AsRef<WindowControl>>)>,
}

impl Default for TabOpts {
	fn default() -> Self {
		Self {
			position: dpi(0, 0),
			size: dpi(80, 50),
			control_style: co::TCS::NoValue,
			control_ex_style: co::TCS_EX::NoValue,
			window_style: co::WS::CHILD | co::WS::GROUP | co::WS::TABSTOP | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			items: Vec::default(),
		}
	}
}
