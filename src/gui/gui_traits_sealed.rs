use crate::gui::base::Base;
use crate::gui::resizer::{Horz, Vert};
use crate::kernel::decl::WinResult;
use crate::user::decl::HWND;

pub trait GuiSealedBase {
	fn as_base(&self) -> &Base;
}

pub trait GuiSealedParent {
	fn add_to_resizer(&self,
		hchild: HWND, horz: Horz, vert: Vert) -> WinResult<()>;
}
