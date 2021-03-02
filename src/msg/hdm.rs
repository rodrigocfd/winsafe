//! Header control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-header-control-reference-messages),
//! whose constants have [`HDM`](crate::co::HDM) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::msg::{Message, wm::Wm};

/// [`HDM_GETITEMCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-getitemcount)
/// message, which has no parameters.
///
/// Return type: `WinResult<u32>`.
pub struct GetItemCount {}

impl Message for GetItemCount {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::HDM::GETITEMCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}
