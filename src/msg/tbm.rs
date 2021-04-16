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
			h => Some(HWND { ptr: h as *mut _ }),
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
