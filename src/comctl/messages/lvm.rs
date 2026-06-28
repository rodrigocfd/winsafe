use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::macros::*;
use crate::msg::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`LVM_APPROXIMATEVIEWRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-approximateviewrect)
/// message parameters.
///
/// Return type: `SIZE`.
pub struct LvmApproximateViewRect {
	pub num_items: Option<u32>,
	pub proposed_x: Option<u16>,
	pub proposed_y: Option<u16>,
}

impl MsgSend for LvmApproximateViewRect {
	type RetType = SIZE;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		SIZE::from(v as u32)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmArrange {
	pub arrangement: co::LVA,
}

impl MsgSend for LvmArrange {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::ARRANGE.into(),
			wparam: self.arrangement.raw() as _,
			lparam: 0,
		}
	}
}

pub_struct_msg_empty! { LvmCancelEditLabel: co::LVM::CANCELEDITLABEL.into();
	/// [`LVM_CANCELEDITLABEL`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-canceleditlabel)
}

/// [`LVM_CREATEDRAGIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-createdragimage)
/// message parameters.
///
/// Return type: `SysResult<HIMAGELIST>`.
pub struct LvmCreateDragImage<'a> {
	pub index: u32,
	pub img_location: &'a mut RECT,
}

impl<'a> MsgSend for LvmCreateDragImage<'a> {
	type RetType = SysResult<HIMAGELIST>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmDeleteAllItems {}

impl MsgSend for LvmDeleteAllItems {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmDeleteColumn {
	pub index: u32,
}

impl MsgSend for LvmDeleteColumn {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmDeleteItem {
	pub index: u32,
}

impl MsgSend for LvmDeleteItem {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmEditLabel {
	pub index: Option<u32>,
}

impl MsgSend for LvmEditLabel {
	type RetType = SysResult<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmEnableGroupView {
	pub enable: bool,
}

impl MsgSend for LvmEnableGroupView {
	type RetType = SysResult<bool>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v != 0)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmEnsureVisible {
	pub index: u32,
	pub entirely_visible: bool,
}

impl MsgSend for LvmEnsureVisible {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmFindItem<'a, 'b> {
	pub start_index: Option<u32>,
	pub lvfindinfo: &'b LVFINDINFO<'a>,
}

impl<'a, 'b> MsgSend for LvmFindItem<'a, 'b> {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetBkColor {}

impl MsgSend for LvmGetBkColor {
	type RetType = COLORREF;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetBkImage<'a, 'b> {
	pub lvbkimage: &'b mut LVBKIMAGE<'a>,
}

impl<'a, 'b> MsgSend for LvmGetBkImage<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetCallbackMask {}

impl MsgSend for LvmGetCallbackMask {
	type RetType = co::LVIS;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::LVIS::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetColumn<'a, 'b> {
	pub index: u32,
	pub lvcolumn: &'b mut LVCOLUMN<'a>,
}

impl<'a, 'b> MsgSend for LvmGetColumn<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetColumnOrderArray<'a> {
	pub indexes: &'a mut Vec<u32>,
}

impl<'a> MsgSend for LvmGetColumnOrderArray<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetColumnWidth {
	pub index: u32,
}

impl MsgSend for LvmGetColumnWidth {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetCountPerPage {}

impl MsgSend for LvmGetCountPerPage {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetEditControl {}

impl MsgSend for LvmGetEditControl {
	type RetType = Option<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetEmptyText<'a> {
	pub text: &'a mut WString,
}

impl<'a> MsgSend for LvmGetEmptyText<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetExtendedListViewStyle {}

impl MsgSend for LvmGetExtendedListViewStyle {
	type RetType = co::LVS_EX;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::LVS_EX::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetFocusedGroup {}

impl MsgSend for LvmGetFocusedGroup {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetFooterInfo<'a, 'b> {
	pub info: &'b mut LVFOOTERINFO<'a>,
}

impl<'a, 'b> MsgSend for LvmGetFooterInfo<'a, 'b> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetFooterItem<'a, 'b> {
	pub index: u32,
	pub info: &'b mut LVFOOTERITEM<'a>,
}

impl<'a, 'b> MsgSend for LvmGetFooterItem<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetFooterItemRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for LvmGetFooterItemRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetFooterRect<'a> {
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for LvmGetFooterRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetGroupCount {}

impl MsgSend for LvmGetGroupCount {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetGroupInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	pub id: u32,
	pub info: &'h mut LVGROUP<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> MsgSend for LvmGetGroupInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetGroupInfoByIndex<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	pub index: u32,
	pub info: &'h mut LVGROUP<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> MsgSend
	for LvmGetGroupInfoByIndex<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h>
{
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetGroupMetrics<'a> {
	pub info: &'a mut LVGROUPMETRICS,
}

impl<'a> MsgSend for LvmGetGroupMetrics<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetGroupRect<'a> {
	pub id: u32,
	pub flags: co::LVGGR,
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for LvmGetGroupRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		self.rect.top = self.flags.raw();

		Wm {
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
pub struct LvmGetGroupState {
	pub id: u32,
	pub mask: co::LVGS,
}

impl MsgSend for LvmGetGroupState {
	type RetType = co::LVGS;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::LVGS::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::GETGROUPSTATE.into(),
			wparam: self.id as _,
			lparam: self.mask.raw() as _,
		}
	}
}

/// [`LVM_GETHEADER`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getheader)
/// message, which has no parameters.
///
/// Return type: `SysResult<HWND>`.
pub struct LvmGetHeader {}

impl MsgSend for LvmGetHeader {
	type RetType = SysResult<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetHotCursor {}

impl MsgSend for LvmGetHotCursor {
	type RetType = SysResult<HCURSOR>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HCURSOR::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetHotItem {}

impl MsgSend for LvmGetHotItem {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|idx| idx as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetHoverTime {}

impl MsgSend for LvmGetHoverTime {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|idx| idx as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetImageList {
	pub kind: co::LVSIL,
}

impl MsgSend for LvmGetImageList {
	type RetType = Option<HIMAGELIST>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::GETIMAGELIST.into(),
			wparam: self.kind.raw() as _,
			lparam: 0,
		}
	}
}

/// [`LVM_GETINSERTMARK`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getinsertmark)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct LvmGetInsertMark<'a> {
	pub info: &'a mut LVINSERTMARK,
}

impl<'a> MsgSend for LvmGetInsertMark<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetInsertMarkColor {}

impl MsgSend for LvmGetInsertMarkColor {
	type RetType = COLORREF;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetInsertMarkRect<'a> {
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for LvmGetInsertMarkRect<'a> {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetISearchString<'a> {
	pub buffer: Option<&'a mut WString>,
}

impl<'a> MsgSend for LvmGetISearchString<'a> {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|c| c as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::GETISEARCHSTRING.into(),
			wparam: 0,
			lparam: self
				.buffer
				.as_mut()
				.map_or(0, |buf| unsafe { buf.as_mut_ptr() } as _),
		}
	}
}

/// [`LVM_GETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct LvmGetItem<'a, 'b> {
	pub lvitem: &'b mut LVITEM<'a>,
}

impl<'a, 'b> MsgSend for LvmGetItem<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetItemCount {}

impl MsgSend for LvmGetItemCount {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetItemIndexRect<'a, 'b> {
	pub lvitemindex: &'a LVITEMINDEX,
	pub rect: &'b mut RECT,
	pub index: u32,
	pub portion: co::LVIR,
}

impl<'a, 'b> MsgSend for LvmGetItemIndexRect<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		self.rect.top = self.index as _;
		self.rect.left = self.portion.raw() as _;

		Wm {
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
pub struct LvmGetItemPosition<'a> {
	pub index: u32,
	pub pos: &'a mut POINT,
}

impl<'a> MsgSend for LvmGetItemPosition<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetItemRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
	pub portion: co::LVIR,
}

impl<'a> MsgSend for LvmGetItemRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		self.rect.left = self.portion.raw() as _;

		Wm {
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
pub struct LvmGetItemSpacing {
	pub is_small_icon_view: bool,
}

impl MsgSend for LvmGetItemSpacing {
	type RetType = SIZE;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		SIZE::from(v as u32)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetItemState {
	pub index: u32,
	pub mask: co::LVIS,
}

impl MsgSend for LvmGetItemState {
	type RetType = co::LVIS;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::LVIS::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::GETITEMSTATE.into(),
			wparam: self.index as _,
			lparam: self.mask.raw() as _,
		}
	}
}

/// [`LVM_GETITEMTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getitemtext)
/// message parameters.
///
/// Return type: `u32`.
pub struct LvmGetItemText<'a, 'b> {
	pub index: u32,
	pub lvitem: &'b mut LVITEM<'a>,
}

impl<'a, 'b> MsgSend for LvmGetItemText<'a, 'b> {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetNextItem {
	pub initial_index: Option<u32>,
	pub relationship: co::LVNI,
}

impl MsgSend for LvmGetNextItem {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::GETNEXTITEM.into(),
			wparam: self.initial_index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.relationship.raw() as _,
		}
	}
}

/// [`LVM_GETNEXTITEMINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getnextitemindex)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct LvmGetNextItemIndex<'a> {
	pub initial_item: &'a mut LVITEMINDEX,
	pub relationship: co::LVNI,
}

impl<'a> MsgSend for LvmGetNextItemIndex<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::GETNEXTITEMINDEX.into(),
			wparam: self.initial_item as *mut _ as _,
			lparam: self.relationship.raw() as _,
		}
	}
}

/// [`LVM_GETNUMBEROFWORKAREAS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getnumberofworkareas)
/// message parameters.
///
/// Return type: `()`.
pub struct LvmGetNumberOfWorkAreas<'a> {
	pub num: &'a mut u32,
}

impl<'a> MsgSend for LvmGetNumberOfWorkAreas<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetOrigin<'a> {
	pub origin: &'a mut POINT,
}

impl<'a> MsgSend for LvmGetOrigin<'a> {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetOutlineColor {}

impl MsgSend for LvmGetOutlineColor {
	type RetType = COLORREF;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetSelectedColumn {}

impl MsgSend for LvmGetSelectedColumn {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetSelectedCount {}

impl MsgSend for LvmGetSelectedCount {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetSelectionMark {}

impl MsgSend for LvmGetSelectionMark {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetStringWidth {
	pub text: WString,
}

impl MsgSend for LvmGetStringWidth {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|len| len as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::GETSTRINGWIDTH.into(),
			wparam: 0,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`LVM_GETSUBITEMRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-getsubitemrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct LvmGetSubItemRect<'a> {
	pub item_index: u32,
	pub subitem_index: u32,
	pub rect: &'a mut RECT,
	pub portion: co::LVIR,
}

impl<'a> MsgSend for LvmGetSubItemRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		self.rect.left = self.portion.raw() as _;
		self.rect.top = self.subitem_index as _;

		Wm {
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
pub struct LvmGetTextBkColor {}

impl MsgSend for LvmGetTextBkColor {
	type RetType = COLORREF;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetTextColor {}

impl MsgSend for LvmGetTextColor {
	type RetType = COLORREF;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetTileInfo<'a, 'b> {
	pub info: &'b mut LVTILEINFO<'a>,
}

impl<'a, 'b> MsgSend for LvmGetTileInfo<'a, 'b> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetTileViewInfo<'a> {
	pub info: &'a mut LVTILEVIEWINFO,
}

impl<'a> MsgSend for LvmGetTileViewInfo<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetTooltips {}

impl MsgSend for LvmGetTooltips {
	type RetType = Option<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetTopIndex {}

impl MsgSend for LvmGetTopIndex {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetUnicodeFormat {}

impl MsgSend for LvmGetUnicodeFormat {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetView {}

impl MsgSend for LvmGetView {
	type RetType = co::LV_VIEW;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::LV_VIEW::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetViewRect<'a> {
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for LvmGetViewRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmGetWorkAreas<'a> {
	pub rects: &'a mut [RECT],
}

impl<'a> MsgSend for LvmGetWorkAreas<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmHasGroup {
	pub id: u32,
}

impl MsgSend for LvmHasGroup {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmHitTest<'a> {
	pub info: &'a mut LVHITTESTINFO,
}

impl<'a> MsgSend for LvmHitTest<'a> {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmInsertColumn<'a, 'b> {
	pub index: u32,
	pub column: &'b LVCOLUMN<'a>,
}

impl<'a, 'b> MsgSend for LvmInsertColumn<'a, 'b> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::INSERTCOLUMN.into(),
			wparam: self.index as _,
			lparam: self.column as *const _ as _,
		}
	}
}

/// [`LVM_INSERTGROUP`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-insertgroup)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct LvmInsertGroup<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	pub group: &'h LVGROUP<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> MsgSend for LvmInsertGroup<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::INSERTGROUP.into(),
			wparam: 0,
			lparam: self.group as *const _ as _,
		}
	}
}

/// [`LVM_INSERTGROUPSORTED`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-insertgroupsorted)
/// message parameters.
///
/// Return type: `()`.
pub struct LvmInsertGroupSorted<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	pub group: &'h LVINSERTGROUPSORTED<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> MsgSend
	for LvmInsertGroupSorted<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h>
{
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::INSERTGROUPSORTED.into(),
			wparam: 0,
			lparam: self.group as *const _ as _,
		}
	}
}

/// [`LVM_INSERTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-insertitem)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct LvmInsertItem<'a, 'b> {
	pub item: &'b LVITEM<'a>,
}

impl<'a, 'b> MsgSend for LvmInsertItem<'a, 'b> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::INSERTITEM.into(),
			wparam: 0,
			lparam: self.item as *const _ as _,
		}
	}
}

/// [`LVM_INSERTMARKHITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-insertmarkhittest)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct LvmInsertMarkHitTest<'a> {
	pub point: POINT,
	pub insert_mark: &'a LVINSERTMARK,
}

impl<'a, 'b> MsgSend for LvmInsertMarkHitTest<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::INSERTMARKHITTEST.into(),
			wparam: &self.point as *const _ as _,
			lparam: self.insert_mark as *const _ as _,
		}
	}
}

/// [`LVM_ISGROUPVIEWENABLED`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-isgroupviewenabled)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct LvmIsGroupViewEnabled {}

impl MsgSend for LvmIsGroupViewEnabled {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmIsItemVisible {
	pub index: u32,
}

impl MsgSend for LvmIsItemVisible {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmMapIdToIndex {
	pub id: u32,
}

impl MsgSend for LvmMapIdToIndex {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmMapIndexToId {
	pub index: u32,
}

impl MsgSend for LvmMapIndexToId {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmRedrawItems {
	pub first_index: u32,
	pub last_index: u32,
}

impl MsgSend for LvmRedrawItems {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::REDRAWITEMS.into(),
			wparam: self.first_index as _,
			lparam: self.last_index as _,
		}
	}
}

pub_struct_msg_empty! { LvmRemoveAllGroups: co::LVM::REMOVEALLGROUPS.into();
	/// [`LVM_REMOVEALLGROUPS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-removeallgroups)
	/// message, which has no parameters.
	///
	/// Return type: `SysResult<()>`.
}

/// [`LVM_REMOVEGROUP`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-removegroup)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct LvmRemoveGroup {
	pub id: u32,
}

impl MsgSend for LvmRemoveGroup {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|id| id as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmScroll {
	pub horizontal: i32,
	pub vertical: i32,
}

impl MsgSend for LvmScroll {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetBkColor {
	pub color: Option<COLORREF>,
}

impl MsgSend for LvmSetBkColor {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETBKCOLOR.into(),
			wparam: 0,
			lparam: self.color.map_or(co::CLR::NONE.raw(), |c| c.raw()) as _,
		}
	}
}

/// [`LVM_SETBKIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setbkimage)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct LvmSetBkImage<'a, 'b> {
	pub lvbkimage: &'b LVBKIMAGE<'a>,
}

impl<'a, 'b> MsgSend for LvmSetBkImage<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetCallbackMask {
	pub mask: co::LVIS,
}

impl MsgSend for LvmSetCallbackMask {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETCALLBACKMASK.into(),
			wparam: self.mask.raw() as _,
			lparam: 0,
		}
	}
}

/// [`LVM_SETCOLUMN`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setcolumn)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct LvmSetColumn<'a, 'b> {
	pub index: u32,
	pub lvcolumn: &'b LVCOLUMN<'a>,
}

impl<'a, 'b> MsgSend for LvmSetColumn<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetColumnOrderArray<'a> {
	pub order: &'a [u32],
}

impl<'a> MsgSend for LvmSetColumnOrderArray<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETCOLUMNORDERARRAY.into(),
			wparam: self.order.len() as _,
			lparam: vec_ptr(self.order) as _,
		}
	}
}

/// [`LVM_SETCOLUMNWIDTH`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setcolumnwidth)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct LvmSetColumnWidth {
	pub index: u32,
	pub width: u32,
}

impl MsgSend for LvmSetColumnWidth {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetExtendedListViewStyle {
	pub style: co::LVS_EX,
	pub mask: co::LVS_EX,
}

impl MsgSend for LvmSetExtendedListViewStyle {
	type RetType = co::LVS_EX;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::LVS_EX::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETEXTENDEDLISTVIEWSTYLE.into(),
			wparam: self.style.raw() as _,
			lparam: self.mask.raw() as _,
		}
	}
}

/// [`LVM_SETGROUPINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setgroupinfo)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct LvmSetGroupInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	pub id: u32,
	pub info: &'h LVGROUP<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> MsgSend for LvmSetGroupInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetGroupMetrics<'a> {
	pub info: &'a LVGROUPMETRICS,
}

impl<'a> MsgSend for LvmSetGroupMetrics<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetHotCursor<'a> {
	pub hcursor: Option<&'a HCURSOR>,
}

impl<'a> MsgSend for LvmSetHotCursor<'a> {
	type RetType = Option<HCURSOR>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HCURSOR::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETHOTCURSOR.into(),
			wparam: 0,
			lparam: self.hcursor.map_or(0, |h| h.ptr() as _),
		}
	}
}

/// [`LVM_SETHOTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-sethotitem)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct LvmSetHotItem {
	pub index: Option<u32>,
}

impl MsgSend for LvmSetHotItem {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetHoverTime {
	pub ms: Option<u32>,
}

impl MsgSend for LvmSetHoverTime {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetIconSpacing {
	pub size: SIZE,
}

impl MsgSend for LvmSetIconSpacing {
	type RetType = SIZE;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		SIZE::from(v as u32)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetImageList {
	pub kind: co::LVSIL,
	pub himagelist: Option<HIMAGELIST>,
}

impl MsgSend for LvmSetImageList {
	type RetType = Option<HIMAGELIST>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETIMAGELIST.into(),
			wparam: self.kind.raw() as _,
			lparam: self.himagelist.as_ref().map_or(0, |h| h.ptr() as _),
		}
	}
}

/// [`LVM_SETINFOTIP`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setinfotip)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct LvmSetInfoTip<'a, 'b> {
	pub info: &'b LVSETINFOTIP<'a>,
}

impl<'a, 'b> MsgSend for LvmSetInfoTip<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetInsertMark<'a> {
	pub info: &'a LVINSERTMARK,
}

impl<'a> MsgSend for LvmSetInsertMark<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetInsertMarkColor {
	pub color: COLORREF,
}

impl MsgSend for LvmSetInsertMarkColor {
	type RetType = COLORREF;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETINSERTMARKCOLOR.into(),
			wparam: 0,
			lparam: u32::from(self.color) as _,
		}
	}
}

/// [`LVM_SETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct LvmSetItem<'a, 'b> {
	pub lvitem: &'b LVITEM<'a>,
}

impl<'a, 'b> MsgSend for LvmSetItem<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetItemCount {
	pub count: u32,
	pub behavior: Option<co::LVSICF>,
}

impl MsgSend for LvmSetItemCount {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETITEMCOUNT.into(),
			wparam: self.count as _,
			lparam: self.behavior.unwrap_or_default().raw() as _,
		}
	}
}

/// [`LVM_SETITEMINDEXSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setitemindexstate)
/// message parameters.
///
/// Return type: `HrResult<()>`.
pub struct LvmSetItemIndexState<'a, 'b, 'c> {
	pub lvitemindex: &'a LVITEMINDEX,
	pub lvitem: &'c LVITEM<'b>,
}

impl<'a, 'b, 'c> MsgSend for LvmSetItemIndexState<'a, 'b, 'c> {
	type RetType = HrResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		HrRet(v as _).to_hrresult()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETITEMINDEXSTATE.into(),
			wparam: self.lvitemindex as *const _ as _,
			lparam: self.lvitem as *const _ as _,
		}
	}
}

/// [`LVM_SETITEMPOSITION`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setitemposition)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct LvmSetItemPosition {
	pub index: u32,
	pub position: POINT,
}

impl MsgSend for LvmSetItemPosition {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetItemPosition32 {
	pub index: u32,
	pub position: POINT,
}

impl MsgSend for LvmSetItemPosition32 {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetItemState<'a, 'b> {
	pub index: Option<u32>,
	pub lvitem: &'b LVITEM<'a>,
}

impl<'a, 'b> MsgSend for LvmSetItemState<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetItemText<'a, 'b> {
	pub index: u32,
	pub lvitem: &'b LVITEM<'a>,
}

impl<'a, 'b> MsgSend for LvmSetItemText<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetOutlineColor {
	pub color: COLORREF,
}

impl MsgSend for LvmSetOutlineColor {
	type RetType = COLORREF;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETOUTLINECOLOR.into(),
			wparam: 0,
			lparam: u32::from(self.color) as _,
		}
	}
}

/// [`LVM_SETSELECTEDCOLUMN`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setselectedcolumn)
/// message parameters.
///
/// Return type: `()`.
pub struct LvmSetSelectedColumn {
	pub index: u32,
}

impl MsgSend for LvmSetSelectedColumn {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetSelectionMark {
	pub index: Option<u32>,
}

impl MsgSend for LvmSetSelectionMark {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetTextBkColor {
	pub color: Option<COLORREF>,
}

impl MsgSend for LvmSetTextBkColor {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETTEXTBKCOLOR.into(),
			wparam: 0,
			lparam: self.color.map_or(co::CLR::NONE.raw(), |c| c.raw()) as _,
		}
	}
}

/// [`LVM_SETTEXTCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-settextcolor)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct LvmSetTextColor {
	pub color: Option<COLORREF>,
}

impl MsgSend for LvmSetTextColor {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETTEXTCOLOR.into(),
			wparam: 0,
			lparam: self.color.map_or(co::CLR::NONE.raw(), |c| c.raw()) as _,
		}
	}
}

/// [`LVM_SETTILEINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-settileinfo)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct LvmSetTileInfo<'a, 'b> {
	pub info: &'b LVTILEINFO<'a>,
}

impl<'a, 'b> MsgSend for LvmSetTileInfo<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetTileViewInfo<'a> {
	pub info: &'a LVTILEVIEWINFO,
}

impl<'a> MsgSend for LvmSetTileViewInfo<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetTooltips<'a> {
	pub htooltips: Option<&'a HWND>,
}

impl<'a> MsgSend for LvmSetTooltips<'a> {
	type RetType = Option<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETTOOLTIPS.into(),
			wparam: self.htooltips.map_or(0, |h| h.ptr() as _),
			lparam: 0,
		}
	}
}

/// [`LVM_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setunicodeformat)
/// message parameters.
///
/// Return type: `bool`.
pub struct LvmSetUnicodeFormat {
	pub use_unicode: bool,
}

impl MsgSend for LvmSetUnicodeFormat {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSetView {
	pub view: co::LV_VIEW,
}

impl MsgSend for LvmSetView {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETVIEW.into(),
			wparam: self.view.raw() as _,
			lparam: 0,
		}
	}
}

/// [`LVM_SETWORKAREAS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-setworkareas)
/// message parameters.
///
/// Return type: `()`.
pub struct LvmSetWorkAreas<'a> {
	pub rects: Option<&'a [RECT]>,
}

impl<'a> MsgSend for LvmSetWorkAreas<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::SETWORKAREAS.into(),
			wparam: self.rects.map_or(0, |r| r.len() as _),
			lparam: self.rects.map_or(0, |r| vec_ptr(r) as _),
		}
	}
}

/// [`LVM_SORTGROUPS`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-sortgroups)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct LvmSortGroups {
	pub callback: Option<PFNLVGROUPCOMPARE>,
	pub param: Option<isize>,
}

impl MsgSend for LvmSortGroups {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSortItems {
	pub param: isize,
	pub callback: PFNLVCOMPARE,
}

impl MsgSend for LvmSortItems {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSortItemsEx {
	pub param: isize,
	pub callback: PFNLVCOMPARE,
}

impl MsgSend for LvmSortItemsEx {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmSubItemHitTest<'a> {
	pub info: &'a mut LVHITTESTINFO,
}

impl<'a> MsgSend for LvmSubItemHitTest<'a> {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct LvmUpdate {
	pub index: u32,
}

impl MsgSend for LvmUpdate {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::LVM::UPDATE.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}
