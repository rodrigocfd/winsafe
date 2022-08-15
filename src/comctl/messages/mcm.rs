use crate::co;
use crate::kernel::decl::{SysResult, SYSTEMTIME};
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::decl::RECT;
use crate::user::privs::zero_as_err;

/// [`MCM_GETCURSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getcursel)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetCurSel<'a> {
	pub info: &'a mut SYSTEMTIME,
}

unsafe impl<'a> MsgSend for GetCurSel<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCURSEL.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`MCM_GETMINREQRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getminreqrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetMinReqRect<'a> {
	pub bounds_rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetMinReqRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETMINREQRECT.into(),
			wparam: 0,
			lparam: self.bounds_rect as *mut _ as _,
		}
	}
}

/// [`MCM_GETMONTHDELTA`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getmonthdelta)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetMonthDelta {}

unsafe impl MsgSend for GetMonthDelta {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
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
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetCurSel<'a> {
	pub info: &'a SYSTEMTIME,
}

unsafe impl<'a> MsgSend for SetCurSel<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
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
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetCurrentView {
	pub view: co::MCMV,
}

unsafe impl MsgSend for SetCurrentView {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
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
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetToday<'a> {
	pub today: &'a SYSTEMTIME,
}

unsafe impl<'a> MsgSend for SetToday<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETTODAY.into(),
			wparam: 0,
			lparam: self.today as *const _ as _,
		}
	}
}
