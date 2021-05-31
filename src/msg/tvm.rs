//! Tree view control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-tree-view-control-reference-messages),
//! whose constants have [`LVM`](crate::co::TVM) prefix.

use crate::co;
use crate::aliases::WinResult;
use crate::msg::{MsgSend, WndMsg};

/// [`TVM_SETEXTENDEDSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/tvm-setextendedstyle)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetExtendedStyle {
	pub style: co::TVS_EX,
	pub mask: co::TVS_EX,
}

impl MsgSend for SetExtendedStyle {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match co::ERROR(v as _) {
			co::ERROR::S_OK => Ok(()),
			err => Err(err),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETEXTENDEDSTYLE.into(),
			wparam: self.style.0 as _,
			lparam: self.mask.0 as _,
		}
	}
}
