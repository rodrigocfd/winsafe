use crate::aliases::WinResult;
use crate::co;
use crate::handles::HFONT;
use crate::msg::{Message, Wm};
use crate::msg::macros::ref_to_lp;
use crate::structs::{COLORREF, DATETIMEPICKERINFO, SIZE};

empty_msg! { DtmCloseMonthCal, co::WM::DTM_CLOSEMONTHCAL,
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
			msg_id: co::WM::DTM_GETDATETIMEPICKERINFO,
			wparam: 0,
			lparam: ref_to_lp(self.info),
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
			msg_id: co::WM::DTM_GETIDEALSIZE,
			wparam: 0,
			lparam: ref_to_lp(self.size),
		}
	}
}

//------------------------------------------------------------------------------

/// [`DTM_GETMCCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getmccolor)
/// message parameters.
///
/// Return type: `WinResult<COLORREF>`.
pub struct DtmGetMcColor {
	pub color: co::MCSC,
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
			msg_id: co::WM::DTM_GETMCCOLOR,
			wparam: u8::from(self.color) as usize,
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
			msg_id: co::WM::DTM_GETMCFONT,
			wparam: 0,
			lparam: 0,
		}
	}
}
