use crate::co;
use crate::msg::macros::{lparam_to_mut_ref, ref_to_lparam};
use crate::msg::WmAny;
use crate::structs::LVITEM;

/// [`LVM_INSERTITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/lvm-insertitem)
/// message parameters.
pub struct LvmInsertItem<'a, 'b> {
	pub lvitem: &'b mut LVITEM<'a>,
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
			lvitem: lparam_to_mut_ref(p),
		}
	}
}