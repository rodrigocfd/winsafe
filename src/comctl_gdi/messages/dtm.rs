use crate::co;
use crate::gdi::decl::HFONT;
use crate::kernel::decl::SysResult;
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::privs::zero_as_err;

/// [`DTM_GETMCFONT`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getmcfont)
/// message, which has no parameters.
///
/// Return type: `SysResult<HFONT>`.
pub struct GetMcFont {}

unsafe impl MsgSend for GetMcFont {
	type RetType = SysResult<HFONT>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HFONT(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETMCFONT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`DTM_SETMCFONT`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-setmcfont)
/// message parameters.
///
/// Return type: `()`.
pub struct SetMcFont<'a> {
	pub hfont: &'a HFONT,
	pub redraw: bool,
}

unsafe impl<'a> MsgSend for SetMcFont<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETMCFONT.into(),
			wparam: self.hfont.0 as _,
			lparam: self.redraw as _,
		}
	}
}
