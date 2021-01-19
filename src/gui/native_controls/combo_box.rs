use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::gui::events::{ComboBoxEvents, MsgEvents};
use crate::gui::globals::{auto_ctrl_id, ui_font};
use crate::gui::native_controls::native_control_base::NativeControlBase;
use crate::gui::native_controls::opts_id::OptsId;
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;
use crate::msg;
use crate::structs::{POINT, SIZE};
use crate::WString;

/// Native
/// [combo box](https://docs.microsoft.com/en-us/windows/win32/controls/about-combo-boxes)
/// control.
#[derive(Clone)]
pub struct ComboBox {
	base: Arc<
		NativeControlBase<ComboBoxEvents, ComboBoxOpts>,
	>,
}

unsafe impl Send for ComboBox {}
unsafe impl Sync for ComboBox {}

impl Child for ComboBox {
	fn hctrl_ref(&self) -> &HWND {
		self.base.hctrl_ref()
	}
}

impl ComboBox {
	/// Instantiates a new `ComboBox` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: ComboBoxOpts) -> ComboBox {
		let opts = ComboBoxOpts::define_ctrl_id(opts);
		let me = Self {
			base: Arc::new(
				NativeControlBase::new(
					parent,
					ComboBoxEvents::new(parent, opts.ctrl_id),
					OptsId::Wnd(opts),
				),
			),
		};
		me.add_creation_to_parent(parent);
		me
	}

	/// Instantiates a new `Button` object, to be loaded from a dialog resource
	/// with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> ComboBox {
		let me = Self {
			base: Arc::new(
				NativeControlBase::new(
					parent,
					ComboBoxEvents::new(parent, ctrl_id),
					OptsId::Dlg(ctrl_id),
				),
			),
		};
		me.add_creation_to_parent(parent);
		me
	}

	fn add_creation_to_parent(&self, parent: &dyn Parent) {
		let me = self.clone();
		parent.add_child_to_be_created(
			Box::new(move || {
				match me.base.opts_id() {
					OptsId::Wnd(opts) => {
						let our_hwnd = me.base.create_window( // may panic
							"COMBOBOX", None, opts.pos,
							SIZE::new(opts.width as i32, 0),
							opts.ctrl_id,
							opts.ex_window_style,
							opts.window_style | opts.combo_box_style.into(),
						)?;

						our_hwnd.SendMessage(msg::WmSetFont{ hfont: ui_font(), redraw: true });
						Ok(())
					},
					OptsId::Dlg(ctrl_id) => me.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
				}
			})
		);
	}

	/// Returns the underlying handle for this control.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is created.
	pub fn hwnd(&self) -> HWND {
		*self.hctrl_ref()
	}

	/// Returns the control ID.
	pub fn ctrl_id(&self) -> u16 {
		match self.base.opts_id() {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}

	/// Exposes the button events.
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	pub fn on(&self) -> &ComboBoxEvents {
		self.base.on()
	}

	/// Exposes the subclass events. If at least one event exists, the control
	/// will be
	/// [subclassed](https://docs.microsoft.com/en-us/windows/win32/controls/subclassing-overview).
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	pub fn on_subclass(&self) -> &MsgEvents {
		self.base.on_subclass()
	}

	/// Adds new texts.
	pub fn add_items(&self, items: &[&str]) -> WinResult<()> {
		for text in items.iter() {
			self.hwnd().SendMessage(msg::CbAddString { text })?;
		}
		Ok(())
	}

	/// Retrieves the text at the given position, if any.
	pub fn item(&self, index: u32) -> Option<String> {
		self.hwnd().SendMessage(msg::CbGetLbTextLen { index })
			.ok()
			.and_then(|len| {
				let mut buf = WString::new_alloc_buffer(len as usize + 1);
				self.hwnd().SendMessage(msg::CbGetLbText { index, text: &mut buf })
					.ok()
					.map(|_| buf.to_string())
			})
	}

	/// Retrieves the index of the currently selected item, if any.
	pub fn selected_index(&self) -> Option<u32> {
		self.hwnd().SendMessage(msg::CbGetCurSel {})
	}

	/// Retrieves the currently selected text, if any.
	pub fn selected_item(&self) -> Option<String> {
		self.selected_index()
			.and_then(|idx| self.item(idx))
	}

	/// Sets the currently selected text, or clears it.
	pub fn set_selected_item(&self, index: Option<u32>) {
		self.hwnd().SendMessage(msg::CbSetCurSel { index });
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`ComboBox`](crate::gui::ComboBox) programatically with
/// [`ComboBox::new`](crate::gui::ComboBox::new).
pub struct ComboBoxOpts {
	/// Control position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to 0 x 0.
	pub pos: POINT,
	/// Control width, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to 120.
	pub width: u32,
	/// Combo box styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `CBS::DROPDOWNLIST`.
	///
	/// Suggestions:
	/// * replace with `CBS::DROPDOWN` to allow the user to type a text;
	/// * add `CBS::SORT` to automatically sort the items.
	pub combo_box_style: co::CBS,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	pub ex_window_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
}

impl Default for ComboBoxOpts {
	fn default() -> Self {
		Self {
			pos: POINT::new(0, 0),
			width: 120,
			ctrl_id: 0,
			combo_box_style: co::CBS::DROPDOWNLIST,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			ex_window_style: co::WS_EX::LEFT,
		}
	}
}

impl ComboBoxOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
