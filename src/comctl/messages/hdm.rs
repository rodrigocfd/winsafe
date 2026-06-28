use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`HDM_CLEARFILTER`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-clearfilter)
/// message parameters.
///
/// Return type: `bool`.
pub struct HdmClearFilter {
	pub filter: Option<u32>,
}

impl MsgSend for HdmClearFilter {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::CLEARFILTER.into(),
			wparam: self.filter.map_or(-1, |f| f as i32) as _,
			lparam: 0,
		}
	}
}

/// [`HDM_CREATEDRAGIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-createdragimage)
/// message parameters.
///
/// Return type: `SysResult<HIMAGELIST>`.
pub struct HdmCreateDragImage {
	pub index: u32,
}

impl MsgSend for HdmCreateDragImage {
	type RetType = SysResult<HIMAGELIST>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::CREATEDRAGIMAGE.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`HDM_DELETEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-deleteitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct HdmDeleteItem {
	pub index: u32,
}

impl MsgSend for HdmDeleteItem {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::DELETEITEM.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`HDM_EDITFILTER`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-editfilter)
/// message parameters.
///
/// Return type: `bool`.
pub struct HdmEditFilter {
	pub index: u32,
	pub discard_changes: bool,
}

impl MsgSend for HdmEditFilter {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::EDITFILTER.into(),
			wparam: self.index as _,
			lparam: self.discard_changes as _,
		}
	}
}

/// [`HDM_GETBITMAPMARGIN`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getbitmapmargin)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct HdmGetBitmapMargin {}

impl MsgSend for HdmGetBitmapMargin {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::GETBITMAPMARGIN.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`HDM_GETFOCUSEDITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getfocuseditem)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct HdmGetFocusedItem {}

impl MsgSend for HdmGetFocusedItem {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::GETFOCUSEDITEM.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`HDM_GETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct HdmGetImageList {
	pub kind: co::HDSIL,
}

impl MsgSend for HdmGetImageList {
	type RetType = Option<HIMAGELIST>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::GETIMAGELIST.into(),
			wparam: self.kind.raw() as _,
			lparam: 0,
		}
	}
}

/// [`HDM_GETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getitem)
/// message parameters.
///
/// Return type: `()`.
pub struct HdmGetItem<'a, 'b> {
	pub index: u32,
	pub hditem: &'b mut HDITEM<'a>,
}

impl<'a, 'b> MsgSend for HdmGetItem<'a, 'b> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		() // docs are wrong: always returns zero
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::GETITEM.into(),
			wparam: self.index as _,
			lparam: self.hditem as *mut _ as _,
		}
	}
}

/// [`HDM_GETITEMCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getitemcount)
/// message, which has no parameters.
///
/// Return type: `SysResult<u32>`.
pub struct HdmGetItemCount {}

impl MsgSend for HdmGetItemCount {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::GETITEMCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`HDM_GETITEMDROPDOWNRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getitemdropdownrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct HdmGetItemDropDownRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for HdmGetItemDropDownRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::GETITEMDROPDOWNRECT.into(),
			wparam: self.index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`HDM_GETITEMRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getitemrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct HdmGetItemRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for HdmGetItemRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::GETITEMRECT.into(),
			wparam: self.index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`HDM_GETORDERARRAY`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getorderarray)
/// message parameters.
///
/// Return type `SysResult<()>`.
pub struct HdmGetOrderArray<'a> {
	pub buffer: &'a mut [u32],
}

impl<'a> MsgSend for HdmGetOrderArray<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::GETORDERARRAY.into(),
			wparam: self.buffer.len(),
			lparam: self.buffer.as_mut_ptr() as _,
		}
	}
}

/// [`HDM_GETOVERFLOWRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getoverflowrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct HdmGetOverflowRect<'a> {
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for HdmGetOverflowRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::GETOVERFLOWRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`HDM_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getunicodeformat)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct HdmGetUnicodeFormat {}

impl MsgSend for HdmGetUnicodeFormat {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`HDM_HITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-hittest)
/// message parameters.
///
/// Return type: `u32`.
pub struct HdmHitTest<'a> {
	pub test_info: &'a mut HDHITTESTINFO,
}

impl<'a> MsgSend for HdmHitTest<'a> {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::HITTEST.into(),
			wparam: 0,
			lparam: self.test_info as *mut _ as _,
		}
	}
}

/// [`HDM_INSERTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-insertitem)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct HdmInsertItem<'a, 'b> {
	pub index_after: u32,
	pub item: &'b HDITEM<'a>,
}

impl<'a, 'b> MsgSend for HdmInsertItem<'a, 'b> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::INSERTITEM.into(),
			wparam: self.index_after as _,
			lparam: self.item as *const _ as _,
		}
	}
}

/// [`HDM_LAYOUT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-layout)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct HdmLayout<'a, 'b, 'c> {
	pub hdlayout: &'c mut HDLAYOUT<'a, 'b>,
}

impl<'a, 'b, 'c> MsgSend for HdmLayout<'a, 'b, 'c> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::LAYOUT.into(),
			wparam: 0,
			lparam: self.hdlayout as *mut _ as _,
		}
	}
}

/// [`HDM_ORDERTOINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-ordertoindex)
/// message parameters.
///
/// Return type: `u32`.
pub struct HdmOrderToIndex {
	pub order: u32,
}

impl MsgSend for HdmOrderToIndex {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::ORDERTOINDEX.into(),
			wparam: self.order as _,
			lparam: 0,
		}
	}
}

/// [`HDM_SETBITMAPMARGIN`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-setbitmapmargin)
/// message parameters.
///
/// Return type: `u32`.
pub struct HdmSetBitmapMargin {
	pub width: u32,
}

impl MsgSend for HdmSetBitmapMargin {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::SETBITMAPMARGIN.into(),
			wparam: self.width as _,
			lparam: 0,
		}
	}
}

/// [`HDM_SETFILTERCHANGETIMEOUT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-setfilterchangetimeout)
/// message parameters.
///
/// Return type: `u32`.
pub struct HdmSetFilterChangeTimeout {
	pub timeout_ms: u32,
}

impl MsgSend for HdmSetFilterChangeTimeout {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::SETFILTERCHANGETIMEOUT.into(),
			wparam: self.timeout_ms as _,
			lparam: 0,
		}
	}
}

/// [`HDM_SETFOCUSEDITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-setfocuseditem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct HdmSetFocusedItem {
	pub index: u32,
}

impl MsgSend for HdmSetFocusedItem {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::SETFILTERCHANGETIMEOUT.into(),
			wparam: 0,
			lparam: self.index as _,
		}
	}
}

/// [`HDM_SETHOTDIVIDER`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-sethotdivider)
/// message parameters.
///
/// Return type: `u32`.
pub struct HdmSetHotDivider {
	pub value: PtIdx,
}

impl MsgSend for HdmSetHotDivider {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::SETHOTDIVIDER.into(),
			wparam: match self.value {
				PtIdx::Pt(_) => true,
				PtIdx::Idx(_) => false,
			} as _,
			lparam: match self.value {
				PtIdx::Pt(pt) => pt.into(),
				PtIdx::Idx(idx) => idx,
			} as _,
		}
	}
}

/// [`HDM_SETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-setimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct HdmSetImageList {
	pub kind: co::HDSIL,
	pub himagelist: Option<HIMAGELIST>,
}

impl MsgSend for HdmSetImageList {
	type RetType = Option<HIMAGELIST>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::SETIMAGELIST.into(),
			wparam: self.kind.raw() as _,
			lparam: self.himagelist.as_ref().map_or(0, |h| h.ptr() as _),
		}
	}
}

/// [`HDM_SETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-setitem)
/// message parameters.
///
/// Return type: `()`.
pub struct HdmSetItem<'a, 'b> {
	pub index: u32,
	pub hditem: &'b HDITEM<'a>,
}

impl<'a, 'b> MsgSend for HdmSetItem<'a, 'b> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		() // docs are wrong: always returns zero
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::SETITEM.into(),
			wparam: self.index as _,
			lparam: self.hditem as *const _ as _,
		}
	}
}

/// [`HDM_SETORDERARRAY`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-setorderarray)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct HdmSetOrderArray<'a> {
	pub buffer: &'a [u32],
}

impl<'a> MsgSend for HdmSetOrderArray<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::SETORDERARRAY.into(),
			wparam: self.buffer.len(),
			lparam: vec_ptr(self.buffer) as _,
		}
	}
}

/// [`HDM_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-setunicodeformat)
/// message parameters.
///
/// Return type: `bool`.
pub struct HdmSetUnicodeFormat {
	pub use_unicode: bool,
}

impl MsgSend for HdmSetUnicodeFormat {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::HDM::SETUNICODEFORMAT.into(),
			wparam: self.use_unicode as _,
			lparam: 0,
		}
	}
}
