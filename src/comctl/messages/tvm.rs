use crate::co;
use crate::comctl::decl::{HIMAGELIST, HTREEITEM, TVINSERTSTRUCT, TVITEMEX};
use crate::comctl::privs::CLR_DEFAULT;
use crate::kernel::decl::SysResult;
use crate::msg::WndMsg;
use crate::prelude::{Handle, MsgSend};
use crate::user::decl::{COLORREF, HWND, RECT};
use crate::user::privs::{minus1_as_none, zero_as_err, zero_as_none};

/// [`TVM_DELETEITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-deleteitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct DeleteItem {
	pub hitem: HTREEITEM,
}

unsafe impl MsgSend for DeleteItem {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::DELETEITEM.into(),
			wparam: 0,
			lparam: self.hitem.0 as _,
		}
	}
}

/// [`TVM_EDITLABEL`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-editlabel)
/// message parameters.
///
/// Return type: `SysResult<HWND>.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct EditLabel {
	pub hitem: HTREEITEM,
}

unsafe impl MsgSend for EditLabel {
	type RetType = SysResult<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HWND(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::EDITLABEL.into(),
			wparam: 0,
			lparam: self.hitem.0 as _,
		}
	}
}

/// [`TVM_ENDEDITLABELNOW`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-endeditlabelnow)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct EndEditLabelNow {
	pub save: bool,
}

unsafe impl MsgSend for EndEditLabelNow {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::ENDEDITLABELNOW.into(),
			wparam: !self.save as _,
			lparam: 0,
		}
	}
}

/// [`TVM_ENSUREVISIBLE`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-ensurevisible)
/// message parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct EnsureVisible {
	pub hitem: HTREEITEM,
}

unsafe impl MsgSend for EnsureVisible {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::ENSUREVISIBLE.into(),
			wparam: 0,
			lparam: self.hitem.0 as _,
		}
	}
}

/// [`TVM_EXPAND`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-expand)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct Expand {
	pub action: co::TVE,
	pub hitem: HTREEITEM,
}

unsafe impl MsgSend for Expand {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::EXPAND.into(),
			wparam: self.action.0 as _,
			lparam: self.hitem.0 as _,
		}
	}
}

/// [`TVM_GETBKCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getbkcolor)
/// message, which has no parameters.
///
/// Return type: `Option<COLORREF>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetBkColor {}

unsafe impl MsgSend for GetBkColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| COLORREF(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETBKCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetCount {}

unsafe impl MsgSend for GetCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETEDITCONTROL`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-geteditcontrol)
/// message, which has no parameters.
///
/// Return type: `SysResult<HWND>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetEditControl {}

unsafe impl MsgSend for GetEditControl {
	type RetType = SysResult<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HWND(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETEDITCONTROL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETEXTENDEDSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getextendedstyle)
/// message, which has no parameters.
///
/// Return type: `co::TVS_EX`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetExtendedStyle {}

unsafe impl MsgSend for GetExtendedStyle {
	type RetType = co::TVS_EX;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::TVS_EX(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETEXTENDEDSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETIMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetImageList {
	pub kind: co::TVSIL,
}

unsafe impl MsgSend for GetImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HIMAGELIST(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETIMAGELIST.into(),
			wparam: self.kind.0 as _,
			lparam: 0,
		}
	}
}

/// [`TVM_GETINDENT`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getindent)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetIndent {}

unsafe impl MsgSend for GetIndent {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETINDENT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETINSERTMARKCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getinsertmarkcolor)
/// message, which has no parameters.
///
/// Return type: `COLORREF`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetInsertMarkColor {}

unsafe impl MsgSend for GetInsertMarkColor {
	type RetType = COLORREF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		COLORREF(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETINSERTMARKCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetItem<'a, 'b> {
	pub tvitem: &'b mut TVITEMEX<'a>,
}

unsafe impl<'a, 'b> MsgSend for GetItem<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETITEM.into(),
			wparam: 0,
			lparam: self.tvitem as *mut _ as _,
		}
	}
}

/// [`TVM_GETITEMHEIGHT`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getitemheight)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetItemHeight {}

unsafe impl MsgSend for GetItemHeight {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
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
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetItemRect<'a> {
	pub text_only: bool,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetItemRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETITEMRECT.into(),
			wparam: self.text_only as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TVM_GETITEMSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getitemstate)
/// message parameters.
///
/// Return type: `co::TVIS`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetItemState {
	pub hitem: HTREEITEM,
	pub mask: co::TVIS,
}

unsafe impl MsgSend for GetItemState {
	type RetType = co::TVIS;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::TVIS(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETITEMSTATE.into(),
			wparam: self.hitem.0 as _,
			lparam: self.mask.0 as _,
		}
	}
}

/// [`TVM_GETLINECOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getlinecolor)
/// message, which has no parameters.
///
/// Return type: `Option<COLORREF>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetLineColor {}

unsafe impl MsgSend for GetLineColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as u32 {
			CLR_DEFAULT => None,
			c => Some(COLORREF(c))
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETLINECOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETNEXTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getnextitem)
/// message parameters.
///
/// Return type: `Option<HTREEITEM>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetNextItem {
	pub relationship: co::TVGN,
	pub hitem: Option<HTREEITEM>,
}

unsafe impl MsgSend for GetNextItem {
	type RetType = Option<HTREEITEM>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HTREEITEM(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETNEXTITEM.into(),
			wparam: self.relationship.0 as _,
			lparam: self.hitem.unwrap_or(HTREEITEM::NULL).0 as _,
		}
	}
}

/// [`TVM_GETSCROLLTIME`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getscrolltime)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetScrollTime {}

unsafe impl MsgSend for GetScrollTime {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETSCROLLTIME.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETTEXTCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-gettextcolor)
/// message, which has no parameters.
///
/// Return type: `Option<COLORREF>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetTextColor {}

unsafe impl MsgSend for GetTextColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| COLORREF(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETTEXTCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETTOOLTIPS`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-gettooltips)
/// message, which has no parameters.
///
/// Return type: `Option<HWND>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetTooltips {}

unsafe impl MsgSend for GetTooltips {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HWND(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETTOOLTIPS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETVISIBLECOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-getvisiblecount)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetVisibleCount {}

unsafe impl MsgSend for GetVisibleCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETVISIBLECOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_INSERTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-insertitem)
/// message parameters.
///
/// Return type: `SysResult<HTREEITEM>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct InsertItem<'a, 'b> {
	pub tvinsertstruct: &'b TVINSERTSTRUCT<'a>,
}

unsafe impl<'a, 'b> MsgSend for InsertItem<'a, 'b> {
	type RetType = SysResult<HTREEITEM>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HTREEITEM(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
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
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SelectItem {
	pub action: co::TVGN,
	pub hitem: HTREEITEM,
}

unsafe impl MsgSend for SelectItem {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SELECTITEM.into(),
			wparam: self.action.0 as _,
			lparam: self.hitem.0 as _,
		}
	}
}

/// [`TVM_SETHOT`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-sethot)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetHot {
	pub hitem: Option<HTREEITEM>,
}

unsafe impl MsgSend for SetHot {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETHOT.into(),
			wparam: 0,
			lparam: self.hitem.map_or(0, |h| h.0 as _),
		}
	}
}

/// [`TVM_SETIMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-setimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetImageList {
	pub kind: co::TVSIL,
	pub himglist: Option<HIMAGELIST>,
}

unsafe impl MsgSend for SetImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HIMAGELIST(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETIMAGELIST.into(),
			wparam: self.kind.0 as _,
			lparam: self.himglist.map_or(0, |h| h.0 as _),
		}
	}
}

/// [`TVM_SETINDENT`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-setindent)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetIndent {
	pub width: u32,
}

unsafe impl MsgSend for SetIndent {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
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
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetItem<'a, 'b> {
	pub tvitem: &'b TVITEMEX<'a>,
}

unsafe impl<'a, 'b> MsgSend for SetItem<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
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
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct ShowInfoTip {
	pub hitem: HTREEITEM,
}

unsafe impl MsgSend for ShowInfoTip {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SHOWINFOTIP.into(),
			wparam: 0,
			lparam: self.hitem.0 as _,
		}
	}
}

/// [`TVM_SORTCHILDREN`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-sortchildren)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SortChildren {
	pub recursive: bool,
}

unsafe impl MsgSend for SortChildren {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SORTCHILDREN.into(),
			wparam: self.recursive as _,
			lparam: 0,
		}
	}
}
