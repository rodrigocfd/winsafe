use std::any::Any;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::funcs::PostQuitMessage;
use crate::gui::events::{LabelEvents, WindowEvents};
use crate::gui::native_controls::native_control_base::{NativeControlBase, OptsId};
use crate::gui::privs::{auto_ctrl_id, calc_text_bound_box, multiply_dpi, ui_font};
use crate::gui::traits::{baseref_from_parent, Child, Parent};
use crate::handles::HWND;
use crate::msg::wm;
use crate::structs::POINT;

/// Native
/// [label](https://docs.microsoft.com/en-us/windows/win32/controls/about-static-controls)
/// control.
///
/// Implements [`Child`](crate::gui::Child) trait.
#[derive(Clone)]
pub struct Label(Arc<Obj>);

struct Obj { // actual fields of Label
	base: NativeControlBase,
	opts_id: OptsId<LabelOpts>,
	events: LabelEvents,
}

unsafe impl Send for Label {}
unsafe impl Sync for Label {}

impl Child for Label {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl Label {
	/// Instantiates a new `Label` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: LabelOpts) -> Label {
		let parent_ref = baseref_from_parent(parent);
		let opts = LabelOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(parent_ref),
					opts_id: OptsId::Wnd(opts),
					events: LabelEvents::new(parent_ref, ctrl_id),
				},
			),
		);

		parent_ref.privileged_events_ref().wm(parent_ref.create_wm(), {
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});

		new_self
	}

	/// Instantiates a new `CheckBox` object, to be loaded from a dialog resource
	/// with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> Label {
		let parent_ref = baseref_from_parent(parent);

		let new_self = Self(
			Arc::new(
				Obj {
					base: NativeControlBase::new(parent_ref),
					opts_id: OptsId::Dlg(ctrl_id),
					events: LabelEvents::new(parent_ref, ctrl_id),
				},
			),
		);

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
				OptsId::Dlg(ctrl_id) => {
					self.0.base.create_dlg(*ctrl_id).map(|_| ())?; // may panic
					self.adjust_size_to_fit_text(&self.text()?)
				}
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

	hwnd_ctrlid_on_onsubclass!(LabelEvents);

	/// Sets the text by calling [`SetWindowText`](crate::HWND::SetWindowText),
	/// and resizes the control to exactly fit the new text.
	pub fn set_text(&self, text: &str) -> WinResult<()> {
		self.hwnd().SetWindowText(text)?;
		self.adjust_size_to_fit_text(text)
	}

	/// Retrieves the text by calling
	/// [`GetWindowTextStr`](crate::HWND::GetWindowText).
	pub fn text(&self) -> WinResult<String> {
		self.hwnd().GetWindowTextStr()
	}

	fn adjust_size_to_fit_text(&self, text: &str) -> WinResult<()> {
		let bound_box = calc_text_bound_box(text)?;
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
