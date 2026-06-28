use crate::co;
use crate::comctl::privs::*;
use crate::decl::*;
use crate::macros::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

pub_struct_msg_empty! { DtmCloseMonthCal: co::DTM::CLOSEMONTHCAL.into();
	/// [`DTM_CLOSEMONTHCAL`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-closemonthcal)
}

/// [`DTM_GETDATETIMEPICKERINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getdatetimepickerinfo)
/// message parameters.
///
/// Return type: `()`.
pub struct DtmGetDateTimePickerInfo<'a> {
	pub info: &'a mut DATETIMEPICKERINFO,
}

impl<'a> MsgSend for DtmGetDateTimePickerInfo<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::DTM::GETDATETIMEPICKERINFO.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`DTM_GETIDEALSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getidealsize)
/// message parameters.
///
/// Return type: `()`.
pub struct DtmGetIdealSize<'a> {
	pub size: &'a mut SIZE,
}

impl<'a> MsgSend for DtmGetIdealSize<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::DTM::GETIDEALSIZE.into(),
			wparam: 0,
			lparam: self.size as *mut _ as _,
		}
	}
}

/// [`DTM_GETMCCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getmccolor)
/// message parameters.
///
/// Return type: `SysResult<COLORREF>`.
pub struct DtmGetMcColor {
	pub color_index: co::MCSC,
}

impl MsgSend for DtmGetMcColor {
	type RetType = SysResult<COLORREF>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| unsafe { COLORREF::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::DTM::GETMCCOLOR.into(),
			wparam: self.color_index.raw() as _,
			lparam: 0,
		}
	}
}

/// [`DTM_GETMCSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getmcstyle)
/// message, which has no parameters.
///
/// Return type: `SysResult<co::MCS>`.
pub struct DtmGetMcStyle {}

impl MsgSend for DtmGetMcStyle {
	type RetType = SysResult<co::MCS>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|v| unsafe { co::MCS::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::DTM::GETMCSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`DTM_GETMONTHCAL`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getmonthcal)
/// message, which has no parameters.
///
/// Return type: `SysResult<HWND>`.
pub struct DtmGetMonthCal {}

impl MsgSend for DtmGetMonthCal {
	type RetType = SysResult<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::DTM::GETMONTHCAL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`DTM_GETRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getrange)
/// message parameters.
///
/// Return type: `co::GDTR`.
pub struct DtmGetRange<'a> {
	pub system_times: &'a mut [SYSTEMTIME; 2],
}

impl<'a> MsgSend for DtmGetRange<'a> {
	type RetType = co::GDTR;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::GDTR::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::DTM::GETRANGE.into(),
			wparam: 0,
			lparam: self.system_times as *mut _ as _,
		}
	}
}

/// [`DTM_GETSYSTEMTIME`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getsystemtime)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct DtmGetSystemTime<'a> {
	pub system_time: &'a mut SYSTEMTIME,
}

impl<'a> MsgSend for DtmGetSystemTime<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		const GDT_NONE: i32 = co::GDT::NONE.raw() as _;
		match v as i32 {
			GDT_ERROR => Err(co::ERROR::BAD_ARGUMENTS),
			GDT_NONE => Err(co::ERROR::INVALID_DATA),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::DTM::GETSYSTEMTIME.into(),
			wparam: 0,
			lparam: self.system_time as *mut _ as _,
		}
	}
}

/// [`DTM_SETFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-setformat)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct DtmSetFormat {
	pub format_string: Option<WString>,
}

impl MsgSend for DtmSetFormat {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::DTM::SETFORMAT.into(),
			wparam: 0,
			lparam: self.format_string.as_ref().map_or(0, |ws| ws.as_ptr() as _),
		}
	}
}

/// [`DTM_SETMCCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-setmccolor)
/// message parameters.
///
/// Return type: `SysResult<COLORREF>`.
pub struct DtmSetMcColor {
	pub color_index: co::MCSC,
	pub color: COLORREF,
}

impl MsgSend for DtmSetMcColor {
	type RetType = SysResult<COLORREF>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| unsafe { COLORREF::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::DTM::SETMCCOLOR.into(),
			wparam: self.color_index.raw() as _,
			lparam: u32::from(self.color) as _,
		}
	}
}

/// [`DTM_SETMCSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-setmcstyle)
/// message parameters.
///
/// Return type: `SysResult<co::MCS>`.
pub struct DtmSetMcStyle {
	pub style: co::MCS,
}

impl MsgSend for DtmSetMcStyle {
	type RetType = SysResult<co::MCS>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|v| unsafe { co::MCS::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::DTM::SETMCSTYLE.into(),
			wparam: 0,
			lparam: self.style.raw() as _,
		}
	}
}

/// [`DTM_SETRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-setrange)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct DtmSetRange<'a> {
	pub valid: co::GDTR,
	pub system_times: &'a mut [SYSTEMTIME; 2],
}

impl<'a> MsgSend for DtmSetRange<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::DTM::SETRANGE.into(),
			wparam: self.valid.raw() as _,
			lparam: self.system_times as *mut _ as _,
		}
	}
}

/// [`DTM_SETSYSTEMTIME`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-setsystemtime)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct DtmSetSystemTime<'a> {
	pub system_time: Option<&'a SYSTEMTIME>,
}

impl<'a> MsgSend for DtmSetSystemTime<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::DTM::SETSYSTEMTIME.into(),
			wparam: self
				.system_time
				.map_or(co::GDT::NONE.raw(), |_| co::GDT::VALID.raw()) as _,
			lparam: self.system_time.map_or(0, |st| st as *const _ as _),
		}
	}
}
