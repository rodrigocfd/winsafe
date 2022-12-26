use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::gui::base::Base;
use crate::gui::events::{DateTimePickerEvents, WindowEvents};
use crate::gui::layout_arranger::{Horz, Vert};
use crate::gui::native_controls::base_native_control::{
	BaseNativeControl, OptsId,
};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi_or_dtu, ui_font};
use crate::kernel::decl::SYSTEMTIME;
use crate::msg::{dtm, wm};
use crate::prelude::{
	GuiChild, GuiChildFocus, GuiEvents, GuiNativeControl,
	GuiNativeControlEvents, GuiParent, GuiWindow, Handle, user_Hwnd,
};
use crate::user::decl::{HWND, HwndPlace, POINT, SIZE};

struct Obj { // actual fields of DateTimePicker
	base: BaseNativeControl,
	opts_id: OptsId<DateTimePickerOpts>,
	events: DateTimePickerEvents,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Native
/// [date and time picker](https://learn.microsoft.com/en-us/windows/win32/controls/date-and-time-picker-controls)
/// control.
#[derive(Clone)]
pub struct DateTimePicker(Pin<Arc<Obj>>);

unsafe impl Send for DateTimePicker {}

impl GuiWindow for DateTimePicker {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiChild for DateTimePicker {
	fn ctrl_id(&self) -> u16 {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}
}

impl GuiChildFocus for DateTimePicker {}

impl GuiNativeControl for DateTimePicker {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl GuiNativeControlEvents<DateTimePickerEvents> for DateTimePicker {
	fn on(&self) -> &DateTimePickerEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl DateTimePicker {
	/// Instantiates a new `DateTimePicker` object, to be created on the parent
	/// window with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `DateTimePicker` in an event closure.
	#[must_use]
	pub fn new(
		parent: &impl GuiParent, opts: DateTimePickerOpts) -> DateTimePicker
	{
		let parent_ref = unsafe { Base::from_guiparent(parent) };
		let opts = DateTimePickerOpts::define_ctrl_id(opts);
		let (ctrl_id, horz, vert) = (opts.ctrl_id, opts.horz_resize, opts.vert_resize);

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_ref),
					opts_id: OptsId::Wnd(opts),
					events: DateTimePickerEvents::new(parent_ref, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_ref.privileged_on().wm(parent_ref.creation_msg(), move |_| {
			self2.create(horz, vert);
			Ok(None) // not meaningful
		});

		new_self
	}

	/// Instantiates a new `DateTimePicker` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `DateTimePicker` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert)) -> DateTimePicker
	{
		let parent_ref = unsafe { Base::from_guiparent(parent) };

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_ref),
					opts_id: OptsId::Dlg(ctrl_id),
					events: DateTimePickerEvents::new(parent_ref, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_ref.privileged_on().wm_init_dialog(move |_| {
			self2.create(resize_behavior.0, resize_behavior.1);
			Ok(true) // not meaningful
		});

		new_self
	}

	fn create(&self, horz: Horz, vert: Vert) {
		if vert == Vert::Resize {
			panic!("DateTimePicker cannot be resized with Vert::Resize.");
		}

		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				let mut sz = SIZE::new(opts.width as _, 21); // default height
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), Some(&mut sz));

				self.0.base.create_window(
					"SysDateTimePick32", None, pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.date_time_picker_style.into(),
				);

				if sz.cx == 0 { // use ideal width?
					let mut sz_ideal = SIZE::default();
					self.hwnd().SendMessage(dtm::GetIdealSize {
						size: &mut sz_ideal,
					});
					sz.cx = sz_ideal.cx; // already adjusted for DPI

					self.hwnd().SetWindowPos(
						HwndPlace::None, POINT::default(), sz,
						co::SWP::NOZORDER | co::SWP::NOMOVE).unwrap();
				}

				self.hwnd().SendMessage(wm::SetFont {
					hfont: unsafe { ui_font().raw_copy() },
					redraw: true,
				});
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id),
		}

		self.0.base.parent().add_to_layout_arranger(self.hwnd(), horz, vert);
	}

	/// Retrieves the currently selected date by sending a
	/// [`dtm::GetSystemTime`](crate::msg::dtm::GetSystemTime) message.
	pub fn date(&self, st: &mut SYSTEMTIME) {
		self.hwnd()
			.SendMessage(dtm::GetSystemTime { system_time: st })
			.unwrap()
	}

	/// Sets the currently selected date by sending a
	/// [`dtm::SetSystemTime`](crate::msg::dtm::SetSystemTime) message.
	pub fn set_date(&self, st: &SYSTEMTIME) {
		self.hwnd()
			.SendMessage(dtm::SetSystemTime { system_time: Some(st) })
			.unwrap()
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`DateTimePicker`](crate::gui::DateTimePicker)
/// programmatically with
/// [`DateTimePicker::new`](crate::gui::DateTimePicker::new).
pub struct DateTimePickerOpts {
	/// Control position within parent client area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Control width, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to ideal width retrieved with
	/// [`dtm::GetIdealSize`](crate::msg::dtm::GetIdealSize) message, usually
	/// around 250.
	pub width: u32,
	/// Date and time picker styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `DTS::LONGDATEFORMAT`.
	pub date_time_picker_style: co::DTS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
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
	/// Horizontal behavior when the parent is resized.
	///
	/// Defaults to `Horz::None`.
	pub horz_resize: Horz,
	/// Vertical behavior when the parent is resized.
	///
	/// Defaults to `Vert::None`.
	///
	/// **Note:** A `DateTimePicker` cannot be resized vertically, so it will
	/// panic if you use `Vert::Resize`.
	pub vert_resize: Vert,
}

impl Default for DateTimePickerOpts {
	fn default() -> Self {
		Self {
			position: POINT::new(0, 0),
			width: 0,
			date_time_picker_style: co::DTS::LONGDATEFORMAT,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			horz_resize: Horz::None,
			vert_resize: Vert::None,
		}
	}
}

impl DateTimePickerOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
