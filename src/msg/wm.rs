//! Generic window
//! [messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues),
//! whose constants have `WM` prefix.

use crate::aliases::TIMERPROC;
use crate::co;
use crate::enums::{HwndHmenu, NccspRect, WsWsex};
use crate::funcs::{HIWORD, LOWORD, MAKEDWORD};
use crate::handles::{HBRUSH, HDC, HDROP, HFONT, HICON, HMENU, HRGN, HWND};
use crate::msg::{Message, MessageHandleable};
use crate::msg::macros::{lp_to_point, point_to_lp};
use crate::privs::FAPPCOMMAND_MASK;
use crate::structs::{
	CREATESTRUCT,
	MINMAXINFO,
	NMHDR,
	POINT,
	RECT,
	SIZE,
	STYLESTRUCT_WS_EX,
	STYLESTRUCT_WS,
	WINDOWPOS,
};

/// Generic
/// [window message](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues)
/// parameters.
///
/// Return type: `isize`.
#[derive(Copy, Clone)]
pub struct Wm {
	/// The [`co::WM`](crate::co::WM) constant that identifies the window message.
	pub msg_id: co::WM,
	/// First message parameter.
	pub wparam: usize,
	/// Second message parameter.
	pub lparam: isize,
}

impl Message for Wm {
	type RetType = isize;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v
	}

	fn as_generic_wm(&self) -> Wm {
		*self
	}
}

impl MessageHandleable for Wm {
	fn from_generic_wm(p: Wm) -> Self {
		p
	}
}

//------------------------------------------------------------------------------

/// [`WM_ACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-activate)
/// message parameters.
///
/// Return type: `()`.
pub struct Activate {
	pub event: co::WA,
	pub is_minimized: bool,
	pub hwnd: HWND,
}

impl Message for Activate {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::ACTIVATE,
			wparam: MAKEDWORD(self.event.0, self.is_minimized as u16) as usize,
			lparam: self.hwnd.ptr as isize,
		}
	}
}

impl MessageHandleable for Activate {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			event: co::WA(LOWORD(p.wparam as u32)),
			is_minimized: HIWORD(p.wparam as u32) != 0,
			hwnd: HWND { ptr: p.lparam as *mut _ },
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_ACTIVATEAPP`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-activateapp)
/// message parameters.
///
/// Return type: `()`.
pub struct ActivateApp {
	pub is_being_activated: bool,
	pub thread_id: u32,
}

impl Message for ActivateApp {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::ACTIVATEAPP,
			wparam: self.is_being_activated as usize,
			lparam: self.thread_id as isize,
		}
	}
}

impl MessageHandleable for ActivateApp {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			is_being_activated: p.wparam != 0,
			thread_id: p.lparam as u32,
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_APPCOMMAND`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-appcommand)
/// message parameters.
///
/// Return type: `()`.
pub struct AppCommand {
	pub hwnd_owner: HWND,
	pub app_command: co::APPCOMMAND,
	pub u_device: co::FAPPCOMMAND,
	pub keys: co::MK,
}

impl Message for AppCommand {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::APPCOMMAND,
			wparam: self.hwnd_owner.ptr as usize,
			lparam: MAKEDWORD(self.keys.into(), self.app_command.0 | self.u_device.0) as isize,
		}
	}
}

impl MessageHandleable for AppCommand {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			hwnd_owner: HWND { ptr: p.wparam as *mut _ },
			app_command: co::APPCOMMAND(HIWORD(p.lparam as u32) & !FAPPCOMMAND_MASK),
			u_device: co::FAPPCOMMAND(HIWORD(p.lparam as u32) & FAPPCOMMAND_MASK),
			keys: co::MK(LOWORD(p.lparam as u32)),
		}
	}
}

//------------------------------------------------------------------------------

empty_msg_handleable! { CancelMode, co::WM::CANCELMODE,
	/// [`WM_CANCELMODE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-cancelmode)
	/// message, which has no parameters.
	///
	/// Return type: `()`.
}

empty_msg_handleable! { ChildActivate, co::WM::CHILDACTIVATE,
	/// [`WM_CHILDACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-childactivate)
	/// message, which has no parameters.
	///
	/// Return type: `()`.
}

empty_msg_handleable! { Close, co::WM::CLOSE,
	/// [`WM_CLOSE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-close)
	/// message, which has no parameters.
	///
	/// Return type: `()`.
}

//------------------------------------------------------------------------------

/// [`WM_COMMAND`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-command)
/// message parameters.
///
/// Return type: `()`.
///
/// You'll normally want to match against `code` and `ctrl_id` to identify the
/// event.
pub struct Command {
	pub code: co::CMD,
	pub ctrl_id: u16,
	pub ctrl_hwnd: Option<HWND>,
}

impl Message for Command {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::COMMAND,
			wparam: MAKEDWORD(self.ctrl_id, self.code.into()) as usize,
			lparam: match self.ctrl_hwnd {
				Some(h) => h.ptr as isize,
				None => 0,
			},
		}
	}
}

impl MessageHandleable for Command {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			code: co::CMD(HIWORD(p.wparam as u32)),
			ctrl_id: LOWORD(p.wparam as u32),
			ctrl_hwnd: match p.lparam {
				0 => None,
				ptr => Some(HWND { ptr: ptr as *mut _ }),
			},
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_CONTEXTMENU`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-contextmenu)
/// message parameters.
///
/// Return type: `()`.
pub struct ContextMenu {
	pub hwnd: HWND,
	pub cursor_pos: POINT,
}

impl Message for ContextMenu {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CONTEXTMENU,
			wparam: self.hwnd.ptr as usize,
			lparam: point_to_lp(self.cursor_pos),
		}
	}
}

impl MessageHandleable for ContextMenu {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			hwnd: HWND { ptr: p.wparam as *mut _ },
			cursor_pos: lp_to_point(p),
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_CREATE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-create)
/// message parameters.
///
/// Return type: `i32`.
pub struct Create<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

impl<'a, 'b, 'c> Message for Create<'a, 'b, 'c> {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as i32
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CREATE,
			wparam: 0,
			lparam: self.createstruct as *const _ as isize,
		}
	}
}

impl<'a, 'b, 'c> MessageHandleable for Create<'a, 'b, 'c> {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			createstruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

//------------------------------------------------------------------------------

ctl_color_msg! { CtlColorBtn, co::WM::CTLCOLORBTN,
	/// [`WM_CTLCOLORBTN`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorbtn)
	/// message parameters.
	///
	/// Return type: `HBRUSH`.
}

ctl_color_msg! { CtlColorDlg, co::WM::CTLCOLORDLG,
	/// [`WM_CTLCOLORDLG`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-ctlcolordlg)
	/// message parameters.
	///
	/// Return type: `HBRUSH`.
}

ctl_color_msg! { CtlColorEdit, co::WM::CTLCOLOREDIT,
	/// [`WM_CTLCOLOREDIT`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcoloredit)
	/// message parameters.
	///
	/// Return type: `HBRUSH`.
}

ctl_color_msg! { CtlColorListBox, co::WM::CTLCOLORLISTBOX,
	/// [`WM_CTLCOLORLISTBOX`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorlistbox)
	/// message parameters.
	///
	/// Return type: `HBRUSH`.
}

ctl_color_msg! { CtlColorScrollBar, co::WM::CTLCOLORSCROLLBAR,
	/// [`WM_CTLCOLORSCROLLBAR`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorscrollbar)
	/// message parameters.
	///
	/// Return type: `HBRUSH`.
}

ctl_color_msg! { CtlColorStatic, co::WM::CTLCOLORSTATIC,
	/// [`WM_CTLCOLORSTATIC`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorstatic)
	/// message parameters.
	///
	/// Return type: `HBRUSH`.
}

//------------------------------------------------------------------------------

empty_msg_handleable! { Destroy, co::WM::DESTROY,
	/// [`WM_DESTROY`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-destroy)
	/// message, which has no parameters.
	///
	/// Return type: `()`.
}

//------------------------------------------------------------------------------

/// [`WM_DROPFILES`](https://docs.microsoft.com/en-us/windows/win32/shell/wm-dropfiles)
/// message parameters.
///
/// Return type: `()`.
pub struct DropFiles {
	pub hdrop: HDROP,
}

impl Message for DropFiles {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::DROPFILES,
			wparam: self.hdrop.ptr as usize,
			lparam: 0,
		}
	}
}

impl MessageHandleable for DropFiles {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			hdrop: HDROP { ptr: p.wparam as *mut _ },
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_ENABLE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-enable)
/// message parameters.
///
/// Return type: `()`.
pub struct Enable {
	pub has_been_enabled: bool,
}

impl Message for Enable {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::ENABLE,
			wparam: self.has_been_enabled as usize,
			lparam: 0,
		}
	}
}

impl MessageHandleable for Enable {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			has_been_enabled: p.wparam != 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_ENDSESSION`](https://docs.microsoft.com/en-us/windows/win32/shutdown/wm-endsession)
/// message parameters.
///
/// Return type: `()`.
pub struct EndSession {
	pub is_session_being_ended: bool,
	pub event: co::ENDSESSION,
}

impl Message for EndSession {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::ENDSESSION,
			wparam: self.is_session_being_ended as usize,
			lparam: self.event.0 as isize,
		}
	}
}

impl MessageHandleable for EndSession {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			is_session_being_ended: p.wparam != 0,
			event: co::ENDSESSION(p.lparam as u32),
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_ENTERIDLE`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-enteridle)
/// message parameters.
///
/// Return type: `()`.
pub struct EnterIdle {
	pub reason: co::MSGF,
	pub handle: HwndHmenu,
}

impl Message for EnterIdle {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::ENTERIDLE,
			wparam: self.reason.0 as usize,
			lparam: self.handle.as_isize(),
		}
	}
}

impl MessageHandleable for EnterIdle {
	fn from_generic_wm(p: Wm) -> Self {
		let reason = co::MSGF(p.wparam as u8);
		Self {
			reason,
			handle: match reason {
				co::MSGF::DIALOGBOX => HwndHmenu::Hwnd(HWND { ptr: p.lparam as *mut _ }),
				_ => HwndHmenu::Hmenu(HMENU { ptr: p.lparam as *mut _ }),
			},
		}
	}
}

//------------------------------------------------------------------------------

empty_msg_handleable! { EnterSizeMove, co::WM::ENTERSIZEMOVE,
	/// [`WM_ENTERSIZEMOVE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-entersizemove)
	/// message, which has no parameters.
	///
	/// Return type: `()`.
}

//------------------------------------------------------------------------------

/// [`WM_ERASEBKGND`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-erasebkgnd)
/// message parameters.
///
/// Return type: `i32`.
pub struct EraseBkgnd {
	pub hdc: HDC,
}

impl Message for EraseBkgnd {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as i32
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::ERASEBKGND,
			wparam: self.hdc.ptr as usize,
			lparam: 0,
		}
	}
}

impl MessageHandleable for EraseBkgnd {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			hdc: HDC { ptr: p.wparam as *mut _ },
		}
	}
}

//------------------------------------------------------------------------------

empty_msg_handleable! { ExitSizeMove, co::WM::EXITSIZEMOVE,
	/// [`WM_EXITSIZEMOVE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-exitsizemove)
	/// message, which has no parameters.
	///
	/// Return type: `()`.
}

//------------------------------------------------------------------------------

/// [`WM_GETMINMAXINFO`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-getminmaxinfo)
/// message parameters.
///
/// Return type: `()`.
pub struct GetMinMaxInfo<'a> {
	pub info: &'a mut MINMAXINFO,
}

impl<'a> Message for GetMinMaxInfo<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::GETMINMAXINFO,
			wparam: 0,
			lparam: self.info as *const _ as isize,
		}
	}
}

impl<'a> MessageHandleable for GetMinMaxInfo<'a> {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			info: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_INITDIALOG`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
/// message parameters.
///
/// Return type: `bool`.
pub struct InitDialog {
	pub hwnd_focus: HWND,
	pub additional_data: isize,
}

impl Message for InitDialog {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::INITDIALOG,
			wparam: self.hwnd_focus.ptr as usize,
			lparam: self.additional_data,
		}
	}
}

impl MessageHandleable for InitDialog {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			hwnd_focus: HWND { ptr: p.wparam as *mut _ },
			additional_data: p.lparam,
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_INITMENUPOPUP`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-initmenupopup)
/// message parameters.
///
/// Return type: `()`.
pub struct InitMenuPopup {
	pub hmenu: HMENU,
	pub item_pos: u16,
	pub is_window_menu: bool,
}

impl Message for InitMenuPopup {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::INITMENUPOPUP,
			wparam: self.hmenu.ptr as usize,
			lparam: MAKEDWORD(self.item_pos, self.is_window_menu as u16) as isize,
		}
	}
}

impl MessageHandleable for InitMenuPopup {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			hmenu: HMENU { ptr: p.wparam as *mut _ },
			item_pos: LOWORD(p.lparam as u32),
			is_window_menu: HIWORD(p.lparam as u32) != 0,
		}
	}
}

//------------------------------------------------------------------------------

button_msg! { LButtonDblClk, co::WM::LBUTTONDBLCLK,
	/// [`WM_LBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondblclk)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { LButtonDown, co::WM::LBUTTONDOWN,
	/// [`WM_LBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { LButtonUp, co::WM::LBUTTONUP,
	/// [`WM_LBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttonup)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { MButtonDblClk, co::WM::MBUTTONDBLCLK,
	/// [`WM_MBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondblclk)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { MButtonDown, co::WM::MBUTTONDOWN,
	/// [`WM_MBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondown)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { MButtonUp, co::WM::MBUTTONUP,
	/// [`WM_MBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttonup)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { MouseHover, co::WM::MOUSEHOVER,
	/// [`WM_MOUSEHOVER`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousehover)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { MouseMove, co::WM::MOUSEMOVE,
	/// [`WM_MOUSEMOVE`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove)
	/// message parameters.
	///
	/// Return type: `()`.
}

//------------------------------------------------------------------------------

/// [`WM_MOVE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-move)
/// message parameters.
///
/// Return type: `()`.
pub struct Move {
	pub coords: POINT,
}

impl Message for Move {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::MOVE,
			wparam: 0,
			lparam: point_to_lp(self.coords),
		}
	}
}

impl MessageHandleable for Move {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			coords: lp_to_point(p),
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_MOVING`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-moving)
/// message parameters.
///
/// Return type: `()`.
pub struct Moving<'a> {
	pub window_pos: &'a mut RECT,
}

impl<'a> Message for Moving<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::MOVING,
			wparam: 0,
			lparam: self.window_pos as *const _ as isize,
		}
	}
}

impl<'a> MessageHandleable for Moving<'a> {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			window_pos: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_NCCALCSIZE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize)
/// message parameters.
///
/// Return type: `WVR`.
pub struct NcCalcSize<'a, 'b> {
	pub data: NccspRect<'a, 'b>,
}

impl<'a, 'b> Message for NcCalcSize<'a, 'b> {
	type RetType = co::WVR;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::WVR(v as u32)
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::NCCALCSIZE,
			wparam: match &self.data {
				NccspRect::Nccsp(_) => true as usize,
				NccspRect::Rect(_) => false as usize,
			},
			lparam: match &self.data {
				NccspRect::Nccsp(nccalc) => *nccalc as *const _ as isize,
				NccspRect::Rect(rc) => *rc as *const _ as isize,
			},
		}
	}
}

impl<'a, 'b> MessageHandleable for NcCalcSize<'a, 'b> {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			data: match p.wparam {
				0 => NccspRect::Rect(unsafe { &mut *(p.lparam as *mut _) }),
				_ => NccspRect::Nccsp(unsafe { &mut *(p.lparam as *mut _) }),
			},
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_NCCREATE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-nccreate)
/// message parameters.
///
/// Return type: `bool`.
pub struct NcCreate<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

impl<'a, 'b, 'c> Message for NcCreate<'a, 'b, 'c> {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::NCCREATE,
			wparam: 0,
			lparam: self.createstruct as *const _ as isize,
		}
	}
}

impl<'a, 'b, 'c> MessageHandleable for NcCreate<'a, 'b, 'c> {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			createstruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

//------------------------------------------------------------------------------

empty_msg_handleable! { NcDestroy, co::WM::NCDESTROY,
	/// [`WM_NCDESTROY`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-ncdestroy)
	/// message, which has no parameters.
	///
	/// Return type: `()`.
}

//------------------------------------------------------------------------------

/// [`WM_NCPAINT`](https://docs.microsoft.com/en-us/windows/win32/gdi/wm-ncpaint)
/// message parameters.
///
/// Return type: `()`.
pub struct NcPaint {
	pub updated_hrgn: HRGN,
}

impl Message for NcPaint {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::NCPAINT,
			wparam: self.updated_hrgn.ptr as usize,
			lparam: 0,
		}
	}
}

impl MessageHandleable for NcPaint {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			updated_hrgn: HRGN { ptr: p.wparam as *mut _ },
		}
	}
}

//------------------------------------------------------------------------------

empty_msg_handleable! { Null, co::WM::NULL,
	/// [`WM_NULL`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-null)
	/// message, which has no parameters.
	///
	/// Return type: `()`.
}

//------------------------------------------------------------------------------

/// [`WM_NOTIFY`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify)
/// message parameters.
///
/// Return type: `isize`.
#[derive(Copy, Clone)]
pub struct Notify<'a> {
	pub nmhdr: &'a NMHDR,
}

impl<'a> Message for Notify<'a> {
	type RetType = isize;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::NOTIFY,
			wparam: self.nmhdr.hwndFrom.ptr as usize,
			lparam: self.nmhdr as *const NMHDR as isize,
		}
	}
}

impl<'a> MessageHandleable for Notify<'a> {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			nmhdr: unsafe { &*(p.lparam as *const NMHDR) },
		}
	}
}

impl<'a> Notify<'a> {
	/// Casts the `NMHDR` reference into a derived struct.
	///
	/// You should always prefer the specific notifications, which perform this
	/// conversion for you.
	pub unsafe fn cast_nmhdr<T>(&self) -> &T {
		&*(self.nmhdr as *const NMHDR as *const T)
	}

	/// Casts the `NMHDR` mutable reference into a derived struct.
	///
	/// You should always prefer the specific notifications, which perform this
	/// conversion for you.
	pub unsafe fn cast_nmhdr_mut<T>(&self) -> &mut T {
		&mut *(self.nmhdr as *const NMHDR as *mut T)
	}
}

//------------------------------------------------------------------------------

empty_msg_handleable! { Paint, co::WM::PAINT,
	/// [`WM_PAINT`](https://docs.microsoft.com/en-us/windows/win32/gdi/wm-paint)
	/// message, which has no parameters.
	///
	/// Return type: `()`.
}

//------------------------------------------------------------------------------

/// [`WM_QUERYOPEN`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-queryopen)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct QueryOpen {}

impl Message for QueryOpen {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::QUERYOPEN,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MessageHandleable for QueryOpen {
	fn from_generic_wm(_: Wm) -> Self {
		Self {}
	}
}

//------------------------------------------------------------------------------

button_msg! { RButtonDblClk, co::WM::RBUTTONDBLCLK,
	/// [`WM_RBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondblclk)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { RButtonDown, co::WM::RBUTTONDOWN,
	/// [`WM_RBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondown)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { RButtonUp, co::WM::RBUTTONUP,
	/// [`WM_RBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttonup)
	/// message parameters.
	///
	/// Return type: `()`.
}

//------------------------------------------------------------------------------

/// [`WM_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-setfocus)
/// message parameters.
///
/// Return type: `()`.
pub struct SetFocus {
	pub hwnd_losing_focus: HWND,
}

impl Message for SetFocus {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::SETFOCUS,
			wparam: self.hwnd_losing_focus.ptr as usize,
			lparam: 0,
		}
	}
}

impl MessageHandleable for SetFocus {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			hwnd_losing_focus: HWND { ptr: p.wparam as *mut _ },
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_SETFONT`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-setfont)
/// message parameters.
///
/// Return type: `()`.
pub struct SetFont {
	pub hfont: HFONT,
	pub redraw: bool,
}

impl Message for SetFont {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::SETFONT,
			wparam: self.hfont.ptr as usize,
			lparam: MAKEDWORD(self.redraw as u16, 0) as isize,
		}
	}
}

impl MessageHandleable for SetFont {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			hfont: HFONT { ptr: p.wparam as *mut _ },
			redraw: LOWORD(p.lparam as u32) != 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_SETICON`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-seticon)
/// message parameters.
///
/// Return type: `Option<HICON>`.
pub struct SetIcon {
	pub size: co::ICON_SZ,
	pub hicon: HICON,
}

impl Message for SetIcon {
	type RetType = Option<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => None,
			v => Some(HICON { ptr: v as *mut _ }),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::SETICON,
			wparam: self.size.0 as usize,
			lparam: self.hicon.ptr as isize,
		}
	}
}

impl MessageHandleable for SetIcon {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			size: co::ICON_SZ(p.wparam as u8),
			hicon: HICON { ptr: p.lparam as *mut _ },
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_SHOWWINDOW`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow)
/// message parameters.
///
/// Return type: `()`.
pub struct ShowWindow {
	pub being_shown: bool,
	pub status: co::SW_S,
}

impl Message for ShowWindow {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::SHOWWINDOW,
			wparam: self.being_shown as usize,
			lparam: self.status.0 as isize,
		}
	}
}

impl MessageHandleable for ShowWindow {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			being_shown: p.wparam != 0,
			status: co::SW_S(p.lparam as u8),
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_SIZE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-size)
/// message parameters.
///
/// Return type: `()`.
pub struct Size {
	pub request: co::SIZE_R,
	pub client_area: SIZE,
}

impl Message for Size {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::SIZE,
			wparam: self.request.0 as usize,
			lparam: MAKEDWORD(
				self.client_area.cx as u16,
				self.client_area.cy as u16) as isize,
		}
	}
}

impl MessageHandleable for Size {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			request: co::SIZE_R(p.wparam as u8),
			client_area: SIZE::new(
				LOWORD(p.lparam as u32) as i32,
				HIWORD(p.lparam as u32) as i32,
			),
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_SIZING`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-sizing)
/// message parameters.
///
/// Return type: `()`.
pub struct Sizing<'a> {
	pub window_edge: co::WMSZ,
	pub coords: &'a mut RECT,
}

impl<'a> Message for Sizing<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::SIZING,
			wparam: self.window_edge.0 as usize,
			lparam: self.coords as *const _ as isize,
		}
	}
}

impl<'a> MessageHandleable for Sizing<'a> {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			window_edge: co::WMSZ(p.wparam as u8),
			coords: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_STYLECHANGED`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanged)
/// message parameters.
///
/// Return type: `()`.
pub struct StyleChanged<'a> {
	pub change: co::GWL_C,
	pub stylestruct: WsWsex<'a>,
}

impl<'a> Message for StyleChanged<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::STYLECHANGED,
			wparam: self.change.0 as usize,
			lparam: match self.stylestruct {
				WsWsex::Ws(ws) => ws as *const STYLESTRUCT_WS as isize,
				WsWsex::Wsex(wsx) => wsx as *const STYLESTRUCT_WS_EX as isize,
			},
		}
	}
}

impl<'a> MessageHandleable for StyleChanged<'a> {
	fn from_generic_wm(p: Wm) -> Self {
		let change = co::GWL_C(p.wparam as i8);
		Self {
			change,
			stylestruct: match change {
				co::GWL_C::STYLE => WsWsex::Ws(unsafe { &*(p.lparam as *const _) }),
				_ => WsWsex::Wsex(unsafe { &*(p.lparam as *const _) }),
			},
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_STYLECHANGING`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanging)
/// message parameters.
///
/// Return type: `()`.
pub struct StyleChanging<'a> {
	pub change: co::GWL_C,
	pub stylestruct: WsWsex<'a>,
}

impl<'a> Message for StyleChanging<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::STYLECHANGING,
			wparam: self.change.0 as usize,
			lparam: match self.stylestruct {
				WsWsex::Ws(ws) => ws as *const STYLESTRUCT_WS as isize,
				WsWsex::Wsex(wsx) => wsx as *const STYLESTRUCT_WS_EX as isize,
			},
		}
	}
}

impl<'a> MessageHandleable for StyleChanging<'a> {
	fn from_generic_wm(p: Wm) -> Self {
		let change = co::GWL_C(p.wparam as i8);
		Self {
			change,
			stylestruct: match change {
				co::GWL_C::STYLE => WsWsex::Ws(unsafe { &*(p.lparam as *const _) }),
				_ => WsWsex::Wsex(unsafe { &*(p.lparam as *const _) }),
			},
		}
	}
}

//------------------------------------------------------------------------------

empty_msg_handleable! { ThemeChanged, co::WM::THEMECHANGED,
	/// [`WM_THEMECHANGED`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-themechanged)
	/// message, which has no parameters.
	///
	/// Return type: `()`.
}

//------------------------------------------------------------------------------

/// [`WM_TIMER`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-timer)
/// message parameters.
///
/// Return type: `()`.
pub struct Timer {
	pub timer_id: u32,
	pub timer_proc: Option<TIMERPROC>,
}

impl Message for Timer {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::TIMER,
			wparam: self.timer_id as usize,
			lparam: match self.timer_proc {
				Some(proc) => proc as isize,
				None => 0,
			},
		}
	}
}

impl MessageHandleable for Timer {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			timer_id: p.wparam as u32,
			timer_proc: match p.lparam {
				0 => None,
				addr => unsafe { std::mem::transmute(addr) },
			},
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_WINDOWPOSCHANGED`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanged)
/// message parameters.
///
/// Return type: `()`.
pub struct WindowPosChanged<'a> {
	pub windowpos: &'a WINDOWPOS,
}

impl<'a> Message for WindowPosChanged<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::WINDOWPOSCHANGED,
			wparam: 0,
			lparam: self.windowpos as *const _ as isize,
		}
	}
}

impl<'a> MessageHandleable for WindowPosChanged<'a> {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			windowpos: unsafe { &*(p.lparam as *const _) },
		}
	}
}

//------------------------------------------------------------------------------

/// [`WM_WINDOWPOSCHANGING`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanging)
/// message parameters.
///
/// Return type: `()`.
pub struct WindowPosChanging<'a> {
	pub windowpos: &'a WINDOWPOS,
}

impl<'a> Message for WindowPosChanging<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::WINDOWPOSCHANGING,
			wparam: 0,
			lparam: self.windowpos as *const _ as isize,
		}
	}
}

impl<'a> MessageHandleable for WindowPosChanging<'a> {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			windowpos: unsafe { &*(p.lparam as *const _) },
		}
	}
}

//------------------------------------------------------------------------------

button_msg! { XButtonDblClk, co::WM::XBUTTONDBLCLK,
	/// [`WM_XBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondblclk)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { XButtonDown, co::WM::XBUTTONDOWN,
	/// [`WM_XBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondown)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { XButtonUp, co::WM::XBUTTONUP,
	/// [`WM_XBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttonup)
	/// message parameters.
	///
	/// Return type: `()`.
}
