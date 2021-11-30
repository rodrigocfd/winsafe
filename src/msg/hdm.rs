//! Header control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-header-control-reference-messages),
//! whose constants have [`HDM`](crate::co::HDM) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::enums::PtIdx;
use crate::funcs::MAKEDWORD;
use crate::handles::HIMAGELIST;
use crate::msg::{MsgSend, WndMsg};
use crate::msg::macros::{zero_as_err, zero_as_none};
use crate::structs::{HDHITTESTINFO, HDITEM, HDLAYOUT, RECT};

/// [`HDM_CLEARFILTER`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-clearfilter)
/// message parameters.
///
/// Return type: `bool`.
pub struct ClearFilter {
	pub filter: Option<u32>,
}

impl MsgSend for ClearFilter {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::CLEARFILTER.into(),
			wparam: self.filter.map_or(-1, |f| f as i32) as _,
			lparam: 0,
		}
	}
}

/// [`HDM_CREATEDRAGIMAGE`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-createdragimage)
/// message parameters.
///
/// Return type: `WinResult<HIMAGELIST>`.
pub struct CreateDragImage {
	pub index: u32,
}

impl MsgSend for CreateDragImage {
	type RetType = WinResult<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HIMAGELIST(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::CREATEDRAGIMAGE.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`HDM_DELETEITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-deleteitem)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct DeleteItem {
	pub index: u32,
}

impl MsgSend for DeleteItem {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::DELETEITEM.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`HDM_EDITFILTER`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-editfilter)
/// message parameters.
///
/// Return type: `bool`.
pub struct EditFilter {
	pub index: u32,
	pub discard_changes: bool,
}

impl MsgSend for EditFilter {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::EDITFILTER.into(),
			wparam: self.index as _,
			lparam: self.discard_changes as _,
		}
	}
}

/// [`HDM_GETBITMAPMARGIN`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-getbitmapmargin)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetBitmapMargin {}

impl MsgSend for GetBitmapMargin {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETBITMAPMARGIN.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`HDM_GETFOCUSEDITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-getfocuseditem)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetFocusedItem {}

impl MsgSend for GetFocusedItem {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETFOCUSEDITEM.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`HDM_GETIMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-getimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct GetImageList {
	pub which: co::HDSIL,
}

impl MsgSend for GetImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HIMAGELIST(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETIMAGELIST.into(),
			wparam: self.which.0 as _,
			lparam: 0,
		}
	}
}

/// [`HDM_GETITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-getitem)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetItem<'a, 'b> {
	pub index: u32,
	pub hditem: &'b mut HDITEM<'a>,
}

impl<'a, 'b> MsgSend for GetItem<'a, 'b> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETITEM.into(),
			wparam: self.index as _,
			lparam: self.hditem as *mut _ as _,
		}
	}
}

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
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
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
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETITEMDROPDOWNRECT.into(),
			wparam: self.index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

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
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETITEMRECT.into(),
			wparam: self.index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

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
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETORDERARRAY.into(),
			wparam: self.buffer.len(),
			lparam: self.buffer.as_mut_ptr() as _,
		}
	}
}

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
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETOVERFLOWRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`HDM_GETUNICODEFORMAT`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-getunicodeformat)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct GetUnicodeFormat {}

impl MsgSend for GetUnicodeFormat {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`HDM_HITTEST`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-hittest)
/// message parameters.
///
/// Return type: `u32`.
pub struct HitTest<'a> {
	pub test_info: &'a mut HDHITTESTINFO,
}

impl<'a> MsgSend for HitTest<'a> {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::HITTEST.into(),
			wparam: 0,
			lparam: self.test_info as *mut _ as _,
		}
	}
}

/// [`HDM_INSERTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-insertitem)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct InsertItem<'a, 'b> {
	pub index_after: u32,
	pub hditem: &'b HDITEM<'a>,
}

impl<'a, 'b> MsgSend for InsertItem<'a, 'b> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			index => Ok(index as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::INSERTITEM.into(),
			wparam: self.index_after as _,
			lparam: self.hditem as *const _ as _,
		}
	}
}

/// [`HDM_LAYOUT`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-layout)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct Layout<'a, 'b, 'c> {
	pub hdlayout: &'c mut HDLAYOUT<'a, 'b>,
}

impl<'a, 'b, 'c> MsgSend for Layout<'a, 'b, 'c> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::LAYOUT.into(),
			wparam: 0,
			lparam: self.hdlayout as *mut _ as _,
		}
	}
}

/// [`HDM_ORDERTOINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-ordertoindex)
/// message parameters.
///
/// Return type: `u32`.
pub struct OrderToIndex {
	pub order: u32,
}

impl MsgSend for OrderToIndex {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::ORDERTOINDEX.into(),
			wparam: self.order as _,
			lparam: 0,
		}
	}
}

/// [`HDM_SETBITMAPMARGIN`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-setbitmapmargin)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetBitmapMargin {
	pub width: u32,
}

impl MsgSend for SetBitmapMargin {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETBITMAPMARGIN.into(),
			wparam: self.width as _,
			lparam: 0,
		}
	}
}

/// [`HDM_SETFILTERCHANGETIMEOUT`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-setfilterchangetimeout)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetFilterChangeTimeout {
	pub timeout_ms: u32,
}

impl MsgSend for SetFilterChangeTimeout {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETFILTERCHANGETIMEOUT.into(),
			wparam: self.timeout_ms as _,
			lparam: 0,
		}
	}
}

/// [`HDM_SETFOCUSEDITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-setfocuseditem)
/// message parameters.
///
/// Return type: `bool`.
pub struct SetFocusedItem {
	pub index: u32,
}

impl MsgSend for SetFocusedItem {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETFILTERCHANGETIMEOUT.into(),
			wparam: 0,
			lparam: self.index as _,
		}
	}
}

/// [`HDM_SETHOTDIVIDER`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-sethotdivider)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetHotDivider {
	pub value: PtIdx,
}

impl MsgSend for SetHotDivider {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETHOTDIVIDER.into(),
			wparam: match self.value {
				PtIdx::Pt(_) => true,
				PtIdx::Idx(_) => false,
			} as _,
			lparam: match self.value {
				PtIdx::Pt(pt) => MAKEDWORD(pt.x as _, pt.y as _),
				PtIdx::Idx(idx) => idx,
			} as _,
		}
	}
}

/// [`HDM_SETIMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-setimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct SetImageList {
	pub which: co::HDSIL,
	pub himagelist: Option<HIMAGELIST>,
}

impl MsgSend for SetImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|h| HIMAGELIST(h as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETIMAGELIST.into(),
			wparam: self.which.0 as _,
			lparam: self.himagelist.map_or(0, |h| h.0 as _),
		}
	}
}

/// [`HDM_SETITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-setitem)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetItem<'a, 'b> {
	pub index: u32,
	pub hditem: &'b HDITEM<'a>,
}

impl<'a, 'b> MsgSend for SetItem<'a, 'b> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETITEM.into(),
			wparam: self.index as _,
			lparam: self.hditem as *const _ as _,
		}
	}
}

/// [`HDM_SETORDERARRAY`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-setorderarray)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetOrderArray<'a> {
	pub buffer: &'a [u32],
}

impl<'a> MsgSend for SetOrderArray<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETORDERARRAY.into(),
			wparam: self.buffer.len(),
			lparam: self.buffer.as_ptr() as _,
		}
	}
}

/// [`HDM_SETUNICODEFORMAT`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-setunicodeformat)
/// message parameters.
///
/// Return type: `bool`.
pub struct SetUnicodeFormat {
	pub use_unicode: bool,
}

impl MsgSend for SetUnicodeFormat {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETUNICODEFORMAT.into(),
			wparam: self.use_unicode as _,
			lparam: 0,
		}
	}
}
