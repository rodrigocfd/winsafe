use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::funcs::PostQuitMessage;
use crate::gui::events::LabelEvents;
use crate::gui::native_controls::base_native_control::{BaseNativeControl, OptsId};
use crate::gui::privs::{auto_ctrl_id, calc_text_bound_box, multiply_dpi, ui_font};
use crate::gui::traits::{baseref_from_parent, Parent};
use crate::handles::HWND;
use crate::msg::wm;
use crate::structs::{POINT, SIZE};
use crate::WString;

/// Native
/// [label](https://docs.microsoft.com/en-us/windows/win32/controls/about-static-controls)
/// control.
///
/// Implements [`Child`](crate::gui::Child) trait.
#[derive(Clone)]
pub struct Label(Arc<Obj>);

struct Obj { // actual fields of Label
	base: BaseNativeControl,
	opts_id: OptsId<LabelOpts>,
	events: LabelEvents,
}

impl_send_sync_child!(Label);

impl Label {
	/// Instantiates a new `Label` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: LabelOpts) -> Label {
		let parent_base_ref = baseref_from_parent(parent);
		let opts = LabelOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Wnd(opts),
					events: LabelEvents::new(parent_base_ref, ctrl_id),
				},
			),
		);

		parent_base_ref.privileged_events_ref().wm(parent_base_ref.creation_wm(), {
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});

		new_self
	}

	/// Instantiates a new `CheckBox` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> Label {
		let parent_base_ref = baseref_from_parent(parent);

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Dlg(ctrl_id),
					events: LabelEvents::new(parent_base_ref, ctrl_id),
				},
			),
		);

		parent_base_ref.privileged_events_ref().wm_init_dialog({
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

					let mut sz = opts.size;
					if sz.cx == -1 && sz.cy == -1 {
						sz = calc_text_bound_box(&opts.text)?; // resize to fit text
					} else {
						multiply_dpi(None, Some(&mut sz))?; // user-defined size
					}

					let our_hwnd = self.0.base.create_window( // may panic
						"STATIC", Some(&opts.text), pos, sz,
						opts.ctrl_id,
						opts.window_ex_style,
						opts.window_style | opts.label_style.into(),
					)?;

					our_hwnd.SendMessage(wm::SetFont{ hfont: ui_font(), redraw: true });
					Ok(())
				},
				OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

	pub_fn_ctrlid_hwnd_on_onsubclass!(LabelEvents);

	/// Resizes the control to exactly fit current text.
	pub fn resize_to_text(&self) -> WinResult<()> {
		self.resize_to_given_text(&self.text_str()?)
	}

	/// Sets the text by calling [`SetWindowText`](crate::HWND::SetWindowText).
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui;
	///
	/// let my_label: gui::Label; // initialized somewhere
	///
	/// my_label.set_text("This my text").unwrap();
	/// ```
	pub fn set_text(&self, text: &str) -> WinResult<()> {
		self.hwnd().SetWindowText(text)
	}

	/// Calls [`set_text`](crate::gui::Label::set_text) and resizes the control
	/// to exactly fit the new text.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui;
	///
	/// let my_label: gui::Label; // initialized somewhere
	///
	/// my_label.set_text_and_resize("This my text").unwrap();
	/// ```
	pub fn set_text_and_resize(&self, text: &str) -> WinResult<()> {
		self.set_text(text)?;
		self.resize_to_given_text(text)
	}

	/// Retrieves the text in the control by calling
	/// [`GetWindowText`](crate::HWND::GetWindowText).
	///
	/// The passed buffer will be automatically allocated.
	///
	/// This method can be more performant than
	/// [`text_str`](crate::gui::Label::text_str) because the buffer can be
	/// reused, avoiding multiple allocations. However, it has the inconvenient
	/// of the manual conversion from [`WString`](crate::WString) to `String`.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{gui, WString};
	///
	/// let my_label: gui::Label; // initialized somewhere
	///
	/// let mut buf = WString::default;
	/// my_label.text(&mut buf).unwrap();
	///
	/// println!("The text is: {}", buf.to_string());
	/// ```
	pub fn text(&self, buf: &mut WString) -> WinResult<()> {
		self.hwnd().GetWindowText(buf).map(|_| ())
	}

	/// A more convenient [`text`](crate::gui::Label::text), which directly
	/// returns a `String` instead of requiring an external buffer.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui;
	///
	/// let my_label: gui::Label; // initialized somewhere
	///
	/// println!("The text is: {}", my_label.text().unwrap());
	/// ```
	pub fn text_str(&self) -> WinResult<String> {
		self.hwnd().GetWindowTextStr()
	}

	fn resize_to_given_text(&self, text: &str) -> WinResult<()> {
		let bound_box = calc_text_bound_box(text)?;
		self.hwnd().SetWindowPos(
			HwndPlace::None, 0, 0, bound_box.cx, bound_box.cy,
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
	pub window_ex_style: co::WS_EX,

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
			size: SIZE::new(-1, -1), // will resize to fit the text
			baseline_text_align: false,
			label_style: co::SS::LEFT | co::SS::NOTIFY,
			window_style: co::WS::CHILD | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
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