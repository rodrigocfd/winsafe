use crate::co;
use crate::decl::*;
use crate::macros::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`CB_ADDSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-addstring)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct CbAddString {
	pub text: WString,
}

impl MsgSend for CbAddString {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			CB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::ADDSTRING.into(),
			wparam: 0,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`CB_DELETESTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-deletestring)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct CbDeleteString {
	pub index: u32,
}

impl MsgSend for CbDeleteString {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::DELETESTRING.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`CB_DIR`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-dir)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct CbDir {
	pub attributes: co::DDL,
	pub path: WString,
}

impl MsgSend for CbDir {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			CB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::DIR.into(),
			wparam: self.attributes.raw() as _,
			lparam: self.path.as_ptr() as _,
		}
	}
}

/// [`CB_FINDSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-findstring)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct CbFindString {
	pub preceding_index: Option<u32>,
	pub text: WString,
}

impl MsgSend for CbFindString {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::FINDSTRING.into(),
			wparam: self.preceding_index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`CB_FINDSTRINGEXACT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-findstringexact)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct CbFindStringExact {
	pub preceding_index: Option<u32>,
	pub text: WString,
}

impl MsgSend for CbFindStringExact {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::FINDSTRINGEXACT.into(),
			wparam: self.preceding_index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`CB_GETCOMBOBOXINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getcomboboxinfo)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct CbGetComboBoxInfo<'a> {
	pub data: &'a mut COMBOBOXINFO,
}

impl<'a> MsgSend for CbGetComboBoxInfo<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETCOMBOBOXINFO.into(),
			wparam: 0,
			lparam: self.data as *mut _ as _,
		}
	}
}

/// [`CB_GETCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getcount)
/// message, which has no parameters.
///
/// Return type: `SysResult<u32>`.
pub struct CbGetCount {}

impl MsgSend for CbGetCount {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|count| count as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getcursel)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
pub struct CbGetCurSel {}

impl MsgSend for CbGetCurSel {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETCURSEL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETDROPPEDCONTROLRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getdroppedcontrolrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct CbGetDroppedControlRect<'a> {
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for CbGetDroppedControlRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETDROPPEDCONTROLRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`CB_GETDROPPEDSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getdroppedstate)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct CbGetDroppedState {}

impl MsgSend for CbGetDroppedState {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETDROPPEDSTATE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETDROPPEDWIDTH`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getdroppedwidth)
/// message, which has no parameters.
///
/// Return type: `SysResult<u32>`.
pub struct CbGetDroppedWidth {}

impl MsgSend for CbGetDroppedWidth {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			cx => Ok(cx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETDROPPEDWIDTH.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETEDITSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-geteditsel)
/// message, which has no parameters.
///
/// Return type: `(i32, i32)`.
pub struct CbGetEditSel {}

impl MsgSend for CbGetEditSel {
	type RetType = (i32, i32);

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _) as _, HIWORD(v as _) as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETEDITSEL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETEXTENDEDUI`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getextendedui)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct CbGetExtendedUi {}

impl MsgSend for CbGetExtendedUi {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETEXTENDEDUI.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETHORIZONTALEXTENT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-gethorizontalextent)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct CbGetHorizontalExtent {}

impl MsgSend for CbGetHorizontalExtent {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETHORIZONTALEXTENT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETITEMDATA`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getitemdata)
/// message parameters.
///
/// Return type: `SysResult<isize>`.
pub struct CbGetItemData {
	pub index: u32,
}

impl MsgSend for CbGetItemData {
	type RetType = SysResult<isize>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(v),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETITEMDATA.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`CB_GETITEMHEIGHT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getitemheight)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct CbGetItemHeight {
	pub component: i32,
}

impl MsgSend for CbGetItemHeight {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			cy => Ok(cy as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETITEMHEIGHT.into(),
			wparam: self.component as _,
			lparam: 0,
		}
	}
}

/// [`CB_GETLBTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getlbtext)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct CbGetLbText<'a> {
	pub index: u32,
	pub text: &'a mut WString,
}

impl<'a> MsgSend for CbGetLbText<'a> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			n => Ok(n as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETLBTEXT.into(),
			wparam: self.index as _,
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`CB_GETLBTEXTLEN`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getlbtextlen)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct CbGetLbTextLen {
	pub index: u32,
}

impl MsgSend for CbGetLbTextLen {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			n => Ok(n as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETLBTEXTLEN.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`CB_GETLOCALE`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getlocale)
/// message, which has no parameters.
///
/// Return type: `LANGID`.
pub struct CbGetLocale {}

impl MsgSend for CbGetLocale {
	type RetType = LANGID;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { LANGID::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETLOCALE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETTOPINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-gettopindex)
/// message, which has no parameters.
///
/// Return type: `SysResult<u32>`.
pub struct CbGetTopIndex {}

impl MsgSend for CbGetTopIndex {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETTOPINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_INITSTORAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-initstorage)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct CbInitStorage {
	pub num_items: u32,
	pub memory_bytes: u32,
}

impl MsgSend for CbInitStorage {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			n => Ok(n as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::INITSTORAGE.into(),
			wparam: self.num_items as _,
			lparam: self.memory_bytes as _,
		}
	}
}

/// [`CB_INSERTSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-insertstring)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct CbInsertString {
	pub index: Option<u32>,
	pub text: WString,
}

impl MsgSend for CbInsertString {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			CB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::INSERTSTRING.into(),
			wparam: self.index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`CB_LIMITTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-limittext)
/// message parameters.
///
/// Return type: `()`.
pub struct CbLimitText {
	pub max_chars: Option<u32>,
}

impl MsgSend for CbLimitText {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::LIMITTEXT.into(),
			wparam: self.max_chars.unwrap_or_default() as _,
			lparam: 0,
		}
	}
}

pub_struct_msg_empty! { CbResetContent: co::CB::RESETCONTENT.into();
	/// [`CB_RESETCONTENT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-resetcontent)
}

/// [`CB_SELECTSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-selectstring)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct CbSelectString {
	pub preceding_index: Option<u32>,
	pub search_text: WString,
}

impl MsgSend for CbSelectString {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::SELECTSTRING.into(),
			wparam: self.preceding_index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.search_text.as_ptr() as _,
		}
	}
}

/// [`CB_SETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setcursel)
/// message parameters.
///
/// Return type: `()`.
pub struct CbSetCurSel {
	pub index: Option<u32>,
}

impl MsgSend for CbSetCurSel {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::SETCURSEL.into(),
			wparam: match self.index {
				Some(index) => index as i32,
				None => -1,
			} as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETDROPPEDWIDTH`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setdroppedwidth)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct CbSetDroppedWidth {
	pub min_width: u32,
}

impl MsgSend for CbSetDroppedWidth {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			cx => Ok(cx as _),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::SETDROPPEDWIDTH.into(),
			wparam: self.min_width as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETEDITSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-seteditsel)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct CbSetEditSel {
	pub start_pos: Option<u32>,
	pub end_pos: Option<u32>,
}

impl MsgSend for CbSetEditSel {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::SETEDITSEL.into(),
			wparam: 0,
			lparam: MAKEDWORD(
				self.start_pos.map_or(-1, |pos| pos as i16) as _,
				self.end_pos.map_or(-1, |pos| pos as i16) as _,
			) as _,
		}
	}
}

/// [`CB_SETEXTENDEDUI`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setextendedui)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct CbSetExtendedUi {
	pub use_extended_ui: bool,
}

impl MsgSend for CbSetExtendedUi {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::SETEXTENDEDUI.into(),
			wparam: self.use_extended_ui as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETHORIZONTALEXTENT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-sethorizontalextent)
/// message parameters.
///
/// Return type: `()`.
pub struct CbSetHorizontalExtent {
	pub scrollable_width: u32,
}

impl MsgSend for CbSetHorizontalExtent {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::SETHORIZONTALEXTENT.into(),
			wparam: self.scrollable_width as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETITEMDATA`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setitemdata)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct CbSetItemData {
	pub index: u32,
	pub data: isize,
}

impl MsgSend for CbSetItemData {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::SETITEMDATA.into(),
			wparam: self.index as _,
			lparam: self.data,
		}
	}
}

/// [`CB_SETITEMHEIGHT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setitemheight)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct CbSetItemHeight {
	pub component: i32,
	pub height: u32,
}

impl MsgSend for CbSetItemHeight {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::SETITEMHEIGHT.into(),
			wparam: self.component as _,
			lparam: self.height as _,
		}
	}
}

/// [`CB_SETLOCALE`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setlocale)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct CbSetLocale {
	pub locale: LANGID,
}

impl MsgSend for CbSetLocale {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::SETLOCALE.into(),
			wparam: u16::from(self.locale) as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETTOPINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-settopindex)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct CbSetTopIndex {
	pub index: u32,
}

impl MsgSend for CbSetTopIndex {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::SETTOPINDEX.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`CB_SHOWDROPDOWN`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-showdropdown)
/// message parameters.
///
/// Return type: `()`.
pub struct CbShowDropDown {
	pub show: bool,
}

impl MsgSend for CbShowDropDown {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::SHOWDROPDOWN.into(),
			wparam: self.show as _,
			lparam: 0,
		}
	}
}
