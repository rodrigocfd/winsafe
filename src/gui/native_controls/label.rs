use std::any::Any;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::gui::events::{EventsView, LabelEvents, WindowEvents};
use crate::gui::native_controls::base_native_control::{BaseNativeControl, OptsId};
use crate::gui::privs::{auto_ctrl_id, calc_text_bound_box, multiply_dpi, ui_font};
use crate::gui::resizer::{Horz, Vert};
use crate::gui::traits::{
	AsAny,
	Child,
	NativeControl,
	NativeControlEvents,
	Parent,
	TextControl,
	Window,
};
use crate::handles::HWND;
use crate::msg::wm;
use crate::structs::{POINT, SIZE};

/// Native
/// [label](https://docs.microsoft.com/en-us/windows/win32/controls/about-static-controls)
/// control.
#[derive(Clone)]
pub struct Label(Arc<Obj>);

struct Obj { // actual fields of Label
	base: BaseNativeControl,
	opts_id: OptsId<LabelOpts>,
	events: LabelEvents,
}

unsafe impl Send for Label {}

impl AsAny for Label {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl Window for Label {
	fn hwnd(&self) -> HWND {
		self.0.base.hwnd()
	}
}

impl Child for Label {
	fn ctrl_id(&self) -> u16 {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}
}

impl NativeControl for Label {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl NativeControlEvents<LabelEvents> for Label {
	fn on(&self) -> &LabelEvents {
		if !self.0.base.hwnd().is_null() {
			panic!("Cannot add events after the control creation.");
		} else if !self.0.base.parent_base().hwnd().is_null() {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl TextControl for Label {}

impl Label {
	/// Instantiates a new `Label` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &impl Parent, opts: LabelOpts) -> Label {
		let opts = LabelOpts::define_ctrl_id(opts);
		let (ctrl_id, horz, vert) = (opts.ctrl_id, opts.horz_resize, opts.vert_resize);
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Wnd(opts),
					events: LabelEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm(parent.as_base().wmcreate_or_wminitdialog(), {
			let self2 = new_self.clone();
			move |_| { self2.create(horz, vert)?; Ok(0) }
		});
		new_self
	}

	/// Instantiates a new `CheckBox` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(
		parent: &impl Parent,
		ctrl_id: u16,
		horz_resize: Horz, vert_resize: Vert) -> Label
	{
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Dlg(ctrl_id),
					events: LabelEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm_init_dialog({
			let self2 = new_self.clone();
			move |_| { self2.create(horz_resize, vert_resize)?; Ok(true) }
		});
		new_self
	}

	fn create(&self, horz: Horz, vert: Vert) -> WinResult<()> {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				multiply_dpi(Some(&mut pos), None)?;

				let mut sz = opts.size;
				if sz.cx == -1 && sz.cy == -1 {
					sz = calc_text_bound_box(&opts.text)?; // resize to fit text
				} else {
					multiply_dpi(None, Some(&mut sz))?; // user-defined size
				}

				let our_hwnd = self.0.base.create_window(
					"STATIC", Some(&opts.text), pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.label_style.into(),
				)?;

				our_hwnd.SendMessage(wm::SetFont { hfont: ui_font(), redraw: true });
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ())?,
		}

		self.0.base.parent_base().add_to_resizer(self.hwnd(), horz, vert)
	}

	/// Calls [`set_text`](crate::gui::TextControl::set_text) and resizes the control
	/// to exactly fit the new text.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::gui;
	///
	/// let my_label: gui::Label; // initialized somewhere
	///
	/// my_label.set_text_and_resize("This my text")?;
	/// ```
	pub fn set_text_and_resize(&self, text: &str) -> WinResult<()> {
		self.set_text(text)?;
		let bound_box = calc_text_bound_box(text)?;
		self.hwnd().SetWindowPos(
			HwndPlace::None, POINT::default(), bound_box,
			co::SWP::NOZORDER | co::SWP::NOMOVE)
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`Label`](crate::gui::Label) programmatically with
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
	/// Control size, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to the size needed to fit the text.
	pub size: SIZE,
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
	pub window_ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
	/// Horizontal behavior when the parent is resized.
	///
	/// Defaults to `Horz::None`.
	pub horz_resize: Horz,
	/// Vertical behavior when the parent is resized.
	///
	/// Defaults to `Vert::None`.
	pub vert_resize: Vert,
}

impl Default for LabelOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: POINT::new(0, 0),
			size: SIZE::new(-1, -1), // will resize to fit the text
			label_style: co::SS::LEFT | co::SS::NOTIFY,
			window_style: co::WS::CHILD | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			horz_resize: Horz::None,
			vert_resize: Vert::None,
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
