use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::funcs::PostQuitMessage;
use crate::gui::events::DateTimePickerEvents;
use crate::gui::native_controls::base_native_control::{BaseNativeControl, OptsId};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi, ui_font};
use crate::gui::traits::{baseref_from_parent, Parent};
use crate::handles::HWND;
use crate::msg::{dtm, wm};
use crate::structs::{POINT, SIZE, SYSTEMTIME};

/// Native
/// [date and time picker](https://docs.microsoft.com/en-us/windows/win32/controls/date-and-time-picker-controls)
/// control.
///
/// Implements [`Child`](crate::gui::Child) trait.
#[derive(Clone)]
pub struct DateTimePicker(Arc<Obj>);

struct Obj { // actual fields of DateTimePicker
	base: BaseNativeControl,
	opts_id: OptsId<DateTimePickerOpts>,
	events: DateTimePickerEvents,
}

impl_send_sync_debug_child!(DateTimePicker);

impl DateTimePicker {
	/// Instantiates a new `DateTimePicker` object, to be created on the parent
	/// window with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: DateTimePickerOpts) -> DateTimePicker {
		let parent_base_ref = baseref_from_parent(parent);
		let opts = DateTimePickerOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Wnd(opts),
					events: DateTimePickerEvents::new(parent_base_ref, ctrl_id),
				},
			),
		);

		parent_base_ref.privileged_events_ref().wm(parent_base_ref.creation_wm(), {
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});

		new_self
	}

	/// Instantiates a new `DateTimePicker` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> DateTimePicker {
		let parent_base_ref = baseref_from_parent(parent);

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Dlg(ctrl_id),
					events: DateTimePickerEvents::new(parent_base_ref, ctrl_id),
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
					let mut sz = SIZE::new(opts.width as _, 21); // default height
					multiply_dpi(Some(&mut pos), Some(&mut sz))?;

					let our_hwnd = self.0.base.create_window( // may panic
						"SysDateTimePick32", None, pos, sz,
						opts.ctrl_id,
						opts.window_ex_style,
						opts.window_style | opts.date_time_picker_style.into(),
					)?;

					if sz.cx == 0 { // use ideal width?
						let mut sz_ideal = SIZE::default();
						our_hwnd.SendMessage(dtm::GetIdealSize { size: &mut sz_ideal });
						sz.cx = sz_ideal.cx; // already adjusted for DPI

						our_hwnd.SetWindowPos(HwndPlace::None, 0, 0, sz.cx, sz.cy,
							co::SWP::NOZORDER | co::SWP::NOMOVE)?;
					}

					our_hwnd.SendMessage(wm::SetFont{ hfont: ui_font(), redraw: true });
					Ok(())
				},
				OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

	pub_fn_hwnd!();
	pub_fn_ctrlid!();
	pub_fn_focus!();
	pub_fn_onsubclass!();
	pub_fn_on!(DateTimePickerEvents);

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
