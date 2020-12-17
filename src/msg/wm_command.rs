use std::ffi::c_void;

use crate::co;
use crate::funcs::{HIWORD, LOWORD, MAKEDWORD};
use crate::handles::HWND;
use crate::msg::WmAny;

/// [`WM_COMMAND`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-command)
/// message parameters.
pub struct WmCommand {
	pub code: co::CMD,
	pub ctrl_id: u16,
	pub ctrl_hwnd: HWND,
}

impl From<WmCommand> for WmAny {
	fn from(p: WmCommand) -> WmAny {
		WmAny {
			msg: co::WM::COMMAND,
			wparam: MAKEDWORD(p.ctrl_id, p.code.into()) as usize,
			lparam: unsafe { p.ctrl_hwnd.as_ptr() } as isize,
		}
	}
}

impl From<WmAny> for WmCommand {
	fn from(p: WmAny) -> WmCommand {
		WmCommand {
			code: co::CMD::from(HIWORD(p.wparam as u32)),
			ctrl_id: LOWORD(p.wparam as u32),
			ctrl_hwnd: unsafe { HWND::from_ptr(p.lparam as *mut c_void) },
		}
	}
}