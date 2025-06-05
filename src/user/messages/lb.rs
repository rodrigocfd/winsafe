use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`LB_ADDFILE`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-addfile)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct AddFile {
	pub text: WString,
}

impl MsgSend for AddFile {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::ADDFILE.into(),
			wparam: 0,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`LB_ADDSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-addstring)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct AddString {
	pub text: WString,
}

impl MsgSend for AddString {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::ADDSTRING.into(),
			wparam: 0,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`LB_DELETESTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-deletestring)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct DeleteString {
	pub index: u32,
}

impl MsgSend for DeleteString {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::DELETESTRING.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LB_DIR`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-dir)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct Dir {
	pub attributes: co::DDL,
	pub path: WString,
}

impl MsgSend for Dir {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::DIR.into(),
			wparam: self.attributes.raw() as _,
			lparam: self.path.as_ptr() as _,
		}
	}
}

/// [`LB_FINDSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-findstring)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct FindString {
	pub preceding_index: Option<u32>,
	pub text: WString,
}

impl MsgSend for FindString {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::FINDSTRING.into(),
			wparam: self.preceding_index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`LB_FINDSTRINGEXACT`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-findstringexact)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct FindStringExact {
	pub preceding_index: Option<u32>,
	pub text: WString,
}

impl MsgSend for FindStringExact {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::FINDSTRINGEXACT.into(),
			wparam: self.preceding_index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`LB_GETANCHORINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-getanchorindex)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetAnchorIndex {}

impl MsgSend for GetAnchorIndex {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETANCHORINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETCARETINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-getcaretindex)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetCaretIndex {}

impl MsgSend for GetCaretIndex {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETCARETINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-getcount)
/// message, which has no parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetCount {}

impl MsgSend for GetCount {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-getcursel)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
pub struct GetCurSel {}

impl MsgSend for GetCurSel {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETCURSEL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETHORIZONTALEXTENT`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-gethorizontalextent)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetHorizontalExtent {}

impl MsgSend for GetHorizontalExtent {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETHORIZONTALEXTENT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETITEMDATA`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-getitemdata)
/// message parameters.
///
/// Return type: `SysResult<isize>`.
pub struct GetItemData {
	pub index: u32,
}

impl MsgSend for GetItemData {
	type RetType = SysResult<isize>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		const LB_ERR_ISIZE: isize = LB_ERR as _;
		match v {
			LB_ERR_ISIZE => Err(co::ERROR::BAD_ARGUMENTS),
			data => Ok(data),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETITEMDATA.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LB_GETITEMHEIGHT`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-getitemheight)
/// message parameters.
///
/// Return type: `SysResult<u8>`.
pub struct GetItemHeight {
	pub index: Option<u32>,
}

impl MsgSend for GetItemHeight {
	type RetType = SysResult<u8>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			height => Ok(height as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETITEMHEIGHT.into(),
			wparam: self.index.unwrap_or(0) as _,
			lparam: 0,
		}
	}
}

/// [`LB_GETITEMRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-getitemrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetItemRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for GetItemRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETITEMRECT.into(),
			wparam: self.index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`LB_GETLISTBOXINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-getlistboxinfo)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetListBoxInfo {}

impl MsgSend for GetListBoxInfo {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETLISTBOXINFO.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETLOCALE`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-getlocale)
/// message, which has no parameters.
///
/// Return type: `LCID`.
pub struct GetLocale {}

impl MsgSend for GetLocale {
	type RetType = LCID;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { LCID::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETLOCALE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-getsel)
/// message parameters.
///
/// Return type: `SysResult<bool>`.
pub struct GetSel {
	pub index: u32,
}

impl MsgSend for GetSel {
	type RetType = SysResult<bool>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			status => Ok(status != 0),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETSEL.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LB_GETSELCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-getselcount)
/// message, which has no parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetSelCount {}

impl MsgSend for GetSelCount {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETSELCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETSELITEMS`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-getselitems)
/// message parameters.
///
/// Return type `SysResult<u32>`.
pub struct GetSelItems<'a> {
	pub buffer: &'a mut [u32],
}

impl<'a> MsgSend for GetSelItems<'a> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETSELITEMS.into(),
			wparam: self.buffer.len(),
			lparam: self.buffer.as_mut_ptr() as _,
		}
	}
}

/// [`LB_GETTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-gettext)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetText<'a> {
	pub index: u32,
	pub text: &'a mut WString,
}

impl<'a> MsgSend for GetText<'a> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			nchars => Ok(nchars as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETTEXT.into(),
			wparam: self.index as _,
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`LB_GETTEXTLEN`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-gettextlen)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetTextLen {
	pub index: u32,
}

impl MsgSend for GetTextLen {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			nchars => Ok(nchars as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETTEXTLEN.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LB_GETTOPINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-gettopindex)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetTopIndex {}

impl MsgSend for GetTopIndex {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETTOPINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_INITSTORAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-initstorage)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct InitStorage {
	pub num_items: u32,
	pub memory_bytes: u32,
}

impl MsgSend for InitStorage {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERRSPACE => Err(co::ERROR::BAD_ARGUMENTS),
			n_items => Ok(n_items as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::INITSTORAGE.into(),
			wparam: self.num_items as _,
			lparam: self.memory_bytes as _,
		}
	}
}

/// [`LB_INSERTSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-insertstring)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct InsertString {
	pub insertion_index: Option<u32>,
	pub text: WString,
}

impl MsgSend for InsertString {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::INSERTSTRING.into(),
			wparam: self.insertion_index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`LB_ITEMFROMPOINT`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-itemfrompoint)
/// message parameters.
///
/// Return type: `(i32, bool)`.
pub struct ItemFromPoint {
	pub coords: POINT,
}

impl MsgSend for ItemFromPoint {
	type RetType = (i32, bool);

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _) as _, HIWORD(v as _) == 1)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::ITEMFROMPOINT.into(),
			wparam: 0,
			lparam: u32::from(self.coords) as _,
		}
	}
}

pub_struct_msg_empty! { ResetContent: co::LB::RESETCONTENT.into();
	/// [`LB_RESETCONTENT`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-resetcontent)
}

/// [`LB_SELECTSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-selectstring)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct SelectString {
	pub index: Option<u32>,
	pub prefix: WString,
}

impl MsgSend for SelectString {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SELECTSTRING.into(),
			wparam: self.index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.prefix.as_ptr() as _,
		}
	}
}

/// [`LB_SELITEMRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-selitemrange)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SelItemRange {
	pub select: bool,
	pub first_item: u16,
	pub last_item: u16,
}

impl MsgSend for SelItemRange {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SELITEMRANGE.into(),
			wparam: self.select as _,
			lparam: MAKEDWORD(self.first_item, self.last_item) as _,
		}
	}
}

/// [`LB_SELITEMRANGEEX`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-selitemrangeex)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SelItemRangeEx {
	pub first_index: u32,
	pub last_index: u32,
}

impl MsgSend for SelItemRangeEx {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SELITEMRANGEEX.into(),
			wparam: self.first_index as _,
			lparam: self.last_index as _,
		}
	}
}

/// [`LB_SETANCHORINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-setanchorindex)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetAnchorIndex {
	pub index: u32,
}

impl MsgSend for SetAnchorIndex {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETANCHORINDEX.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LB_SETCARETINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-setcaretindex)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetCaretIndex {
	pub index: u32,
	pub at_least_partially_visible: bool,
}

impl MsgSend for SetCaretIndex {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETCARETINDEX.into(),
			wparam: self.index as _,
			lparam: self.at_least_partially_visible as _,
		}
	}
}

/// [`LB_SETCOLUMNWIDTH`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-setcolumnwidth)
/// message parameters.
///
/// Return type: `()`.
pub struct SetColumnWidth {
	pub width: u32,
}

impl MsgSend for SetColumnWidth {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETCOLUMNWIDTH.into(),
			wparam: self.width as _,
			lparam: 0,
		}
	}
}

/// [`LB_SETCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-setcount)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetCount {
	pub new_count: u32,
}

impl MsgSend for SetCount {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETCOUNT.into(),
			wparam: self.new_count as _,
			lparam: 0,
		}
	}
}

/// [`LB_SETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-setcursel)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetCurSel {
	pub index: Option<u32>,
}

impl MsgSend for SetCurSel {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		if let None = self.index {
			Ok(())
		} else {
			match v as i32 {
				LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
				_ => Ok(()),
			}
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETCURSEL.into(),
			wparam: self.index.map_or(-1, |idx| idx as i32) as _,
			lparam: 0,
		}
	}
}

/// [`LB_SETHORIZONTALEXTENT`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-sethorizontalextent)
/// message parameters.
///
/// Return type: `()`.
pub struct SetHorizontalExtent {
	pub width: u32,
}

impl MsgSend for SetHorizontalExtent {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETHORIZONTALEXTENT.into(),
			wparam: self.width as _,
			lparam: 0,
		}
	}
}

/// [`LB_SETITEMDATA`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-setitemdata)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetItemData {
	pub index: u32,
	pub data: isize,
}

impl MsgSend for SetItemData {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETITEMDATA.into(),
			wparam: self.index as _,
			lparam: self.data,
		}
	}
}

/// [`LB_SETITEMHEIGHT`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-setitemheight)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetItemHeight {
	pub index: Option<u32>,
	pub height: u8,
}

impl MsgSend for SetItemHeight {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETITEMHEIGHT.into(),
			wparam: self.index.unwrap_or(0) as _,
			lparam: self.height as _,
		}
	}
}

/// [`LB_SETLOCALE`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-setlocale)
/// message parameters.
///
/// Return type: `SysResult<LCID>`.
pub struct SetLocale {
	pub locale: LCID,
}

impl MsgSend for SetLocale {
	type RetType = SysResult<LCID>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			lcid => Ok(unsafe { LCID::from_raw(lcid as _) }),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETLOCALE.into(),
			wparam: u32::from(self.locale) as _,
			lparam: 0,
		}
	}
}

/// [`LB_SETSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-setsel)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetSel {
	pub select: bool,
	pub index: Option<u32>,
}

impl MsgSend for SetSel {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETSEL.into(),
			wparam: self.select as _,
			lparam: self.index.map_or(-1, |idx| idx as i32) as _,
		}
	}
}

/// [`LB_SETTABSTOPS`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-settabstops)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetTabStops<'a> {
	pub tab_stops: &'a [u32],
}

impl<'a> MsgSend for SetTabStops<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETTABSTOPS.into(),
			wparam: self.tab_stops.len(),
			lparam: vec_ptr(self.tab_stops) as _,
		}
	}
}

/// [`LB_SETTOPINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-settopindex)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetTopIndex {
	pub index: u32,
}

impl MsgSend for SetTopIndex {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETTOPINDEX.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}
