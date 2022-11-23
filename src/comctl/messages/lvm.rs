use crate::co;
use crate::comctl::decl::{
	HIMAGELIST, LVBKIMAGE, LVCOLUMN, LVFINDINFO, LVFOOTERINFO, LVFOOTERITEM,
	LVGROUP, LVGROUPMETRICS, LVHITTESTINFO, LVINSERTGROUPSORTED, LVINSERTMARK,
	LVITEM, LVITEMINDEX, LVSETINFOTIP, LVTILEINFO, LVTILEVIEWINFO, PFNLVCOMPARE,
	PFNLVGROUPCOMPARE,
};
use crate::kernel::decl::{MAKEDWORD, SysResult, WString};
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::decl::{COLORREF, HCURSOR, HWND, POINT, RECT, SIZE};
use crate::user::privs::{
	minus1_as_err, minus1_as_none, zero_as_err, zero_as_none,
};

/// [`LVM_APPROXIMATEVIEWRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-approximateviewrect)
/// message parameters.
///
/// Return type: `SIZE`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct ApproximateViewRect {
	pub num_items: Option<u32>,
	pub proposed_x: Option<u16>,
	pub proposed_y: Option<u16>,
}

unsafe impl MsgSend for ApproximateViewRect {
	type RetType = SIZE;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		SIZE::from(v as u32)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::APPROXIMATEVIEWRECT.into(),
			wparam: self.num_items.map_or(-1, |n| n as i32) as _,
			lparam: MAKEDWORD(
				self.proposed_x.map_or(-1, |x| x as i32) as _,
				self.proposed_y.map_or(-1, |y| y as i32) as _,
			) as _,
		}
	}
}


/// [`LVM_ARRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-arrange)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct Arrange {
	pub arrangement: co::LVA,
}

unsafe impl MsgSend for Arrange {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::ARRANGE.into(),
			wparam: self.arrangement.0 as _,
			lparam: 0,
		}
	}
}

pub_struct_msg_empty! { CancelEditLabel: co::LVM::CANCELEDITLABEL.into(); "comctl";
	/// [`LVM_CANCELEDITLABEL`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-canceleditlabel)
}

/// [`LVM_CREATEDRAGIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-createdragimage)
/// message parameters.
///
/// Return type: `SysResult<HIMAGELIST>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct CreateDragImage<'a> {
	pub index: u32,
	pub img_location: &'a mut RECT,
}

unsafe impl<'a> MsgSend for CreateDragImage<'a> {
	type RetType = SysResult<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|h| HIMAGELIST(h as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::CREATEDRAGIMAGE.into(),
			wparam: self.index as _,
			lparam: self.img_location as *mut _ as _,
		}
	}
}

/// [`LVM_DELETEALLITEMS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-deleteallitems)
/// message, which has no parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct DeleteAllItems {}

unsafe impl MsgSend for DeleteAllItems {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::DELETEALLITEMS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_DELETECOLUMN`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-deletecolumn)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct DeleteColumn {
	pub index: u32,
}

unsafe impl MsgSend for DeleteColumn {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::DELETECOLUMN.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LVM_DELETEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-deleteitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct DeleteItem {
	pub index: u32,
}

unsafe impl MsgSend for DeleteItem {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::DELETEITEM.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LVM_EDITLABEL`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-editlabel)
/// message parameters.
///
/// Return type: `SysResult<HWND>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct EditLabel {
	pub index: Option<u32>,
}

unsafe impl MsgSend for EditLabel {
	type RetType = SysResult<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|h| HWND(h as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::EDITLABEL.into(),
			wparam: self.index.map_or(-1, |i| i as i32) as _,
			lparam: 0,
		}
	}
}

/// [`LVM_ENABLEGROUPVIEW`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-enablegroupview)
/// message parameters.
///
/// Return type: `SysResult<bool>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct EnableGroupView {
	pub enable: bool,
}

unsafe impl MsgSend for EnableGroupView {
	type RetType = SysResult<bool>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_err(v).map(|v| v != 0)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::ENABLEGROUPVIEW.into(),
			wparam: self.enable as _,
			lparam: 0,
		}
	}
}

/// [`LVM_ENSUREVISIBLE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-ensurevisible)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct EnsureVisible {
	pub index: u32,
	pub entirely_visible: bool,
}

unsafe impl MsgSend for EnsureVisible {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::ENSUREVISIBLE.into(),
			wparam: self.index as _,
			lparam: !self.entirely_visible as _,
		}
	}
}

/// [`LVM_FINDITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-finditem)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct FindItem<'a, 'b> {
	pub start_index: Option<u32>,
	pub lvfindinfo: &'b LVFINDINFO<'a>,
}

unsafe impl<'a, 'b> MsgSend for FindItem<'a, 'b> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::FINDITEM.into(),
			wparam: self.start_index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.lvfindinfo as *const _ as _,
		}
	}
}

/// [`LVM_GETBKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getbkcolor)
/// message, which has no parameters.
///
/// Return type: `COLORREF`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetBkColor {}

unsafe impl MsgSend for GetBkColor {
	type RetType = COLORREF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		COLORREF(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETBKCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETBKIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getbkimage)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetBkImage<'a, 'b> {
	pub lvbkimage: &'b mut LVBKIMAGE<'a>,
}

unsafe impl<'a, 'b> MsgSend for GetBkImage<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETBKIMAGE.into(),
			wparam: 0,
			lparam: self.lvbkimage as *mut _ as _,
		}
	}
}

/// [`LVM_GETCALLBACKMASK`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getcallbackmask)
/// message, which has no parameters.
///
/// Return type: `co::LVIS`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetCallbackMask {}

unsafe impl MsgSend for GetCallbackMask {
	type RetType = co::LVIS;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LVIS(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETCALLBACKMASK.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETCOLUMN`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getcolumn)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetColumn<'a, 'b> {
	pub index: u32,
	pub lvcolumn: &'b mut LVCOLUMN<'a>,
}

unsafe impl<'a, 'b> MsgSend for GetColumn<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETCOLUMN.into(),
			wparam: self.index as _,
			lparam: self.lvcolumn as *mut _ as _,
		}
	}
}

/// [`LVM_GETCOLUMNORDERARRAY`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getcolumnorderarray)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetColumnOrderArray<'a> {
	pub indexes: &'a mut Vec<u32>,
}

unsafe impl<'a> MsgSend for GetColumnOrderArray<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETCOLUMNORDERARRAY.into(),
			wparam: self.indexes.len() as _,
			lparam: self.indexes.as_mut_ptr() as _,
		}
	}
}

/// [`LVM_GETCOLUMNWIDTH`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getcolumnwidth)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetColumnWidth {
	pub index: u32,
}

unsafe impl MsgSend for GetColumnWidth {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_err(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETCOLUMNWIDTH.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LVM_GETCOUNTPERPAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getcountperpage)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetCountPerPage {}

unsafe impl MsgSend for GetCountPerPage {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETCOUNTPERPAGE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_EDITCONTROL`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-geteditcontrol)
/// message, which has no parameters.
///
/// Return type: `Option<HWND>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetEditControl {}

unsafe impl MsgSend for GetEditControl {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|h| HWND(h as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETEDITCONTROL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETEMPTYTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getemptytext)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetEmptyText<'a> {
	pub text: &'a mut WString,
}

unsafe impl<'a> MsgSend for GetEmptyText<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETEMPTYTEXT.into(),
			wparam: self.text.buf_len(),
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`LVM_GETEXTENDEDLISTVIEWSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getextendedlistviewstyle)
/// message, which has no parameters.
///
/// Return type: `co::LVS_EX`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetExtendedListViewStyle {}

unsafe impl MsgSend for GetExtendedListViewStyle {
	type RetType = co::LVS_EX;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LVS_EX(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETEXTENDEDLISTVIEWSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETFOCUSEDGROUP`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getfocusedgroup)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetFocusedGroup {}

unsafe impl MsgSend for GetFocusedGroup {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETFOCUSEDGROUP.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETFOOTERINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getfooterinfo)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetFooterInfo<'a, 'b> {
	pub info: &'b mut LVFOOTERINFO<'a>,
}

unsafe impl<'a, 'b> MsgSend for GetFooterInfo<'a, 'b> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETFOOTERINFO.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`LVM_GETFOOTERITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getfooteritem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetFooterItem<'a, 'b> {
	pub index: u32,
	pub info: &'b mut LVFOOTERITEM<'a>,
}

unsafe impl<'a, 'b> MsgSend for GetFooterItem<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETFOOTERITEM.into(),
			wparam: self.index as _,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`LVM_GETFOOTERITEMRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getfooteritemrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetFooterItemRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetFooterItemRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETFOOTERITEMRECT.into(),
			wparam: self.index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`LVM_GETFOOTERRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getfooterrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetFooterRect<'a> {
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetFooterRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETFOOTERRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`LVM_GROUPCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getgroupcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetGroupCount {}

unsafe impl MsgSend for GetGroupCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETGROUPCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GROUPINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getgroupinfo)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetGroupInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	pub id: u32,
	pub info: &'h mut LVGROUP<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
}

unsafe impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> MsgSend for GetGroupInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_err(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETGROUPINFO.into(),
			wparam: self.id as _,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`LVM_GETGROUPINFOBYINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getgroupinfobyindex)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetGroupInfoByIndex<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	pub index: u32,
	pub info: &'h mut LVGROUP<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
}

unsafe impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> MsgSend for GetGroupInfoByIndex<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETGROUPINFOBYINDEX.into(),
			wparam: self.index as _,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`LVM_GETGROUPMETRICS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getgroupmetrics)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetGroupMetrics<'a> {
	pub info: &'a mut LVGROUPMETRICS,
}

unsafe impl<'a> MsgSend for GetGroupMetrics<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETGROUPMETRICS.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`LVM_GETGROUPRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getgrouprect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetGroupRect<'a> {
	pub id: u32,
	pub flags: co::LVGGR,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetGroupRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		self.rect.top = self.flags.0;

		WndMsg {
			msg_id: co::LVM::GETGROUPRECT.into(),
			wparam: self.id as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`LVM_GETGROUPSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getgroupstate)
/// message parameters.
///
/// Return type: `co::LVGS`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetGroupState {
	pub id: u32,
	pub mask: co::LVGS,
}

unsafe impl MsgSend for GetGroupState {
	type RetType = co::LVGS;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LVGS(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETGROUPSTATE.into(),
			wparam: self.id as _,
			lparam: self.mask.0 as _,
		}
	}
}

/// [`LVM_GETHEADER`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getheader)
/// message, which has no parameters.
///
/// Return type: `SysResult<HWND>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetHeader {}

unsafe impl MsgSend for GetHeader {
	type RetType = SysResult<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HWND(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETHEADER.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETHOTCURSOR`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-gethotcursor)
/// message, which has no parameters.
///
/// Return type: `SysResult<HCURSOR>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetHotCursor {}

unsafe impl MsgSend for GetHotCursor {
	type RetType = SysResult<HCURSOR>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HCURSOR(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETHOTCURSOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETHOTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-gethotitem)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetHotItem {}

unsafe impl MsgSend for GetHotItem {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|idx| idx as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETHOTITEM.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETHOVERTIME`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-gethovertime)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetHoverTime {}

unsafe impl MsgSend for GetHoverTime {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|idx| idx as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETHOVERTIME.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetImageList {
	pub kind: co::LVSIL,
}

unsafe impl MsgSend for GetImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HIMAGELIST(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETIMAGELIST.into(),
			wparam: self.kind.0 as _,
			lparam: 0,
		}
	}
}

/// [`LVM_GETINSERTMARK`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getinsertmark)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetInsertMark<'a> {
	pub info: &'a mut LVINSERTMARK,
}

unsafe impl<'a> MsgSend for GetInsertMark<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETINSERTMARK.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`LVM_GETINSERTMARKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getinsertmarkcolor)
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
			msg_id: co::LVM::GETINSERTMARKCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETINSERTMARKRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getinsertmarkrect)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetInsertMarkRect<'a> {
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetInsertMarkRect<'a> {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETINSERTMARKRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`LVM_GETISEARCHSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getisearchstring)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetISearchString<'a> {
	pub buffer: Option<&'a mut WString>,
}

unsafe impl<'a> MsgSend for GetISearchString<'a> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|c| c as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETISEARCHSTRING.into(),
			wparam: 0,
			lparam: self.buffer.as_mut().map_or(0, |buf| unsafe { buf.as_mut_ptr() } as _),
		}
	}
}

/// [`LVM_GETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetItem<'a, 'b> {
	pub lvitem: &'b mut LVITEM<'a>,
}

unsafe impl<'a, 'b> MsgSend for GetItem<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETITEM.into(),
			wparam: 0,
			lparam: self.lvitem as *mut _ as _,
		}
	}
}

/// [`LVM_GETITEMCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getitemcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetItemCount {}

unsafe impl MsgSend for GetItemCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETITEMCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETITEMINDEXRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getitemindexrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetItemIndexRect<'a, 'b> {
	pub lvitemindex: &'a LVITEMINDEX,
	pub rect: &'b mut RECT,
	pub index: u32,
	pub portion: co::LVIR,
}

unsafe impl<'a, 'b> MsgSend for GetItemIndexRect<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		self.rect.top = self.index as _;
		self.rect.left = self.portion.0 as _;

		WndMsg {
			msg_id: co::LVM::GETITEMINDEXRECT.into(),
			wparam: self.lvitemindex as *const _ as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`LVM_GETITEMPOSITION`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getitemposition)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetItemPosition<'a> {
	pub index: u32,
	pub pos: &'a mut POINT,
}

unsafe impl<'a> MsgSend for GetItemPosition<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETITEMPOSITION.into(),
			wparam: self.index as _,
			lparam: self.pos as *mut _ as _,
		}
	}
}

/// [`LVM_GETITEMRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getitemrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetItemRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
	pub portion: co::LVIR,
}

unsafe impl<'a> MsgSend for GetItemRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		self.rect.left = self.portion.0 as _;

		WndMsg {
			msg_id: co::LVM::GETITEMRECT.into(),
			wparam: self.index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`LVM_GETITEMSPACING`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getitemspacing)
/// message parameters.
///
/// Return type: `SIZE`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetItemSpacing {
	pub is_small_icon_view: bool,
}

unsafe impl MsgSend for GetItemSpacing {
	type RetType = SIZE;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		SIZE::from(v as u32)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETITEMSTATE.into(),
			wparam: self.is_small_icon_view as _,
			lparam: 0,
		}
	}
}

/// [`LVM_GETITEMSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getitemstate)
/// message parameters.
///
/// Return type: `co::LVIS`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetItemState {
	pub index: u32,
	pub mask: co::LVIS,
}

unsafe impl MsgSend for GetItemState {
	type RetType = co::LVIS;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LVIS(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETITEMSTATE.into(),
			wparam: self.index as _,
			lparam: self.mask.0 as _,
		}
	}
}

/// [`LVM_GETITEMTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getitemtext)
/// message parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetItemText<'a, 'b> {
	pub index: u32,
	pub lvitem: &'b mut LVITEM<'a>,
}

unsafe impl<'a, 'b> MsgSend for GetItemText<'a, 'b> {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETITEMTEXT.into(),
			wparam: self.index as _,
			lparam: self.lvitem as *mut _ as _,
		}
	}
}

/// [`LVM_GETNEXTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getnextitem)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetNextItem {
	pub initial_index: Option<u32>,
	pub relationship: co::LVNI,
}

unsafe impl MsgSend for GetNextItem {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETNEXTITEM.into(),
			wparam: self.initial_index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.relationship.0 as _,
		}
	}
}

/// [`LVM_GETNEXTITEMINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getnextitemindex)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetNextItemIndex<'a> {
	pub initial_item: &'a mut LVITEMINDEX,
	pub relationship: co::LVNI,
}

unsafe impl<'a> MsgSend for GetNextItemIndex<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETNEXTITEMINDEX.into(),
			wparam: self.initial_item as *mut _ as _,
			lparam: self.relationship.0 as _,
		}
	}
}

/// [`LVM_GETNUMBEROFWORKAREAS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getnumberofworkareas)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetNumberOfWorkAreas<'a> {
	pub num: &'a mut u32,
}

unsafe impl<'a> MsgSend for GetNumberOfWorkAreas<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETNUMBEROFWORKAREAS.into(),
			wparam: 0,
			lparam: self.num as *mut _ as _,
		}
	}
}

/// [`LVM_GETORIGIN`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getorigin)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetOrigin<'a> {
	pub origin: &'a mut POINT,
}

unsafe impl<'a> MsgSend for GetOrigin<'a> {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETORIGIN.into(),
			wparam: 0,
			lparam: self.origin as *mut _ as _,
		}
	}
}

/// [`LVM_GETOUTLINECOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getoutlinecolor)
/// message, which has no parameters.
///
/// Return type: `COLORREF`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetOutlineColor {}

unsafe impl MsgSend for GetOutlineColor {
	type RetType = COLORREF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		COLORREF(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETOUTLINECOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETSELECTEDCOLUMN`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getselectedcolumn)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetSelectedColumn {}

unsafe impl MsgSend for GetSelectedColumn {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETSELECTEDCOLUMN.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETSELECTEDCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getselectedcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetSelectedCount {}

unsafe impl MsgSend for GetSelectedCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETSELECTEDCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETSELECTIONMARK`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getselectionmark)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetSelectionMark {}

unsafe impl MsgSend for GetSelectionMark {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETSELECTIONMARK.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETSTRINGWIDTH`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getstringwidth)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetStringWidth {
	pub text: WString,
}

unsafe impl MsgSend for GetStringWidth {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|len| len as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETSTRINGWIDTH.into(),
			wparam: 0,
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`LVM_GETSUBITEMRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getsubitemrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetSubItemRect<'a> {
	pub item_index: u32,
	pub subitem_index: u32,
	pub rect: &'a mut RECT,
	pub portion: co::LVIR,
}

unsafe impl<'a> MsgSend for GetSubItemRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		self.rect.left = self.portion.0 as _;
		self.rect.top = self.subitem_index as _;

		WndMsg {
			msg_id: co::LVM::GETSUBITEMRECT.into(),
			wparam: self.item_index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`LVM_GETTEXTBKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-gettextbkcolor)
/// message, which has no parameters.
///
/// Return type: `COLORREF`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetTextBkColor {}

unsafe impl MsgSend for GetTextBkColor {
	type RetType = COLORREF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		COLORREF(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETTEXTBKCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETTEXTCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-gettextcolor)
/// message, which has no parameters.
///
/// Return type: `COLORREF`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetTextColor {}

unsafe impl MsgSend for GetTextColor {
	type RetType = COLORREF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		COLORREF(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETTEXTCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETTILEINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-gettileinfo)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetTileInfo<'a, 'b> {
	pub info: &'b mut LVTILEINFO<'a>,
}

unsafe impl<'a, 'b> MsgSend for GetTileInfo<'a, 'b> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETTILEINFO.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`LVM_GETTILEVIEWINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-gettileviewinfo)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetTileViewInfo<'a> {
	pub info: &'a mut LVTILEVIEWINFO,
}

unsafe impl<'a> MsgSend for GetTileViewInfo<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETTILEVIEWINFO.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`LVM_GETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-gettooltips)
/// message, which has no parameters.
///
/// Return type: `Option<HWND>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetTooltips {}

unsafe impl MsgSend for GetTooltips {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|h| HWND(h as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETTOOLTIPS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETTOPINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-gettopindex)
/// message, which has no parameters.
///
/// In case of error or when there are no items this message returns zero, so
/// other checks must be made.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetTopIndex {}

unsafe impl MsgSend for GetTopIndex {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETTOPINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getunicodeformat)
/// message, which has no parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetUnicodeFormat {}

unsafe impl MsgSend for GetUnicodeFormat {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETVIEW`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getview)
/// message, which has no parameters.
///
/// Return type: `co::LV_VIEW`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetView {}

unsafe impl MsgSend for GetView {
	type RetType = co::LV_VIEW;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LV_VIEW(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETVIEW.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETVIEWRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getviewrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetViewRect<'a> {
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetViewRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETVIEWRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`LVM_GETWORKAREAS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getworkareas)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetWorkAreas<'a> {
	pub rects: &'a mut [RECT],
}

unsafe impl<'a> MsgSend for GetWorkAreas<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETWORKAREAS.into(),
			wparam: self.rects.len() as _,
			lparam: self.rects.as_mut_ptr() as _,
		}
	}
}

/// [`LVM_HASGROUP`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-hasgroup)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct HasGroup {
	pub id: u32,
}

unsafe impl MsgSend for HasGroup {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::HASGROUP.into(),
			wparam: self.id as _,
			lparam: 0,
		}
	}
}

/// [`LVM_HITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-hittest)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct HitTest<'a> {
	pub info: &'a mut LVHITTESTINFO,
}

unsafe impl<'a> MsgSend for HitTest<'a> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::HITTEST.into(),
			wparam: -1 as _,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`LVM_INSERTCOLUMN`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-insertcolumn)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct InsertColumn<'a, 'b> {
	pub index: u32,
	pub lvcolumn: &'b LVCOLUMN<'a>,
}

unsafe impl<'a, 'b> MsgSend for InsertColumn<'a, 'b> {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_err(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::INSERTCOLUMN.into(),
			wparam: self.index as _,
			lparam: self.lvcolumn as *const _ as _,
		}
	}
}

/// [`LVM_INSERTGROUP`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-insertgroup)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct InsertGroup<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	pub lvgroup: &'h LVGROUP<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
}

unsafe impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> MsgSend for InsertGroup<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_err(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::INSERTGROUP.into(),
			wparam: 0,
			lparam: self.lvgroup as *const _ as _,
		}
	}
}

/// [`LVM_INSERTGROUPSORTED`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-insertgroupsorted)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct InsertGroupSorted<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	pub lvgroup: &'h LVINSERTGROUPSORTED<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
}

unsafe impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> MsgSend for InsertGroupSorted<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::INSERTGROUPSORTED.into(),
			wparam: 0,
			lparam: self.lvgroup as *const _ as _,
		}
	}
}

/// [`LVM_INSERTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-insertitem)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct InsertItem<'a, 'b> {
	pub lvitem: &'b LVITEM<'a>,
}

unsafe impl<'a, 'b> MsgSend for InsertItem<'a, 'b> {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_err(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::INSERTITEM.into(),
			wparam: 0,
			lparam: self.lvitem as *const _ as _,
		}
	}
}

/// [`LVM_INSERTMARKHITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-insertmarkhittest)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct InsertMarkHitTest<'a> {
	pub point: POINT,
	pub lvinsertmark: &'a LVINSERTMARK,
}

unsafe impl<'a, 'b> MsgSend for InsertMarkHitTest<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::INSERTMARKHITTEST.into(),
			wparam: &self.point as *const _ as _,
			lparam: self.lvinsertmark as *const _ as _,
		}
	}
}

/// [`LVM_ISGROUPVIEWENABLED`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-isgroupviewenabled)
/// message, which has no parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct IsGroupViewEnabled {}

unsafe impl MsgSend for IsGroupViewEnabled {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::ISGROUPVIEWENABLED.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_ISITEMVISIBLE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-isitemvisible)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct IsItemVisible {
	pub index: u32,
}

unsafe impl MsgSend for IsItemVisible {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::ISITEMVISIBLE.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LVM_MAPIDTOINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-mapidtoindex)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct MapIdToIndex {
	pub id: u32,
}

unsafe impl MsgSend for MapIdToIndex {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::MAPIDTOINDEX.into(),
			wparam: self.id as _,
			lparam: 0,
		}
	}
}

/// [`LVM_MAPINDEXTOID`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-mapindextoid)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct MapIndexToId {
	pub index: u32,
}

unsafe impl MsgSend for MapIndexToId {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::MAPINDEXTOID.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LVM_REDRAWITEMS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-redrawitems)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct RedrawItems {
	pub first_index: u32,
	pub last_index: u32,
}

unsafe impl MsgSend for RedrawItems {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::REDRAWITEMS.into(),
			wparam: self.first_index as _,
			lparam: self.last_index as _,
		}
	}
}

pub_struct_msg_empty! { RemoveAllGroups: co::LVM::REMOVEALLGROUPS.into(); "comctl";
	/// [`LVM_REMOVEALLGROUPS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-removeallgroups)
	/// message, which has no parameters.
	///
	/// Return type: `SysResult<()>`.
}

/// [`LVM_REMOVEGROUP`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-removegroup)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct RemoveGroup {
	pub id: u32,
}

unsafe impl MsgSend for RemoveGroup {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|id| id as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::REMOVEGROUP.into(),
			wparam: self.id as _,
			lparam: 0,
		}
	}
}

/// [`LVM_SCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-scroll)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct Scroll {
	pub horizontal: i32,
	pub vertical: i32,
}

unsafe impl MsgSend for Scroll {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SCROLL.into(),
			wparam: self.horizontal as _,
			lparam: self.vertical as _,
		}
	}
}

/// [`LVM_SETBKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setbkcolor)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetBkColor {
	pub color: Option<COLORREF>,
}

unsafe impl MsgSend for SetBkColor {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETBKCOLOR.into(),
			wparam: 0,
			lparam: self.color.map_or(co::CLR::NONE.0, |c| c.0) as _,
		}
	}
}

/// [`LVM_SETBKIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setbkimage)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetBkImage<'a, 'b> {
	pub lvbkimage: &'b LVBKIMAGE<'a>,
}

unsafe impl<'a, 'b> MsgSend for SetBkImage<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETBKIMAGE.into(),
			wparam: 0,
			lparam: self.lvbkimage as *const _ as _,
		}
	}
}

/// [`LVM_SETCALLBACKMASK`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setcallbackmask)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetCallbackMask {
	pub mask: co::LVIS,
}

unsafe impl MsgSend for SetCallbackMask {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETCALLBACKMASK.into(),
			wparam: self.mask.0 as _,
			lparam: 0,
		}
	}
}

/// [`LVM_SETCOLUMN`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setcolumn)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetColumn<'a, 'b> {
	pub index: u32,
	pub lvcolumn: &'b LVCOLUMN<'a>,
}

unsafe impl<'a, 'b> MsgSend for SetColumn<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETCOLUMN.into(),
			wparam: self.index as _,
			lparam: self.lvcolumn as *const _ as _,
		}
	}
}

/// [`LVM_SETCOLUMNORDERARRAY`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setcolumnorderarray)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetColumnOrderArray<'a> {
	pub order: &'a [u32],
}

unsafe impl<'a> MsgSend for SetColumnOrderArray<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETCOLUMNORDERARRAY.into(),
			wparam: self.order.len() as _,
			lparam: self.order.as_ptr() as _,
		}
	}
}

/// [`LVM_SETCOLUMNWIDTH`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setcolumnwidth)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetColumnWidth {
	pub index: u32,
	pub width: u32,
}

unsafe impl MsgSend for SetColumnWidth {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETCOLUMNWIDTH.into(),
			wparam: self.index as _,
			lparam: self.width as _,
		}
	}
}

/// [`LVM_SETEXTENDEDLISTVIEWSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setextendedlistviewstyle)
/// message parameters.
///
/// Return type: `co::LVS_EX`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetExtendedListViewStyle {
	pub style: co::LVS_EX,
	pub mask: co::LVS_EX,
}

unsafe impl MsgSend for SetExtendedListViewStyle {
	type RetType = co::LVS_EX;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LVS_EX(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETEXTENDEDLISTVIEWSTYLE.into(),
			wparam: self.style.0 as _,
			lparam: self.mask.0 as _,
		}
	}
}

/// [`LVM_SETGROUPINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setgroupinfo)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetGroupInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	pub id: u32,
	pub info: &'h LVGROUP<'a, 'b, 'c, 'd, 'e, 'f, 'g>
}

unsafe impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> MsgSend for SetGroupInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_err(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETGROUPINFO.into(),
			wparam: self.id as _,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`LVM_SETGROUPMETRICS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setgroupmetrics)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetGroupMetrics<'a> {
	pub info: &'a LVGROUPMETRICS,
}

unsafe impl<'a> MsgSend for SetGroupMetrics<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETGROUPMETRICS.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`LVM_SETHOTCURSOR`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-sethotcursor)
/// message parameters.
///
/// Return type: `Option<HCURSOR>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetHotCursor {
	pub hcursor: Option<HCURSOR>,
}

unsafe impl MsgSend for SetHotCursor {
	type RetType = Option<HCURSOR>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HCURSOR(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETHOTCURSOR.into(),
			wparam: 0,
			lparam: self.hcursor.as_ref().map_or(0, |h| h.0 as _),
		}
	}
}

/// [`LVM_SETHOTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-sethotitem)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetHotItem {
	pub index: Option<u32>,
}

unsafe impl MsgSend for SetHotItem {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETHOTITEM.into(),
			wparam: self.index.unwrap_or(0) as _,
			lparam: 0,
		}
	}
}

/// [`LVM_SETHOVERTIME`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-sethovertime)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetHoverTime {
	pub ms: Option<u32>,
}

unsafe impl MsgSend for SetHoverTime {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETHOVERTIME.into(),
			wparam: self.ms.map_or(-1, |ms| ms as i32) as _,
			lparam: 0,
		}
	}
}

/// [`LVM_SETICONSPACING`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-seticonspacing)
/// message parameters.
///
/// Return type: `SIZE`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetIconSpacing {
	pub size: SIZE,
}

unsafe impl MsgSend for SetIconSpacing {
	type RetType = SIZE;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		SIZE::from(v as u32)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETICONSPACING.into(),
			wparam: 0,
			lparam: u32::from(self.size) as _,
		}
	}
}

/// [`LVM_SETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetImageList {
	pub kind: co::LVSIL,
	pub himagelist: HIMAGELIST,
}

unsafe impl MsgSend for SetImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HIMAGELIST(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETIMAGELIST.into(),
			wparam: self.kind.0 as _,
			lparam: self.himagelist.0 as _,
		}
	}
}

/// [`LVM_SETINFOTIP`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setinfotip)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetInfoTip<'a, 'b> {
	pub info: &'b LVSETINFOTIP<'a>,
}

unsafe impl<'a, 'b> MsgSend for SetInfoTip<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETINFOTIP.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`LVM_SETINSERTMARK`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setinsertmark)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetInsertMark<'a> {
	pub info: &'a LVINSERTMARK,
}

unsafe impl<'a> MsgSend for SetInsertMark<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETINSERTMARK.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`LVM_SETINSERTMARKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setinsertmarkcolor)
/// message parameters.
///
/// Return type: `COLORREF`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetInsertMarkColor {
	pub color: COLORREF,
}

unsafe impl MsgSend for SetInsertMarkColor {
	type RetType = COLORREF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		COLORREF(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETINSERTMARKCOLOR.into(),
			wparam: 0,
			lparam: self.color.0 as _,
		}
	}
}

/// [`LVM_SETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetItem<'a, 'b> {
	pub lvitem: &'b LVITEM<'a>,
}

unsafe impl<'a, 'b> MsgSend for SetItem<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETITEM.into(),
			wparam: 0,
			lparam: self.lvitem as *const _ as _,
		}
	}
}

/// [`LVM_SETITEMCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setitemcount)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetItemCount {
	pub count: u32,
	pub behavior: co::LVSICF,
}

unsafe impl MsgSend for SetItemCount {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETITEMCOUNT.into(),
			wparam: self.count as _,
			lparam: self.behavior.0 as _,
		}
	}
}

/// [`LVM_SETITEMPOSITION`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setitemposition)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetItemPosition {
	pub index: u32,
	pub position: POINT,
}

unsafe impl MsgSend for SetItemPosition {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETITEMPOSITION.into(),
			wparam: self.index as _,
			lparam: u32::from(self.position) as _,
		}
	}
}

/// [`LVM_SETITEMPOSITION32`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setitemposition32)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetItemPosition32 {
	pub index: u32,
	pub position: POINT,
}

unsafe impl MsgSend for SetItemPosition32 {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETITEMPOSITION32.into(),
			wparam: self.index as _,
			lparam: &self.position as *const _ as _,
		}
	}
}

/// [`LVM_SETITEMSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setitemstate)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetItemState<'a, 'b> {
	pub index: Option<u32>,
	pub lvitem: &'b LVITEM<'a>,
}

unsafe impl<'a, 'b> MsgSend for SetItemState<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETITEMSTATE.into(),
			wparam: self.index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.lvitem as *const _ as _,
		}
	}
}

/// [`LVM_SETITEMTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setitemtext)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetItemText<'a, 'b> {
	pub index: u32,
	pub lvitem: &'b LVITEM<'a>,
}

unsafe impl<'a, 'b> MsgSend for SetItemText<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETITEMTEXT.into(),
			wparam: self.index as _,
			lparam: self.lvitem as *const _ as _,
		}
	}
}

/// [`LVM_SETOUTLINECOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setoutlinecolor)
/// message parameters.
///
/// Return type: `COLORREF`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetOutlineColor {
	pub color: COLORREF,
}

unsafe impl MsgSend for SetOutlineColor {
	type RetType = COLORREF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		COLORREF(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETOUTLINECOLOR.into(),
			wparam: 0,
			lparam: self.color.0 as _,
		}
	}
}

/// [`LVM_SETSELECTEDCOLUMN`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setselectedcolumn)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetSelectedColumn {
	pub index: u32,
}

unsafe impl MsgSend for SetSelectedColumn {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETSELECTEDCOLUMN.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LVM_SETSELECTIONMARK`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setselectionmark)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetSelectionMark {
	pub index: Option<u32>,
}

unsafe impl MsgSend for SetSelectionMark {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETSELECTIONMARK.into(),
			wparam: 0,
			lparam: self.index.map_or(-1, |idx| idx as i32) as _,
		}
	}
}

/// [`LVM_SETTEXTBKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-settextbkcolor)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetTextBkColor {
	pub color: Option<COLORREF>,
}

unsafe impl MsgSend for SetTextBkColor {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETTEXTBKCOLOR.into(),
			wparam: 0,
			lparam: self.color.map_or(co::CLR::NONE.0 as _, |c| c.0 as _),
		}
	}
}

/// [`LVM_SETTEXTCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-settextcolor)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetTextColor {
	pub color: Option<COLORREF>,
}

unsafe impl MsgSend for SetTextColor {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETTEXTCOLOR.into(),
			wparam: 0,
			lparam: self.color.map_or(co::CLR::NONE.0 as _, |c| c.0 as _),
		}
	}
}

/// [`LVM_SETTILEINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-settileinfo)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetTileInfo<'a, 'b> {
	pub info: &'b LVTILEINFO<'a>,
}

unsafe impl<'a, 'b> MsgSend for SetTileInfo<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETTILEINFO.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`LVM_SETTILEVIEWINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-settileviewinfo)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetTileViewInfo<'a> {
	pub info: &'a LVTILEVIEWINFO,
}

unsafe impl<'a> MsgSend for SetTileViewInfo<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETTILEVIEWINFO.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`LVM_SETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-settooltips)
/// message parameters.
///
/// Return type: `Option<HWND>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetTooltips {
	pub tooltip: Option<HWND>,
}

unsafe impl MsgSend for SetTooltips {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|h| HWND(h as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETTOOLTIPS.into(),
			wparam: self.tooltip.as_ref().map_or(0, |h| h.0 as _),
			lparam: 0,
		}
	}
}

/// [`LVM_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setunicodeformat)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetUnicodeFormat {
	pub use_unicode: bool,
}

unsafe impl MsgSend for SetUnicodeFormat {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETUNICODEFORMAT.into(),
			wparam: self.use_unicode as _,
			lparam: 0,
		}
	}
}

/// [`LVM_SETVIEW`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setview)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetView {
	pub view: co::LV_VIEW,
}

unsafe impl MsgSend for SetView {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETVIEW.into(),
			wparam: self.view.0 as _,
			lparam: 0,
		}
	}
}

/// [`LVM_SETWORKAREAS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setworkareas)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetWorkAreas<'a> {
	pub rects: Option<&'a [RECT]>,
}

unsafe impl<'a> MsgSend for SetWorkAreas<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETWORKAREAS.into(),
			wparam: self.rects.map_or(0, |r| r.len() as _),
			lparam: self.rects.map_or(0, |r| r.as_ptr() as _),
		}
	}
}

/// [`LVM_SORTGROUPS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-sortgroups)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SortGroups {
	pub callback: Option<PFNLVGROUPCOMPARE>,
	pub param: Option<isize>,
}

unsafe impl MsgSend for SortGroups {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SORTGROUPS.into(),
			wparam: self.callback.map_or(0, |cb| cb as _),
			lparam: self.param.unwrap_or(0),
		}
	}
}

/// [`LVM_SORTITEMS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-sortitems)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SortItems {
	pub param: isize,
	pub callback: PFNLVCOMPARE,
}

unsafe impl MsgSend for SortItems {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SORTITEMS.into(),
			wparam: self.param as _,
			lparam: self.callback as _,
		}
	}
}

/// [`LVM_SORTITEMSEX`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-sortitemsex)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SortItemsEx {
	pub param: isize,
	pub callback: PFNLVCOMPARE,
}

unsafe impl MsgSend for SortItemsEx {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SORTITEMSEX.into(),
			wparam: self.param as _,
			lparam: self.callback as _,
		}
	}
}

/// [`LVM_SUBITEMHITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-subitemhittest)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SubItemHitTest<'a> {
	pub info: &'a mut LVHITTESTINFO,
}

unsafe impl<'a> MsgSend for SubItemHitTest<'a> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SUBITEMHITTEST.into(),
			wparam: -1 as _,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`LVM_UPDATE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-update)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct Update {
	pub index: u32,
}

unsafe impl MsgSend for Update {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::UPDATE.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}
