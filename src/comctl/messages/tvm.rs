use crate::co;
use crate::comctl::privs::*;
use crate::decl::*;
use crate::msg::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`TVM_CREATEDRAGIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-createdragimage)
/// message parameters.
///
/// Return type: `SysResult<HIMAGELIST>`.
pub struct TvmCreateDragImage<'a> {
	pub hitem: &'a HTREEITEM,
}

impl<'a> MsgSend for TvmCreateDragImage<'a> {
	type RetType = SysResult<HIMAGELIST>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::CREATEDRAGIMAGE.into(),
			wparam: 0,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_DELETEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-deleteitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TvmDeleteItem<'a> {
	pub hitem: &'a HTREEITEM,
}

impl<'a> MsgSend for TvmDeleteItem<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::DELETEITEM.into(),
			wparam: 0,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_EDITLABEL`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-editlabel)
/// message parameters.
///
/// Return type: `SysResult<HWND>`.
pub struct TvmEditLabel<'a> {
	pub hitem: &'a HTREEITEM,
}

impl<'a> MsgSend for TvmEditLabel<'a> {
	type RetType = SysResult<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::EDITLABEL.into(),
			wparam: 0,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_ENDEDITLABELNOW`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-endeditlabelnow)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TvmEndEditLabelNow {
	pub save: bool,
}

impl MsgSend for TvmEndEditLabelNow {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::ENDEDITLABELNOW.into(),
			wparam: !self.save as _,
			lparam: 0,
		}
	}
}

/// [`TVM_ENSUREVISIBLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-ensurevisible)
/// message parameters.
///
/// Return type: `u32`.
pub struct TvmEnsureVisible<'a> {
	pub hitem: &'a HTREEITEM,
}

impl<'a> MsgSend for TvmEnsureVisible<'a> {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::ENSUREVISIBLE.into(),
			wparam: 0,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_EXPAND`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-expand)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TvmExpand<'a> {
	pub action: co::TVE,
	pub hitem: &'a HTREEITEM,
}

impl<'a> MsgSend for TvmExpand<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::EXPAND.into(),
			wparam: self.action.raw() as _,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_GETBKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getbkcolor)
/// message, which has no parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct TvmGetBkColor {}

impl MsgSend for TvmGetBkColor {
	type RetType = Option<COLORREF>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| unsafe { COLORREF::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETBKCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TvmGetCount {}

impl MsgSend for TvmGetCount {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETEDITCONTROL`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-geteditcontrol)
/// message, which has no parameters.
///
/// Return type: `SysResult<HWND>`.
pub struct TvmGetEditControl {}

impl MsgSend for TvmGetEditControl {
	type RetType = SysResult<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETEDITCONTROL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETEXTENDEDSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getextendedstyle)
/// message, which has no parameters.
///
/// Return type: `co::TVS_EX`.
pub struct TvmGetExtendedStyle {}

impl MsgSend for TvmGetExtendedStyle {
	type RetType = co::TVS_EX;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::TVS_EX::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETEXTENDEDSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct TvmGetImageList {
	pub kind: co::TVSIL,
}

impl MsgSend for TvmGetImageList {
	type RetType = Option<HIMAGELIST>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETIMAGELIST.into(),
			wparam: self.kind.raw() as _,
			lparam: 0,
		}
	}
}

/// [`TVM_GETINDENT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getindent)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TvmGetIndent {}

impl MsgSend for TvmGetIndent {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETINDENT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETINSERTMARKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getinsertmarkcolor)
/// message, which has no parameters.
///
/// Return type: `COLORREF`.
pub struct TvmGetInsertMarkColor {}

impl MsgSend for TvmGetInsertMarkColor {
	type RetType = COLORREF;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETINSERTMARKCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETISEARCHSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getisearchstring)
/// message parameters.
///
/// Return type: `u32`.
pub struct TvmGetISearchString<'a> {
	pub buf: &'a mut WString,
}

impl<'a> MsgSend for TvmGetISearchString<'a> {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETISEARCHSTRING.into(),
			wparam: 0,
			lparam: unsafe { self.buf.as_mut_ptr() } as _,
		}
	}
}

/// [`TVM_GETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TvmGetItem<'a, 'b> {
	pub tvitem: &'b mut TVITEMEX<'a>,
}

impl<'a, 'b> MsgSend for TvmGetItem<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETITEM.into(),
			wparam: 0,
			lparam: self.tvitem as *mut _ as _,
		}
	}
}

/// [`TVM_GETITEMHEIGHT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getitemheight)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TvmGetItemHeight {}

impl MsgSend for TvmGetItemHeight {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETITEMHEIGHT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETITEMRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getitemrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TvmGetItemRect<'a> {
	pub text_only: bool,
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for TvmGetItemRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETITEMRECT.into(),
			wparam: self.text_only as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TVM_GETITEMSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getitemstate)
/// message parameters.
///
/// Return type: `co::TVIS`.
pub struct TvmGetItemState<'a> {
	pub hitem: &'a HTREEITEM,
	pub mask: co::TVIS,
}

impl<'a> MsgSend for TvmGetItemState<'a> {
	type RetType = co::TVIS;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::TVIS::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETITEMSTATE.into(),
			wparam: self.hitem.ptr() as _,
			lparam: self.mask.raw() as _,
		}
	}
}

/// [`TVM_GETLINECOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getlinecolor)
/// message, which has no parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct TvmGetLineColor {}

impl MsgSend for TvmGetLineColor {
	type RetType = Option<COLORREF>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as u32 {
			CLR_DEFAULT => None,
			c => Some(unsafe { COLORREF::from_raw(c) }),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETLINECOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETNEXTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getnextitem)
/// message parameters.
///
/// Return type: `Option<HTREEITEM>`.
pub struct TvmGetNextItem<'a> {
	pub relationship: co::TVGN,
	pub hitem: Option<&'a HTREEITEM>,
}

impl<'a> MsgSend for TvmGetNextItem<'a> {
	type RetType = Option<HTREEITEM>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HTREEITEM::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETNEXTITEM.into(),
			wparam: self.relationship.raw() as _,
			lparam: self.hitem.map_or(0, |h| h.ptr() as _),
		}
	}
}

/// [`TVM_GETSCROLLTIME`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getscrolltime)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TvmGetScrollTime {}

impl MsgSend for TvmGetScrollTime {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETSCROLLTIME.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETTEXTCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-gettextcolor)
/// message, which has no parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct TvmGetTextColor {}

impl MsgSend for TvmGetTextColor {
	type RetType = Option<COLORREF>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| unsafe { COLORREF::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETTEXTCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-gettooltips)
/// message, which has no parameters.
///
/// Return type: `Option<HWND>`.
pub struct TvmGetTooltips {}

impl MsgSend for TvmGetTooltips {
	type RetType = Option<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETTOOLTIPS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`HDM_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getunicodeformat)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct TvmGetUnicodeFormat {}

impl MsgSend for TvmGetUnicodeFormat {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETVISIBLECOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getvisiblecount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TvmGetVisibleCount {}

impl MsgSend for TvmGetVisibleCount {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::GETVISIBLECOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_HITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-hittest)
/// message parameters.
///
/// Return type: `Option<HTREEITEM>`.
pub struct TvmHitTest<'a> {
	pub info: &'a TVHITTESTINFO,
}

impl<'a> MsgSend for TvmHitTest<'a> {
	type RetType = Option<HTREEITEM>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HTREEITEM::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::HITTEST.into(),
			wparam: 0,
			lparam: &mut self.info as *mut _ as _,
		}
	}
}

/// [`TVM_INSERTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-insertitem)
/// message parameters.
///
/// Return type: `SysResult<HTREEITEM>`.
pub struct TvmInsertItem<'a, 'b> {
	pub item: &'b TVINSERTSTRUCT<'a>,
}

impl<'a, 'b> MsgSend for TvmInsertItem<'a, 'b> {
	type RetType = SysResult<HTREEITEM>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HTREEITEM::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::INSERTITEM.into(),
			wparam: 0,
			lparam: self.item as *const _ as _,
		}
	}
}

/// [`TVM_MAPACCIDTOHTREEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-mapaccidtohtreeitem)
/// message parameters.
///
/// Return type: `Option<HTREEITEM>`.
pub struct TvmMapAccIdToHtreeitem {
	pub acc_id: u32,
}

impl MsgSend for TvmMapAccIdToHtreeitem {
	type RetType = SysResult<HTREEITEM>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HTREEITEM::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::MAPACCIDTOHTREEITEM.into(),
			wparam: self.acc_id as _,
			lparam: 0,
		}
	}
}

/// [`TVM_MAPHTREEITEMTOACCID`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-maphtreeitemtoaccid)
/// message parameters.
///
/// Return type: `u32`.
pub struct TvmMapHtreeitemToAccId<'a> {
	pub hitem: &'a HTREEITEM,
}

impl<'a> MsgSend for TvmMapHtreeitemToAccId<'a> {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::MAPHTREEITEMTOACCID.into(),
			wparam: self.hitem.ptr() as _,
			lparam: 0,
		}
	}
}

/// [`TVM_SELECTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-selectitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TvmSelectItem<'a> {
	pub action: co::TVGN,
	pub hitem: &'a HTREEITEM,
}

impl<'a> MsgSend for TvmSelectItem<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SELECTITEM.into(),
			wparam: self.action.raw() as _,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_SETAUTOSCROLLINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setautoscrollinfo)
/// message parameters.
///
/// Return type: `()`.
pub struct TvmSetAutoScrollInfo {
	pub pixels_per_second: u32,
	pub redraw_interval: u32,
}

impl MsgSend for TvmSetAutoScrollInfo {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETAUTOSCROLLINFO.into(),
			wparam: self.pixels_per_second as _,
			lparam: self.redraw_interval as _,
		}
	}
}

/// [`TVM_SETBKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setbkcolor)
/// message parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct TvmSetBkColor {
	pub color: Option<COLORREF>,
}

impl MsgSend for TvmSetBkColor {
	type RetType = Option<COLORREF>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => None,
			v => Some(unsafe { COLORREF::from_raw(v as _) }),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETBKCOLOR.into(),
			wparam: 0,
			lparam: self.color.map_or(-1, |color| u32::from(color) as _),
		}
	}
}

/// [`TVM_SETBORDER`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setborder)
/// message parameters.
///
/// Return type: `(u16, u16)`.
pub struct TvmSetBorder {
	pub action: co::TVSBF,
	pub left: u16,
	pub top: u16,
}

impl MsgSend for TvmSetBorder {
	type RetType = (u16, u16);

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), HIWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETBORDER.into(),
			wparam: self.action.raw() as _,
			lparam: MAKEDWORD(self.left, self.top) as _,
		}
	}
}

/// [`TVM_SETEXTENDEDSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setextendedstyle)
/// message parameters.
///
/// Return type: `HrResult<()>`.
pub struct TvmSetExtendedStyle {
	pub style: co::TVS_EX,
	pub mask: co::TVS_EX,
}

impl MsgSend for TvmSetExtendedStyle {
	type RetType = HrResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		HrRet(v as _).to_hrresult()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETEXTENDEDSTYLE.into(),
			wparam: self.style.raw() as _,
			lparam: self.mask.raw() as _,
		}
	}
}

/// [`TVM_SETHOT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-sethot)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TvmSetHot<'a> {
	pub hitem: Option<&'a HTREEITEM>,
}

impl<'a> MsgSend for TvmSetHot<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETHOT.into(),
			wparam: 0,
			lparam: self.hitem.map_or(0, |h| h.ptr() as _),
		}
	}
}

/// [`TVM_SETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct TvmSetImageList {
	pub kind: co::TVSIL,
	pub himagelist: Option<HIMAGELIST>,
}

impl MsgSend for TvmSetImageList {
	type RetType = Option<HIMAGELIST>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETIMAGELIST.into(),
			wparam: self.kind.raw() as _,
			lparam: self.himagelist.as_ref().map_or(0, |h| h.ptr() as _),
		}
	}
}

/// [`TVM_SETINDENT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setindent)
/// message parameters.
///
/// Return type: `()`.
pub struct TvmSetIndent {
	pub width: u32,
}

impl MsgSend for TvmSetIndent {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETINDENT.into(),
			wparam: self.width as _,
			lparam: 0,
		}
	}
}

/// [`TVM_SETINSERTMARK`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setinsertmark)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TvmSetInsertMark<'a> {
	pub insert_after: bool,
	pub hitem: &'a HTREEITEM,
}

impl<'a> MsgSend for TvmSetInsertMark<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETINSERTMARK.into(),
			wparam: self.insert_after as _,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_SETINSERTMARKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setinsertmarkcolor)
/// message parameters.
///
/// Return type: `COLORREF`.
pub struct TvmSetInsertMarkColor {
	pub color: COLORREF,
}

impl MsgSend for TvmSetInsertMarkColor {
	type RetType = COLORREF;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETINSERTMARKCOLOR.into(),
			wparam: 0,
			lparam: u32::from(self.color) as _,
		}
	}
}

/// [`TVM_SETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TvmSetItem<'a, 'b> {
	pub tvitem: &'b TVITEMEX<'a>,
}

impl<'a, 'b> MsgSend for TvmSetItem<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETITEM.into(),
			wparam: 0,
			lparam: self.tvitem as *const _ as _,
		}
	}
}

/// [`TVM_SETITEMHEIGHT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setitemheight)
/// message parameters.
///
/// Return type: `u32`.
pub struct TvmSetItemHeight {
	pub height: Option<u32>,
}

impl MsgSend for TvmSetItemHeight {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETITEMHEIGHT.into(),
			wparam: self.height.map_or(-1, |h| h as _) as _,
			lparam: 0,
		}
	}
}

/// [`TVM_SETLINECOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setlinecolor)
/// message parameters.
///
/// Return type: `COLORREF`.
pub struct TvmSetLineColor {
	pub color: Option<COLORREF>,
}

impl MsgSend for TvmSetLineColor {
	type RetType = COLORREF;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETLINECOLOR.into(),
			wparam: 0,
			lparam: self.color.map_or(CLR_DEFAULT, |c| c.into()) as _,
		}
	}
}

/// [`TVM_SETSCROLLTIME`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setscrolltime)
/// message parameters.
///
/// Return type: `u32`.
pub struct TvmSetScrollTime {
	pub time_ms: u32,
}

impl MsgSend for TvmSetScrollTime {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETSCROLLTIME.into(),
			wparam: self.time_ms as _,
			lparam: 0,
		}
	}
}

/// [`TVM_SETTEXTCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-settextcolor)
/// message parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct TvmSetTextColor {
	pub color: Option<COLORREF>,
}

impl MsgSend for TvmSetTextColor {
	type RetType = Option<COLORREF>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => None,
			v => Some(unsafe { COLORREF::from_raw(v as _) }),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETTEXTCOLOR.into(),
			wparam: 0,
			lparam: self.color.map_or(-1, |color| u32::from(color) as _),
		}
	}
}

/// [`TVM_SETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-settooltips)
/// message parameters.
///
/// Return type: `Option<HWND>`.
pub struct TvmSetTooltips<'a> {
	pub htooltips: Option<&'a HWND>,
}

impl<'a> MsgSend for TvmSetTooltips<'a> {
	type RetType = Option<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETTOOLTIPS.into(),
			wparam: self.htooltips.map_or(0, |h| h.ptr() as _),
			lparam: 0,
		}
	}
}

/// [`TVM_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setunicodeformat)
/// message parameters.
///
/// Return type: `bool`.
pub struct TvmSetUnicodeFormat {
	pub use_unicode: bool,
}

impl MsgSend for TvmSetUnicodeFormat {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SETUNICODEFORMAT.into(),
			wparam: self.use_unicode as _,
			lparam: 0,
		}
	}
}

/// [`TVM_SHOWINFOTIP`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-showinfotip)
/// message parameters.
///
/// Return type: `()`.
pub struct TvmShowInfoTip<'a> {
	pub hitem: &'a HTREEITEM,
}

impl<'a> MsgSend for TvmShowInfoTip<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SHOWINFOTIP.into(),
			wparam: 0,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_SORTCHILDREN`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-sortchildren)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TvmSortChildren {
	pub recursive: bool,
}

impl MsgSend for TvmSortChildren {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SORTCHILDREN.into(),
			wparam: self.recursive as _,
			lparam: 0,
		}
	}
}

/// [`TVM_SORTCHILDRENCB`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-sortchildrencb)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TvmSortChildrenCb<'a> {
	pub info: &'a TVSORTCB,
}

impl<'a> MsgSend for TvmSortChildrenCb<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TVM::SORTCHILDRENCB.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}
