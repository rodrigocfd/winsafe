use crate::aliases::WinResult;
use crate::co;
use crate::handles::{HWND, HFONT};
use crate::msg::{Message, Wm};
use crate::privs::GDT_ERROR;
use crate::structs::{COLORREF, DATETIMEPICKERINFO, SIZE, SYSTEMTIME};
use crate::WString;

empty_msg! { DtmCloseMonthCal, co::DTM::CLOSEMONTHCAL.into(),
	/// [`DTM_CLOSEMONTHCAL`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-closemonthcal)
	/// message, which has no parameters.
	///
	/// Return type: `WinResult<()>`.
}

//------------------------------------------------------------------------------

/// [`DTM_GETDATETIMEPICKERINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getdatetimepickerinfo)
/// message parameters.
///
/// Return type: `()`.
pub struct DtmGetDateTimePickerInfo<'a> {
	pub info: &'a mut DATETIMEPICKERINFO,
}

impl<'a> Message for DtmGetDateTimePickerInfo<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::DTM::GETDATETIMEPICKERINFO.into(),
			wparam: 0,
			lparam: self.info as *const _ as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`DTM_GETIDEALSIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getidealsize)
/// message parameters.
///
/// Return type: `()`.
pub struct DtmGetIdealSize<'a> {
	pub size: &'a mut SIZE,
}

impl<'a> Message for DtmGetIdealSize<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::DTM::GETIDEALSIZE.into(),
			wparam: 0,
			lparam: self.size as *const _ as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`DTM_GETMCCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getmccolor)
/// message parameters.
///
/// Return type: `WinResult<COLORREF>`.
pub struct DtmGetMcColor {
	pub color_index: co::MCSC,
}

impl Message for DtmGetMcColor {
	type RetType = WinResult<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			rgb => Ok(COLORREF(rgb as u32)),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::DTM::GETMCCOLOR.into(),
			wparam: self.color_index.0 as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`DTM_GETMCFONT`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getmcfont)
/// message, which has no parameters.
///
/// Return type: `WinResult<HFONT>`.
pub struct DtmGetMcFont {}

impl Message for DtmGetMcFont {
	type RetType = WinResult<HFONT>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			p => Ok(HFONT { ptr: p as *mut _ }),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::DTM::GETMCFONT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`DTM_GETMCSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getmcstyle)
/// message, which has no parameters.
///
/// Return type: `WinResult<MCS>`.
pub struct DtmGetMcStyle {}

impl Message for DtmGetMcStyle {
	type RetType = WinResult<co::MCS>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			v => Ok(co::MCS(v as u32)),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::DTM::GETMCSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`DTM_GETMONTHCAL`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getmonthcal)
/// message, which has no parameters.
///
/// Return type: `WinResult<HWND>`.
pub struct DtmGetMonthCalendar {}

impl Message for DtmGetMonthCalendar {
	type RetType = WinResult<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			p => Ok(HWND { ptr: p as *mut _ }),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::DTM::GETMONTHCAL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`DTM_GETRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getrange)
/// message parameters.
///
/// Return type: `GDTR`.
pub struct DtmGetRange<'a> {
	pub system_times: &'a mut [SYSTEMTIME; 2],
}

impl<'a> Message for DtmGetRange<'a> {
	type RetType = co::GDTR;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::GDTR(v as u32)
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::DTM::GETRANGE.into(),
			wparam: 0,
			lparam: self.system_times as *const _ as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`DTM_GETSYSTEMTIME`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getsystemtime)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct DtmGetSystemTime<'a> {
	pub system_time: &'a mut SYSTEMTIME,
}

impl<'a> Message for DtmGetSystemTime<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		const GDT_NONE: i32 = co::GDT::NONE.0 as i32;
		match v as i32 {
			GDT_ERROR => Err(co::ERROR::BAD_ARGUMENTS),
			GDT_NONE => Err(co::ERROR::INVALID_DATA),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::DTM::GETSYSTEMTIME.into(),
			wparam: 0,
			lparam: self.system_time as *const _ as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`DTM_SETFORMAT`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setformat)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct DtmSetFormat<'a> {
	pub format_string: Option<&'a str>,
}

impl<'a> Message for DtmSetFormat<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::DTM::SETFORMAT.into(),
			wparam: 0,
			lparam: match self.format_string {
				None => 0,
				Some(fmtstr) => (unsafe { WString::from_str(fmtstr).as_ptr() }) as isize,
			},
		}
	}
}

//------------------------------------------------------------------------------

/// [`DTM_SETMCCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setmccolor)
/// message parameters.
///
/// Return type: `WinResult<COLORREF>`.
pub struct DtmSetMcColor {
	pub color_index: co::MCSC,
	pub color: COLORREF,
}

impl Message for DtmSetMcColor {
	type RetType = WinResult<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			rgb => Ok(COLORREF(rgb as u32)),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::DTM::SETMCCOLOR.into(),
			wparam: self.color_index.0 as usize,
			lparam: self.color.0 as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`DTM_SETMCFONT`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setmcfont)
/// message parameters.
///
/// Return type: `()`.
pub struct DtmSetMcFont {
	pub hfont: HFONT,
	pub redraw: bool,
}

impl Message for DtmSetMcFont {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::DTM::SETMCFONT.into(),
			wparam: self.hfont.ptr as usize,
			lparam: self.redraw as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`DTM_SETMCSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setmcstyle)
/// message parameters.
///
/// Return type: `WinResult<MCS>`.
pub struct DtmSetMcStyle {
	pub style: co::MCS,
}

impl Message for DtmSetMcStyle {
	type RetType = WinResult<co::MCS>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			s => Ok(co::MCS(s as u32)),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::DTM::SETMCSTYLE.into(),
			wparam: 0,
			lparam: self.style.0 as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`DTM_SETRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setrange)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct DtmSetRange<'a> {
	pub valid: co::GDTR,
	pub system_times: &'a mut [SYSTEMTIME; 2],
}

impl<'a> Message for DtmSetRange<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::DTM::SETRANGE.into(),
			wparam: self.valid.0 as usize,
			lparam: self.system_times as *const _ as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`DTM_SETSYSTEMTIME`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setsystemtime)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct DtmSetSystemTime<'a> {
	pub system_time: Option<&'a SYSTEMTIME>,
}

impl<'a> Message for DtmSetSystemTime<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::DTM::SETSYSTEMTIME.into(),
			wparam: match self.system_time {
				None => co::GDT::NONE.0,
				Some(_) => co::GDT::VALID.0,
			} as usize,
			lparam: match self.system_time {
				None => 0,
				Some(st) => st as *const _ as isize,
			},
		}
	}
}
