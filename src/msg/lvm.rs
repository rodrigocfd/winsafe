//! List view control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-view-control-reference-messages),
//! whose constants have [`LVM`](crate::co::LVM) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::{HIWORD, LOWORD, MAKEDWORD};
use crate::handles::HWND;
use crate::msg::{MsgSend, WndMsg};
use crate::structs::{
	COLORREF,
	LVCOLUMN,
	LVFINDINFO,
	LVHITTESTINFO,
	LVITEM,
	RECT,
	SIZE,
};

/// [`LVM_APPROXIMATEVIEWRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-approximateviewrect)
/// message parameters.
///
/// Return type: `SIZE`.
pub struct ApproximateViewRect {
	pub num_items: Option<u32>,
	pub proposed_x: Option<u16>,
	pub proposed_y: Option<u16>,
}

impl MsgSend for ApproximateViewRect {
	type RetType = SIZE;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		SIZE::new(LOWORD(v as _) as _, HIWORD(v as _) as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::APPROXIMATEVIEWRECT.into(),
			wparam: match self.num_items {
				None => -1,
				Some(num) => num as isize,
			} as _,
			lparam: MAKEDWORD(
				match self.proposed_x {
					None => -1,
					Some(x) => x as i32,
				} as _,
				match self.proposed_y {
					None => -1,
					Some(y) => y as i32,
				} as _,
			) as _,
		}
	}
}

/// [`LVM_ARRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-arrange)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct Arrange {
	pub arrangement: co::LVA,
}

impl MsgSend for Arrange {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::ARRANGE.into(),
			wparam: self.arrangement.0 as _,
			lparam: 0,
		}
	}
}

pub_struct_msg_empty! { CancelEditLabel, co::LVM::CANCELEDITLABEL.into(),
	/// [`LVM_CANCELEDITLABEL`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-canceleditlabel)
}

/// [`LVM_DELETEALLITEMS`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-deleteallitems)
/// message, which has no parameters.
///
/// Return type: `WinResult<()>`.
pub struct DeleteAllItems {}

impl MsgSend for DeleteAllItems {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::DELETEALLITEMS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_DELETEITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-deleteitem)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct DeleteItem {
	pub index: u32,
}

impl MsgSend for DeleteItem {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::DELETEITEM.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LVM_ENSUREVISIBLE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-ensurevisible)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct EnsureVisible {
	pub index: u32,
	pub entirely_visible: bool,
}

impl MsgSend for EnsureVisible {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::ENSUREVISIBLE.into(),
			wparam: self.index as _,
			lparam: !self.entirely_visible as _,
		}
	}
}

/// [`LVM_FINDITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-finditem)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct FindItem<'a, 'b> {
	pub start_index: Option<u32>,
	pub lvfindinfo: &'b LVFINDINFO<'a>,
}

impl<'a, 'b> MsgSend for FindItem<'a, 'b> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => None,
			i => Some(i as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::FINDITEM.into(),
			wparam: self.start_index.map(|i| i as i32).unwrap_or(-1) as _,
			lparam: self.lvfindinfo as *const _ as _,
		}
	}
}

/// [`LVM_GETBKCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getbkcolor)
/// message, which has no parameters.
///
/// Return type: `COLORREF`.
pub struct GetBkColor {}

impl MsgSend for GetBkColor {
	type RetType = COLORREF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		COLORREF(v as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETBKCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getcolumn)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetColumn<'a, 'b> {
	pub index: u32,
	pub lvcolumn: &'b mut LVCOLUMN<'a>,
}

impl<'a, 'b> MsgSend for GetColumn<'a, 'b> {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETCOLUMN.into(),
			wparam: self.index as _,
			lparam: self.lvcolumn as *const _ as _,
		}
	}
}

/// [`LVM_GETCOLUMNWIDTH`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getcolumnwidth)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct GetColumnWidth {
	pub index: u32,
}

impl MsgSend for GetColumnWidth {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			i => Ok(i as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETCOLUMNWIDTH.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LVM_GETEXTENDEDLISTVIEWSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getextendedlistviewstyle)
/// message, which has no parameters.
///
/// Return type: `co::LVS_EX`.
pub struct GetExtendedListViewStyle {}

impl MsgSend for GetExtendedListViewStyle {
	type RetType = co::LVS_EX;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LVS_EX(v as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETEXTENDEDLISTVIEWSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETHEADER`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getheader)
/// message, which has no parameters.
///
/// Return type: `WinResult<HWND>`.
pub struct GetHeader {}

impl MsgSend for GetHeader {
	type RetType = WinResult<HWND>;

	fn_convert_ret_winresult_handle!(HWND);

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETHEADER.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETITEMCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getitemcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetItemCount {}

impl MsgSend for GetItemCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETITEMCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETITEMSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getitemstate)
/// message parameters.
///
/// Return type: `co::LVIS`.
pub struct GetItemState {
	pub index: u32,
	pub mask: co::LVIS,
}

impl MsgSend for GetItemState {
	type RetType = co::LVIS;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LVIS(v as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETITEMSTATE.into(),
			wparam: self.index as _,
			lparam: self.mask.0 as _,
		}
	}
}

/// [`LVM_GETITEMTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getitemtext)
/// message parameters.
///
/// Return type: `u32`.
pub struct GetItemText<'a, 'b> {
	pub index: u32,
	pub lvitem: &'b mut LVITEM<'a>,
}

impl<'a, 'b> MsgSend for GetItemText<'a, 'b> {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETITEMTEXT.into(),
			wparam: self.index as _,
			lparam: self.lvitem as *const _ as _,
		}
	}
}

/// [`LVM_GETNEXTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getnextitem)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct GetNextItem {
	pub initial_index: Option<u32>,
	pub relationship: co::LVNI,
}

impl MsgSend for GetNextItem {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETNEXTITEM.into(),
			wparam: self.initial_index.map(|i| i as i32).unwrap_or(-1) as _,
			lparam: self.relationship.0 as _,
		}
	}
}

/// [`LVM_GETITEMRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getitemrect)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetItemRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
	pub portion: co::LVIR,
}

impl<'a> MsgSend for GetItemRect<'a> {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		let ptr_rect = self.rect as *const _ as *mut RECT;
		unsafe { &mut *ptr_rect }.left = self.portion.0 as _;

		WndMsg {
			msg_id: co::LVM::GETITEMRECT.into(),
			wparam: self.index as _,
			lparam: self.rect as *const _ as _,
		}
	}
}

/// [`LVM_GETSELECTEDCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getselectedcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetSelectedCount {}

impl MsgSend for GetSelectedCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETSELECTEDCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_GETVIEW`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getview)
/// message, which has no parameters.
///
/// Return type: `co::LV_VIEW`.
pub struct GetView {}

impl MsgSend for GetView {
	type RetType = co::LV_VIEW;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LV_VIEW(v as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::GETVIEW.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}
/// [`LVM_HITTEST`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-hittest)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct HitTest<'a> {
	pub info: &'a mut LVHITTESTINFO,
}

impl<'a> MsgSend for HitTest<'a> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => None,
			i => Some(i as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::HITTEST.into(),
			wparam: -1 as _,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`LVM_INSERTCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-insertcolumn)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct InsertColumn<'a, 'b> {
	pub index: u32,
	pub lvcolumn: &'b LVCOLUMN<'a>,
}

impl<'a, 'b> MsgSend for InsertColumn<'a, 'b> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			i => Ok(i as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::INSERTCOLUMN.into(),
			wparam: self.index as _,
			lparam: self.lvcolumn as *const _ as _,
		}
	}
}

/// [`LVM_INSERTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-insertitem)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct InsertItem<'a, 'b> {
	pub lvitem: &'b LVITEM<'a>,
}

impl<'a, 'b> MsgSend for InsertItem<'a, 'b> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			i => Ok(i as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::INSERTITEM.into(),
			wparam: 0,
			lparam: self.lvitem as *const _ as _,
		}
	}
}

/// [`LVM_ISGROUPVIEWENABLED`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-isgroupviewenabled)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct IsGroupViewEnabled {}

impl MsgSend for IsGroupViewEnabled {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::ISGROUPVIEWENABLED.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LVM_ISITEMVISIBLE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-isitemvisible)
/// message parameters.
///
/// Return type: `bool`.
pub struct IsItemVisible {
	pub index: u32,
}

impl MsgSend for IsItemVisible {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::ISITEMVISIBLE.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LVM_REDRAWITEMS`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-redrawitems)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct RedrawItems {
	pub first_index: u32,
	pub last_index: u32,
}

impl MsgSend for RedrawItems {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::REDRAWITEMS.into(),
			wparam: self.first_index as _,
			lparam: self.last_index as _,
		}
	}
}

/// [`LVM_SCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-scroll)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct Scroll {
	pub horizontal: i32,
	pub vertical: i32,
}

impl MsgSend for Scroll {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SCROLL.into(),
			wparam: self.horizontal as _,
			lparam: self.vertical as _,
		}
	}
}

/// [`LVM_SETCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setcolumn)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetColumn<'a, 'b> {
	pub index: u32,
	pub lvcolumn: &'b LVCOLUMN<'a>,
}

impl<'a, 'b> MsgSend for SetColumn<'a, 'b> {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETCOLUMN.into(),
			wparam: self.index as _,
			lparam: self.lvcolumn as *const _ as _,
		}
	}
}

/// [`LVM_SETCOLUMNWIDTH`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setcolumnwidth)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetColumnWidth {
	pub index: u32,
	pub width: u32,
}

impl MsgSend for SetColumnWidth {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETCOLUMNWIDTH.into(),
			wparam: self.index as _,
			lparam: self.width as _,
		}
	}
}

/// [`LVM_SETEXTENDEDLISTVIEWSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setextendedlistviewstyle)
/// message parameters.
///
/// Return type: `co::LVS_EX`.
pub struct SetExtendedListViewStyle {
	pub style: co::LVS_EX,
	pub mask: co::LVS_EX,
}

impl MsgSend for SetExtendedListViewStyle {
	type RetType = co::LVS_EX;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LVS_EX(v as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETEXTENDEDLISTVIEWSTYLE.into(),
			wparam: self.style.0 as _,
			lparam: self.mask.0 as _,
		}
	}
}

/// [`LVM_SETITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitem)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetItem<'a, 'b> {
	pub lvitem: &'b LVITEM<'a>,
}

impl<'a, 'b> MsgSend for SetItem<'a, 'b> {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETITEM.into(),
			wparam: 0,
			lparam: self.lvitem as *const _ as _,
		}
	}
}

/// [`LVM_SETITEMSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitemstate)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetItemState<'a, 'b> {
	pub index: Option<u32>,
	pub lvitem: &'b LVITEM<'a>,
}

impl<'a, 'b> MsgSend for SetItemState<'a, 'b> {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETITEMSTATE.into(),
			wparam: self.index.map(|i| i as i32).unwrap_or(-1) as _,
			lparam: self.lvitem as *const _ as _,
		}
	}
}

/// [`LVM_SETITEMTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitemtext)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetItemText<'a, 'b> {
	pub index: u32,
	pub lvitem: &'b LVITEM<'a>,
}

impl<'a, 'b> MsgSend for SetItemText<'a, 'b> {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETITEMTEXT.into(),
			wparam: self.index as _,
			lparam: self.lvitem as *const _ as _,
		}
	}
}

/// [`LVM_SETSELECTEDCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setselectedcolumn)
/// message parameters.
///
/// Return type: `()`.
pub struct SetSelectedColumn {
	pub index: u32,
}

impl MsgSend for SetSelectedColumn {
	type RetType = ();

	fn_convert_ret_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETSELECTEDCOLUMN.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LVM_SETVIEW`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setview)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetView {
	pub view: co::LV_VIEW,
}

impl MsgSend for SetView {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::SETVIEW.into(),
			wparam: self.view.0 as _,
			lparam: 0,
		}
	}
}

/// [`LVM_UPDATE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-update)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct Update {
	pub index: u32,
}

impl MsgSend for Update {
	type RetType = WinResult<()>;

	fn_convert_ret_winresult_void!();

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::LVM::UPDATE.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}
