use std::any::Any;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::gui::events::{DateTimePickerEvents, EventsView, WindowEvents};
use crate::gui::native_controls::base_native_control::{BaseNativeControl, OptsId};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi, ui_font};
use crate::gui::resizer::{Horz, Vert};
use crate::gui::traits::{
	AsAny,
	Child,
	FocusControl,
	NativeControl,
	NativeControlEvents,
	Parent,
	Window,
};
use crate::handles::{Handle, HWND};
use crate::msg::{dtm, wm};
use crate::structs::{POINT, SIZE, SYSTEMTIME};

/// Native
/// [date and time picker](https://docs.microsoft.com/en-us/windows/win32/controls/date-and-time-picker-controls)
/// control.
#[derive(Clone)]
pub struct DateTimePicker(Arc<Obj>);

struct Obj { // actual fields of DateTimePicker
	base: BaseNativeControl,
	opts_id: OptsId<DateTimePickerOpts>,
	events: DateTimePickerEvents,
}

unsafe impl Send for DateTimePicker {}

impl AsAny for DateTimePicker {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl Window for DateTimePicker {
	fn hwnd(&self) -> HWND {
		self.0.base.hwnd()
	}
}

impl Child for DateTimePicker {
	fn ctrl_id(&self) -> u16 {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}
}

impl NativeControl for DateTimePicker {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl NativeControlEvents<DateTimePickerEvents> for DateTimePicker {
	fn on(&self) -> &DateTimePickerEvents {
		if !self.0.base.hwnd().is_null() {
			panic!("Cannot add events after the control creation.");
		} else if !self.0.base.parent_base().hwnd().is_null() {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl FocusControl for DateTimePicker {}

impl DateTimePicker {
	/// Instantiates a new `DateTimePicker` object, to be created on the parent
	/// window with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(
		parent: &impl Parent, opts: DateTimePickerOpts) -> DateTimePicker
	{
		let opts = DateTimePickerOpts::define_ctrl_id(opts);
		let (ctrl_id, horz, vert) = (opts.ctrl_id, opts.horz_resize, opts.vert_resize);
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Wnd(opts),
					events: DateTimePickerEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm(parent.as_base().wmcreate_or_wminitdialog(), {
			let self2 = new_self.clone();
			move |_| { self2.create(horz, vert)?; Ok(0) }
		});
		new_self
	}

	/// Instantiates a new `DateTimePicker` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(
		parent: &impl Parent, ctrl_id: u16,
		horz_resize: Horz, vert_resize: Vert) -> DateTimePicker
	{
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Dlg(ctrl_id),
					events: DateTimePickerEvents::new(parent.as_base(), ctrl_id),
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
		if vert == Vert::Resize {
			panic!("DateTimePicker cannot be resized with Vert::Resize.");
		}

		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				let mut sz = SIZE::new(opts.width as _, 21); // default height
				multiply_dpi(Some(&mut pos), Some(&mut sz))?;

				let our_hwnd = self.0.base.create_window(
					"SysDateTimePick32", None, pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.date_time_picker_style.into(),
				)?;

				if sz.cx == 0 { // use ideal width?
					let mut sz_ideal = SIZE::default();
					our_hwnd.SendMessage(dtm::GetIdealSize { size: &mut sz_ideal });
					sz.cx = sz_ideal.cx; // already adjusted for DPI

					our_hwnd.SetWindowPos(HwndPlace::None, POINT::default(), sz,
						co::SWP::NOZORDER | co::SWP::NOMOVE)?;
				}

				our_hwnd.SendMessage(wm::SetFont { hfont: ui_font(), redraw: true });
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ())?,
		}

		self.0.base.parent_base().add_to_resizer(self.hwnd(), horz, vert)
	}

	/// Retrieves the currently selected date by sending a
	/// [`dtm::GetSystemTime`](crate::msg::dtm::GetSystemTime) message.
	pub fn date(&self, st: &mut SYSTEMTIME) -> WinResult<()> {
		self.hwnd().SendMessage(dtm::GetSystemTime { system_time: st })
	}

	/// Sets the currently selected date by sending a
	/// [`dtm::SetSystemTime`](crate::msg::dtm::SetSystemTime) message.
	pub fn set_date(&self, st: &SYSTEMTIME) -> WinResult<()> {
		self.hwnd().SendMessage(dtm::SetSystemTime { system_time: Some(st) })
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`DateTimePicker`](crate::gui::DateTimePicker)
/// programmatically with
/// [`DateTimePicker::new`](crate::gui::DateTimePicker::new).
pub struct DateTimePickerOpts {
	/// Control position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Control width, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to ideal width retrieved with
	/// [`dtm::GetIdealSize`](crate::msg::dtm::GetIdealSize) message, usually
	/// around 250.
	pub width: u32,
	/// Date and time picker styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `DTS::LONGDATEFORMAT`.
	pub date_time_picker_style: co::DTS,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
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
