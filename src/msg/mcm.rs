//! Month calendar control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-month-calendar-control-reference-messages),
//! whose constants have [`MCM`](crate::co::MCM) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::msg::{MsgSend, WndMsg};
use crate::msg::macros::zero_as_err;
use crate::structs::{RECT, SYSTEMTIME};

/// [`MCM_GETCURSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getcursel)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetCurSel<'a> {
	pub info: &'a mut SYSTEMTIME,
}

impl<'a> MsgSend for GetCurSel<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCURSEL.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`MCM_GETMINREQRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getminreqrect)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetMinReqRect<'a> {
	pub bounding_rect: &'a mut RECT,
}

impl<'a> MsgSend for GetMinReqRect<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETMINREQRECT.into(),
			wparam: 0,
			lparam: self.bounding_rect as *const _ as _,
		}
	}
}

/// [`MCM_GETMONTHDELTA`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getmonthdelta)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetMonthDelta {}

impl MsgSend for GetMonthDelta {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETMONTHDELTA.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_SETCURSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-setcursel)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetCurSel<'a> {
	pub info: &'a SYSTEMTIME,
}

impl<'a> MsgSend for SetCurSel<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETCURSEL.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`MCM_SETCURRENTVIEW`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-setcurrentview)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetCurrentView {
	pub view: co::MCMV,
}

impl MsgSend for SetCurrentView {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETCURRENTVIEW.into(),
			wparam: 0,
			lparam: self.view.0 as _,
		}
	}
}

/// [`MCM_SETTODAY`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-settoday)
/// message parameters.
///
/// Return type: `()`.
pub struct SetToday<'a> {
	pub today: &'a SYSTEMTIME,
}

impl<'a> MsgSend for SetToday<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETTODAY.into(),
			wparam: 0,
			lparam: self.today as *const _ as _,
		}
	}
}
