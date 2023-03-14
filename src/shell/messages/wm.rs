use crate::co;
use crate::msg::WndMsg;
use crate::prelude::{Handle, MsgSend, MsgSendRecv};
use crate::shell::decl::HDROP;

/// [`WM_DROPFILES`](https://learn.microsoft.com/en-us/windows/win32/shell/wm-dropfiles)
/// message parameters.
///
/// Return type: `()`.
pub struct DropFiles {
	pub hdrop: HDROP,
}

unsafe impl MsgSend for DropFiles {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::DROPFILES,
			wparam: self.hdrop.as_ptr() as _,
			lparam: 0,
		}
	}
}

unsafe impl MsgSendRecv for DropFiles {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hdrop: unsafe { HDROP::from_ptr(p.wparam as _) },
		}
	}
}
