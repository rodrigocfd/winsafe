//! Edit control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-messages),
//! whose constants have [`EM`](crate::co::EM) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::msg::{MsgSend, WndMsg};

/// [`EN_CANUNDO`](https://docs.microsoft.com/en-us/windows/win32/controls/em-canundo)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct CanUndo {}

impl MsgSend for CanUndo {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::CANUNDO.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`EM_UNDO`](https://docs.microsoft.com/en-us/windows/win32/controls/em-undo)
/// message, which has no parameters.
///
/// Return type: `WinResult<()>`.
pub struct Undo {}

impl MsgSend for Undo {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::UNDO.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}
