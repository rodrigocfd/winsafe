use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;

/// [`TB_SAVERESTORE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-saverestore)
/// message parameters.
///
/// Return type: `()`.
pub struct SaveRestore<'a, 'b> {
	pub save: bool,
	pub info: &'b mut TBSAVEPARAMS<'a>,
}

impl<'a, 'b> MsgSend for SaveRestore<'a, 'b> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SAVERESTORE.into(),
			wparam: self.save as _,
			lparam: self.info as *const _ as _,
		}
	}
}
