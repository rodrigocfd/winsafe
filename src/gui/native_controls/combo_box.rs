use std::any::Any;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::PostQuitMessage;
use crate::gui::events::{ComboBoxEvents, WindowEvents};
use crate::gui::native_controls::combo_box_items::ComboBoxItems;
use crate::gui::native_controls::native_control_base::{NativeControlBase, OptsId};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi, ui_font};
use crate::gui::traits::{baseref_from_parent, Child, Parent};
use crate::handles::HWND;
use crate::msg::wm;
use crate::structs::{POINT, SIZE};

/// Native
/// [combo box](https://docs.microsoft.com/en-us/windows/win32/controls/about-combo-boxes)
/// control.
///
/// Implements [`Child`](crate::gui::Child) trait.
#[derive(Clone)]
pub struct ComboBox(Arc<Obj>);

struct Obj { // actual fields of ComboBox
	base: NativeControlBase,
	opts_id: OptsId<ComboBoxOpts>,
	events: ComboBoxEvents,
	items: ComboBoxItems,
}

unsafe impl Send for ComboBox {}
unsafe impl Sync for ComboBox {}

impl Child for ComboBox {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl ComboBox {
	/// Instantiates a new `ComboBox` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: ComboBoxOpts) -> ComboBox {
		let parent_ref = baseref_from_parent(parent);
		let opts = ComboBoxOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(parent_ref),
					opts_id: OptsId::Wnd(opts),
					events: ComboBoxEvents::new(parent_ref, ctrl_id),
					items: ComboBoxItems::new(parent_ref.hwnd_ref()), // wrong HWND, just to construct the object
				},
			),
		);
		new_self.0.items.set_hwnd_ref(new_self.0.base.hwnd_ref()); // correct HWND

		parent_ref.privileged_events_ref().wm(parent_ref.create_wm(), {
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});

		new_self
	}

	/// Instantiates a new `ComboBox` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> ComboBox {
		let parent_ref = baseref_from_parent(parent);

		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(parent_ref),
					opts_id: OptsId::Dlg(ctrl_id),
					events: ComboBoxEvents::new(parent_ref, ctrl_id),
					items: ComboBoxItems::new(parent_ref.hwnd_ref()), // wrong HWND, just to construct the object
				},
			),
		);
		new_self.0.items.set_hwnd_ref(new_self.0.base.hwnd_ref()); // correct HWND

		parent_ref.privileged_events_ref().wm_init_dialog({
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
					let mut sz = SIZE::new(opts.width as _, 0);
					if opts.baseline_text_align { pos.y -= 1; }
					multiply_dpi(Some(&mut pos), Some(&mut sz))?;

					let our_hwnd = self.0.base.create_window( // may panic
						"COMBOBOX", None, pos, sz,
						opts.ctrl_id,
						opts.ex_window_style,
						opts.window_style | opts.combo_box_style.into(),
					)?;

					our_hwnd.SendMessage(wm::SetFont{ hfont: ui_font(), redraw: true });
					Ok(())
				},
				OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

	ctrlid_hwnd_on_onsubclass!(ComboBoxEvents);

	/// Item methods.
	pub fn items(&self) -> &ComboBoxItems {
		&self.0.items
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`ComboBox`](crate::gui::ComboBox) programmatically with
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
	pub baseline_text_align: bool,
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
			baseline_text_align: false,
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
