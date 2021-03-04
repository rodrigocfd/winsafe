//! Header control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-header-control-reference-messages),
//! whose constants have [`HDM`](crate::co::HDM) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::msg::{MsgSend, WndMsg};
use crate::structs::RECT;

/// [`HDM_GETITEMCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-getitemcount)
/// message, which has no parameters.
///
/// Return type: `WinResult<u32>`.
pub struct GetItemCount {}

impl MsgSend for GetItemCount {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETITEMCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`HDM_GETITEMDROPDOWNRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-getitemdropdownrect)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetItemDropDownRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for GetItemDropDownRect<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETITEMDROPDOWNRECT.into(),
			wparam: self.index as usize,
			lparam: self.rect as *const _ as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`HDM_GETITEMRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-getitemrect)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetItemRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for GetItemRect<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETITEMRECT.into(),
			wparam: self.index as usize,
			lparam: self.rect as *const _ as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`HDM_GETORDERARRAY`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-getorderarray)
/// message parameters.
///
/// Return type `WinResult<()>`.
pub struct GetOrderArray<'a> {
	pub buffer: &'a mut [u32],
}

impl<'a> MsgSend for GetOrderArray<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETORDERARRAY.into(),
			wparam: self.buffer.len(),
			lparam: self.buffer.as_ptr() as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`HDM_GETOVERFLOWRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-getoverflowrect)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetOverflowRect<'a> {
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for GetOverflowRect<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETOVERFLOWRECT.into(),
			wparam: 0,
			lparam: self.rect as *const _ as isize,
		}
	}
}
