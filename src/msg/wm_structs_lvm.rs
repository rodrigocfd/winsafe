use crate::co;
use crate::msg::macros::{lparam_to_ref, ref_to_lparam};
use crate::msg::Wm;
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

impl<'a, 'b> From<LvmInsertColumn<'a, 'b>> for Wm {
	fn from(p: LvmInsertColumn) -> Self {
		Self {
			msg_id: co::WM::LVM_INSERTCOLUMN,
			wparam: p.index as usize,
			lparam: ref_to_lparam(p.lvcolumn),
		}
	}
}

impl<'a, 'b> From<Wm> for LvmInsertColumn<'a, 'b> {
	fn from(p: Wm) -> Self {
		Self {
			index: p.wparam as i32,
			lvcolumn: lparam_to_ref(p),
		}
	}
}

/// [`LVM_INSERTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-insertitem)
/// message parameters.
pub struct LvmInsertItem<'a, 'b> {
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> From<LvmInsertItem<'a, 'b>> for Wm {
	fn from(p: LvmInsertItem) -> Self {
		Self {
			msg_id: co::WM::LVM_INSERTITEM,
			wparam: 0,
			lparam: ref_to_lparam(p.lvitem),
		}
	}
}

impl<'a, 'b> From<Wm> for LvmInsertItem<'a, 'b> {
	fn from(p: Wm) -> Self {
		Self {
			lvitem: lparam_to_ref(p),
		}
	}
}

/// [`LVM_ISITEMVISIBLE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-isitemvisible)
/// message parameters.
pub struct LvmIsItemVisible {
	pub index: i32,
}

impl From<LvmIsItemVisible> for Wm {
	fn from(p: LvmIsItemVisible) -> Self {
		Self {
			msg_id: co::WM::LVM_ISITEMVISIBLE,
			wparam: p.index as usize,
			lparam: 0,
		}
	}
}

impl From<Wm> for LvmIsItemVisible {
	fn from(p: Wm) -> Self {
		Self {
			index: p.wparam as i32,
		}
	}
}

/// [`LVM_SCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-scroll)
/// message parameters.
pub struct LvmScroll {
	pub horizontal: i32,
	pub vertical: i32,
}

impl From<LvmScroll> for Wm {
	fn from(p: LvmScroll) -> Self {
		Self {
			msg_id: co::WM::LVM_SCROLL,
			wparam: p.horizontal as usize,
			lparam: p.vertical as isize,
		}
	}
}

impl From<Wm> for LvmScroll {
	fn from(p: Wm) -> Self {
		Self {
			horizontal: p.wparam as i32,
			vertical: p.lparam as i32,
		}
	}
}

/// [`LVM_SETCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setcolumn)
/// message parameters.
pub struct LvmSetColumn<'a, 'b> {
	pub index: i32,
	pub lvcolumn: &'b s::LVCOLUMN<'a>,
}

impl<'a, 'b> From<LvmSetColumn<'a, 'b>> for Wm {
	fn from(p: LvmSetColumn) -> Self {
		Self {
			msg_id: co::WM::LVM_SETCOLUMN,
			wparam: p.index as usize,
			lparam: ref_to_lparam(p.lvcolumn),
		}
	}
}

impl<'a, 'b> From<Wm> for LvmSetColumn<'a, 'b> {
	fn from(p: Wm) -> Self {
		Self {
			index: p.wparam as i32,
			lvcolumn: lparam_to_ref(p),
		}
	}
}

/// [`LVM_SETITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitem)
/// message parameters.
pub struct LvmSetItem<'a, 'b> {
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> From<LvmSetItem<'a, 'b>> for Wm {
	fn from(p: LvmSetItem) -> Self {
		Self {
			msg_id: co::WM::LVM_SETITEM,
			wparam: 0,
			lparam: ref_to_lparam(p.lvitem),
		}
	}
}

impl<'a, 'b> From<Wm> for LvmSetItem<'a, 'b> {
	fn from(p: Wm) -> Self {
		Self {
			lvitem: lparam_to_ref(p),
		}
	}
}

/// [`LVM_SETITEMTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-setitemtext)
/// message parameters.
pub struct LvmSetItemText<'a, 'b> {
	pub index: i32,
	pub lvitem: &'b s::LVITEM<'a>,
}

impl<'a, 'b> From<LvmSetItemText<'a, 'b>> for Wm {
	fn from(p: LvmSetItemText) -> Self {
		Self {
			msg_id: co::WM::LVM_SETITEMTEXT,
			wparam: p.index as usize,
			lparam: ref_to_lparam(p.lvitem),
		}
	}
}

impl<'a, 'b> From<Wm> for LvmSetItemText<'a, 'b> {
	fn from(p: Wm) -> Self {
		Self {
			index: p.wparam as i32,
			lvitem: lparam_to_ref(p),
		}
	}
}

/// [`LVM_UPDATE`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-update)
/// message parameters.
pub struct LvmUpdate {
	pub index: i32,
}

impl From<LvmUpdate> for Wm {
	fn from(p: LvmUpdate) -> Self {
		Self {
			msg_id: co::WM::LVM_UPDATE,
			wparam: p.index as usize,
			lparam: 0,
		}
	}
}

impl From<Wm> for LvmUpdate {
	fn from(p: Wm) -> Self {
		Self {
			index: p.wparam as i32,
		}
	}
}