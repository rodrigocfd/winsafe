use crate::aliases::WinResult;
use crate::co;
use crate::handles::HICON;
use crate::msg::{Message, Wm};

/// [`STM_GETICON`](https://docs.microsoft.com/en-us/windows/win32/controls/stm-geticon)
/// message, which has no parameters.
///
/// Return type: `WinResult<HICON>`.
pub struct StmGetIcon {}

impl Message for StmGetIcon {
	type RetType = WinResult<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			p => Ok(HICON { ptr: p as *mut _ }),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::STM_GETICON,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`STM_SETICON`](https://docs.microsoft.com/en-us/windows/win32/controls/stm-seticon)
/// message parameters.
///
/// Return type: `WinResult<HICON>`.
pub struct StmSetIcon {
	pub icon: HICON,
}

impl Message for StmSetIcon {
	type RetType = WinResult<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			p => Ok(HICON { ptr: p as *mut _ }),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::STM_SETICON,
			wparam: self.icon.ptr as usize,
			lparam: 0,
		}
	}
}
