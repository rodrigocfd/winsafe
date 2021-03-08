//! List box control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-box-control-reference-messages),
//! whose constants have [`LB`](crate::co::LB) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::{HIWORD, LOWORD, MAKEDWORD};
use crate::msg::{MsgSend, WndMsg};
use crate::msg::macros::point_to_lp;
use crate::privs::{LB_ERR, LB_ERRSPACE};
use crate::structs::{POINT, RECT};
use crate::WString;

/// [`LB_ADDFILE`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-addfile)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct AddFile<'a> {
	pub text: &'a str,
}

impl<'a> MsgSend for AddFile<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR | LB_ERRSPACE => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::ADDSTRING.into(),
			wparam: 0,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as isize,
		}
	}
}

/// [`LB_ADDSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-addstring)
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
			LB_ERR | LB_ERRSPACE => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::ADDSTRING.into(),
			wparam: 0,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as isize,
		}
	}
}

/// [`LB_DELETESTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-deletestring)
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
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::DELETESTRING.into(),
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}

/// [`LB_DIR`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-dir)
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
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::DELETESTRING.into(),
			wparam: self.attributes.0 as usize,
			lparam: unsafe { WString::from_str(self.path).as_ptr() } as isize,
		}
	}
}

/// [`LB_FINDSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-findstring)
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
			LB_ERR => None,
			idx => Some(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::FINDSTRING.into(),
			wparam: match self.preceding_index {
				None => -1,
				Some(idx) => idx as i32,
			} as usize,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as isize,
		}
	}
}

/// [`LB_FINDSTRINGEXACT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-findstringexact)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct FindStringExact<'a> {
	pub preceding_index: Option<u32>,
	pub text: &'a str,
}

impl<'a> MsgSend for FindStringExact<'a> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => None,
			idx => Some(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::FINDSTRINGEXACT.into(),
			wparam: match self.preceding_index {
				None => -1,
				Some(idx) => idx as i32,
			} as usize,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as isize,
		}
	}
}

/// [`LB_GETANCHORINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getanchorindex)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetAnchorIndex {}

impl MsgSend for GetAnchorIndex {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETANCHORINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETCARETINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getcaretindex)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetCaretIndex {}

impl MsgSend for GetCaretIndex {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETCARETINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getcount)
/// message, which has no parameters.
///
/// Return type: `WinResult<u32>`.
pub struct GetCount {}

impl MsgSend for GetCount {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETCURSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getcursel)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
pub struct GetCurSel {}

impl MsgSend for GetCurSel {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => None,
			idx => Some(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETCURSEL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETHORIZONTALEXTENT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-gethorizontalextent)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetHorizontalExtent {}

impl MsgSend for GetHorizontalExtent {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETHORIZONTALEXTENT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETITEMDATA`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getitemdata)
/// message parameters.
///
/// Return type: `WinResult<isize>`.
pub struct GetItemData {
	pub index: u32,
}

impl MsgSend for GetItemData {
	type RetType = WinResult<isize>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		const LB_ERR_ISIZE: isize = LB_ERR as _;
		match v {
			LB_ERR_ISIZE => Err(co::ERROR::BAD_ARGUMENTS),
			data => Ok(data),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETITEMDATA.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETITEMHEIGHT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getitemheight)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct GetItemHeight {
	pub index: Option<u32>,
}

impl MsgSend for GetItemHeight {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			height => Ok(height as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETITEMHEIGHT.into(),
			wparam: self.index.unwrap_or(0) as usize,
			lparam: 0,
		}
	}
}

/// [`LB_GETITEMRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getitemrect)
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
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETITEMRECT.into(),
			wparam: self.index as usize,
			lparam: self.rect as *const _ as isize,
		}
	}
}

/// [`LB_GETLISTBOXINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getlistboxinfo)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetListBoxInfo {}

impl MsgSend for GetListBoxInfo {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETLISTBOXINFO.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getsel)
/// message parameters.
///
/// Return type: `WinResult<bool>`.
pub struct GetSel {
	pub index: u32,
}

impl MsgSend for GetSel {
	type RetType = WinResult<bool>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			status => Ok(status != 0),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETSEL.into(),
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}

/// [`LB_GETSELCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getselcount)
/// message, which has no parameters.
///
/// Return type: `WinResult<u32>`.
pub struct GetSelCount {}

impl MsgSend for GetSelCount {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETSELCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETSELITEMS`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getselitems)
/// message parameters.
///
/// Return type `WinResult<u32>`.
pub struct GetSelItems<'a> {
	pub buffer: &'a mut [u32],
}

impl<'a> MsgSend for GetSelItems<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETSELITEMS.into(),
			wparam: self.buffer.len(),
			lparam: self.buffer.as_ptr() as isize,
		}
	}
}

/// [`LB_GETTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-gettext)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct GetText<'a> {
	pub index: u32,
	pub text: &'a mut WString,
}

impl<'a> MsgSend for GetText<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			nchars => Ok(nchars as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETTEXT.into(),
			wparam: self.index as usize,
			lparam: unsafe { self.text.as_ptr() } as isize,
		}
	}
}

/// [`LB_GETTEXTLEN`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-gettextlen)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct GetTextLen {
	pub index: u32,
}

impl MsgSend for GetTextLen {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			nchars => Ok(nchars as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETTEXTLEN.into(),
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}

/// [`LB_GETTOPINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-gettopindex)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct GetTopIndex {}

impl MsgSend for GetTopIndex {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETTOPINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_INITSTORAGE`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-initstorage)
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
			LB_ERRSPACE => Err(co::ERROR::BAD_ARGUMENTS),
			n_items => Ok(n_items as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::INITSTORAGE.into(),
			wparam: self.num_items as usize,
			lparam: self.memory_bytes as isize,
		}
	}
}

/// [`LB_INSERTSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-insertstring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct InsertString<'a> {
	pub text: &'a str,
}

impl<'a> MsgSend for InsertString<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR | LB_ERRSPACE => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::INSERTSTRING.into(),
			wparam: 0,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as isize,
		}
	}
}

/// [`LB_ITEMFROMPOINT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-itemfrompoint)
/// message parameters.
///
/// Return type: `(i32, bool)`.
pub struct ItemFromPoint {
	pub coords: POINT,
}

impl MsgSend for ItemFromPoint {
	type RetType = (i32, bool);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as u32) as i32, HIWORD(v as u32) == 1)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::ITEMFROMPOINT.into(),
			wparam: 0,
			lparam: point_to_lp(self.coords),
		}
	}
}

empty_msg! { ResetContent, co::LB::RESETCONTENT.into(),
	/// [`LB_RESETCONTENT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-resetcontent)
}

/// [`LB_SELECTSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-selectstring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct SelectString<'a> {
	pub index: Option<u32>,
	pub prefix: &'a str,
}

impl<'a> MsgSend for SelectString<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR | LB_ERRSPACE => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SELECTSTRING.into(),
			wparam: match self.index {
				None => -1,
				Some(idx) => idx as i32,
			} as usize,
			lparam: unsafe { WString::from_str(self.prefix).as_ptr() } as isize,
		}
	}
}

/// [`LB_SELITEMRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-selitemrange)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SelItemRange {
	pub select: bool,
	pub first_item: u32,
	pub last_item: u32,
}

impl MsgSend for SelItemRange {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SELITEMRANGE.into(),
			wparam: self.select as usize,
			lparam: MAKEDWORD(self.first_item as u16, self.last_item as u16) as isize,
		}
	}
}

/// [`LB_SETANCHORINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-setanchorindex)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetAnchorIndex {
	pub index: u32,
}

impl MsgSend for SetAnchorIndex {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETANCHORINDEX.into(),
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}
