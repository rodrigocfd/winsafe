use crate::aliases::WinResult;
use crate::co;
use crate::funcs::GetLastError;
use crate::msg::{Message, Wm};
use crate::msg::macros::ref_to_lp;
use crate::structs as s;

/// [`LVM_GETCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getcolumn)
/// message parameters.
pub struct LvmGetColumn<'a, 'b> {
	pub index: i32,
	pub lvcolumn: &'b s::LVCOLUMN<'a>,
}

impl<'a, 'b> Message for LvmGetColumn<'a, 'b> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(GetLastError()),
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
pub struct LvmGetColumnWidth {
	pub index: i32,
}

impl Message for LvmGetColumnWidth {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(GetLastError()),
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

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::LV_VIEW::from(v as u32)
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
pub struct LvmInsertColumn<'a, 'b> {
	pub index: i32,
	pub lvcolumn: &'b s::LVCOLUMN<'a>,
}

impl<'a, 'b> Message for LvmInsertColumn<'a, 'b> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(GetLastError()),
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
pub struct LvmInsertItem<'a, 'b> {
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> Message for LvmInsertItem<'a, 'b> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(GetLastError()),
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
/// message parameters.
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
pub struct LvmRedrawItems {
	pub first_index: u32,
	pub last_index: u32,
}

impl Message for LvmRedrawItems {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
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
pub struct LvmScroll {
	pub horizontal: i32,
	pub vertical: i32,
}

impl Message for LvmScroll {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
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
pub struct LvmSetColumn<'a, 'b> {
	pub index: i32,
	pub lvcolumn: &'b s::LVCOLUMN<'a>,
}

impl<'a, 'b> Message for LvmSetColumn<'a, 'b> {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
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

/// [`LVM_SETITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitem)
/// message parameters.
pub struct LvmSetItem<'a, 'b> {
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> Message for LvmSetItem<'a, 'b> {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
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

/// [`LVM_SETITEMTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitemtext)
/// message parameters.
pub struct LvmSetItemText<'a, 'b> {
	pub index: i32,
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> Message for LvmSetItemText<'a, 'b> {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
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

/// [`LVM_SETVIEW`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setview)
/// message parameters.
pub struct LvmSetView {
	pub view: co::LV_VIEW,
}

impl Message for LvmSetView {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_SETVIEW,
			wparam: u32::from(self.view) as usize,
			lparam: 0,
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

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::LVM_UPDATE,
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}
