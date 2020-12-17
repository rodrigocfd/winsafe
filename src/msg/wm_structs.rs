use std::ffi::c_void;

use crate::co;
use crate::funcs::{HIWORD, LOWORD, MAKEDWORD};
use crate::handles::HWND;
use crate::msg::WmAny;
use crate::structs::CREATESTRUCT;

/// [`WM_CLOSE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-close)
/// message parameters.
#[derive(Copy, Clone)]
pub struct WmClose {}

impl From<WmClose> for WmAny {
	fn from(_: WmClose) -> WmAny {
		WmAny {
			msg: co::WM::CLOSE,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl From<WmAny> for WmClose {
	fn from(_: WmAny) -> WmClose {
		WmClose {}
	}
}

/// [`WM_COMMAND`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-command)
/// message parameters.
#[derive(Copy, Clone)]
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

/// [`WM_CREATE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-create)
/// message parameters.
#[derive(Copy, Clone)]
pub struct WmCreate<'a> {
	pub createstruct: &'a CREATESTRUCT,
}

impl<'a> From<WmCreate<'a>> for WmAny {
	fn from(p: WmCreate) -> WmAny {
		WmAny {
			msg: co::WM::CREATE,
			wparam: 0,
			lparam: p.createstruct as *const CREATESTRUCT as isize,
		}
	}
}

impl<'a> From<WmAny> for WmCreate<'a> {
	fn from(p: WmAny) -> WmCreate<'a> {
		WmCreate {
			createstruct: unsafe { (p.lparam as *const CREATESTRUCT).as_ref() }.unwrap(),
		}
	}
}

/// [`WM_INITDIALOG`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
/// message parameters.
#[derive(Copy, Clone)]
pub struct WmInitDialog {
	pub hwnd_focus: HWND,
	pub additional_data: isize,
}

impl From<WmInitDialog> for WmAny {
	fn from(p: WmInitDialog) -> WmAny {
		WmAny {
			msg: co::WM::INITDIALOG,
			wparam: unsafe { p.hwnd_focus.as_ptr() } as usize,
			lparam: p.additional_data,
		}
	}
}

impl From<WmAny> for WmInitDialog {
	fn from(p: WmAny) -> WmInitDialog {
		WmInitDialog {
			hwnd_focus: unsafe { HWND::from_ptr(p.wparam as *mut c_void) },
			additional_data: p.lparam,
		}
	}
}