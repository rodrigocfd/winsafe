//! Static control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-static-control-reference-messages),
//! whose constants have [`STM`](crate::co::STM) prefix.

use crate::co;
use crate::kernel::decl::WinResult;
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::decl::HICON;
use crate::user::privs::zero_as_err;

/// [`STM_GETICON`](https://docs.microsoft.com/en-us/windows/win32/controls/stm-geticon)
/// message, which has no parameters.
///
/// Return type: `WinResult<HICON>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetIcon {}

unsafe impl MsgSend for GetIcon {
	type RetType = WinResult<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HICON(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::STM::GETICON.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`STM_SETICON`](https://docs.microsoft.com/en-us/windows/win32/controls/stm-seticon)
/// message parameters.
///
/// Return type: `WinResult<HICON>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetIcon {
	pub icon: HICON,
}

unsafe impl MsgSend for SetIcon {
	type RetType = WinResult<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HICON(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::STM::SETICON.into(),
			wparam: self.icon.0 as _,
			lparam: 0,
		}
	}
}
