use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, collections::*, events::*, privs::*};
use crate::prelude::*;

struct ComboBoxObj {
	base: BaseCtrl,
	events: ComboBoxEvents,
	_pin: PhantomPinned,
}

native_ctrl! { ComboBox: ComboBoxObj => ComboBoxEvents;
	/// Native
	/// [combo box](https://learn.microsoft.com/en-us/windows/win32/controls/about-combo-boxes)
	/// control.
}

impl ComboBox {
	/// Instantiates a new `ComboBox` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `ComboBox` in an event closure.
	///
	/// Panics if vertical resizing behavior is
	/// [`Vert::Resize`](crate::gui::Vert::Resize).
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// let cmb = gui::ComboBox::new(
	///     &wnd,
	///     gui::ComboBoxOpts {
	///         position: (10, 10),
	///         width: 140,
	///         items: vec![
	///             "Avocado".to_owned(),
	///             "Banana".to_owned(),
	///             "Grape".to_owned(),
	///             "Orange".to_owned(),
	///         ],
	///         selected_item: Some(0),
	///         ..Default::default()
	///     },
	/// );
	/// ```
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: ComboBoxOpts) -> Self {
		if opts.resize_behavior.1 == Vert::Resize {
			panic!("ComboBox cannot be resized with Vert::Resize.");
		}

		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(
			Arc::pin(
				ComboBoxObj {
					base: BaseCtrl::new(ctrl_id),
					events: ComboBoxEvents::new(parent, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent.as_ref().before_on().wm(parent.as_ref().is_dlg().create_msg(), move |_| {
			self2.0.base.create_window(opts.window_ex_style, "COMBOBOX", None,
				opts.window_style | opts.control_style.into(), opts.position.into(),
				SIZE::new(opts.width, 0), &parent2)?;
			ui_font::set(self2.hwnd())?;
			self2.items().add(&opts.items)?;
			self2.items().select(opts.selected_item);
			parent2.as_ref().add_to_layout(self2.hwnd(), opts.resize_behavior)?;
			Ok(0) // ignored
		});

		new_self
	}

	/// Instantiates a new `ComboBox` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `ComboBox` in an event closure.
	///
	/// Panics if vertical resizing behavior is
	/// [`Vert::Resize`](crate::gui::Vert::Resize).
	#[must_use]
	pub fn new_dlg(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self
	{
		if resize_behavior.1 == Vert::Resize {
			panic!("ComboBox cannot be resized with Vert::Resize.");
		}

		let new_self = Self(
			Arc::pin(
				ComboBoxObj {
					base: BaseCtrl::new(ctrl_id),
					events: ComboBoxEvents::new(parent, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent.as_ref().before_on().wm_init_dialog(move |_| {
			self2.0.base.assign_dlg(&parent2)?;
			parent2.as_ref().add_to_layout(self2.hwnd(), resize_behavior)?;
			Ok(true) // ignored
		});

		new_self
	}

	/// Item methods.
	#[must_use]
	pub const fn items(&self) -> ComboBoxItems<'_> {
		ComboBoxItems::new(self)
	}
}

/// Options to create a [`ComboBox`](crate::gui::ComboBox) programmatically with
/// [`ComboBox::new`](crate::gui::ComboBox::new).
pub struct ComboBoxOpts {
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Control width to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi_x(120)`.
	pub width: i32,
	/// Combo box styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `CBS::DROPDOWNLIST`.
	///
	/// Suggestions:
	/// * replace with `CBS::DROPDOWN` to allow the user to type a text;
	/// * add `CBS::SORT` to automatically sort the items.
	pub control_style: co::CBS,
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
	/// **Note:** A `ComboBox` cannot be resized vertically, so it will panic if
	/// you use `Vert::Resize`.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),

	/// Items to be added.
	///
	/// Defaults to none.
	pub items: Vec<String>,
	/// Index of the item initially selected. The item must exist.
	///
	/// Defaults to `None`.
	pub selected_item: Option<u32>,
}

impl Default for ComboBoxOpts {
	fn default() -> Self {
		Self {
			position: dpi(0, 0),
			width: dpi_x(120),
			control_style: co::CBS::DROPDOWNLIST,
			window_style: co::WS::CHILD | co::WS::GROUP | co::WS::TABSTOP | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			items: Vec::default(),
			selected_item: None,
		}
	}
}
