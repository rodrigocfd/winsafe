use std::ffi::c_void;

use crate::aliases::TIMERPROC;
use crate::co;
use crate::funcs::{HIWORD, LOWORD, MAKEDWORD};
use crate::handles::{HDC, HDROP, HMENU, HRGN, HWND};
use crate::msg::macros::{lparam_to_mut_ref, lparam_to_ref, ref_to_lparam};
use crate::priv_funcs::FAPPCOMMAND_MASK;
use crate::structs::{CREATESTRUCT, NMHDR, RECT};

/// Generic
/// [window message](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues)
/// parameters.
#[derive(Copy, Clone)]
pub struct Wm {
	/// The [`co::WM`](crate::co::WM) constant that identifies the window message.
	pub msg_id: co::WM,
	/// First message parameter.
	pub wparam: usize,
	/// Second message parameter.
	pub lparam: isize,
}

//------------------------------------------------------------------------------

/// [`WM_ACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-activate)
/// message parameters.
pub struct WmActivate {
	pub event: co::WA,
	pub is_minimized: bool,
	pub hwnd: HWND,
}

impl From<WmActivate> for Wm {
	fn from(p: WmActivate) -> Self {
		Self {
			msg_id: co::WM::ACTIVATE,
			wparam: MAKEDWORD(u16::from(p.event), p.is_minimized as u16) as usize,
			lparam: unsafe { p.hwnd.as_ptr() } as isize,
		}
	}
}

impl From<Wm> for WmActivate {
	fn from(p: Wm) -> Self {
		Self {
			event: co::WA::from(LOWORD(p.wparam as u32)),
			is_minimized: HIWORD(p.wparam as u32) != 0,
			hwnd: unsafe { HWND::from_ptr(p.lparam as *mut c_void) },
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_ACTIVATEAPP`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-activateapp)
/// message parameters.
pub struct WmActivateApp {
	pub is_being_activated: bool,
	pub thread_id: u32,
}

impl From<WmActivateApp> for Wm {
	fn from(p: WmActivateApp) -> Self {
		Self {
			msg_id: co::WM::ACTIVATEAPP,
			wparam: p.is_being_activated as usize,
			lparam: p.thread_id as isize,
		}
	}
}

impl From<Wm> for WmActivateApp {
	fn from(p: Wm) -> Self {
		Self {
			is_being_activated: p.wparam != 0,
			thread_id: p.lparam as u32,
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_APPCOMMAND`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-appcommand)
/// message parameters.
pub struct WmAppCommand {
	pub hwnd_owner: HWND,
	pub app_command: co::APPCOMMAND,
	pub u_device: co::FAPPCOMMAND,
	pub keys: co::MK,
}

impl From<WmAppCommand> for Wm {
	fn from(p: WmAppCommand) -> Self {
		Self {
			msg_id: co::WM::APPCOMMAND,
			wparam: unsafe { p.hwnd_owner.as_ptr() } as usize,
			lparam: MAKEDWORD(p.keys.into(), u16::from(p.app_command) | u16::from(p.u_device)) as isize,
		}
	}
}

impl From<Wm> for WmAppCommand {
	fn from(p: Wm) -> Self {
		Self {
			hwnd_owner: unsafe { HWND::from_ptr(p.wparam as *mut c_void) },
			app_command: co::APPCOMMAND::from(HIWORD(p.lparam as u32) & !FAPPCOMMAND_MASK),
			u_device: co::FAPPCOMMAND::from(HIWORD(p.lparam as u32) & FAPPCOMMAND_MASK),
			keys: co::MK::from(LOWORD(p.lparam as u32)),
		}
	}
}

//------------------------------------------------------------------------------

empty_msg! { WmClose, co::WM::CLOSE,
	/// [`WM_CLOSE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-close)
	/// message, which has no parameters.
}

//------------------------------------------------------------------------------

/// [`WM_COMMAND`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-command)
/// message parameters.
///
/// You'll normally want to match against `code` and `ctrl_id` to identify the
/// event.
pub struct WmCommand {
	pub code: co::CMD,
	pub ctrl_id: u16,
	pub ctrl_hwnd: HWND,
}

impl From<WmCommand> for Wm {
	fn from(p: WmCommand) -> Self {
		Self {
			msg_id: co::WM::COMMAND,
			wparam: MAKEDWORD(p.ctrl_id, p.code.into()) as usize,
			lparam: unsafe { p.ctrl_hwnd.as_ptr() } as isize,
		}
	}
}

impl From<Wm> for WmCommand {
	fn from(p: Wm) -> Self {
		Self {
			code: co::CMD::from(HIWORD(p.wparam as u32)),
			ctrl_id: LOWORD(p.wparam as u32),
			ctrl_hwnd: unsafe { HWND::from_ptr(p.lparam as *mut c_void) },
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_CREATE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-create)
/// message parameters.
pub struct WmCreate<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

impl<'a, 'b, 'c> From<WmCreate<'a, 'b, 'c>> for Wm {
	fn from(p: WmCreate) -> Self {
		Self {
			msg_id: co::WM::CREATE,
			wparam: 0,
			lparam: ref_to_lparam(p.createstruct),
		}
	}
}

impl<'a, 'b, 'c> From<Wm> for WmCreate<'a, 'b, 'c> {
	fn from(p: Wm) -> Self {
		Self {
			createstruct: lparam_to_ref(p),
		}
	}
}

//------------------------------------------------------------------------------

ctl_color_msg! { WmCtlColorBtn, co::WM::CTLCOLORBTN,
	/// [`WM_CTLCOLORBTN`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorbtn)
	/// message parameters.
}

ctl_color_msg! { WmCtlColorDlg, co::WM::CTLCOLORDLG,
	/// [`WM_CTLCOLORDLG`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-ctlcolordlg)
	/// message parameters.
}

ctl_color_msg! { WmCtlColorEdit, co::WM::CTLCOLOREDIT,
	/// [`WM_CTLCOLOREDIT`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcoloredit)
	/// message parameters.
}

ctl_color_msg! { WmCtlColorListBox, co::WM::CTLCOLORLISTBOX,
	/// [`WM_CTLCOLORLISTBOX`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorlistbox)
	/// message parameters.
}

ctl_color_msg! { WmCtlColorScrollBar, co::WM::CTLCOLORSCROLLBAR,
	/// [`WM_CTLCOLORSCROLLBAR`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorscrollbar)
	/// message parameters.
}

ctl_color_msg! { WmCtlColorStatic, co::WM::CTLCOLORSTATIC,
	/// [`WM_CTLCOLORSTATIC`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorstatic)
	/// message parameters.
}

//------------------------------------------------------------------------------

empty_msg! { WmDestroy, co::WM::DESTROY,
	/// [`WM_DESTROY`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-destroy)
	/// message, which has no parameters.
}

//------------------------------------------------------------------------------

/// [`WM_DROPFILES`](https://docs.microsoft.com/en-us/windows/win32/shell/wm-dropfiles)
/// message parameters.
pub struct WmDropFiles {
	pub hdrop: HDROP,
}

impl From<WmDropFiles> for Wm {
	fn from(p: WmDropFiles) -> Self {
		Self {
			msg_id: co::WM::DROPFILES,
			wparam: unsafe { p.hdrop.as_ptr() } as usize,
			lparam: 0,
		}
	}
}

impl From<Wm> for WmDropFiles {
	fn from(p: Wm) -> Self {
		Self {
			hdrop: unsafe { HDROP::from_ptr(p.wparam as *mut c_void) },
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_ENDSESSION`](https://docs.microsoft.com/en-us/windows/win32/shutdown/wm-endsession)
/// message parameters.
pub struct WmEndSession {
	pub is_session_being_ended: bool,
	pub event: co::ENDSESSION,
}

impl From<WmEndSession> for Wm {
	fn from(p: WmEndSession) -> Self {
		Self {
			msg_id: co::WM::ENDSESSION,
			wparam: p.is_session_being_ended as usize,
			lparam: u32::from(p.event) as isize,
		}
	}
}

impl From<Wm> for WmEndSession {
	fn from(p: Wm) -> Self {
		Self {
			is_session_being_ended: p.wparam != 0,
			event: co::ENDSESSION::from(p.lparam as u32),
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_INITDIALOG`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
/// message parameters.
pub struct WmInitDialog {
	pub hwnd_focus: HWND,
	pub additional_data: isize,
}

impl From<WmInitDialog> for Wm {
	fn from(p: WmInitDialog) -> Self {
		Self {
			msg_id: co::WM::INITDIALOG,
			wparam: unsafe { p.hwnd_focus.as_ptr() } as usize,
			lparam: p.additional_data,
		}
	}
}

impl From<Wm> for WmInitDialog {
	fn from(p: Wm) -> Self {
		Self {
			hwnd_focus: unsafe { HWND::from_ptr(p.wparam as *mut c_void) },
			additional_data: p.lparam,
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_INITMENUPOPUP`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-initmenupopup)
/// message parameters.
pub struct WmInitMenuPopup {
	pub hmenu: HMENU,
	pub item_pos: u16,
	pub is_window_menu: bool,
}

impl From<WmInitMenuPopup> for Wm {
	fn from(p: WmInitMenuPopup) -> Self {
		Self {
			msg_id: co::WM::INITMENUPOPUP,
			wparam: unsafe { p.hmenu.as_ptr() } as usize,
			lparam: MAKEDWORD(p.item_pos, p.is_window_menu as u16) as isize,
		}
	}
}

impl From<Wm> for WmInitMenuPopup {
	fn from(p: Wm) -> Self {
		Self {
			hmenu: unsafe { HMENU::from_ptr(p.wparam as *mut c_void) },
			item_pos: LOWORD(p.lparam as u32),
			is_window_menu: HIWORD(p.lparam as u32) != 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_NCCREATE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-nccreate)
/// message parameters.
pub struct WmNcCreate<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

impl<'a, 'b, 'c> From<WmNcCreate<'a, 'b, 'c>> for Wm {
	fn from(p: WmNcCreate) -> Self {
		Self {
			msg_id: co::WM::NCCREATE,
			wparam: 0,
			lparam: ref_to_lparam(p.createstruct),
		}
	}
}

impl<'a, 'b, 'c> From<Wm> for WmNcCreate<'a, 'b, 'c> {
	fn from(p: Wm) -> Self {
		Self {
			createstruct: lparam_to_ref(p),
		}
	}
}

//------------------------------------------------------------------------------

empty_msg! { WmNcDestroy, co::WM::NCDESTROY,
	/// [`WM_NCDESTROY`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-ncdestroy)
	/// message, which has no parameters.
}

//------------------------------------------------------------------------------

/// [`WM_NCPAINT`](https://docs.microsoft.com/en-us/windows/win32/gdi/wm-ncpaint)
/// message parameters.
pub struct WmNcPaint {
	pub updated_hrgn: HRGN,
}

impl From<WmNcPaint> for Wm {
	fn from(p: WmNcPaint) -> Self {
		Self {
			msg_id: co::WM::NCPAINT,
			wparam: unsafe { p.updated_hrgn.as_ptr() } as usize,
			lparam: 0,
		}
	}
}

impl From<Wm> for WmNcPaint {
	fn from(p: Wm) -> Self {
		Self {
			updated_hrgn: unsafe { HRGN::from_ptr(p.wparam as *mut c_void) },
		}
	}
}

//------------------------------------------------------------------------------

empty_msg! { WmNull, co::WM::NULL,
	/// [`WM_NULL`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-null)
	/// message, which has no parameters.
}

//------------------------------------------------------------------------------

/// [`WM_NOTIFY`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify)
/// message parameters.
#[derive(Copy, Clone)]
pub struct WmNotify<'a> {
	pub nmhdr: &'a NMHDR,
}

impl<'a> From<WmNotify<'a>> for Wm {
	fn from(p: WmNotify) -> Self {
		Self {
			msg_id: co::WM::NOTIFY,
			wparam: unsafe { p.nmhdr.hwndFrom.as_ptr() } as usize,
			lparam: p.nmhdr as *const NMHDR as isize,
		}
	}
}

impl<'a> From<Wm> for WmNotify<'a> {
	fn from(p: Wm) -> Self {
		Self {
			nmhdr: unsafe { &*(p.lparam as *const NMHDR) },
		}
	}
}

impl<'a> WmNotify<'a> {
	/// Casts the `NMHDR` reference into a derived struct.
	///
	/// You should always prefer the specific notification handlers, which
	/// perform this conversion for you.
	pub unsafe fn cast_nmhdr<T>(&self) -> &T {
		&*(self.nmhdr as *const NMHDR as *const T)
	}
}

//------------------------------------------------------------------------------

empty_msg! { WmPaint, co::WM::PAINT,
	/// [`WM_PAINT`](https://docs.microsoft.com/en-us/windows/win32/gdi/wm-paint)
	/// message, which has no parameters.
}

//------------------------------------------------------------------------------

/// [`WM_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-setfocus)
/// message parameters.
pub struct WmSetFocus {
	pub hwnd_losing_focus: HWND,
}

impl From<WmSetFocus> for Wm {
	fn from(p: WmSetFocus) -> Self {
		Self {
			msg_id: co::WM::SETFOCUS,
			wparam: unsafe { p.hwnd_losing_focus.as_ptr() } as usize,
			lparam: 0,
		}
	}
}

impl From<Wm> for WmSetFocus {
	fn from(p: Wm) -> Self {
		Self {
			hwnd_losing_focus: unsafe { HWND::from_ptr(p.wparam as *mut c_void) },
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_SIZE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-size)
/// message parameters.
pub struct WmSize {
	pub request: co::SIZE,
	pub width: u16,
	pub height: u16,
}

impl From<WmSize> for Wm {
	fn from(p: WmSize) -> Self {
		Self {
			msg_id: co::WM::SIZE,
			wparam: i32::from(p.request) as usize,
			lparam: MAKEDWORD(p.width, p.height) as isize,
		}
	}
}

impl From<Wm> for WmSize {
	fn from(p: Wm) -> Self {
		Self {
			request: co::SIZE::from(p.wparam as i32),
			width: LOWORD(p.lparam as u32),
			height: HIWORD(p.lparam as u32),
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_SIZING`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-sizing)
/// message parameters.
pub struct WmSizing<'a> {
	pub window_edge: co::WMSZ,
	pub coords: &'a mut RECT,
}

impl<'a> From<WmSizing<'a>> for Wm {
	fn from(p: WmSizing) -> Self {
		Self {
			msg_id: co::WM::SIZING,
			wparam: i32::from(p.window_edge) as usize,
			lparam: ref_to_lparam(p.coords),
		}
	}
}

impl<'a> From<Wm> for WmSizing<'a> {
	fn from(p: Wm) -> Self {
		Self {
			window_edge: co::WMSZ::from(p.wparam as i32),
			coords: lparam_to_mut_ref(p),
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_TIMER`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-timer)
/// message parameters.
pub struct WmTimer {
	pub timer_id: u32,
	pub timer_proc: Option<TIMERPROC>,
}

impl From<WmTimer> for Wm {
	fn from(p: WmTimer) -> Self {
		Self {
			msg_id: co::WM::TIMER,
			wparam: p.timer_id as usize,
			lparam: match p.timer_proc {
				Some(proc) => proc as isize,
				None => 0,
			},
		}
	}
}

impl From<Wm> for WmTimer {
	fn from(p: Wm) -> Self {
		Self {
			timer_id: p.wparam as u32,
			timer_proc: match p.lparam {
				0 => None,
				addr => unsafe { std::mem::transmute(addr) },
			}
		}
	}
}