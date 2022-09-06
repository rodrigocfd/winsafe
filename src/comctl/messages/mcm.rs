use crate::co;
use crate::comctl::decl::{MCGRIDINFO, MCHITTESTINFO};
use crate::kernel::decl::{HIWORD, LOWORD, SysResult, SYSTEMTIME};
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::decl::{COLORREF, RECT};
use crate::user::privs::zero_as_err;

/// [`MCM_GETCALENDARBORDER`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getcalendarborder)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetCalendarBorder {}

unsafe impl MsgSend for GetCalendarBorder {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCALENDARBORDER.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETCALENDARCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getcalendarcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetCalendarCount {}

unsafe impl MsgSend for GetCalendarCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCALENDARCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETCALENDARGRIDINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getcalendargridinfo)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetCalendarGridInfo<'a, 'b> {
	pub info: &'b mut MCGRIDINFO<'a>,
}

unsafe impl<'a, 'b> MsgSend for GetCalendarGridInfo<'a, 'b> {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCALENDARGRIDINFO.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`MCM_GETCALID`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getcalid)
/// message, which has no parameters.
///
/// Return type: `co::CAL`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetCalId {}

unsafe impl MsgSend for GetCalId {
	type RetType = co::CAL;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::CAL(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCALID.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getcolor)
/// message parameters.
///
/// Return type: `SysResult<COLORREF>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetColor {
	pub which: co::MCSC,
}

unsafe impl MsgSend for GetColor {
	type RetType = SysResult<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			v => Ok(COLORREF(v as _)),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCOLOR.into(),
			wparam: self.which.0 as _,
			lparam: 0,
		}
	}
}

/// [`MCM_GETCURRENTVIEW`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getcurrentview)
/// message, which has no parameters.
///
/// Return type: `co::MCMV`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetCurrentView {}

unsafe impl MsgSend for GetCurrentView {
	type RetType = co::MCMV;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::MCMV(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCURRENTVIEW.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

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

/// [`MCM_GETFIRSTDAYOFWEEK`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getfirstdayofweek)
/// message, which has no parameters.
///
/// Return type: `(bool, u16)`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetFirstDayOfWeek {}

unsafe impl MsgSend for GetFirstDayOfWeek {
	type RetType = (bool, u16);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(HIWORD(v as _) != 0, LOWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETFIRSTDAYOFWEEK.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETMAXSELCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getmaxselcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetMaxSelCount {}

unsafe impl MsgSend for GetMaxSelCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETMAXSELCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETMAXTODAYWIDTH`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getmaxtodaywidth)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetMaxTodayWidth {}

unsafe impl MsgSend for GetMaxTodayWidth {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETMAXTODAYWIDTH.into(),
			wparam: 0,
			lparam: 0,
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

/// [`MCM_GETMONTHRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getmonthrange)
/// message parameters.
///
/// Return value: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetMonthRange<'a> {
	pub scope: co::GMR,
	pub limits: &'a mut [SYSTEMTIME; 2],
}

unsafe impl<'a> MsgSend for GetMonthRange<'a> {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETMONTHRANGE.into(),
			wparam: self.scope.0 as _,
			lparam: self.limits.as_ptr() as _,
		}
	}
}

/// [`MCM_GETRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getrange)
/// message parameters.
///
/// Return type: `co::GDTR`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetRange<'a> {
	pub limits: &'a mut [SYSTEMTIME; 2],
}

unsafe impl<'a> MsgSend for GetRange<'a> {
	type RetType = co::GDTR;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::GDTR(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETRANGE.into(),
			wparam: 0,
			lparam: self.limits.as_ptr() as _,
		}
	}
}

/// [`MCM_GETSELRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getselrange)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetSelRange<'a> {
	pub limits: &'a mut [SYSTEMTIME; 2],
}

unsafe impl<'a> MsgSend for GetSelRange<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETSELRANGE.into(),
			wparam: 0,
			lparam: self.limits.as_ptr() as _,
		}
	}
}

/// [`MCM_GETTODAY`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-gettoday)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetToday<'a> {
	pub info: &'a mut SYSTEMTIME,
}

unsafe impl<'a> MsgSend for GetToday<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETTODAY.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`MCM_GETUNICODEFORMAT`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-getunicodeformat)
/// message, which has no parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetUnicodeFormat {}

unsafe impl MsgSend for GetUnicodeFormat {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_HITTEST`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-hittest)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct HitTest<'a> {
	pub test_info: &'a mut MCHITTESTINFO,
}

unsafe impl<'a> MsgSend for HitTest<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::HITTEST.into(),
			wparam: 0,
			lparam: self.test_info as *mut _ as _,
		}
	}
}

/// [`MCM_SETCALENDARBORDER`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-setcalendarborder)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetCalendarBorder {
	pub border: bool,
	pub pixels: u32,
}

unsafe impl MsgSend for SetCalendarBorder {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETCALENDARBORDER.into(),
			wparam: self.border as _,
			lparam: self.pixels as _,
		}
	}
}

/// [`MCM_SETCALID`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-setcalid)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetCalId {
	pub id: co::CAL,
}

unsafe impl MsgSend for SetCalId {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETCALID.into(),
			wparam: self.id.0 as _,
			lparam: 0,
		}
	}
}

/// [`MCM_SETCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/mcm-setcolor)
/// message parameters.
///
/// Return type: `Option<COLORREF>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetColor {
	pub which: co::MCSC,
	pub color: COLORREF,
}

unsafe impl MsgSend for SetColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => None,
			v => Some(COLORREF(v as _)),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETCOLOR.into(),
			wparam: self.which.0 as _,
			lparam: self.color.0 as _,
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
