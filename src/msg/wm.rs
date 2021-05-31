//! Generic window
//! [messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues),
//! whose constants have [`WM`](crate::co::WM) prefix.

use crate::aliases::TIMERPROC;
use crate::co;
use crate::enums::{
	AccelMenuCtrl,
	AccelMenuCtrlData,
	HwndFocus,
	HwndHmenu,
	HwndPointId,
	NccspRect,
};
use crate::funcs::{HIBYTE, HIWORD, LOBYTE, LOWORD, MAKEDWORD, MAKEWORD};
use crate::handles::{HBRUSH, HDC, HDROP, HFONT, HICON, HMENU, HRGN, HWND};
use crate::msg::{MsgSend, MsgSendRecv, WndMsg};
use crate::msg::macros::{lp_to_point, point_to_lp};
use crate::privs::FAPPCOMMAND_MASK;
use crate::structs::{
	CREATESTRUCT,
	HELPINFO,
	MINMAXINFO,
	NMHDR,
	POINT,
	RECT,
	SIZE,
	STYLESTRUCT,
	TITLEBARINFOEX,
	WINDOWPOS,
};

/// [`WM_ACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-activate)
/// message parameters.
///
/// Return type: `()`.
pub struct Activate {
	pub event: co::WA,
	pub is_minimized: bool,
	pub hwnd: HWND,
}

impl MsgSend for Activate {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ACTIVATE,
			wparam: MAKEDWORD(self.event.0, self.is_minimized as _) as _,
			lparam: self.hwnd.ptr as _,
		}
	}
}

impl MsgSendRecv for Activate {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			event: co::WA(LOWORD(p.wparam as _)),
			is_minimized: HIWORD(p.wparam as _) != 0,
			hwnd: HWND { ptr: p.lparam as _ },
		}
	}
}

/// [`WM_ACTIVATEAPP`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-activateapp)
/// message parameters.
///
/// Return type: `()`.
pub struct ActivateApp {
	pub is_being_activated: bool,
	pub thread_id: u32,
}

impl MsgSend for ActivateApp {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ACTIVATEAPP,
			wparam: self.is_being_activated as _,
			lparam: self.thread_id as _,
		}
	}
}

impl MsgSendRecv for ActivateApp {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			is_being_activated: p.wparam != 0,
			thread_id: p.lparam as _,
		}
	}
}

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

impl MsgSend for AppCommand {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::APPCOMMAND,
			wparam: self.hwnd_owner.ptr as _,
			lparam: MAKEDWORD(self.keys.into(), self.app_command.0 | self.u_device.0) as _,
		}
	}
}

impl MsgSendRecv for AppCommand {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_owner: HWND { ptr: p.wparam as _ },
			app_command: co::APPCOMMAND(HIWORD(p.lparam as _) & !FAPPCOMMAND_MASK),
			u_device: co::FAPPCOMMAND(HIWORD(p.lparam as _) & FAPPCOMMAND_MASK),
			keys: co::MK(LOWORD(p.lparam as _)),
		}
	}
}

pub_struct_msg_empty_handleable! { CancelMode, co::WM::CANCELMODE,
	/// [`WM_CANCELMODE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-cancelmode)
}

pub_struct_msg_empty_handleable! { ChildActivate, co::WM::CHILDACTIVATE,
	/// [`WM_CHILDACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-childactivate)
}

pub_struct_msg_empty_handleable! { Close, co::WM::CLOSE,
	/// [`WM_CLOSE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-close)
}

pub_struct_msg_char! { Char, co::WM::CHAR,
	/// [`WM_CHAR`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-char)
}

/// [`WM_COMMAND`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-command)
/// message parameters.
///
/// Return type: `()`.
pub struct Command {
	pub event: AccelMenuCtrl,
}

impl MsgSend for Command {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::COMMAND,
			wparam: match self.event {
				AccelMenuCtrl::Accel(id) => MAKEDWORD(id, 1) as _,
				AccelMenuCtrl::Menu(id) => MAKEDWORD(id, 0) as _,
				AccelMenuCtrl::Ctrl(data) => MAKEDWORD(data.ctrl_id, data.notif_code.0) as _,
			},
			lparam: match self.event {
				AccelMenuCtrl::Accel(_) => co::CMD::Accelerator.0 as _,
				AccelMenuCtrl::Menu(_) => co::CMD::Menu.0 as _,
				AccelMenuCtrl::Ctrl(data) => data.ctrl_hwnd.ptr as _,
			},
		}
	}
}

impl MsgSendRecv for Command {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			event: match HIWORD(p.wparam as _) {
				1 => AccelMenuCtrl::Accel(LOWORD(p.wparam as _)),
				0 => AccelMenuCtrl::Menu(LOWORD(p.wparam as _)),
				code => AccelMenuCtrl::Ctrl(
					AccelMenuCtrlData {
						notif_code: co::CMD(code),
						ctrl_id: LOWORD(p.wparam as _),
						ctrl_hwnd: HWND { ptr: p.lparam as _ },
					},
				),
			},
		}
	}
}

/// [`WM_CONTEXTMENU`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-contextmenu)
/// message parameters.
///
/// Return type: `()`.
pub struct ContextMenu {
	pub hwnd: HWND,
	pub cursor_pos: POINT,
}

impl MsgSend for ContextMenu {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::CONTEXTMENU,
			wparam: self.hwnd.ptr as _,
			lparam: point_to_lp(self.cursor_pos),
		}
	}
}

impl MsgSendRecv for ContextMenu {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd: HWND { ptr: p.wparam as _ },
			cursor_pos: lp_to_point(p),
		}
	}
}

/// [`WM_CREATE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-create)
/// message parameters.
///
/// Return type: `i32`.
pub struct Create<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

impl<'a, 'b, 'c> MsgSend for Create<'a, 'b, 'c> {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::CREATE,
			wparam: 0,
			lparam: self.createstruct as *const _ as _,
		}
	}
}

impl<'a, 'b, 'c> MsgSendRecv for Create<'a, 'b, 'c> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			createstruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

pub_struct_msg_ctlcolor! { CtlColorBtn, co::WM::CTLCOLORBTN,
	/// [`WM_CTLCOLORBTN`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorbtn)
}

pub_struct_msg_ctlcolor! { CtlColorDlg, co::WM::CTLCOLORDLG,
	/// [`WM_CTLCOLORDLG`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-ctlcolordlg)
}

pub_struct_msg_ctlcolor! { CtlColorEdit, co::WM::CTLCOLOREDIT,
	/// [`WM_CTLCOLOREDIT`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcoloredit)
}

pub_struct_msg_ctlcolor! { CtlColorListBox, co::WM::CTLCOLORLISTBOX,
	/// [`WM_CTLCOLORLISTBOX`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorlistbox)
}

pub_struct_msg_ctlcolor! { CtlColorScrollBar, co::WM::CTLCOLORSCROLLBAR,
	/// [`WM_CTLCOLORSCROLLBAR`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorscrollbar)
}

pub_struct_msg_ctlcolor! { CtlColorStatic, co::WM::CTLCOLORSTATIC,
	/// [`WM_CTLCOLORSTATIC`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorstatic)
}

pub_struct_msg_char! { DeadChar, co::WM::DEADCHAR,
	/// [`WM_DEADCHAR`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-deadchar)
}

pub_struct_msg_empty_handleable! { Destroy, co::WM::DESTROY,
	/// [`WM_DESTROY`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-destroy)
}

/// [`WM_DROPFILES`](https://docs.microsoft.com/en-us/windows/win32/shell/wm-dropfiles)
/// message parameters.
///
/// Return type: `()`.
pub struct DropFiles {
	pub hdrop: HDROP,
}

impl MsgSend for DropFiles {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::DROPFILES,
			wparam: self.hdrop.ptr as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for DropFiles {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hdrop: HDROP { ptr: p.wparam as _ },
		}
	}
}

/// [`WM_ENABLE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-enable)
/// message parameters.
///
/// Return type: `()`.
pub struct Enable {
	pub has_been_enabled: bool,
}

impl MsgSend for Enable {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENABLE,
			wparam: self.has_been_enabled as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for Enable {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			has_been_enabled: p.wparam != 0,
		}
	}
}

/// [`WM_ENDSESSION`](https://docs.microsoft.com/en-us/windows/win32/shutdown/wm-endsession)
/// message parameters.
///
/// Return type: `()`.
pub struct EndSession {
	pub is_session_being_ended: bool,
	pub event: co::ENDSESSION,
}

impl MsgSend for EndSession {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENDSESSION,
			wparam: self.is_session_being_ended as _,
			lparam: self.event.0 as _,
		}
	}
}

impl MsgSendRecv for EndSession {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			is_session_being_ended: p.wparam != 0,
			event: co::ENDSESSION(p.lparam as _),
		}
	}
}

/// [`WM_ENTERIDLE`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-enteridle)
/// message parameters.
///
/// Return type: `()`.
pub struct EnterIdle {
	pub reason: co::MSGF,
	pub handle: HwndHmenu,
}

impl MsgSend for EnterIdle {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENTERIDLE,
			wparam: self.reason.0 as _,
			lparam: self.handle.as_isize(),
		}
	}
}

impl MsgSendRecv for EnterIdle {
	fn from_generic_wm(p: WndMsg) -> Self {
		let reason = co::MSGF(p.wparam as _);
		Self {
			reason,
			handle: match reason {
				co::MSGF::DIALOGBOX => HwndHmenu::Hwnd(HWND { ptr: p.lparam as _ }),
				_ => HwndHmenu::Hmenu(HMENU { ptr: p.lparam as _ }),
			},
		}
	}
}

pub_struct_msg_empty_handleable! { EnterSizeMove, co::WM::ENTERSIZEMOVE,
	/// [`WM_ENTERSIZEMOVE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-entersizemove)
}

/// [`WM_ENTERMENULOOP`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-entermenuloop)
/// message parameters.
///
/// Return type: `()`.
pub struct EnterMenuLoop {
	pub with_trackpopupmenu: bool,
}

impl MsgSend for EnterMenuLoop {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENTERMENULOOP,
			wparam: self.with_trackpopupmenu as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for EnterMenuLoop {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			with_trackpopupmenu: p.wparam != 0,
		}
	}
}

/// [`WM_ERASEBKGND`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-erasebkgnd)
/// message parameters.
///
/// Return type: `i32`.
pub struct EraseBkgnd {
	pub hdc: HDC,
}

impl MsgSend for EraseBkgnd {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ERASEBKGND,
			wparam: self.hdc.ptr as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for EraseBkgnd {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hdc: HDC { ptr: p.wparam as _ },
		}
	}
}

/// [`WM_EXITMENULOOP`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-exitmenuloop)
/// message parameters.
///
/// Return type: `()`.
pub struct ExitMenuLoop {
	pub is_shortcut: bool,
}

impl MsgSend for ExitMenuLoop {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::EXITMENULOOP,
			wparam: self.is_shortcut as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for ExitMenuLoop {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			is_shortcut: p.wparam != 0,
		}
	}
}

pub_struct_msg_empty_handleable! { ExitSizeMove, co::WM::EXITSIZEMOVE,
	/// [`WM_EXITSIZEMOVE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-exitsizemove)
}

/// [`WM_GETFONT`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-getfont)
/// message, which has no parameters.
///
/// Return type: `Option<HFONT>`.
pub struct GetFont {}

impl MsgSend for GetFont {
	type RetType = Option<HFONT>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => None,
			p => Some(HFONT { ptr: p as _ }),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETFONT,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for GetFont {
	fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

/// [`WM_GETHMENU`](https://docs.microsoft.com/en-us/windows/win32/winmsg/mn-gethmenu)
/// message, which has no parameters.
///
/// Return type: `Option<HMENU>`.
pub struct GetHMenu {}

impl MsgSend for GetHMenu {
	type RetType = Option<HMENU>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => None,
			p => Some(HMENU { ptr: p as _ }),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MN_GETHMENU,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for GetHMenu {
	fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

/// [`WM_GETMINMAXINFO`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-getminmaxinfo)
/// message parameters.
///
/// Return type: `()`.
pub struct GetMinMaxInfo<'a> {
	pub info: &'a mut MINMAXINFO,
}

impl<'a> MsgSend for GetMinMaxInfo<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETMINMAXINFO,
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for GetMinMaxInfo<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			info: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_GETTITLEBARINFOEX`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-gettitlebarinfoex)
/// message parameters.
///
/// Return type: `()`.
pub struct GetTitleBarInfoEx<'a> {
	pub info: &'a mut TITLEBARINFOEX,
}

impl<'a> MsgSend for GetTitleBarInfoEx<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETTITLEBARINFOEX,
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for GetTitleBarInfoEx<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			info: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_HELP`](https://docs.microsoft.com/en-us/windows/win32/shell/wm-help)
/// message parameters.
///
/// Return type: `()`.
pub struct Help<'a> {
	pub helpinfo: &'a HELPINFO,
}

impl<'a> MsgSend for Help<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::HELP,
			wparam: 0,
			lparam: self.helpinfo as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for Help<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			helpinfo: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_HSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-hscroll)
/// message parameters.
///
/// Return type: `()`.
pub struct HScroll {
	pub scroll_box_pos: u16,
	pub request: co::SB_REQ,
	pub hcontrol: Option<HWND>,
}

impl MsgSend for HScroll {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::HSCROLL,
			wparam: MAKEDWORD(self.request.0, self.scroll_box_pos) as _,
			lparam: self.hcontrol.map(|h| h.ptr as _).unwrap_or_default(),
		}
	}
}

impl MsgSendRecv for HScroll {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			scroll_box_pos: HIWORD(p.wparam as _),
			request: co::SB_REQ(LOWORD(p.wparam as _)),
			hcontrol: match p.lparam {
				0 => None,
				ptr => Some(HWND { ptr: ptr as _ }),
			},
		}
	}
}

/// [`WM_INITDIALOG`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
/// message parameters.
///
/// Return type: `bool`.
pub struct InitDialog {
	pub hwnd_focus: HWND,
	pub additional_data: isize,
}

impl MsgSend for InitDialog {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::INITDIALOG,
			wparam: self.hwnd_focus.ptr as _,
			lparam: self.additional_data,
		}
	}
}

impl MsgSendRecv for InitDialog {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_focus: HWND { ptr: p.wparam as _ },
			additional_data: p.lparam,
		}
	}
}

/// [`WM_INITMENUPOPUP`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-initmenupopup)
/// message parameters.
///
/// Return type: `()`.
pub struct InitMenuPopup {
	pub hmenu: HMENU,
	pub item_pos: u16,
	pub is_window_menu: bool,
}

impl MsgSend for InitMenuPopup {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::INITMENUPOPUP,
			wparam: self.hmenu.ptr as _,
			lparam: MAKEDWORD(self.item_pos, self.is_window_menu as _) as _,
		}
	}
}

impl MsgSendRecv for InitMenuPopup {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hmenu: HMENU { ptr: p.wparam as _ },
			item_pos: LOWORD(p.lparam as _),
			is_window_menu: HIWORD(p.lparam as _) != 0,
		}
	}
}

pub_struct_msg_char! { KeyDown, co::WM::KEYDOWN,
	/// [`WM_KEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-keydown)
}

pub_struct_msg_char! { KeyUp, co::WM::KEYUP,
	/// [`WM_KEYUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-keyup)
}

/// [`WM_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-killfocus)
/// message parameters.
///
/// Return type: `()`.
pub struct KillFocus {
	pub hwnd: Option<HWND>,
}

impl MsgSend for KillFocus {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::KILLFOCUS,
			wparam: self.hwnd.map(|h| h.ptr as _).unwrap_or_default(),
			lparam: 0,
		}
	}
}

impl MsgSendRecv for KillFocus {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd: match p.wparam {
				0 => None,
				ptr => Some(HWND { ptr: ptr as _ }),
			},
		}
	}
}

pub_struct_msg_button! { LButtonDblClk, co::WM::LBUTTONDBLCLK,
	/// [`WM_LBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondblclk)
}

pub_struct_msg_button! { LButtonDown, co::WM::LBUTTONDOWN,
	/// [`WM_LBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown)
}

pub_struct_msg_button! { LButtonUp, co::WM::LBUTTONUP,
	/// [`WM_LBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttonup)
}

pub_struct_msg_button! { MButtonDblClk, co::WM::MBUTTONDBLCLK,
	/// [`WM_MBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondblclk)
}

pub_struct_msg_button! { MButtonDown, co::WM::MBUTTONDOWN,
	/// [`WM_MBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondown)
}

pub_struct_msg_button! { MButtonUp, co::WM::MBUTTONUP,
	/// [`WM_MBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttonup)
}

/// [`WM_MENUCOMMAND`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-menucommand)
/// message parameters.
///
/// Return type: `()`.
pub struct MenuCommand {
	pub item_index: u32,
	pub hmenu: HMENU,
}

impl MsgSend for MenuCommand {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MENUCOMMAND,
			wparam: self.item_index as _,
			lparam: self.hmenu.ptr as _,
		}
	}
}

impl MsgSendRecv for MenuCommand {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			item_index: p.wparam as _,
			hmenu: HMENU { ptr: p.lparam as _ },
		}
	}
}

/// [`WM_MENUDRAG`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-menudrag)
/// message parameters.
///
/// Return type: `co::MND`.
pub struct MenuDrag {
	pub position: u32,
	pub hmenu: HMENU,
}

impl MsgSend for MenuDrag {
	type RetType = co::MND;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::MND(v as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MENUDRAG,
			wparam: self.position as _,
			lparam: self.hmenu.ptr as _,
		}
	}
}

impl MsgSendRecv for MenuDrag {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			position: p.wparam as _,
			hmenu: HMENU { ptr: p.lparam as _ },
		}
	}
}

/// [`WM_MENURBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-menurbuttonup)
/// message parameters.
///
/// Return type: `()`.
pub struct MenuRButtonUp {
	pub position: u32,
	pub hmenu: HMENU,
}

impl MsgSend for MenuRButtonUp {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MENURBUTTONUP,
			wparam: self.position as _,
			lparam: self.hmenu.ptr as _,
		}
	}
}

impl MsgSendRecv for MenuRButtonUp {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			position: p.wparam as _,
			hmenu: HMENU { ptr: p.lparam as _ },
		}
	}
}

pub_struct_msg_button! { MouseHover, co::WM::MOUSEHOVER,
	/// [`WM_MOUSEHOVER`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousehover)
}

pub_struct_msg_button! { MouseMove, co::WM::MOUSEMOVE,
	/// [`WM_MOUSEMOVE`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove)
}

/// [`WM_MOVE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-move)
/// message parameters.
///
/// Return type: `()`.
pub struct Move {
	pub coords: POINT,
}

impl MsgSend for Move {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MOVE,
			wparam: 0,
			lparam: point_to_lp(self.coords),
		}
	}
}

impl MsgSendRecv for Move {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			coords: lp_to_point(p),
		}
	}
}

/// [`WM_MOVING`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-moving)
/// message parameters.
///
/// Return type: `()`.
pub struct Moving<'a> {
	pub window_pos: &'a mut RECT,
}

impl<'a> MsgSend for Moving<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MOVING,
			wparam: 0,
			lparam: self.window_pos as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for Moving<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			window_pos: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_NCCALCSIZE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize)
/// message parameters.
///
/// Return type: `co::WVR`.
pub struct NcCalcSize<'a, 'b> {
	pub data: NccspRect<'a, 'b>,
}

impl<'a, 'b> MsgSend for NcCalcSize<'a, 'b> {
	type RetType = co::WVR;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::WVR(v as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NCCALCSIZE,
			wparam: match &self.data {
				NccspRect::Nccsp(_) => true as _,
				NccspRect::Rect(_) => false as _,
			},
			lparam: match &self.data {
				NccspRect::Nccsp(nccalc) => *nccalc as *const _ as _,
				NccspRect::Rect(rc) => *rc as *const _ as _,
			},
		}
	}
}

impl<'a, 'b> MsgSendRecv for NcCalcSize<'a, 'b> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			data: match p.wparam {
				0 => NccspRect::Rect(unsafe { &mut *(p.lparam as *mut _) }),
				_ => NccspRect::Nccsp(unsafe { &mut *(p.lparam as *mut _) }),
			},
		}
	}
}

/// [`WM_NCCREATE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-nccreate)
/// message parameters.
///
/// Return type: `bool`.
pub struct NcCreate<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

impl<'a, 'b, 'c> MsgSend for NcCreate<'a, 'b, 'c> {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NCCREATE,
			wparam: 0,
			lparam: self.createstruct as *const _ as _,
		}
	}
}

impl<'a, 'b, 'c> MsgSendRecv for NcCreate<'a, 'b, 'c> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			createstruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

pub_struct_msg_empty_handleable! { NcDestroy, co::WM::NCDESTROY,
	/// [`WM_NCDESTROY`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-ncdestroy)
}

/// [`WM_NCHITTEST`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-nchittest)
/// message parameters.
///
/// Return type: `co::HT`.
pub struct NcHitTest{
	pub cursor_pos: POINT,
}

impl MsgSend for NcHitTest {
	type RetType = co::HT;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::HT(v as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NCHITTEST,
			wparam: 0,
			lparam: point_to_lp(self.cursor_pos),
		}
	}
}

impl MsgSendRecv for NcHitTest {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			cursor_pos: lp_to_point(p),
		}
	}
}

/// [`WM_NCPAINT`](https://docs.microsoft.com/en-us/windows/win32/gdi/wm-ncpaint)
/// message parameters.
///
/// Return type: `()`.
pub struct NcPaint {
	pub updated_hrgn: HRGN,
}

impl MsgSend for NcPaint {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NCPAINT,
			wparam: self.updated_hrgn.ptr as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for NcPaint {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			updated_hrgn: HRGN { ptr: p.wparam as _ },
		}
	}
}

/// [`WM_NEXTDLGCTL`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-nextdlgctl)
/// message parameters.
///
/// Return type: `()`.
pub struct NextDlgCtl {
	pub hwnd_focus: HwndFocus,
}

impl MsgSend for NextDlgCtl {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NEXTDLGCTL,
			wparam: match self.hwnd_focus {
				HwndFocus::Hwnd(hctl) => hctl.ptr as _,
				HwndFocus::FocusPrev(prev) => prev as _,
			},
			lparam: MAKEDWORD(match self.hwnd_focus {
				HwndFocus::Hwnd(_) => 1,
				HwndFocus::FocusPrev(_) => 0,
			}, 0) as _,
		}
	}
}

impl MsgSendRecv for NextDlgCtl {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_focus: match p.wparam {
				1 => HwndFocus::Hwnd(HWND { ptr: p.wparam as _ }),
				_ => HwndFocus::FocusPrev(p.wparam != 0),
			},
		}
	}
}

pub_struct_msg_empty_handleable! { Null, co::WM::NULL,
	/// [`WM_NULL`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-null)
}

/// [`WM_NOTIFY`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify)
/// message parameters.
///
/// Return type: `isize`.
#[derive(Copy, Clone)]
pub struct Notify<'a> {
	pub nmhdr: &'a NMHDR,
}

impl<'a> MsgSend for Notify<'a> {
	type RetType = isize;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NOTIFY,
			wparam: self.nmhdr.hwndFrom.ptr as _,
			lparam: self.nmhdr as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for Notify<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			nmhdr: unsafe { &*(p.lparam as *const _) },
		}
	}
}

impl<'a> Notify<'a> {
	/// Casts the `NMHDR` reference into a derived struct.
	///
	/// You should always prefer the specific notifications, which perform this
	/// conversion for you.
	pub unsafe fn cast_nmhdr<T>(&self) -> &T {
		&*(self.nmhdr as *const _ as *const _)
	}

	/// Casts the `NMHDR` mutable reference into a derived struct.
	///
	/// You should always prefer the specific notifications, which perform this
	/// conversion for you.
	pub unsafe fn cast_nmhdr_mut<T>(&self) -> &mut T {
		&mut *(self.nmhdr as *const _ as *mut _)
	}
}

pub_struct_msg_empty_handleable! { Paint, co::WM::PAINT,
	/// [`WM_PAINT`](https://docs.microsoft.com/en-us/windows/win32/gdi/wm-paint)
}

/// [`WM_PARENTNOTIFY`](https://docs.microsoft.com/en-us/windows/win32/inputmsg/wm-parentnotify)
/// message parameters.
///
/// Return type: `()`.
pub struct ParentNotify {
	pub event: co::WMPN,
	pub child_id: u16,
	pub data: HwndPointId,
}

impl MsgSend for ParentNotify {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::PARENTNOTIFY,
			wparam: MAKEDWORD(self.event.0, self.child_id) as _,
			lparam: self.data.as_isize(),
		}
	}
}

impl MsgSendRecv for ParentNotify {
	fn from_generic_wm(p: WndMsg) -> Self {
		let event = co::WMPN(LOWORD(p.wparam as _));
		Self {
			event,
			child_id: HIWORD(p.wparam as _),
			data: match event {
				co::WMPN::CREATE | co::WMPN::DESTROY => HwndPointId::Hwnd(HWND { ptr: p.lparam as _ }),
				co::WMPN::POINTERDOWN => HwndPointId::Id(p.lparam as _),
				_ => HwndPointId::Point(lp_to_point(p)),
			},
		}
	}
}

/// [`WM_QUERYOPEN`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-queryopen)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct QueryOpen {}

impl MsgSend for QueryOpen {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::QUERYOPEN,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for QueryOpen {
	fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

pub_struct_msg_button! { RButtonDblClk, co::WM::RBUTTONDBLCLK,
	/// [`WM_RBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondblclk)
}

pub_struct_msg_button! { RButtonDown, co::WM::RBUTTONDOWN,
	/// [`WM_RBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondown)
}

pub_struct_msg_button! { RButtonUp, co::WM::RBUTTONUP,
	/// [`WM_RBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttonup)
}

/// [`WM_SETCURSOR`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-setcursor)
/// message parameters.
///
/// Return type: `bool`.
pub struct SetCursor {
	pub hwnd: HWND,
	pub hit_test: co::HT,
	pub mouse_msg: u16,
}

impl MsgSend for SetCursor {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETCURSOR,
			wparam: self.hwnd.ptr as _,
			lparam: MAKEDWORD(self.hit_test.0, self.mouse_msg) as _,
		}
	}
}

impl MsgSendRecv for SetCursor {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd: HWND { ptr: p.wparam as _ },
			hit_test: co::HT(LOWORD(p.lparam as _)),
			mouse_msg: HIWORD(p.lparam as _),
		}
	}
}

/// [`WM_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-setfocus)
/// message parameters.
///
/// Return type: `()`.
pub struct SetFocus {
	pub hwnd_losing_focus: HWND,
}

impl MsgSend for SetFocus {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETFOCUS,
			wparam: self.hwnd_losing_focus.ptr as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for SetFocus {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_losing_focus: HWND { ptr: p.wparam as _ },
		}
	}
}

/// [`WM_SETFONT`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-setfont)
/// message parameters.
///
/// Return type: `()`.
pub struct SetFont {
	pub hfont: HFONT,
	pub redraw: bool,
}

impl MsgSend for SetFont {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETFONT,
			wparam: self.hfont.ptr as _,
			lparam: MAKEDWORD(self.redraw as _, 0) as _,
		}
	}
}

impl MsgSendRecv for SetFont {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hfont: HFONT { ptr: p.wparam as _ },
			redraw: LOWORD(p.lparam as _) != 0,
		}
	}
}

/// [`WM_SETICON`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-seticon)
/// message parameters.
///
/// Return type: `Option<HICON>`.
pub struct SetIcon {
	pub size: co::ICON_SZ,
	pub hicon: HICON,
}

impl MsgSend for SetIcon {
	type RetType = Option<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => None,
			p => Some(HICON { ptr: p as _ }),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETICON,
			wparam: self.size.0 as _,
			lparam: self.hicon.ptr as _,
		}
	}
}

impl MsgSendRecv for SetIcon {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			size: co::ICON_SZ(p.wparam as _),
			hicon: HICON { ptr: p.lparam as _ },
		}
	}
}

/// [`WM_SHOWWINDOW`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow)
/// message parameters.
///
/// Return type: `()`.
pub struct ShowWindow {
	pub being_shown: bool,
	pub status: co::SW_S,
}

impl MsgSend for ShowWindow {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SHOWWINDOW,
			wparam: self.being_shown as _,
			lparam: self.status.0 as _,
		}
	}
}

impl MsgSendRecv for ShowWindow {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			being_shown: p.wparam != 0,
			status: co::SW_S(p.lparam as _),
		}
	}
}

/// [`WM_SIZE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-size)
/// message parameters.
///
/// Return type: `()`.
pub struct Size {
	pub request: co::SIZE_R,
	pub client_area: SIZE,
}

impl MsgSend for Size {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SIZE,
			wparam: self.request.0 as _,
			lparam: MAKEDWORD(
				self.client_area.cx as _,
				self.client_area.cy as _,
			) as _,
		}
	}
}

impl MsgSendRecv for Size {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			request: co::SIZE_R(p.wparam as _),
			client_area: SIZE::new(
				LOWORD(p.lparam as _) as _,
				HIWORD(p.lparam as _) as _,
			),
		}
	}
}

/// [`WM_SIZING`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-sizing)
/// message parameters.
///
/// Return type: `()`.
pub struct Sizing<'a> {
	pub window_edge: co::WMSZ,
	pub coords: &'a mut RECT,
}

impl<'a> MsgSend for Sizing<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SIZING,
			wparam: self.window_edge.0 as _,
			lparam: self.coords as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for Sizing<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			window_edge: co::WMSZ(p.wparam as _),
			coords: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_STYLECHANGED`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanged)
/// message parameters.
///
/// Return type: `()`.
pub struct StyleChanged<'a> {
	pub change: co::GWL_C,
	pub stylestruct: &'a STYLESTRUCT,
}

impl<'a> MsgSend for StyleChanged<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::STYLECHANGED,
			wparam: self.change.0 as _,
			lparam: self.stylestruct as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for StyleChanged<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		let change = co::GWL_C(p.wparam as _);
		Self {
			change,
			stylestruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

/// [`WM_STYLECHANGING`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanging)
/// message parameters.
///
/// Return type: `()`.
pub struct StyleChanging<'a> {
	pub change: co::GWL_C,
	pub stylestruct: &'a STYLESTRUCT,
}

impl<'a> MsgSend for StyleChanging<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::STYLECHANGING,
			wparam: self.change.0 as _,
			lparam: self.stylestruct as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for StyleChanging<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		let change = co::GWL_C(p.wparam as _);
		Self {
			change,
			stylestruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

pub_struct_msg_char! { SysChar, co::WM::SYSCHAR,
	/// [`WM_SYSCHAR`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-syschar)
}

/// [`WM_SYSCOMMAND`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-syscommand)
/// message parameters.
///
/// Return type: `()`.
pub struct SysCommand {
	pub request: co::SC,
	pub position: POINT,
}

impl MsgSend for SysCommand {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SYSCOMMAND,
			wparam: self.request.0 as _,
			lparam: point_to_lp(self.position),
		}
	}
}

impl MsgSendRecv for SysCommand {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			request: co::SC(p.wparam as _),
			position: lp_to_point(p),
		}
	}
}

pub_struct_msg_char! { SysDeadChar, co::WM::SYSDEADCHAR,
	/// [`WM_SYSDEADCHAR`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-sysdeadchar)
}

pub_struct_msg_char! { SysKeyDown, co::WM::SYSKEYDOWN,
	/// [`WM_SYSKEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-syskeydown)
}

pub_struct_msg_char! { SysKeyUp, co::WM::SYSKEYUP,
	/// [`WM_SYSKEYUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-syskeyup)
}

pub_struct_msg_empty_handleable! { ThemeChanged, co::WM::THEMECHANGED,
	/// [`WM_THEMECHANGED`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-themechanged)
}

/// [`WM_TIMER`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-timer)
/// message parameters.
///
/// Return type: `()`.
pub struct Timer {
	pub timer_id: u32,
	pub timer_proc: Option<TIMERPROC>,
}

impl MsgSend for Timer {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::TIMER,
			wparam: self.timer_id as _,
			lparam: match self.timer_proc {
				Some(proc) => proc as _,
				None => 0,
			},
		}
	}
}

impl MsgSendRecv for Timer {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			timer_id: p.wparam as _,
			timer_proc: match p.lparam {
				0 => None,
				addr => unsafe { std::mem::transmute(addr) },
			},
		}
	}
}

/// [`WM_UNINITMENUPOPUP`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-uninitmenupopup)
/// message parameters.
///
/// Return type: `()`.
pub struct UninitMenuPopup {
	pub hmenu: HMENU,
	pub which: co::MF,
}

impl MsgSend for UninitMenuPopup {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::UNINITMENUPOPUP,
			wparam: self.hmenu.ptr as _,
			lparam: MAKEDWORD(0, self.which.0 as _) as _,
		}
	}
}

impl MsgSendRecv for UninitMenuPopup {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hmenu: HMENU { ptr: p.wparam as _ },
			which: co::MF(LOWORD(p.lparam as _) as _),
		}
	}
}

/// [`WM_VSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-vscroll)
/// message parameters.
///
/// Return type: `()`.
pub struct VScroll {
	pub scroll_box_pos: u16,
	pub request: co::SB_REQ,
	pub hcontrol: Option<HWND>,
}

impl MsgSend for VScroll {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::VSCROLL,
			wparam: MAKEDWORD(self.request.0, self.scroll_box_pos) as _,
			lparam: self.hcontrol.map(|h| h.ptr as _).unwrap_or_default(),
		}
	}
}

impl MsgSendRecv for VScroll {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			scroll_box_pos: HIWORD(p.wparam as _),
			request: co::SB_REQ(LOWORD(p.wparam as _)),
			hcontrol: match p.lparam {
				0 => None,
				ptr => Some(HWND { ptr: ptr as _ }),
			},
		}
	}
}

/// [`WM_WINDOWPOSCHANGED`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanged)
/// message parameters.
///
/// Return type: `()`.
pub struct WindowPosChanged<'a> {
	pub windowpos: &'a WINDOWPOS,
}

impl<'a> MsgSend for WindowPosChanged<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::WINDOWPOSCHANGED,
			wparam: 0,
			lparam: self.windowpos as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WindowPosChanged<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			windowpos: unsafe { &*(p.lparam as *const _) },
		}
	}
}

/// [`WM_WINDOWPOSCHANGING`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanging)
/// message parameters.
///
/// Return type: `()`.
pub struct WindowPosChanging<'a> {
	pub windowpos: &'a WINDOWPOS,
}

impl<'a> MsgSend for WindowPosChanging<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::WINDOWPOSCHANGING,
			wparam: 0,
			lparam: self.windowpos as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WindowPosChanging<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			windowpos: unsafe { &*(p.lparam as *const _) },
		}
	}
}

pub_struct_msg_button! { XButtonDblClk, co::WM::XBUTTONDBLCLK,
	/// [`WM_XBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondblclk)
}

pub_struct_msg_button! { XButtonDown, co::WM::XBUTTONDOWN,
	/// [`WM_XBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondown)
}

pub_struct_msg_button! { XButtonUp, co::WM::XBUTTONUP,
	/// [`WM_XBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttonup)
}
