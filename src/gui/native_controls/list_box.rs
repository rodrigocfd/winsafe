use std::any::Any;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::co;
use crate::gui::events::{ListBoxEvents, WindowEvents};
use crate::gui::native_controls::base_native_control::{BaseNativeControl,
	OptsId};
use crate::gui::native_controls::list_box_items::ListBoxItems;
use crate::gui::privs::{auto_ctrl_id, multiply_dpi_or_dtu, ui_font};
use crate::gui::resizer::{Horz, Vert};
use crate::kernel::decl::WinResult;
use crate::msg::wm;
use crate::prelude::{AsAny, GuiChild, GuiEventsView, GuiFocusControl,
	GuiNativeControl, GuiNativeControlEvents, GuiParent, GuiWindow, Handle,
	UserHwnd};
use crate::user::decl::{HWND, POINT, SIZE};

/// Native
/// [list box](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#check-boxes)
/// control. Not to be confused with the more complex
/// [list view](crate::gui::ListView) control.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
#[derive(Clone)]
pub struct ListBox(Arc<Obj>);

struct Obj { // actual fields of ListBox
	base: BaseNativeControl,
	opts_id: OptsId<ListBoxOpts>,
	events: ListBoxEvents,
}

unsafe impl Send for ListBox {}

impl AsAny for ListBox {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiWindow for ListBox {
	fn hwnd(&self) -> HWND {
		self.0.base.hwnd()
	}
}

impl GuiChild for ListBox {
	fn ctrl_id(&self) -> u16 {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}
}

impl GuiNativeControl for ListBox {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl GuiNativeControlEvents<ListBoxEvents> for ListBox {
	fn on(&self) -> &ListBoxEvents {
		if !self.0.base.hwnd().is_null() {
			panic!("Cannot add events after the control creation.");
		} else if !self.0.base.parent_base().hwnd().is_null() {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl GuiFocusControl for ListBox {}

impl ListBox {
	/// Instantiates a new `ListBox` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::prelude::UserHwnd::CreateWindowEx).
	pub fn new(parent: &impl GuiParent, opts: ListBoxOpts) -> ListBox {
		let opts = ListBoxOpts::define_ctrl_id(opts);
		let (ctrl_id, horz, vert) = (opts.ctrl_id, opts.horz_resize, opts.vert_resize);

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Wnd(opts),
					events: ListBoxEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm(parent.as_base().wmcreate_or_wminitdialog(), {
			let self2 = new_self.clone();
			move |_| self2.create(horz, vert)
				.map_err(|e| e.into())
				.map(|_| 0)
		});

		new_self
	}

	/// Instantiates a new `ListBox` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::prelude::UserHwnd::GetDlgItem).
	pub fn new_dlg(
		parent: &impl GuiParent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert)) -> ListBox
	{
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Dlg(ctrl_id),
					events: ListBoxEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm_init_dialog({
			let self2 = new_self.clone();
			move |_| self2.create(resize_behavior.0, resize_behavior.1)
				.map_err(|e| e.into())
				.map(|_| true)
		});
		new_self
	}

	fn create(&self, horz: Horz, vert: Vert) -> WinResult<()> {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				let mut sz = opts.size;
				multiply_dpi_or_dtu(
					self.0.base.parent_base(), Some(&mut pos), Some(&mut sz))?;

				let our_hwnd = self.0.base.create_window(
					"ListBox", None, pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.list_box_style.into(),
				)?;

				our_hwnd.SendMessage(wm::SetFont { hfont: ui_font(), redraw: true });
				self.items().add(&opts.items)?;
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ())?,
		}

		self.0.base.parent_base().add_to_resizer(self.hwnd(), horz, vert)
	}

	/// Item methods.
	pub fn items<'a>(&'a self) -> ListBoxItems<'a> {
		ListBoxItems {
			hwnd: self.hwnd(),
			owner: PhantomData,
		}
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`ListBox`](crate::gui::ListBox) programmatically with
/// [`ListBox::new`](crate::gui::ListBox::new).
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct ListBoxOpts {
	/// Control position within parent client area, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Control size, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to 50 x 50.
	pub size: SIZE,
	/// List box styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `LBS::NOTIFY`.
	pub list_box_style: co::LBS,
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

	/// Items to be added right away to the control.
	///
	/// Defaults to none.
	pub items: Vec<String>,
	/// Horizontal behavior when the parent is resized.
	///
	/// Defaults to `Horz::None`.
	pub horz_resize: Horz,
	/// Vertical behavior when the parent is resized.
	///
	/// Defaults to `Vert::None`.
	pub vert_resize: Vert,
}

impl Default for ListBoxOpts {
	fn default() -> Self {
		Self {
			position: POINT::new(0, 0),
			size: SIZE::new(50, 50),
			list_box_style: co::LBS::NOTIFY,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
			horz_resize: Horz::None,
			vert_resize: Vert::None,
			items: Vec::default(),
		}
	}
}

impl ListBoxOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
