use crate::co;
use crate::funcs::GetLastError;
use crate::msg::{Message, Wm};
use crate::msg::macros::{lparam_to_ref, ref_to_lparam};
use crate::structs as s;

empty_msg! { LvmGetHeader, co::WM::LVM_GETHEADER,
	/// [`LVM_GETHEADER`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getheader)
	/// message, which has no parameters.
}

empty_msg! { LvmGetItemCount, co::WM::LVM_GETITEMCOUNT,
	/// [`LVM_GETITEMCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getitemcount)
	/// message, which has no parameters.
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
