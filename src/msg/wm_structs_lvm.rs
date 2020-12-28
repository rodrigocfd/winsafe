use crate::co;
use crate::msg::macros::{lparam_to_ref, ref_to_lparam};
use crate::msg::{LResult, WmAny};
use crate::structs as s;

empty_msg! { LvmGetHeader, co::WM::LVM_GETHEADER,
	/// [`LVM_GETHEADER`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getheader)
	/// message, which has no parameters.
}

empty_msg! { LvmGetItemCount, co::WM::LVM_GETITEMCOUNT,
	/// [`LVM_GETITEMCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-getitemcount)
	/// message, which has no parameters.
}

/// [`LVM_INSERTCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-insertcolumn)
/// message parameters.
pub struct LvmInsertColumn<'a, 'b> {
	pub index: i32,
	pub lvcolumn: &'b s::LVCOLUMN<'a>,
}

impl<'a, 'b> From<LvmInsertColumn<'a, 'b>> for WmAny {
	fn from(p: LvmInsertColumn) -> Self {
		Self {
			msg: co::WM::LVM_INSERTCOLUMN,
			wparam: p.index as usize,
			lparam: ref_to_lparam(p.lvcolumn),
		}
	}
}

impl<'a, 'b> From<WmAny> for LvmInsertColumn<'a, 'b> {
	fn from(p: WmAny) -> Self {
		Self {
			index: p.wparam as i32,
			lvcolumn: lparam_to_ref(p),
		}
	}
}

msg_lresult_zero!(LvmInsertColumn, 'a, 'b);

/// [`LVM_INSERTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-insertitem)
/// message parameters.
pub struct LvmInsertItem<'a, 'b> {
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> From<LvmInsertItem<'a, 'b>> for WmAny {
	fn from(p: LvmInsertItem) -> Self {
		Self {
			msg: co::WM::LVM_INSERTITEM,
			wparam: 0,
			lparam: ref_to_lparam(p.lvitem),
		}
	}
}

impl<'a, 'b> From<WmAny> for LvmInsertItem<'a, 'b> {
	fn from(p: WmAny) -> Self {
		Self {
			lvitem: lparam_to_ref(p),
		}
	}
}

msg_lresult_zero!(LvmInsertItem, 'a, 'b);

/// [`LVM_ISITEMVISIBLE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-isitemvisible)
/// message parameters.
pub struct LvmIsItemVisible {
	pub index: i32,
}

impl From<LvmIsItemVisible> for WmAny {
	fn from(p: LvmIsItemVisible) -> Self {
		Self {
			msg: co::WM::LVM_ISITEMVISIBLE,
			wparam: p.index as usize,
			lparam: 0,
		}
	}
}

impl From<WmAny> for LvmIsItemVisible {
	fn from(p: WmAny) -> Self {
		Self {
			index: p.wparam as i32,
		}
	}
}

msg_lresult_zero!(LvmIsItemVisible);

/// [`LVM_SCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-scroll)
/// message parameters.
pub struct LvmScroll {
	pub horizontal: i32,
	pub vertical: i32,
}

impl From<LvmScroll> for WmAny {
	fn from(p: LvmScroll) -> Self {
		Self {
			msg: co::WM::LVM_SCROLL,
			wparam: p.horizontal as usize,
			lparam: p.vertical as isize,
		}
	}
}

impl From<WmAny> for LvmScroll {
	fn from(p: WmAny) -> Self {
		Self {
			horizontal: p.wparam as i32,
			vertical: p.lparam as i32,
		}
	}
}

msg_lresult_zero!(LvmScroll);

/// [`LVM_SETCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setcolumn)
/// message parameters.
pub struct LvmSetColumn<'a, 'b> {
	pub index: i32,
	pub lvcolumn: &'b s::LVCOLUMN<'a>,
}

impl<'a, 'b> From<LvmSetColumn<'a, 'b>> for WmAny {
	fn from(p: LvmSetColumn) -> Self {
		Self {
			msg: co::WM::LVM_SETCOLUMN,
			wparam: p.index as usize,
			lparam: ref_to_lparam(p.lvcolumn),
		}
	}
}

impl<'a, 'b> From<WmAny> for LvmSetColumn<'a, 'b> {
	fn from(p: WmAny) -> Self {
		Self {
			index: p.wparam as i32,
			lvcolumn: lparam_to_ref(p),
		}
	}
}

msg_lresult_zero!(LvmSetColumn, 'a, 'b);

/// [`LVM_SETITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitem)
/// message parameters.
pub struct LvmSetItem<'a, 'b> {
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> From<LvmSetItem<'a, 'b>> for WmAny {
	fn from(p: LvmSetItem) -> Self {
		Self {
			msg: co::WM::LVM_SETITEM,
			wparam: 0,
			lparam: ref_to_lparam(p.lvitem),
		}
	}
}

impl<'a, 'b> From<WmAny> for LvmSetItem<'a, 'b> {
	fn from(p: WmAny) -> Self {
		Self {
			lvitem: lparam_to_ref(p),
		}
	}
}

msg_lresult_zero!(LvmSetItem, 'a, 'b);

/// [`LVM_SETITEMTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitemtext)
/// message parameters.
pub struct LvmSetItemText<'a, 'b> {
	pub index: i32,
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> From<LvmSetItemText<'a, 'b>> for WmAny {
	fn from(p: LvmSetItemText) -> Self {
		Self {
			msg: co::WM::LVM_SETITEMTEXT,
			wparam: p.index as usize,
			lparam: ref_to_lparam(p.lvitem),
		}
	}
}

impl<'a, 'b> From<WmAny> for LvmSetItemText<'a, 'b> {
	fn from(p: WmAny) -> Self {
		Self {
			index: p.wparam as i32,
			lvitem: lparam_to_ref(p),
		}
	}
}

msg_lresult_zero!(LvmSetItemText, 'a, 'b);

/// [`LVM_UPDATE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-update)
/// message parameters.

pub struct LvmUpdate {
	pub index: i32,
}

impl From<LvmUpdate> for WmAny {
	fn from(p: LvmUpdate) -> Self {
		Self {
			msg: co::WM::LVM_UPDATE,
			wparam: p.index as usize,
			lparam: 0,
		}
	}
}

impl From<WmAny> for LvmUpdate {
	fn from(p: WmAny) -> Self {
		Self {
			index: p.wparam as i32,
		}
	}
}

msg_lresult_zero!(LvmUpdate);