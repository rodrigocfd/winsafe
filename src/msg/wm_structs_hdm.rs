use crate::aliases::WinResult;
use crate::co;
use crate::msg::{Message, Wm};

/// [`HDM_GETITEMCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/hdm-getitemcount)
/// message, which has no parameters.
pub struct HdmGetItemCount {}

impl Message for HdmGetItemCount {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::HDM_GETITEMCOUNT,
			wparam: 0,
			lparam: 0,
		}
	}
}
