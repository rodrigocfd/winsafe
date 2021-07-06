//! Date and time picker control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-messages),
//! whose constants have [`DTM`](crate::co::DTM) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::handles::{HWND, HFONT};
use crate::msg::{MsgSend, WndMsg};
use crate::msg::macros::zero_as_err;
use crate::privs::GDT_ERROR;
use crate::structs::{COLORREF, DATETIMEPICKERINFO, SIZE, SYSTEMTIME};
use crate::various::WString;

pub_struct_msg_empty! { CloseMonthCal, co::DTM::CLOSEMONTHCAL.into(),
	/// [`DTM_CLOSEMONTHCAL`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-closemonthcal)
}

/// [`DTM_GETDATETIMEPICKERINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getdatetimepickerinfo)
/// message parameters.
///
/// Return type: `()`.
pub struct GetDateTimePickerInfo<'a> {
	pub info: &'a mut DATETIMEPICKERINFO,
}

impl<'a> MsgSend for GetDateTimePickerInfo<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETDATETIMEPICKERINFO.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`DTM_GETIDEALSIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getidealsize)
/// message parameters.
///
/// Return type: `()`.
pub struct GetIdealSize<'a> {
	pub size: &'a mut SIZE,
}

impl<'a> MsgSend for GetIdealSize<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETIDEALSIZE.into(),
			wparam: 0,
			lparam: self.size as *const _ as _,
		}
	}
}

/// [`DTM_GETMCCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getmccolor)
/// message parameters.
///
/// Return type: `WinResult<COLORREF>`.
pub struct GetMcColor {
	pub color_index: co::MCSC,
}

impl MsgSend for GetMcColor {
	type RetType = WinResult<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			rgb => Ok(COLORREF(rgb as _)),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETMCCOLOR.into(),
			wparam: self.color_index.0 as _,
			lparam: 0,
		}
	}
}

/// [`DTM_GETMCFONT`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getmcfont)
/// message, which has no parameters.
///
/// Return type: `WinResult<HFONT>`.
pub struct GetMcFont {}

impl MsgSend for GetMcFont {
	type RetType = WinResult<HFONT>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HFONT { ptr: p as _ })
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETMCFONT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`DTM_GETMCSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getmcstyle)
/// message, which has no parameters.
///
/// Return type: `WinResult<co::MCS>`.
pub struct GetMcStyle {}

impl MsgSend for GetMcStyle {
	type RetType = WinResult<co::MCS>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			v => Ok(co::MCS(v as _)),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETMCSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`DTM_GETMONTHCAL`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getmonthcal)
/// message, which has no parameters.
///
/// Return type: `WinResult<HWND>`.
pub struct GetMonthCalendar {}

impl MsgSend for GetMonthCalendar {
	type RetType = WinResult<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HWND { ptr: p as _ })
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETMONTHCAL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`DTM_GETRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getrange)
/// message parameters.
///
/// Return type: `co::GDTR`.
pub struct GetRange<'a> {
	pub system_times: &'a mut [SYSTEMTIME; 2],
}

impl<'a> MsgSend for GetRange<'a> {
	type RetType = co::GDTR;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::GDTR(v as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETRANGE.into(),
			wparam: 0,
			lparam: self.system_times as *const _ as _,
		}
	}
}

/// [`DTM_GETSYSTEMTIME`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getsystemtime)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetSystemTime<'a> {
	pub system_time: &'a mut SYSTEMTIME,
}

impl<'a> MsgSend for GetSystemTime<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		const GDT_NONE: i32 = co::GDT::NONE.0 as _;
		match v as i32 {
			GDT_ERROR => Err(co::ERROR::BAD_ARGUMENTS),
			GDT_NONE => Err(co::ERROR::INVALID_DATA),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETSYSTEMTIME.into(),
			wparam: 0,
			lparam: self.system_time as *const _ as _,
		}
	}
}

/// [`DTM_SETFORMAT`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setformat)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetFormat<'a> {
	pub format_string: Option<&'a str>,
}

impl<'a> MsgSend for SetFormat<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETFORMAT.into(),
			wparam: 0,
			lparam: match self.format_string {
				None => 0,
				Some(fmtstr) => (unsafe { WString::from_str(fmtstr).as_ptr() }) as _,
			},
		}
	}
}

/// [`DTM_SETMCCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setmccolor)
/// message parameters.
///
/// Return type: `WinResult<COLORREF>`.
pub struct SetMcColor {
	pub color_index: co::MCSC,
	pub color: COLORREF,
}

impl MsgSend for SetMcColor {
	type RetType = WinResult<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			rgb => Ok(COLORREF(rgb as _)),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETMCCOLOR.into(),
			wparam: self.color_index.0 as _,
			lparam: self.color.0 as _,
		}
	}
}

/// [`DTM_SETMCFONT`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setmcfont)
/// message parameters.
///
/// Return type: `()`.
pub struct SetMcFont {
	pub hfont: HFONT,
	pub redraw: bool,
}

impl MsgSend for SetMcFont {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETMCFONT.into(),
			wparam: self.hfont.ptr as _,
			lparam: self.redraw as _,
		}
	}
}

/// [`DTM_SETMCSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setmcstyle)
/// message parameters.
///
/// Return type: `WinResult<co::MCS>`.
pub struct SetMcStyle {
	pub style: co::MCS,
}

impl MsgSend for SetMcStyle {
	type RetType = WinResult<co::MCS>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			v => Ok(co::MCS(v as _)),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETMCSTYLE.into(),
			wparam: 0,
			lparam: self.style.0 as _,
		}
	}
}

/// [`DTM_SETRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setrange)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetRange<'a> {
	pub valid: co::GDTR,
	pub system_times: &'a mut [SYSTEMTIME; 2],
}

impl<'a> MsgSend for SetRange<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETRANGE.into(),
			wparam: self.valid.0 as _,
			lparam: self.system_times as *const _ as _,
		}
	}
}

/// [`DTM_SETSYSTEMTIME`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setsystemtime)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetSystemTime<'a> {
	pub system_time: Option<&'a SYSTEMTIME>,
}

impl<'a> MsgSend for SetSystemTime<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETSYSTEMTIME.into(),
			wparam: self.system_time.as_ref().map_or(co::GDT::NONE.0, |_| co::GDT::VALID.0) as _,
			lparam: self.system_time.as_ref().map_or(0, |st| st as *const _ as _),
		}
	}
}
