use crate::aliases::WinResult;
use crate::gui::base::Base;
use crate::gui::resizer::{Horz, Vert};
use crate::handles::HWND;

pub trait SealedBase {
	fn as_base(&self) -> &Base;
}

pub trait SealedParent {
	fn add_to_resizer(&self,
		hchild: HWND, horz: Horz, vert: Vert) -> WinResult<()>;
}
