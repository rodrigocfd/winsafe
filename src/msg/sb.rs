//! Status bar control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-status-bars-reference-messages),
//! whose constants have [`SB`](crate::co::SB) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::{HIWORD, LOWORD, MAKEDWORD, MAKEWORD};
use crate::handles::HICON;
use crate::msg::{MsgSend, WndMsg};
use crate::WString;

/// [`SB_GETICON`](https://docs.microsoft.com/en-us/windows/win32/controls/sb-geticon)
/// message parameters.
///
/// Return type: `WinResult<HICON>`.
pub struct GetIcon {
	pub part_index: u8,
}

impl MsgSend for GetIcon {
	type RetType = WinResult<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			ptr => Ok(HICON { ptr: ptr as _ }),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::GETICON.into(),
			wparam: self.part_index as _,
			lparam: 0,
		}
	}
}

/// [`SB_GETPARTS`](https://docs.microsoft.com/en-us/windows/win32/controls/sb-getparts)
/// message parameters.
///
/// Return type: `u8`.
pub struct GetParts<'a> {
	pub right_edges: Option<&'a mut [i32]>,
}

impl<'a> MsgSend for GetParts<'a> {
	type RetType = u8;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::GETPARTS.into(),
			wparam: self.right_edges.as_ref().map_or(0, |re| re.len()),
			lparam: self.right_edges.as_ref().map_or(0, |re| re.as_ptr() as _),
		}
	}
}

/// [`SB_GETTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/sb-gettext)
/// message parameters.
///
/// Return type: `(u16, SBT)`.
pub struct GetText<'a> {
	pub part_index: u8,
	pub text: &'a mut WString,
}

impl<'a> MsgSend for GetText<'a> {
	type RetType = (u16, co::SBT);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), co::SBT(HIWORD(v as _)))
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::GETTEXT.into(),
			wparam: self.part_index as _,
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`SB_GETTEXTLENGTH`](https://docs.microsoft.com/en-us/windows/win32/controls/sb-gettextlength)
/// message parameters.
///
/// Return type: `(u16, SBT)`.
pub struct GetTextLength {
	pub part_index: u8,
}

impl MsgSend for GetTextLength {
	type RetType = (u16, co::SBT);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), co::SBT(HIWORD(v as _)))
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::GETTEXTLENGTH.into(),
			wparam: self.part_index as _,
			lparam: 0,
		}
	}
}

/// [`SB_GETTIPTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/sb-gettiptext)
/// message parameters.
///
/// Return type: `()`.
pub struct GetTipText<'a> {
	pub part_index: u8,
	pub text: &'a mut WString,
}

impl<'a> MsgSend for GetTipText<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::GETTIPTEXT.into(),
			wparam: MAKEDWORD(self.part_index as _, self.text.len() as _) as _,
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`SB_SETICON`](https://docs.microsoft.com/en-us/windows/win32/controls/sb-seticon)
// message parameters.
pub struct SetIcon {
	pub part_index: u8,
	pub hicon: Option<HICON>,
}

impl MsgSend for SetIcon {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::SETICON.into(),
			wparam: self.part_index as _,
			lparam: self.hicon.map(|h| h.ptr as _).unwrap_or_default(),
		}
	}
}

/// [`SB_SETPARTS`](https://docs.microsoft.com/en-us/windows/win32/controls/sb-setparts)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetParts<'a> {
	pub right_edges: &'a [i32],
}

impl<'a> MsgSend for SetParts<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::SETPARTS.into(),
			wparam: self.right_edges.len(),
			lparam: self.right_edges.as_ptr() as _,
		}
	}
}

/// [`SB_SETTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/sb-settext)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetText<'a> {
	pub part_index: u8,
	pub drawing_operation: co::SBT,
	pub text: &'a str,
}

impl<'a> MsgSend for SetText<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::SETTEXT.into(),
			wparam: MAKEDWORD(MAKEWORD(self.part_index, 0), self.drawing_operation.into()) as _,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as _,
		}
	}
}

/// [`SB_SETTIPTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/sb-settiptext)
/// message parameters.
pub struct SetTipText<'a> {
	pub part_index: u8,
	pub text: &'a str,
}

impl<'a> MsgSend for SetTipText<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::SETTIPTEXT.into(),
			wparam: self.part_index as _,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as _,
		}
	}
}

/// [`SB_SIMPLE`](https://docs.microsoft.com/en-us/windows/win32/controls/sb-simple)
/// message parameters.
pub struct Simple {
	pub display_simple: bool,
}

impl MsgSend for Simple {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::SIMPLE.into(),
			wparam: self.display_simple as _,
			lparam: 0,
		}
	}
}
