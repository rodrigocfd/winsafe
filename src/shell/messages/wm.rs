use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::msg::*;
use crate::prelude::*;

/// [`WM_DROPFILES`](https://learn.microsoft.com/en-us/windows/win32/shell/wm-dropfiles)
/// message parameters.
///
/// Return type: `()`.
pub struct DropFiles {
	pub hdrop: DragFinishGuard,
}

impl MsgSend for DropFiles {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::DROPFILES,
			wparam: self.hdrop.ptr() as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for DropFiles {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hdrop: DragFinishGuard::new(HDROP::from_ptr(p.wparam as _)),
		}
	}
}
