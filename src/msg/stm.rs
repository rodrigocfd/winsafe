//! Static control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-static-control-reference-messages),
//! whose constants have [`STM`](crate::co::STM) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::handles::HICON;
use crate::msg::{MsgSend, WndMsg};

/// [`STM_GETICON`](https://docs.microsoft.com/en-us/windows/win32/controls/stm-geticon)
/// message, which has no parameters.
///
/// Return type: `WinResult<HICON>`.
pub struct GetIcon {}

impl MsgSend for GetIcon {
	type RetType = WinResult<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			p => Ok(HICON { ptr: p as *mut _ }),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::STM::GETICON.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`STM_SETICON`](https://docs.microsoft.com/en-us/windows/win32/controls/stm-seticon)
/// message parameters.
///
/// Return type: `WinResult<HICON>`.
pub struct SetIcon {
	pub icon: HICON,
}

impl MsgSend for SetIcon {
	type RetType = WinResult<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			p => Ok(HICON { ptr: p as *mut _ }),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::STM::SETICON.into(),
			wparam: self.icon.ptr as usize,
			lparam: 0,
		}
	}
}
