use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::macros::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`LB_ADDFILE`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-addfile)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct LbAddFile {
	pub text: WString,
}

impl MsgSend for LbAddFile {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbAddString {
	pub text: WString,
}

impl MsgSend for LbAddString {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbDeleteString {
	pub index: u32,
}

impl MsgSend for LbDeleteString {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbDir {
	pub attributes: co::DDL,
	pub path: WString,
}

impl MsgSend for LbDir {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbFindString {
	pub preceding_index: Option<u32>,
	pub text: WString,
}

impl MsgSend for LbFindString {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbFindStringExact {
	pub preceding_index: Option<u32>,
	pub text: WString,
}

impl MsgSend for LbFindStringExact {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetAnchorIndex {}

impl MsgSend for LbGetAnchorIndex {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetCaretIndex {}

impl MsgSend for LbGetCaretIndex {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetCount {}

impl MsgSend for LbGetCount {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetCurSel {}

impl MsgSend for LbGetCurSel {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetHorizontalExtent {}

impl MsgSend for LbGetHorizontalExtent {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetItemData {
	pub index: u32,
}

impl MsgSend for LbGetItemData {
	type RetType = SysResult<isize>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		const LB_ERR_ISIZE: isize = LB_ERR as _;
		match v {
			LB_ERR_ISIZE => Err(co::ERROR::BAD_ARGUMENTS),
			data => Ok(data),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetItemHeight {
	pub index: Option<u32>,
}

impl MsgSend for LbGetItemHeight {
	type RetType = SysResult<u8>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			height => Ok(height as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetItemRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for LbGetItemRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetListBoxInfo {}

impl MsgSend for LbGetListBoxInfo {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetLocale {}

impl MsgSend for LbGetLocale {
	type RetType = LCID;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { LCID::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetSel {
	pub index: u32,
}

impl MsgSend for LbGetSel {
	type RetType = SysResult<bool>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			status => Ok(status != 0),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetSelCount {}

impl MsgSend for LbGetSelCount {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetSelItems<'a> {
	pub buffer: &'a mut [u32],
}

impl<'a> MsgSend for LbGetSelItems<'a> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetText<'a> {
	pub index: u32,
	pub text: &'a mut WString,
}

impl<'a> MsgSend for LbGetText<'a> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			nchars => Ok(nchars as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetTextLen {
	pub index: u32,
}

impl MsgSend for LbGetTextLen {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			nchars => Ok(nchars as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbGetTopIndex {}

impl MsgSend for LbGetTopIndex {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbInitStorage {
	pub num_items: u32,
	pub memory_bytes: u32,
}

impl MsgSend for LbInitStorage {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERRSPACE => Err(co::ERROR::BAD_ARGUMENTS),
			n_items => Ok(n_items as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbInsertString {
	pub insertion_index: Option<u32>,
	pub text: WString,
}

impl MsgSend for LbInsertString {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbItemFromPoint {
	pub coords: POINT,
}

impl MsgSend for LbItemFromPoint {
	type RetType = (i32, bool);

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _) as _, HIWORD(v as _) == 1)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LB::ITEMFROMPOINT.into(),
			wparam: 0,
			lparam: u32::from(self.coords) as _,
		}
	}
}

pub_struct_msg_empty! { LbResetContent: co::LB::RESETCONTENT.into();
	/// [`LB_RESETCONTENT`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-resetcontent)
}

/// [`LB_SELECTSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/lb-selectstring)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct LbSelectString {
	pub index: Option<u32>,
	pub prefix: WString,
}

impl MsgSend for LbSelectString {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbSelItemRange {
	pub select: bool,
	pub first_item: u16,
	pub last_item: u16,
}

impl MsgSend for LbSelItemRange {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbSelItemRangeEx {
	pub first_index: u32,
	pub last_index: u32,
}

impl MsgSend for LbSelItemRangeEx {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbSetAnchorIndex {
	pub index: u32,
}

impl MsgSend for LbSetAnchorIndex {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbSetCaretIndex {
	pub index: u32,
	pub at_least_partially_visible: bool,
}

impl MsgSend for LbSetCaretIndex {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbSetColumnWidth {
	pub width: u32,
}

impl MsgSend for LbSetColumnWidth {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbSetCount {
	pub new_count: u32,
}

impl MsgSend for LbSetCount {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbSetCurSel {
	pub index: Option<u32>,
}

impl MsgSend for LbSetCurSel {
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

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbSetHorizontalExtent {
	pub width: u32,
}

impl MsgSend for LbSetHorizontalExtent {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbSetItemData {
	pub index: u32,
	pub data: isize,
}

impl MsgSend for LbSetItemData {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbSetItemHeight {
	pub index: Option<u32>,
	pub height: u8,
}

impl MsgSend for LbSetItemHeight {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbSetLocale {
	pub locale: LCID,
}

impl MsgSend for LbSetLocale {
	type RetType = SysResult<LCID>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			lcid => Ok(unsafe { LCID::from_raw(lcid as _) }),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbSetSel {
	pub select: bool,
	pub index: Option<u32>,
}

impl MsgSend for LbSetSel {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbSetTabStops<'a> {
	pub tab_stops: &'a [u32],
}

impl<'a> MsgSend for LbSetTabStops<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LbSetTopIndex {
	pub index: u32,
}

impl MsgSend for LbSetTopIndex {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LB::SETTOPINDEX.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}
