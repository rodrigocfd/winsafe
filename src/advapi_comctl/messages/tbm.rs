use crate::advapi_comctl::decl::TBSAVEPARAMS;
use crate::co;
use crate::msg::WndMsg;
use crate::prelude::MsgSend;

/// [`TB_SAVERESTORE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-saverestore)
/// message parameters.
///
/// Return type: `()`.
pub struct SaveRestore<'a, 'b> {
	pub save: bool,
	pub info: &'b mut TBSAVEPARAMS<'a>,
}

unsafe impl<'a, 'b> MsgSend for SaveRestore<'a, 'b> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
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
