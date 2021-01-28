use crate::aliases::WinResult;
use crate::co;
use crate::handles::HWND;
use crate::msg::{Message, Wm};
use crate::msg::macros::ref_to_lp;
use crate::structs as s;

/// [`LVM_DELETEALLITEMS`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-deleteallitems)
/// message, which has no parameters.
///
/// Return type: `WinResult<()>`.
pub struct LvmDeleteAllItems {}

impl Message for LvmDeleteAllItems {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_DELETEALLITEMS,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_DELETEITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-deleteitem)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct LvmDeleteItem {
	pub index: i32,
}

impl Message for LvmDeleteItem {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_DELETEITEM,
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_ENSUREVISIBLE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-ensurevisible)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct LvmEnsureVisible {
	pub index: i32,
	pub entirely_visible: bool,
}

impl Message for LvmEnsureVisible {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_ENSUREVISIBLE,
			wparam: self.index as usize,
			lparam: !self.entirely_visible as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_GETCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getcolumn)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct LvmGetColumn<'a, 'b> {
	pub index: i32,
	pub lvcolumn: &'b mut s::LVCOLUMN<'a>,
}

impl<'a, 'b> Message for LvmGetColumn<'a, 'b> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			i => Ok(i as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_GETCOLUMN,
			wparam: self.index as usize,
			lparam: ref_to_lp(self.lvcolumn),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_GETCOLUMNWIDTH`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getcolumnwidth)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct LvmGetColumnWidth {
	pub index: i32,
}

impl Message for LvmGetColumnWidth {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			i => Ok(i as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_GETCOLUMNWIDTH,
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_GETEXTENDEDLISTVIEWSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getextendedlistviewstyle)
/// message, which has no parameters.
///
/// Return type: `LVS_EX`.
pub struct LvmGetExtendedListViewStyle {}

impl Message for LvmGetExtendedListViewStyle {
	type RetType = co::LVS_EX;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LVS_EX(v as u32)
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_GETEXTENDEDLISTVIEWSTYLE,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_GETHEADER`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getheader)
/// message, which has no parameters.
///
/// Return type: `WinResult<HWND>`.
pub struct LvmGetHeader {}

impl Message for LvmGetHeader {
	type RetType = WinResult<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			p => Ok(HWND { ptr: p as *mut _ }),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_GETHEADER,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_GETNEXTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getnextitem)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct LvmGetNextItem {
	pub initial_index: i32,
	pub relationship: co::LVNI,
}

impl Message for LvmGetNextItem {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => None,
			idx => Some(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_GETNEXTITEM,
			wparam: self.initial_index as usize,
			lparam: self.relationship.0 as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_GETITEMCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getitemcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct LvmGetItemCount {}

impl Message for LvmGetItemCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_GETITEMCOUNT,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_GETITEMSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getitemstate)
/// message parameters.
///
/// Return type: `LVIS`.
pub struct LvmGetItemState {
	pub index: i32,
	pub mask: co::LVIS,
}

impl Message for LvmGetItemState {
	type RetType = co::LVIS;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LVIS(v as u32)
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_GETITEMSTATE,
			wparam: self.index as usize,
			lparam: self.mask.0 as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_GETITEMTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getitemtext)
/// message parameters.
///
/// Return type: `u32`.
pub struct LvmGetItemText<'a, 'b> {
	pub index: i32,
	pub lvitem: &'b mut s::LVITEM<'a>,
}

impl<'a, 'b> Message for LvmGetItemText<'a, 'b> {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_GETITEMTEXT,
			wparam: self.index as usize,
			lparam: ref_to_lp(self.lvitem),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_GETSELECTEDCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getselectedcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct LvmGetSelectedCount {}

impl Message for LvmGetSelectedCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_GETSELECTEDCOUNT,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_GETVIEW`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getview)
/// message, which has no parameters.
///
/// Return type: `LV_VIEW`.
pub struct LvmGetView {}

impl Message for LvmGetView {
	type RetType = co::LV_VIEW;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LV_VIEW(v as u32)
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_GETVIEW,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_INSERTCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-insertcolumn)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct LvmInsertColumn<'a, 'b> {
	pub index: i32,
	pub lvcolumn: &'b s::LVCOLUMN<'a>,
}

impl<'a, 'b> Message for LvmInsertColumn<'a, 'b> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			i => Ok(i as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_INSERTCOLUMN,
			wparam: self.index as usize,
			lparam: ref_to_lp(self.lvcolumn),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_INSERTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-insertitem)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct LvmInsertItem<'a, 'b> {
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> Message for LvmInsertItem<'a, 'b> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			i => Ok(i as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_INSERTITEM,
			wparam: 0,
			lparam: ref_to_lp(self.lvitem),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_ISGROUPVIEWENABLED`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-isgroupviewenabled)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct LvmIsGroupViewEnabled {}

impl Message for LvmIsGroupViewEnabled {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_ISGROUPVIEWENABLED,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_ISITEMVISIBLE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-isitemvisible)
/// message parameters.
///
/// Return type: `bool`.
pub struct LvmIsItemVisible {
	pub index: i32,
}

impl Message for LvmIsItemVisible {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_ISITEMVISIBLE,
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_REDRAWITEMS`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-redrawitems)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct LvmRedrawItems {
	pub first_index: u32,
	pub last_index: u32,
}

impl Message for LvmRedrawItems {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_REDRAWITEMS,
			wparam: self.first_index as usize,
			lparam: self.last_index as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_SCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-scroll)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct LvmScroll {
	pub horizontal: i32,
	pub vertical: i32,
}

impl Message for LvmScroll {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_SCROLL,
			wparam: self.horizontal as usize,
			lparam: self.vertical as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_SETCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setcolumn)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct LvmSetColumn<'a, 'b> {
	pub index: i32,
	pub lvcolumn: &'b s::LVCOLUMN<'a>,
}

impl<'a, 'b> Message for LvmSetColumn<'a, 'b> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_SETCOLUMN,
			wparam: self.index as usize,
			lparam: ref_to_lp(self.lvcolumn),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_SETEXTENDEDLISTVIEWSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setextendedlistviewstyle)
/// message parameters.
///
/// Return type: `LVS_EX`.
pub struct LvmSetExtendedListViewStyle {
	pub style: co::LVS_EX,
	pub mask: co::LVS_EX,
}

impl Message for LvmSetExtendedListViewStyle {
	type RetType = co::LVS_EX;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LVS_EX(v as u32)
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_SETEXTENDEDLISTVIEWSTYLE,
			wparam: self.style.0 as usize,
			lparam: self.mask.0 as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_SETITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitem)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct LvmSetItem<'a, 'b> {
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> Message for LvmSetItem<'a, 'b> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_SETITEM,
			wparam: 0,
			lparam: ref_to_lp(self.lvitem),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_SETITEMSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitemstate)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct LvmSetItemState<'a, 'b> {
	pub index: i32,
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> Message for LvmSetItemState<'a, 'b> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_SETITEMSTATE,
			wparam: self.index as usize,
			lparam: ref_to_lp(self.lvitem),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_SETITEMTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitemtext)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct LvmSetItemText<'a, 'b> {
	pub index: i32,
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> Message for LvmSetItemText<'a, 'b> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_SETITEMTEXT,
			wparam: self.index as usize,
			lparam: ref_to_lp(self.lvitem),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_SETSELECTEDCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setselectedcolumn)
/// message parameters.
///
/// Return type: `()`.
pub struct LvmSetSelectedColumn {
	pub index: u32,
}

impl Message for LvmSetSelectedColumn {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_SETSELECTEDCOLUMN,
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_SETVIEW`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setview)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct LvmSetView {
	pub view: co::LV_VIEW,
}

impl Message for LvmSetView {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_SETVIEW,
			wparam: self.view.0 as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_UPDATE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-update)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct LvmUpdate {
	pub index: i32,
}

impl Message for LvmUpdate {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_UPDATE,
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}
