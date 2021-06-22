//! Trackbar control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-messages)
//! whose constants have [`TRBM`](crate::co::TRBM) prefix.

use crate::co;
use crate::handles::HWND;
use crate::msg::{MsgSend, WndMsg};
use crate::msg::macros::zero_as_none;
use crate::structs::RECT;

/// [`TBM_CLEARSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-clearsel)
/// message parameters.
///
/// Return type: `()`.
pub struct ClearSel {
	pub redraw: bool,
}

impl MsgSend for ClearSel {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::CLEARSEL.into(),
			wparam: self.redraw as _,
			lparam: 0,
		}
	}
}

/// [`TBM_CLEARTICS`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-cleartics)
/// message parameters.
///
/// Return type: `()`.
pub struct ClearTics {
	pub redraw: bool,
}

impl MsgSend for ClearTics {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::CLEARTICS.into(),
			wparam: self.redraw as _,
			lparam: 0,
		}
	}
}

/// [`TBM_GETBUDDY`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-getbuddy)
/// message parameters.
///
/// Return type: `Option<HWND>`.
pub struct GetBuddy {
	pub left_above: bool,
}

impl MsgSend for GetBuddy {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HWND { ptr: p as _ })
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETBUDDY.into(),
			wparam: self.left_above as _,
			lparam: 0,
		}
	}
}

/// [`TBM_GETCHANNELRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-getchannelrect)
/// message parameters.
///
/// Return type: `()`.
pub struct GetChannelRect<'a> {
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for GetChannelRect<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETCHANNELRECT.into(),
			wparam: 0,
			lparam: self.rect as *const _ as _,
		}
	}
}

/// [`TBM_GETLINESIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-getlinesize)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetLineSize {}

impl MsgSend for GetLineSize {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETLINESIZE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETNUMTICS`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-getnumtics)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetNumTics {}

impl MsgSend for GetNumTics {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETNUMTICS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETPAGESIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-getpagesize)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetPageSize {}

impl MsgSend for GetPageSize {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETPAGESIZE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETPOS`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-getpos)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetPos {}

impl MsgSend for GetPos {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETPOS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETRANGEMAX`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-getrangemax)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetRangeMax {}

impl MsgSend for GetRangeMax {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETRANGEMAX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETRANGEMIN`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-getrangemin)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetRangeMin {}

impl MsgSend for GetRangeMin {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETRANGEMIN.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETSELEND`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-getselend)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetSelEnd {}

impl MsgSend for GetSelEnd {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETSELEND.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETSELSTART`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-getselstart)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetSelStart {}

impl MsgSend for GetSelStart {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETSELSTART.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETTHUMBLENGTH`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-getthumblength)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetThumbLength {}

impl MsgSend for GetThumbLength {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETTHUMBLENGTH.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETTHUMBRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-getthumbrect)
/// message parameters.
///
/// Return type: `()`.
pub struct GetThumbRect<'a> {
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for GetThumbRect<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETTHUMBRECT.into(),
			wparam: 0,
			lparam: self.rect as *const _ as _,
		}
	}
}

/// [`TBM_GETTOOLTIPS`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-gettooltips)
/// message, which has no parameters.
///
/// Return type: `Option<HWND>`.
pub struct GetTooltips {}

impl MsgSend for GetTooltips {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HWND { ptr: p as _ })
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETTOOLTIPS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_SETBUDDY`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-setbuddy)
/// message parameters.
///
/// Return type: `Option<HWND>`.
pub struct SetBuddy {
	pub left_above: bool,
	pub hwnd_buddy: HWND,
}

impl MsgSend for SetBuddy {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HWND { ptr: p as _ })
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETBUDDY.into(),
			wparam: self.left_above as _,
			lparam: self.hwnd_buddy.ptr as _,
		}
	}
}

/// [`TBM_SETPAGSIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-setpagesize)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetPageSize {
	pub page_size: u32,
}

impl MsgSend for SetPageSize {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETPAGESIZE.into(),
			wparam: 0,
			lparam: self.page_size as _,
		}
	}
}

/// [`TBM_SETPOS`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-setpos)
/// message parameters.
///
/// Return type: `()`.
pub struct SetPos {
	pub redraw: bool,
	pub pos: u32,
}

impl MsgSend for SetPos {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETPOS.into(),
			wparam: self.redraw as _,
			lparam: self.pos as _,
		}
	}
}

/// [`TBM_SETPOSNOTIFY`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-setposnotify)
/// message parameters.
///
/// Return type: `()`.
pub struct SetPosNotify {
	pub pos: u32,
}

impl MsgSend for SetPosNotify {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETPOSNOTIFY.into(),
			wparam: 0,
			lparam: self.pos as _,
		}
	}
}

/// [`TBM_SETRANGEMAX`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-setrangemax)
/// message parameters.
///
/// Return type: `()`.
pub struct SetRangeMax {
	pub redraw: bool,
	pub max: u32,
}

impl MsgSend for SetRangeMax {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETRANGEMAX.into(),
			wparam: self.redraw as _,
			lparam: self.max as _,
		}
	}
}

/// [`TBM_SETRANGEMIN`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-setrangemin)
/// message parameters.
///
/// Return type: `()`.
pub struct SetRangeMin {
	pub redraw: bool,
	pub min: u32,
}

impl MsgSend for SetRangeMin {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETRANGEMIN.into(),
			wparam: self.redraw as _,
			lparam: self.min as _,
		}
	}
}

/// [`TBM_SETSELEND`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-setselend)
/// message parameters.
///
/// Return type: `()`.
pub struct SetSelEnd {
	pub redraw: bool,
	pub end: u32,
}

impl MsgSend for SetSelEnd {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETSELEND.into(),
			wparam: self.redraw as _,
			lparam: self.end as _,
		}
	}
}

/// [`TBM_SETSELSTART`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-setselstart)
/// message parameters.
///
/// Return type: `()`.
pub struct SetSelStart {
	pub redraw: bool,
	pub start: u32,
}

impl MsgSend for SetSelStart {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETSELSTART.into(),
			wparam: self.redraw as _,
			lparam: self.start as _,
		}
	}
}

/// [`TBM_SETTHUMBLENGTH`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-setthumblength)
/// message parameters.
///
/// Return type: `()`.
pub struct SetThumbLength {
	pub length: u32,
}

impl MsgSend for SetThumbLength {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETTHUMBLENGTH.into(),
			wparam: self.length as _,
			lparam: 0,
		}
	}
}

/// [`TBM_SETTICFREQ`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-setticfreq)
/// message parameters.
///
/// Return type: `()`.
pub struct SetTicFreq {
	pub freq: u32,
}

impl MsgSend for SetTicFreq {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETTICFREQ.into(),
			wparam: self.freq as _,
			lparam: 0,
		}
	}
}

/// [`TBM_SETTOOLTIPS`](https://docs.microsoft.com/en-us/windows/win32/controls/tbm-settooltips)
/// message parameters.
///
/// Return type: `()`.
pub struct SetTooltips {
	pub hwnd_tooltip: HWND,
}

impl MsgSend for SetTooltips {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETTOOLTIPS.into(),
			wparam: self.hwnd_tooltip.ptr as _,
			lparam: 0,
		}
	}
}
