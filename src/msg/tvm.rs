//! Tree view control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-tree-view-control-reference-messages),
//! whose constants have [`TVM`](crate::co::TVM) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::handles::{HIMAGELIST, HTREEITEM};
use crate::msg::{MsgSend, WndMsg};
use crate::structs::{RECT, TVINSERTSTRUCT, TVITEMEX};

/// [`TVM_DELETEITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-deleteitem)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct DeleteItem {
	pub hitem: HTREEITEM,
}

impl MsgSend for DeleteItem {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::DELETEITEM.into(),
			wparam: 0,
			lparam: self.hitem.ptr as _,
		}
	}
}

/// [`TVM_EXPAND`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-expand)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct Expand {
	pub action: co::TVE,
	pub hitem: HTREEITEM,
}

impl MsgSend for Expand {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::EXPAND.into(),
			wparam: self.action.0 as _,
			lparam: self.hitem.ptr as _,
		}
	}
}

/// [`TVM_GETCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetCount {}

impl MsgSend for GetCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETINDENT`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getindent)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetIndent {}

impl MsgSend for GetIndent {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETINDENT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETIMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct GetImageList {
	pub kind: co::TVSIL,
}

impl MsgSend for GetImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => None,
			p => Some(HIMAGELIST { ptr: p as _ }),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETIMAGELIST.into(),
			wparam: self.kind.0 as _,
			lparam: 0,
		}
	}
}

/// [`TVM_GETITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getitem)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetItem<'a, 'b> {
	pub tvitem: &'b mut TVITEMEX<'a>,
}

impl<'a, 'b> MsgSend for GetItem<'a, 'b> {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETITEM.into(),
			wparam: 0,
			lparam: self.tvitem as *const _ as _,
		}
	}
}

/// [`TVM_GETITEMHEIGHT`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getitemheight)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetItemHeight {}

impl MsgSend for GetItemHeight {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETITEMHEIGHT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETITEMRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getitemrect)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetItemRect<'a> {
	pub text_only: bool,
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for GetItemRect<'a> {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETITEMRECT.into(),
			wparam: self.text_only as _,
			lparam: self.rect as *const _ as _,
		}
	}
}

/// [`TVM_GETITEMSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getitemstate)
/// message parameters.
///
/// Return type: `co::TVIS`.
pub struct GetItemState {
	pub hitem: HTREEITEM,
	pub mask: co::TVIS,
}

impl MsgSend for GetItemState {
	type RetType = co::TVIS;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::TVIS(v as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETITEMSTATE.into(),
			wparam: self.hitem.ptr as _,
			lparam: self.mask.0 as _,
		}
	}
}

/// [`TVM_GETNEXTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getnextitem)
/// message parameters.
///
/// Return type: `Option<HTREEITEM>`.
pub struct GetNextItem {
	pub which: co::TVGN,
	pub hitem: HTREEITEM,
}

impl MsgSend for GetNextItem {
	type RetType = Option<HTREEITEM>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => None,
			p => Some(HTREEITEM { ptr: p as _ }),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETNEXTITEM.into(),
			wparam: self.which.0 as _,
			lparam: self.hitem.ptr as _,
		}
	}
}

/// [`TVM_INSERTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-insertitem)
/// message parameters.
///
/// Return type: `WinResult<HTREEITEM>`.
pub struct InsertItem<'a, 'b> {
	pub tvinsertstruct: &'b TVINSERTSTRUCT<'a>,
}

impl<'a, 'b> MsgSend for InsertItem<'a, 'b> {
	type RetType = WinResult<HTREEITEM>;

	fn_convert_ret_winresult_handle!(HTREEITEM);

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::INSERTITEM.into(),
			wparam: 0,
			lparam: self.tvinsertstruct as *const _ as _,
		}
	}
}

/// [`TVM_SELECTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-selectitem)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SelectItem {
	pub action: co::TVGN,
	pub hitem: HTREEITEM,
}

impl MsgSend for SelectItem {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SELECTITEM.into(),
			wparam: self.action.0 as _,
			lparam: self.hitem.ptr as _,
		}
	}
}

/// [`TVM_SETEXTENDEDSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-setextendedstyle)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetExtendedStyle {
	pub style: co::TVS_EX,
	pub mask: co::TVS_EX,
}

impl MsgSend for SetExtendedStyle {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match co::ERROR(v as _) {
			co::ERROR::S_OK => Ok(()),
			err => Err(err),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETEXTENDEDSTYLE.into(),
			wparam: self.style.0 as _,
			lparam: self.mask.0 as _,
		}
	}
}

/// [`TVM_SETHOT`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-sethot)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetHot {
	pub hitem: Option<HTREEITEM>,
}

impl MsgSend for SetHot {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETHOT.into(),
			wparam: 0,
			lparam: self.hitem.map_or(0, |h| h.ptr as _),
		}
	}
}

/// [`TVM_SETIMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-setimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct SetImageList {
	pub kind: co::TVSIL,
	pub himglist: Option<HIMAGELIST>,
}

impl MsgSend for SetImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => None,
			p => Some(HIMAGELIST { ptr: p as _ }),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETIMAGELIST.into(),
			wparam: self.kind.0 as _,
			lparam: self.himglist.map_or(0, |h| h.ptr as _),
		}
	}
}

/// [`TVM_SETINDENT`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-setindent)
/// message parameters.
///
/// Return type: `()`.
pub struct SetIndent {
	pub width: u32,
}

impl MsgSend for SetIndent {
	type RetType = ();

	fn_convert_ret_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETINDENT.into(),
			wparam: self.width as _,
			lparam: 0,
		}
	}
}

/// [`TVM_SETITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-setitem)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetItem<'a, 'b> {
	pub tvitem: &'b TVITEMEX<'a>,
}

impl<'a, 'b> MsgSend for SetItem<'a, 'b> {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETITEM.into(),
			wparam: 0,
			lparam: self.tvitem as *const _ as _,
		}
	}
}

/// [`TVM_SHOWINFOTIP`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-showinfotip)
/// message parameters.
///
/// Return type: `()`.
pub struct ShowInfoTip {
	pub hitem: HTREEITEM,
}

impl MsgSend for ShowInfoTip {
	type RetType = ();

	fn_convert_ret_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SHOWINFOTIP.into(),
			wparam: 0,
			lparam: self.hitem.ptr as _,
		}
	}
}

/// [`TVM_SORTCHILDREN`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-sortchildren)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SortChildren {
	pub recursive: bool,
}

impl MsgSend for SortChildren {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SORTCHILDREN.into(),
			wparam: self.recursive as _,
			lparam: 0,
		}
	}
}
