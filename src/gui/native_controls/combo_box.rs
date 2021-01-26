use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::PostQuitMessage;
use crate::gui::events::{ComboBoxEvents, MsgEvents};
use crate::gui::native_controls::native_control_base::{NativeControlBase, OptsId};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi, ui_font};
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;
use crate::msg;
use crate::structs::{POINT, SIZE};
use crate::WString;

/// Native
/// [combo box](https://docs.microsoft.com/en-us/windows/win32/controls/about-combo-boxes)
/// control.
#[derive(Clone)]
pub struct ComboBox(Arc<Obj>);

struct Obj { // actual fields of ComboBox
	base: NativeControlBase<ComboBoxEvents>,
	opts_id: OptsId<ComboBoxOpts>,
}

unsafe impl Send for ComboBox {}
unsafe impl Sync for ComboBox {}

impl Child for ComboBox {
	fn hctrl_ref(&self) -> &HWND {
		self.0.base.hctrl_ref()
	}
}

impl ComboBox {
	/// Instantiates a new `ComboBox` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: ComboBoxOpts) -> ComboBox {
		let opts = ComboBoxOpts::define_ctrl_id(opts);
		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(
						parent,
						ComboBoxEvents::new(parent, opts.ctrl_id),
					),
					opts_id: OptsId::Wnd(opts),
				},
			),
		);
		parent.privileged_events_ref().wm_create({
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});
		new_self
	}

	/// Instantiates a new `Button` object, to be loaded from a dialog resource
	/// with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> ComboBox {
		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(
						parent,
						ComboBoxEvents::new(parent, ctrl_id),
					),
					opts_id: OptsId::Dlg(ctrl_id),
				},
			),
		);
		parent.privileged_events_ref().wm_init_dialog({
			let me = new_self.clone();
			move |_| { me.create(); true }
		});
		new_self
	}

	fn create(&self) {
		|| -> WinResult<()> {
			match &self.0.opts_id {
				OptsId::Wnd(opts) => {
					let mut pos = opts.position;
					let mut sz = SIZE::new(opts.width as i32, 0);
					if opts.vertical_text_align { pos.y -= 1; }
					multiply_dpi(Some(&mut pos), Some(&mut sz))?;

					let our_hwnd = self.0.base.create_window( // may panic
						"COMBOBOX", None, pos, sz,
						opts.ctrl_id,
						opts.ex_window_style,
						opts.window_style | opts.combo_box_style.into(),
					)?;

					our_hwnd.SendMessage(msg::WmSetFont{ hfont: ui_font(), redraw: true });
					Ok(())
				},
				OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

	hwnd_ctrlid_on_onsubclass!(ComboBoxEvents);

	/// Adds new texts.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::ComboBox;
	///
	/// let cmb_names: ComboBox; // initialize it somewhere...
	///
	/// cmb_names.add_items(&["John", "Mary"]);
	/// ```
	pub fn add_items(&self, items: &[&str]) -> WinResult<()> {
		for text in items.iter() {
			self.hwnd().SendMessage(msg::CbAddString { text })?;
		}
		Ok(())
	}

	/// Deletes all items.
	pub fn delete_all_items(&self) {
		self.hwnd().SendMessage(msg::CbResetContent {})
	}

	/// Deletes the item at the given index.
	pub fn delete_item(&self, index: u32) -> WinResult<()> {
		self.hwnd().SendMessage(msg::CbDeleteString { index })
			.map(|_| ())
	}

	/// Retrieves the text at the given position, if any.
	pub fn item(&self, index: u32) -> Option<String> {
		match self.hwnd().SendMessage(msg::CbGetLbTextLen { index }) {
			Err(err) => {
				PostQuitMessage(err);
				None
			},
			Ok(len) => {
				let mut buf = WString::new_alloc_buffer(len as usize + 1);
				match self.hwnd().SendMessage(msg::CbGetLbText{
					index,
					text: &mut buf,
				}) {
					Err(_) => None,
					Ok(_) => Some(buf.to_string()),
				}
			},
		}
	}

	/// Retrieves the total number of items.
	pub fn item_count(&self) -> WinResult<u32> {
		self.hwnd().SendMessage(msg::CbGetCount {})
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
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Will adjust `position.cy` so that, if the control is placed side-by-side
	/// with an [`Edit`](crate::gui::Edit) control, their texts will be aligned.
	///
	/// Defaults to false.
	pub vertical_text_align: bool,
	/// Control width, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
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
			position: POINT::new(0, 0),
			vertical_text_align: false,
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
