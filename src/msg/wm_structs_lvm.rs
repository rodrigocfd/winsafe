use crate::co;
use crate::funcs::GetLastError;
use crate::msg::{Message, Wm};
use crate::msg::macros::{lparam_to_ref, ref_to_lparam};
use crate::structs as s;

/// [`LVM_GETCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getcolumn)
/// message parameters.
pub struct LvmGetColumn<'a, 'b> {
	pub index: i32,
	pub lvcolumn: &'b s::LVCOLUMN<'a>,
}

impl<'a, 'b> Message for LvmGetColumn<'a, 'b> {
	type RetType = Result<u32, co::ERROR>;

	fn convert_ret(v: isize) -> Result<u32, co::ERROR> {
		match v {
			-1 => Err(GetLastError()),
			i => Ok(i as u32),
		}
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_GETCOLUMN,
			wparam: self.index as usize,
			lparam: ref_to_lparam(self.lvcolumn),
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			index: p.wparam as i32,
			lvcolumn: lparam_to_ref(p),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_GETCOLUMNWIDTH`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getcolumnwidth)
/// message parameters.
pub struct LvmGetColumnWidth {
	pub index: i32,
}

impl Message for LvmGetColumnWidth {
	type RetType = Result<u32, co::ERROR>;

	fn convert_ret(v: isize) -> Result<u32, co::ERROR> {
		match v {
			-1 => Err(GetLastError()),
			i => Ok(i as u32),
		}
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_GETCOLUMNWIDTH,
			wparam: self.index as usize,
			lparam: 0,
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			index: p.wparam as i32,
		}
	}
}

//------------------------------------------------------------------------------

empty_msg! { LvmGetHeader, co::WM::LVM_GETHEADER,
	/// [`LVM_GETHEADER`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getheader)
	/// message, which has no parameters.
}

empty_msg! { LvmGetItemCount, co::WM::LVM_GETITEMCOUNT,
	/// [`LVM_GETITEMCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getitemcount)
	/// message, which has no parameters.
}

//------------------------------------------------------------------------------

/// [`LVM_GETVIEW`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getview)
/// message parameters.
pub struct LvmGetView {}

impl Message for LvmGetView {
	type RetType = co::LV_VIEW;

	fn convert_ret(v: isize) -> co::LV_VIEW {
		co::LV_VIEW::from(v as u32)
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_GETVIEW,
			wparam: 0,
			lparam: 0,
		}
	}

	fn from_generic_wm(_: Wm) -> Self {
		Self {}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_INSERTCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-insertcolumn)
/// message parameters.
pub struct LvmInsertColumn<'a, 'b> {
	pub index: i32,
	pub lvcolumn: &'b s::LVCOLUMN<'a>,
}

impl<'a, 'b> Message for LvmInsertColumn<'a, 'b> {
	type RetType = Result<u32, co::ERROR>;

	fn convert_ret(v: isize) -> Result<u32, co::ERROR> {
		match v {
			-1 => Err(GetLastError()),
			i => Ok(i as u32),
		}
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_INSERTCOLUMN,
			wparam: self.index as usize,
			lparam: ref_to_lparam(self.lvcolumn),
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			index: p.wparam as i32,
			lvcolumn: lparam_to_ref(p),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_INSERTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-insertitem)
/// message parameters.
pub struct LvmInsertItem<'a, 'b> {
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> Message for LvmInsertItem<'a, 'b> {
	type RetType = Result<u32, co::ERROR>;

	fn convert_ret(v: isize) -> Result<u32, co::ERROR> {
		match v {
			-1 => Err(GetLastError()),
			i => Ok(i as u32),
		}
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_INSERTITEM,
			wparam: 0,
			lparam: ref_to_lparam(self.lvitem),
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			lvitem: lparam_to_ref(p),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_ISGROUPVIEWENABLED`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-isgroupviewenabled)
/// message parameters.
pub struct LvmIsGroupViewEnabled {}

impl Message for LvmIsGroupViewEnabled {
	type RetType = bool;

	fn convert_ret(v: isize) -> bool {
		v != 0
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_ISGROUPVIEWENABLED,
			wparam: 0,
			lparam: 0,
		}
	}

	fn from_generic_wm(_: Wm) -> Self {
		Self {}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_ISITEMVISIBLE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-isitemvisible)
/// message parameters.
pub struct LvmIsItemVisible {
	pub index: i32,
}

impl Message for LvmIsItemVisible {
	type RetType = bool;

	fn convert_ret(v: isize) -> bool {
		v != 0
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_ISITEMVISIBLE,
			wparam: self.index as usize,
			lparam: 0,
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			index: p.wparam as i32,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_REDRAWITEMS`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-redrawitems)
/// message parameters.
pub struct LvmRedrawItems {
	pub first_index: u32,
	pub last_index: u32,
}

impl Message for LvmRedrawItems {
	type RetType = Result<(), co::ERROR>;

	fn convert_ret(v: isize) -> Result<(), co::ERROR> {
		match v {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_REDRAWITEMS,
			wparam: self.first_index as usize,
			lparam: self.last_index as isize,
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			first_index: p.wparam as u32,
			last_index: p.lparam as u32,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_SCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-scroll)
/// message parameters.
pub struct LvmScroll {
	pub horizontal: i32,
	pub vertical: i32,
}

impl Message for LvmScroll {
	type RetType = bool;

	fn convert_ret(v: isize) -> bool {
		v != 0
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_SCROLL,
			wparam: self.horizontal as usize,
			lparam: self.vertical as isize,
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			horizontal: p.wparam as i32,
			vertical: p.lparam as i32,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_SETCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setcolumn)
/// message parameters.
pub struct LvmSetColumn<'a, 'b> {
	pub index: i32,
	pub lvcolumn: &'b s::LVCOLUMN<'a>,
}

impl<'a, 'b> Message for LvmSetColumn<'a, 'b> {
	type RetType = bool;

	fn convert_ret(v: isize) -> bool {
		v != 0
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_SETCOLUMN,
			wparam: self.index as usize,
			lparam: ref_to_lparam(self.lvcolumn),
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			index: p.wparam as i32,
			lvcolumn: lparam_to_ref(p),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_SETITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitem)
/// message parameters.
pub struct LvmSetItem<'a, 'b> {
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> Message for LvmSetItem<'a, 'b> {
	type RetType = bool;

	fn convert_ret(v: isize) -> bool {
		v != 0
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_SETITEM,
			wparam: 0,
			lparam: ref_to_lparam(self.lvitem),
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			lvitem: lparam_to_ref(p),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_SETITEMTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitemtext)
/// message parameters.
pub struct LvmSetItemText<'a, 'b> {
	pub index: i32,
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> Message for LvmSetItemText<'a, 'b> {
	type RetType = bool;

	fn convert_ret(v: isize) -> bool {
		v != 0
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_SETITEMTEXT,
			wparam: self.index as usize,
			lparam: ref_to_lparam(self.lvitem),
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			index: p.wparam as i32,
			lvitem: lparam_to_ref(p),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_SETVIEW`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setview)
/// message parameters.
pub struct LvmSetView {
	pub view: co::LV_VIEW,
}

impl Message for LvmSetView {
	type RetType = Result<(), co::ERROR>;

	fn convert_ret(v: isize) -> Result<(), co::ERROR> {
		match v {
			-1 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_SETVIEW,
			wparam: u32::from(self.view) as usize,
			lparam: 0,
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			view: co::LV_VIEW::from(p.wparam as u32),
		}
	}
}

//------------------------------------------------------------------------------

/// [`LVM_UPDATE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-update)
/// message parameters.
pub struct LvmUpdate {
	pub index: i32,
}

impl Message for LvmUpdate {
	type RetType = bool;

	fn convert_ret(v: isize) -> bool {
		v != 0
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_UPDATE,
			wparam: self.index as usize,
			lparam: 0,
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			index: p.wparam as i32,
		}
	}
}
