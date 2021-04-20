//! Trackbar control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-messages)
//! whose constants have [`TBM`](crate::co::TBM) prefix.

use crate::co;
use crate::handles::HWND;
use crate::msg::{MsgSend, WndMsg};
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
			msg_id: co::TBM::CLEARSEL.into(),
			wparam: self.redraw as usize,
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
			msg_id: co::TBM::CLEARTICS.into(),
			wparam: self.redraw as usize,
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
		match v {
			0 => None,
			ptr => Some(HWND { ptr: ptr as *mut _ }),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETBUDDY.into(),
			wparam: self.left_above as usize,
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
			msg_id: co::TBM::GETCHANNELRECT.into(),
			wparam: 0,
			lparam: self.rect as *const _ as isize,
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
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETLINESIZE.into(),
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
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETNUMTICS.into(),
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
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETPAGESIZE.into(),
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
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETPOS.into(),
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
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETRANGEMAX.into(),
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
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETRANGEMIN.into(),
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
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETSELEND.into(),
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
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETSELSTART.into(),
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
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETTHUMBLENGTH.into(),
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
			msg_id: co::TBM::GETTHUMBRECT.into(),
			wparam: 0,
			lparam: self.rect as *const _ as isize,
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
		match v {
			0 => None,
			ptr => Some(HWND { ptr: ptr as *mut _ }),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETTOOLTIPS.into(),
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
		match v {
			0 => None,
			ptr => Some(HWND { ptr: ptr as *mut _ }),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETBUDDY.into(),
			wparam: self.left_above as usize,
			lparam: self.hwnd_buddy.ptr as isize,
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
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETPAGESIZE.into(),
			wparam: 0,
			lparam: self.page_size as isize,
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
			msg_id: co::TBM::SETPOS.into(),
			wparam: self.redraw as usize,
			lparam: self.pos as isize,
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
			msg_id: co::TBM::SETPOSNOTIFY.into(),
			wparam: 0,
			lparam: self.pos as isize,
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
			msg_id: co::TBM::SETRANGEMAX.into(),
			wparam: self.redraw as usize,
			lparam: self.max as isize,
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
			msg_id: co::TBM::SETRANGEMIN.into(),
			wparam: self.redraw as usize,
			lparam: self.min as isize,
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
			msg_id: co::TBM::SETSELEND.into(),
			wparam: self.redraw as usize,
			lparam: self.end as isize,
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
			msg_id: co::TBM::SETSELSTART.into(),
			wparam: self.redraw as usize,
			lparam: self.start as isize,
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
			msg_id: co::TBM::SETTHUMBLENGTH.into(),
			wparam: self.length as usize,
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
			msg_id: co::TBM::SETTICFREQ.into(),
			wparam: self.freq as usize,
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
			msg_id: co::TBM::SETTOOLTIPS.into(),
			wparam: self.hwnd_tooltip.ptr as usize,
			lparam: 0,
		}
	}
}
