use crate::co;
use crate::kernel::decl::{HIWORD, LANGID, LOWORD, MAKEDWORD, WinResult,
	WString};
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::decl::{COMBOBOXINFO, RECT};
use crate::user::privs::{CB_ERR, CB_ERRSPACE, zero_as_err};

/// [`CB_ADDSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-addstring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct AddString {
	pub text: WString,
}

unsafe impl MsgSend for AddString {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			CB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::ADDSTRING.into(),
			wparam: 0,
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`CB_DELETESTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-deletestring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct DeleteString {
	pub index: u32,
}

unsafe impl MsgSend for DeleteString {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
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
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Dir {
	pub attributes: co::DDL,
	pub path: WString,
}

unsafe impl MsgSend for Dir {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			CB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::DIR.into(),
			wparam: self.attributes.0 as _,
			lparam: unsafe { self.path.as_ptr() } as _,
		}
	}
}

/// [`CB_FINDSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-findstring)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct FindString {
	pub preceding_index: Option<u32>,
	pub text: WString,
}

unsafe impl MsgSend for FindString {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::FINDSTRING.into(),
			wparam: self.preceding_index.map_or(-1, |idx| idx as i32) as _,
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`CB_FINDSTRINGEXACT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-findstringexact)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct FindStringExact {
	pub preceding_index: Option<u32>,
	pub text: WString,
}

unsafe impl MsgSend for FindStringExact {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::FINDSTRINGEXACT.into(),
			wparam: self.preceding_index.map_or(-1, |idx| idx as i32) as _,
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`CB_GETCOMBOBOXINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getcomboboxinfo)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetComboBoxInfo<'a> {
	pub data: &'a mut COMBOBOXINFO,
}

unsafe impl<'a> MsgSend for GetComboBoxInfo<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETCOMBOBOXINFO.into(),
			wparam: 0,
			lparam: self.data as *mut _ as _,
		}
	}
}

/// [`CB_GETCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getcount)
/// message, which has no parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetCount {}

unsafe impl MsgSend for GetCount {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|count| count as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
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
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetCurSel {}

unsafe impl MsgSend for GetCurSel {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETCURSEL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETDROPPEDCONTROLRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getdroppedcontrolrect)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetDroppedControlRect<'a> {
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetDroppedControlRect<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETDROPPEDCONTROLRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`CB_GETDROPPEDSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getdroppedstate)
/// message, which has no parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetDroppedState {}

unsafe impl MsgSend for GetDroppedState {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETDROPPEDSTATE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETDROPPEDWIDTH`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getdroppedwidth)
/// message, which has no parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetDroppedWidth {}

unsafe impl MsgSend for GetDroppedWidth {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			cx => Ok(cx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETDROPPEDWIDTH.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETEDITSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-geteditsel)
/// message, which has no parameters.
///
/// Return type: `(i32, i32)`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetEditSel {}

unsafe impl MsgSend for GetEditSel {
	type RetType = (i32, i32);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _) as _, HIWORD(v as _) as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETEDITSEL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETEXTENDEDUI`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getextendedui)
/// message, which has no parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetExtendedUi {}

unsafe impl MsgSend for GetExtendedUi {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETEXTENDEDUI.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETHORIZONTALEXTENT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-gethorizontalextent)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetHorizontalExtent {}

unsafe impl MsgSend for GetHorizontalExtent {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETHORIZONTALEXTENT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETITEMDATA`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getitemdata)
/// message parameters.
///
/// Return type: `WinResult<isize>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetItemData {
	pub index: u32,
}

unsafe impl MsgSend for GetItemData {
	type RetType = WinResult<isize>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(v),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETITEMDATA.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`CB_GETITEMHEIGHT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getitemheight)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetItemHeight {
	pub component: i32,
}

unsafe impl MsgSend for GetItemHeight {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			cy => Ok(cy as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETITEMHEIGHT.into(),
			wparam: self.component as _,
			lparam: 0,
		}
	}
}

/// [`CB_GETLBTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getlbtext)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetLbText<'a> {
	pub index: u32,
	pub text: &'a mut WString,
}

unsafe impl<'a> MsgSend for GetLbText<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			n => Ok(n as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETLBTEXT.into(),
			wparam: self.index as _,
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`CB_GETLBTEXTLEN`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getlbtextlen)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetLbTextLen {
	pub index: u32,
}

unsafe impl MsgSend for GetLbTextLen {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			n => Ok(n as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETLBTEXTLEN.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`CB_GETLOCALE`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getlocale)
/// message, which has no parameters.
///
/// Return type: `LANGID`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetLocale {}

unsafe impl MsgSend for GetLocale {
	type RetType = LANGID;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		LANGID(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETLOCALE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETTOPINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-gettopindex)
/// message, which has no parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetTopIndex {}

unsafe impl MsgSend for GetTopIndex {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
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
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct InitStorage {
	pub num_items: u32,
	pub memory_bytes: u32,
}

unsafe impl MsgSend for InitStorage {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			n => Ok(n as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::INITSTORAGE.into(),
			wparam: self.num_items as _,
			lparam: self.memory_bytes as _,
		}
	}
}

/// [`CB_INSERTSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-insertstring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct InsertString {
	pub index: Option<u32>,
	pub text: WString,
}

unsafe impl MsgSend for InsertString {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			CB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::INSERTSTRING.into(),
			wparam: self.index.map_or(-1, |idx| idx as i32) as _,
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`CB_LIMITTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-limittext)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct LimitText {
	pub max_chars: Option<u32>,
}

unsafe impl MsgSend for LimitText {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::LIMITTEXT.into(),
			wparam: self.max_chars.unwrap_or(0) as _,
			lparam: 0,
		}
	}
}

pub_struct_msg_empty! { ResetContent: co::CB::RESETCONTENT.into(); "user";
	/// [`CB_RESETCONTENT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-resetcontent)
}

/// [`CB_SELECTSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-selectstring)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SelectString {
	pub preceding_index: Option<u32>,
	pub search_text: WString,
}

unsafe impl MsgSend for SelectString {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SELECTSTRING.into(),
			wparam: self.preceding_index.map_or(-1, |idx| idx as i32) as _,
			lparam: unsafe { self.search_text.as_ptr() } as _,
		}
	}
}

/// [`CB_SETCURSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-setcursel)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetCurSel {
	pub index: Option<u32>,
}

unsafe impl MsgSend for SetCurSel {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
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

/// [`CB_SETDROPPEDWIDTH`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-setdroppedwidth)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetDroppedWidth {
	pub min_width: u32,
}

unsafe impl MsgSend for SetDroppedWidth {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			cx => Ok(cx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETDROPPEDWIDTH.into(),
			wparam: self.min_width as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETEDITSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-seteditsel)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetEditSel {
	pub start_pos: Option<u32>,
	pub end_pos: Option<u32>,
}

unsafe impl MsgSend for SetEditSel {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETEDITSEL.into(),
			wparam: 0,
			lparam: MAKEDWORD(
				self.start_pos.map_or(-1, |pos| pos as i16) as _,
				self.end_pos.map_or(-1, |pos| pos as i16) as _,
			) as _,
		}
	}
}

/// [`CB_SETEXTENDEDUI`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-setextendedui)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetExtendedUi {
	pub use_extended_ui: bool,
}

unsafe impl MsgSend for SetExtendedUi {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETEXTENDEDUI.into(),
			wparam: self.use_extended_ui as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETHORIZONTALEXTENT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-sethorizontalextent)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetHorizontalExtent {
	pub scrollable_width: u32,
}

unsafe impl MsgSend for SetHorizontalExtent {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETHORIZONTALEXTENT.into(),
			wparam: self.scrollable_width as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETITEMDATA`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-setitemdata)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetItemData {
	pub index: u32,
	pub data: isize,
}

unsafe impl MsgSend for SetItemData {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETITEMDATA.into(),
			wparam: self.index as _,
			lparam: self.data,
		}
	}
}

/// [`CB_SETITEMHEIGHT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-setitemheight)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetItemHeight {
	pub component: i32,
	pub height: u32,
}

unsafe impl MsgSend for SetItemHeight {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETITEMHEIGHT.into(),
			wparam: self.component as _,
			lparam: self.height as _,
		}
	}
}

/// [`CB_SETLOCALE`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-setlocale)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetLocale {
	pub locale: LANGID,
}

unsafe impl MsgSend for SetLocale {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETLOCALE.into(),
			wparam: self.locale.0 as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETTOPINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-settopindex)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetTopIndex {
	pub index: u32,
}

unsafe impl MsgSend for SetTopIndex {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
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
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct ShowDropDown {
	pub show: bool,
}

unsafe impl MsgSend for ShowDropDown {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SHOWDROPDOWN.into(),
			wparam: self.show as _,
			lparam: 0,
		}
	}
}
