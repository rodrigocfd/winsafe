use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::funcs::PostQuitMessage;
use crate::gui::events::{LabelEvents, WindowEvents};
use crate::gui::native_controls::native_control_base::{NativeControlBase, OptsId};
use crate::gui::privs::{auto_ctrl_id, calc_text_bound_box, multiply_dpi, ui_font};
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;
use crate::msg::wm;
use crate::structs::POINT;

/// Native
/// [label](https://docs.microsoft.com/en-us/windows/win32/controls/about-static-controls)
/// control.
#[derive(Clone)]
pub struct Label(Arc<Obj>);

struct Obj { // actual fields of Label
	base: NativeControlBase<LabelEvents>,
	opts_id: OptsId<LabelOpts>,
}

unsafe impl Send for Label {}
unsafe impl Sync for Label {}

impl Child for Label {
	fn hctrl_ref(&self) -> &HWND {
		self.0.base.hctrl_ref()
	}
}

impl Label {
	/// Instantiates a new `Label` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: LabelOpts) -> Label {
		let opts = LabelOpts::define_ctrl_id(opts);
		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(
						parent,
						LabelEvents::new(parent, opts.ctrl_id),
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

	/// Instantiates a new `CheckBox` object, to be loaded from a dialog resource
	/// with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> Label {
		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(
						parent,
						LabelEvents::new(parent, ctrl_id),
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
					if opts.baseline_text_align { pos.y += 3; }
					multiply_dpi(Some(&mut pos), None)?;

					let bound_box = calc_text_bound_box(&opts.text)?;

					let our_hwnd = self.0.base.create_window( // may panic
						"STATIC", Some(&opts.text), pos, bound_box,
						opts.ctrl_id,
						opts.ex_window_style,
						opts.window_style | opts.label_style.into(),
					)?;

					our_hwnd.SendMessage(wm::SetFont{ hfont: ui_font(), redraw: true });
					Ok(())
				},
				OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

	hwnd_ctrlid_on_onsubclass!(LabelEvents);

	/// Calls [`SetWindowText`](crate::HWND::SetWindowText) and resizes the
	/// control to exactly fit the new text.
	pub fn set_text(&self, text: &str) -> WinResult<()> {
		let bound_box = calc_text_bound_box(text)?;
		self.hwnd().SetWindowText(text)?;
		self.hwnd().SetWindowPos(
			HwndPlace::None, 0, 0, bound_box.cx, bound_box.cy,
			co::SWP::NOZORDER | co::SWP::NOMOVE)
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`Label`](crate::gui::Label) programatically with
/// [`label::new`](crate::gui::Label::new).
pub struct LabelOpts {
	/// Text of the control to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
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
	/// label styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `SS::LEFT | SS:NOTIFY`.
	pub label_style: co::SS,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE`.
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

impl Default for LabelOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: POINT::new(0, 0),
			baseline_text_align: false,
			label_style: co::SS::LEFT | co::SS::NOTIFY,
			window_style: co::WS::CHILD | co::WS::VISIBLE,
			ex_window_style: co::WS_EX::LEFT,
			ctrl_id: 0,
		}
	}
}

impl LabelOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
