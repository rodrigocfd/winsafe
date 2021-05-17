//! Combo box control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-messages),
//! whose constants have [`CB`](crate::co::CB) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::msg::{MsgSend, WndMsg};
use crate::privs::{CB_ERR, CB_ERRSPACE};
use crate::WString;

/// [`CB_ADDSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-addstring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct AddString<'a> {
	pub text: &'a str,
}

impl<'a> MsgSend for AddString<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR | CB_ERRSPACE => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::ADDSTRING.into(),
			wparam: 0,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as _,
		}
	}
}

/// [`CB_DELETESTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-deletestring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct DeleteString {
	pub index: u32,
}

impl MsgSend for DeleteString {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::DELETESTRING.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`CB_DIR`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-dir)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct Dir<'a> {
	pub attributes: co::DDL,
	pub path: &'a str,
}

impl<'a> MsgSend for Dir<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			CB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::DELETESTRING.into(),
			wparam: self.attributes.0 as _,
			lparam: unsafe { WString::from_str(self.path).as_ptr() } as _,
		}
	}
}

/// [`CB_FINDSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-findstring)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct FindString<'a> {
	pub preceding_index: Option<u32>,
	pub text: &'a str,
}

impl<'a> MsgSend for FindString<'a> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::FINDSTRING.into(),
			wparam: self.preceding_index.map(|i| i as i32).unwrap_or(-1) as _,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as _,
		}
	}
}

/// [`CB_GETCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getcount)
/// message, which has no parameters.
///
/// Return type: `WinResult<u32>`.
pub struct GetCount {}

impl MsgSend for GetCount {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETCURSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getcursel)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
pub struct GetCurSel {}

impl MsgSend for GetCurSel {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETCURSEL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETLBTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getlbtext)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct GetLbText<'a> {
	pub index: u32,
	pub text: &'a mut WString,
}

impl<'a> MsgSend for GetLbText<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			nchars => Ok(nchars as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETLBTEXT.into(),
			wparam: self.index as _,
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`CB_GETLBTEXTLEN`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getlbtextlen)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct GetLbTextLen {
	pub index: u32,
}

impl MsgSend for GetLbTextLen {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			nchars => Ok(nchars as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETLBTEXTLEN.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`CB_GETMINVISIBLE`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getminvisible)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetMinVisible {}

impl MsgSend for GetMinVisible {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETMINVISIBLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETTOPINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-gettopindex)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct GetTopIndex {}

impl MsgSend for GetTopIndex {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETTOPINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_INITSTORAGE`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-initstorage)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct InitStorage {
	pub num_items: u32,
	pub memory_bytes: u32,
}

impl MsgSend for InitStorage {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERRSPACE => Err(co::ERROR::BAD_ARGUMENTS),
			n_items => Ok(n_items as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::INITSTORAGE.into(),
			wparam: self.num_items as _,
			lparam: self.memory_bytes as _,
		}
	}
}

pub_struct_msg_empty! { ResetContent, co::CB::RESETCONTENT.into(),
	/// [`CB_RESETCONTENT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-resetcontent)
}

/// [`CB_SETCURSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-setcursel)
/// message parameters.
///
/// Return type: `()`.
pub struct SetCurSel {
	pub index: Option<u32>,
}

impl MsgSend for SetCurSel {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETCURSEL.into(),
			wparam: match self.index {
				Some(index) => index as i32,
				None => -1,
			} as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETMINVISIBLE`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-setminvisible)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetMinVisible {
	pub num_items: u32,
}

impl MsgSend for SetMinVisible {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETMINVISIBLE.into(),
			wparam: self.num_items as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETTOPINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-settopindex)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetTopIndex {
	pub index: u32,
}

impl MsgSend for SetTopIndex {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETTOPINDEX.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`CB_SHOWDROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-showdropdown)
/// message parameters.
///
/// Return type: `()`.
pub struct ShowDropDown {
	pub show: bool,
}

impl MsgSend for ShowDropDown {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SHOWDROPDOWN.into(),
			wparam: self.show as _,
			lparam: 0,
		}
	}
}
