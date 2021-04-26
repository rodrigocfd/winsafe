//! Edit control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-messages),
//! whose constants have [`EM`](crate::co::EM) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::msg::{MsgSend, WndMsg};
use crate::structs::RECT;

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

/// [`EM_GETLIMITTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getlimittext)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetLimitText {}

impl MsgSend for GetLimitText {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETLIMITTEXT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETLINECOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getlinecount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetLineCount {}

impl MsgSend for GetLineCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETLINECOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getrect)
/// message parameters.
///
/// Return type: `RECT`.
pub struct GetRect<'a> {
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for GetRect<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETRECT.into(),
			wparam: 0,
			lparam: self.rect as *const _ as isize,
		}
	}
}

/// [`EM_GETSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getsel)
/// message parameters.
///
/// Return type: `()`.
pub struct GetSel<'a, 'b> {
	pub first_index: Option<&'a mut u32>,
	pub past_last_index: Option<&'b mut u32>,
}

impl<'a, 'b> MsgSend for GetSel<'a, 'b> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETSEL.into(),
			wparam: self.first_index.as_ref().map_or(0, |r| r as *const _ as usize),
			lparam: self.past_last_index.as_ref().map_or(0, |r| r as *const _ as isize),
		}
	}
}

/// [`EM_GETTHUMB`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getthumb)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetThumb {}

impl MsgSend for GetThumb {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETTHUMB.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

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
