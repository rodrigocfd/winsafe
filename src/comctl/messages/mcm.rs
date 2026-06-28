use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`MCM_GETCALENDARBORDER`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcalendarborder)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct McmGetCalendarBorder {}

impl MsgSend for McmGetCalendarBorder {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETCALENDARBORDER.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETCALENDARCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcalendarcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct McmGetCalendarCount {}

impl MsgSend for McmGetCalendarCount {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETCALENDARCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETCALENDARGRIDINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcalendargridinfo)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct McmGetCalendarGridInfo<'a, 'b> {
	pub info: &'b mut MCGRIDINFO<'a>,
}

impl<'a, 'b> MsgSend for McmGetCalendarGridInfo<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETCALENDARGRIDINFO.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`MCM_GETCALID`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcalid)
/// message, which has no parameters.
///
/// Return type: `co::CAL`.
pub struct McmGetCalId {}

impl MsgSend for McmGetCalId {
	type RetType = co::CAL;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::CAL::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETCALID.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcolor)
/// message parameters.
///
/// Return type: `SysResult<COLORREF>`.
pub struct McmGetColor {
	pub which: co::MCSC,
}

impl MsgSend for McmGetColor {
	type RetType = SysResult<COLORREF>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| unsafe { COLORREF::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETCOLOR.into(),
			wparam: self.which.raw() as _,
			lparam: 0,
		}
	}
}

/// [`MCM_GETCURRENTVIEW`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcurrentview)
/// message, which has no parameters.
///
/// Return type: `co::MCMV`.
pub struct McmGetCurrentView {}

impl MsgSend for McmGetCurrentView {
	type RetType = co::MCMV;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::MCMV::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETCURRENTVIEW.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcursel)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct McmGetCurSel<'a> {
	pub info: &'a mut SYSTEMTIME,
}

impl<'a> MsgSend for McmGetCurSel<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETCURSEL.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`MCM_GETFIRSTDAYOFWEEK`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getfirstdayofweek)
/// message, which has no parameters.
///
/// Return type: `(bool, u16)`.
pub struct McmGetFirstDayOfWeek {}

impl MsgSend for McmGetFirstDayOfWeek {
	type RetType = (bool, u16);

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		(HIWORD(v as _) != 0, LOWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETFIRSTDAYOFWEEK.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETMAXSELCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getmaxselcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct McmGetMaxSelCount {}

impl MsgSend for McmGetMaxSelCount {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETMAXSELCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETMAXTODAYWIDTH`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getmaxtodaywidth)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct McmGetMaxTodayWidth {}

impl MsgSend for McmGetMaxTodayWidth {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETMAXTODAYWIDTH.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETMINREQRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getminreqrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct McmGetMinReqRect<'a> {
	pub bounds_rect: &'a mut RECT,
}

impl<'a> MsgSend for McmGetMinReqRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETMINREQRECT.into(),
			wparam: 0,
			lparam: self.bounds_rect as *mut _ as _,
		}
	}
}

/// [`MCM_GETMONTHDELTA`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getmonthdelta)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct McmGetMonthDelta {}

impl MsgSend for McmGetMonthDelta {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETMONTHDELTA.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETMONTHRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getmonthrange)
/// message parameters.
///
/// Return value: `u32`.
pub struct McmGetMonthRange<'a> {
	pub scope: co::GMR,
	pub limits: &'a mut [SYSTEMTIME; 2],
}

impl<'a> MsgSend for McmGetMonthRange<'a> {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETMONTHRANGE.into(),
			wparam: self.scope.raw() as _,
			lparam: self.limits.as_mut_ptr() as _,
		}
	}
}

/// [`MCM_GETRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getrange)
/// message parameters.
///
/// Return type: `co::GDTR`.
pub struct McmGetRange<'a> {
	pub limits: &'a mut [SYSTEMTIME; 2],
}

impl<'a> MsgSend for McmGetRange<'a> {
	type RetType = co::GDTR;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::GDTR::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETRANGE.into(),
			wparam: 0,
			lparam: self.limits.as_mut_ptr() as _,
		}
	}
}

/// [`MCM_GETSELRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getselrange)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct McmGetSelRange<'a> {
	pub limits: &'a mut [SYSTEMTIME; 2],
}

impl<'a> MsgSend for McmGetSelRange<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETSELRANGE.into(),
			wparam: 0,
			lparam: self.limits.as_mut_ptr() as _,
		}
	}
}

/// [`MCM_GETTODAY`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-gettoday)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct McmGetToday<'a> {
	pub info: &'a mut SYSTEMTIME,
}

impl<'a> MsgSend for McmGetToday<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETTODAY.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`MCM_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getunicodeformat)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct McmGetUnicodeFormat {}

impl MsgSend for McmGetUnicodeFormat {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_HITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-hittest)
/// message parameters.
///
/// Return type: `()`.
pub struct McmHitTest<'a> {
	pub test_info: &'a mut MCHITTESTINFO,
}

impl<'a> MsgSend for McmHitTest<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::HITTEST.into(),
			wparam: 0,
			lparam: self.test_info as *mut _ as _,
		}
	}
}

/// [`MCM_SETCALENDARBORDER`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setcalendarborder)
/// message parameters.
///
/// Return type: `()`.
pub struct McmSetCalendarBorder {
	pub border: bool,
	pub pixels: u32,
}

impl MsgSend for McmSetCalendarBorder {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::SETCALENDARBORDER.into(),
			wparam: self.border as _,
			lparam: self.pixels as _,
		}
	}
}

/// [`MCM_SETCALID`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setcalid)
/// message parameters.
///
/// Return type: `()`.
pub struct McmSetCalId {
	pub id: co::CAL,
}

impl MsgSend for McmSetCalId {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::SETCALID.into(),
			wparam: self.id.raw() as _,
			lparam: 0,
		}
	}
}

/// [`MCM_SETCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setcolor)
/// message parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct McmSetColor {
	pub which: co::MCSC,
	pub color: COLORREF,
}

impl MsgSend for McmSetColor {
	type RetType = Option<COLORREF>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| unsafe { COLORREF::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::SETCOLOR.into(),
			wparam: self.which.raw() as _,
			lparam: u32::from(self.color) as _,
		}
	}
}

/// [`MCM_SETCURRENTVIEW`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setcurrentview)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct McmSetCurrentView {
	pub view: co::MCMV,
}

impl MsgSend for McmSetCurrentView {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::SETCURRENTVIEW.into(),
			wparam: 0,
			lparam: self.view.raw() as _,
		}
	}
}

/// [`MCM_SETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setcursel)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct McmSetCurSel<'a> {
	pub info: &'a SYSTEMTIME,
}

impl<'a> MsgSend for McmSetCurSel<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::SETCURSEL.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`MCM_SETDAYSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setdaystate)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct McmSetDayState<'a> {
	pub months: &'a [MONTHDAYSTATE],
}

impl<'a> MsgSend for McmSetDayState<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::SETDAYSTATE.into(),
			wparam: self.months.len(),
			lparam: vec_ptr(self.months) as _,
		}
	}
}

/// [`MCM_SETFIRSTDAYOFWEEK`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setfirstdayofweek)
/// message parameters.
///
/// Return type: `(bool, u16)`.
pub struct McmSetFirstDayOfWeek {
	pub first_day: u8,
}

impl MsgSend for McmSetFirstDayOfWeek {
	type RetType = (bool, u16);

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		(HIWORD(v as _) != 0, LOWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::SETFIRSTDAYOFWEEK.into(),
			wparam: 0,
			lparam: self.first_day as _,
		}
	}
}

/// [`MCM_SETMAXSELCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setmaxselcount)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct McmSetMaxSelCount {
	pub max_days: u8,
}

impl MsgSend for McmSetMaxSelCount {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::SETMAXSELCOUNT.into(),
			wparam: self.max_days as _,
			lparam: 0,
		}
	}
}

/// [`MCM_SETMONTHDELTA`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setmonthdelta)
/// message parameters.
///
/// Return type: `u8`.
pub struct McmSetMonthDelta {
	pub num_months: u8,
}

impl MsgSend for McmSetMonthDelta {
	type RetType = u8;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::SETMONTHDELTA.into(),
			wparam: self.num_months as _,
			lparam: 0,
		}
	}
}

/// [`MCM_SETRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setrange)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct McmSetRange<'a> {
	pub which: co::GDTR,
	pub limits: &'a [SYSTEMTIME; 2],
}

impl<'a> MsgSend for McmSetRange<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::SETRANGE.into(),
			wparam: self.which.raw() as _,
			lparam: vec_ptr(self.limits) as _,
		}
	}
}

/// [`MCM_SETSELRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setselrange)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct McmSetSelRange<'a> {
	pub limits: &'a [SYSTEMTIME; 2],
}

impl<'a> MsgSend for McmSetSelRange<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::SETSELRANGE.into(),
			wparam: 0,
			lparam: vec_ptr(self.limits) as _,
		}
	}
}

/// [`MCM_SETTODAY`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-settoday)
/// message parameters.
///
/// Return type: `()`.
pub struct McmSetToday<'a> {
	pub today: Option<&'a SYSTEMTIME>,
}

impl<'a> MsgSend for McmSetToday<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::SETTODAY.into(),
			wparam: 0,
			lparam: self.today.map_or(0, |today| today as *const _ as _),
		}
	}
}

/// [`MCM_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setunicodeformat)
/// message parameters.
///
/// Return type: `bool`.
pub struct McmSetUnicodeFormat {
	pub use_unicode: bool,
}

impl MsgSend for McmSetUnicodeFormat {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::SETUNICODEFORMAT.into(),
			wparam: self.use_unicode as _,
			lparam: 0,
		}
	}
}

/// [`MCM_SIZERECTTOMIN`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-sizerecttomin)
/// message parameters.
///
/// Return type: `()`.
pub struct McmSizeRectToMin<'a> {
	pub region: &'a mut RECT,
}

impl<'a> MsgSend for McmSizeRectToMin<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::MCM::SIZERECTTOMIN.into(),
			wparam: 0,
			lparam: self.region as *mut _ as _,
		}
	}
}
