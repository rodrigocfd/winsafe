use crate::co;
use crate::msg::WndMsg;
use crate::prelude::{MsgSend, MsgSendRecv};
use crate::shell::decl::HDROP;

/// [`WM_DROPFILES`](https://docs.microsoft.com/en-us/windows/win32/shell/wm-dropfiles)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
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
			wparam: self.hdrop.0 as _,
			lparam: 0,
		}
	}
}

unsafe impl MsgSendRecv for DropFiles {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hdrop: HDROP(p.wparam as _),
		}
	}
}
