use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`WM_ACTIVATE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-activate)
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

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ACTIVATE,
			wparam: MAKEDWORD(self.event.raw(), self.is_minimized as _) as _,
			lparam: self.hwnd.ptr() as _,
		}
	}
}

impl MsgSendRecv for Activate {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			event: co::WA::from_raw(LOWORD(p.wparam as _)),
			is_minimized: HIWORD(p.wparam as _) != 0,
			hwnd: HWND::from_ptr(p.lparam as _),
		}
	}
}

/// [`WM_ACTIVATEAPP`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-activateapp)
/// message parameters.
///
/// Return type: `()`.
pub struct ActivateApp {
	pub is_being_activated: bool,
	pub thread_id: u32,
}

impl MsgSend for ActivateApp {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ACTIVATEAPP,
			wparam: self.is_being_activated as _,
			lparam: self.thread_id as _,
		}
	}
}

impl MsgSendRecv for ActivateApp {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			is_being_activated: p.wparam != 0,
			thread_id: p.lparam as _,
		}
	}
}

/// [`WM_APPCOMMAND`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-appcommand)
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

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::APPCOMMAND,
			wparam: self.hwnd_owner.ptr() as _,
			lparam: MAKEDWORD(self.keys.into(), self.app_command.raw() | self.u_device.raw()) as _,
		}
	}
}

impl MsgSendRecv for AppCommand {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_owner: HWND::from_ptr(p.wparam as _),
			app_command: co::APPCOMMAND::from_raw(HIWORD(p.lparam as _) & !FAPPCOMMAND_MASK),
			u_device: co::FAPPCOMMAND::from_raw(HIWORD(p.lparam as _) & FAPPCOMMAND_MASK),
			keys: co::MK::from_raw(LOWORD(p.lparam as _)),
		}
	}
}

pub_struct_msg_empty_handleable! { CancelMode: co::WM::CANCELMODE;
	/// [`WM_CANCELMODE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-cancelmode)
}

/// [`WM_CAPTURECHANGED`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-capturechanged)
/// message parameters.
///
/// Return type: `()`.
pub struct CaptureChanged {
	pub hwnd_gaining_mouse: HWND,
}

impl MsgSend for CaptureChanged {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::CAPTURECHANGED,
			wparam: self.hwnd_gaining_mouse.ptr() as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for CaptureChanged {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_gaining_mouse: HWND::from_ptr(p.wparam as _),
		}
	}
}

pub_struct_msg_char_code! { Char: co::WM::CHAR;
	/// [`WM_CHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-char)
}

pub_struct_msg_empty_handleable! { ChildActivate: co::WM::CHILDACTIVATE;
	/// [`WM_CHILDACTIVATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-childactivate)
}

pub_struct_msg_empty_handleable! { Close: co::WM::CLOSE;
	/// [`WM_CLOSE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-close)
}

/// [`WM_COMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-command)
/// message parameters.
///
/// Return type: `()`.
pub struct Command {
	pub event: AccelMenuCtrl,
}

impl MsgSend for Command {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::COMMAND,
			wparam: match &self.event {
				AccelMenuCtrl::Accel(id) => MAKEDWORD(*id, 1) as _,
				AccelMenuCtrl::Menu(id) => MAKEDWORD(*id, 0) as _,
				AccelMenuCtrl::Ctrl(data) => MAKEDWORD(data.ctrl_id, data.notif_code.raw()) as _,
			},
			lparam: match &self.event {
				AccelMenuCtrl::Accel(_) => co::CMD::Accelerator.raw() as _,
				AccelMenuCtrl::Menu(_) => co::CMD::Menu.raw() as _,
				AccelMenuCtrl::Ctrl(data) => data.ctrl_hwnd.ptr() as _,
			},
		}
	}
}

impl MsgSendRecv for Command {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			event: match HIWORD(p.wparam as _) {
				1 => AccelMenuCtrl::Accel(LOWORD(p.wparam as _)),
				0 => AccelMenuCtrl::Menu(LOWORD(p.wparam as _)),
				code => AccelMenuCtrl::Ctrl(
					AccelMenuCtrlData {
						notif_code: co::CMD::from_raw(code),
						ctrl_id: LOWORD(p.wparam as _),
						ctrl_hwnd: HWND::from_ptr(p.lparam as _),
					},
				),
			},
		}
	}
}

/// [`WM_CONTEXTMENU`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-contextmenu)
/// message parameters.
///
/// Return type: `()`.
pub struct ContextMenu {
	pub hwnd: HWND,
	pub cursor_pos: POINT,
}

impl MsgSend for ContextMenu {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::CONTEXTMENU,
			wparam: self.hwnd.ptr() as _,
			lparam: u32::from(self.cursor_pos) as _,
		}
	}
}

impl MsgSendRecv for ContextMenu {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd: HWND::from_ptr(p.wparam as _),
			cursor_pos: POINT::from(p.lparam as u32),
		}
	}
}

/// [`WM_CREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create)
/// message parameters.
///
/// Return type: `i32`.
pub struct Create<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

impl<'a, 'b, 'c> MsgSend for Create<'a, 'b, 'c> {
	type RetType = i32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::CREATE,
			wparam: 0,
			lparam: self.createstruct as *const _ as _,
		}
	}
}

impl<'a, 'b, 'c> MsgSendRecv for Create<'a, 'b, 'c> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			createstruct: &*(p.lparam as *const _),
		}
	}
}

/// [`WM_DELETEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-deleteitem)
/// message parameters.
///
/// Return type: `()`.
pub struct DeleteItem<'a> {
	pub control_id: u16,
	pub deleteitemstruct: &'a DELETEITEMSTRUCT,
}

impl<'a> MsgSend for DeleteItem<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::DELETEITEM,
			wparam: self.control_id as _,
			lparam: self.deleteitemstruct as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for DeleteItem<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			control_id: p.wparam as _,
			deleteitemstruct: &mut *(p.lparam as *mut _),
		}
	}
}

pub_struct_msg_empty_handleable! { Destroy: co::WM::DESTROY;
	/// [`WM_DESTROY`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-destroy)
}

/// [`WM_DEVICECHANGE`](https://learn.microsoft.com/en-us/windows/win32/devio/wm-devicechange)
/// message parameters.
///
/// Return type: `()`.
pub struct DeviceChange<'a> {
	pub event: co::DBT,
	pub data: Option<&'a DEV_BROADCAST_HDR>,
}

impl<'a> MsgSend for DeviceChange<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::DEVICECHANGE,
			wparam: self.event.raw() as _,
			lparam: self.data.map_or(0, |d| d as *const _ as _),
		}
	}
}

impl<'a> MsgSendRecv for DeviceChange<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			event: co::DBT::from_raw(p.wparam as _),
			data: if p.lparam == 0 {
				None
			} else {
				Some(&*(p.lparam as *const _))
			}
		}
	}
}

/// [`WM_ENABLE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-enable)
/// message parameters.
///
/// Return type: `()`.
pub struct Enable {
	pub has_been_enabled: bool,
}

impl MsgSend for Enable {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENABLE,
			wparam: self.has_been_enabled as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for Enable {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			has_been_enabled: p.wparam != 0,
		}
	}
}

/// [`WM_ENDSESSION`](https://learn.microsoft.com/en-us/windows/win32/shutdown/wm-endsession)
/// message parameters.
///
/// Return type: `()`.
pub struct EndSession {
	pub is_session_being_ended: bool,
	pub event: co::ENDSESSION,
}

impl MsgSend for EndSession {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENDSESSION,
			wparam: self.is_session_being_ended as _,
			lparam: self.event.raw() as _,
		}
	}
}

impl MsgSendRecv for EndSession {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			is_session_being_ended: p.wparam != 0,
			event: co::ENDSESSION::from_raw(p.lparam as _),
		}
	}
}

/// [`WM_ENTERIDLE`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-enteridle)
/// message parameters.
///
/// Return type: `()`.
pub struct EnterIdle {
	pub reason: co::MSGF,
	pub handle: HwndHmenu,
}

impl MsgSend for EnterIdle {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENTERIDLE,
			wparam: self.reason.raw() as _,
			lparam: self.handle.as_isize(),
		}
	}
}

impl MsgSendRecv for EnterIdle {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		let reason = co::MSGF::from_raw(p.wparam as _);
		Self {
			reason,
			handle: match reason {
				co::MSGF::DIALOGBOX => HwndHmenu::Hwnd(HWND::from_ptr(p.lparam as _)),
				_ => HwndHmenu::Hmenu(HMENU::from_ptr(p.lparam as _)),
			},
		}
	}
}

/// [`WM_ENTERMENULOOP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-entermenuloop)
/// message parameters.
///
/// Return type: `()`.
pub struct EnterMenuLoop {
	pub with_trackpopupmenu: bool,
}

impl MsgSend for EnterMenuLoop {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENTERMENULOOP,
			wparam: self.with_trackpopupmenu as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for EnterMenuLoop {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			with_trackpopupmenu: p.wparam != 0,
		}
	}
}

pub_struct_msg_empty_handleable! { EnterSizeMove: co::WM::ENTERSIZEMOVE;
	/// [`WM_ENTERSIZEMOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-entersizemove)
}

/// [`WM_ERASEBKGND`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-erasebkgnd)
/// message parameters.
///
/// Return type: `i32`.
pub struct EraseBkgnd {
	pub hdc: HDC,
}

impl MsgSend for EraseBkgnd {
	type RetType = i32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ERASEBKGND,
			wparam: self.hdc.ptr() as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for EraseBkgnd {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hdc: HDC::from_ptr(p.wparam as _),
		}
	}
}

/// [`WM_EXITMENULOOP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-exitmenuloop)
/// message parameters.
///
/// Return type: `()`.
pub struct ExitMenuLoop {
	pub is_shortcut: bool,
}

impl MsgSend for ExitMenuLoop {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::EXITMENULOOP,
			wparam: self.is_shortcut as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for ExitMenuLoop {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			is_shortcut: p.wparam != 0,
		}
	}
}

pub_struct_msg_empty_handleable! { ExitSizeMove: co::WM::EXITSIZEMOVE;
	/// [`WM_EXITSIZEMOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-exitsizemove)
}

/// [`WM_GETDLGCODE`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-getdlgcode)
/// message parameters.
///
/// Return type: `co::DLGC`.
pub struct GetDlgCode<'a> {
	pub vkey_code: co::VK,
	pub msg: Option<&'a mut MSG>,
	pub is_query: bool,
}

impl<'a> MsgSend for GetDlgCode<'a> {
	type RetType = co::DLGC;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		co::DLGC::from_raw(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETDLGCODE,
			wparam: self.vkey_code.raw() as _,
			lparam: self.msg.as_mut().map_or(0, |m| m as *mut _ as _),
		}
	}
}

impl<'a> MsgSendRecv for GetDlgCode<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			vkey_code: co::VK::from_raw(p.wparam as _),
			msg: match p.lparam {
				0 => None,
				ptr => Some(&mut *(ptr as *mut _))
			},
			is_query: p.lparam == 0,
		}
	}
}

/// [`WM_GETHMENU`](https://learn.microsoft.com/en-us/windows/win32/winmsg/mn-gethmenu)
/// message, which has no parameters. Originally has `MN` prefix.
///
/// Return type: `Option<HMENU>`.
pub struct GetHMenu {}

impl MsgSend for GetHMenu {
	type RetType = Option<HMENU>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HMENU::from_ptr(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MN_GETHMENU,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for GetHMenu {
	unsafe fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

/// [`WM_GETMINMAXINFO`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-getminmaxinfo)
/// message parameters.
///
/// Return type: `()`.
pub struct GetMinMaxInfo<'a> {
	pub info: &'a mut MINMAXINFO,
}

impl<'a> MsgSend for GetMinMaxInfo<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETMINMAXINFO,
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

impl<'a> MsgSendRecv for GetMinMaxInfo<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			info: &mut *(p.lparam as *mut _),
		}
	}
}

/// [`WM_GETTEXT`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-gettext)
/// message parameters.
///
/// Return type: `u32`.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, msg};
///
/// let hwnd: w::HWND; // initialized somewhere
/// # let hwnd = w::HWND::NULL;
///
/// let needed_len = unsafe { hwnd.SendMessage(msg::wm::GetTextLength {}) };
/// let mut buf = w::WString::new_alloc_buf(needed_len as _);
///
/// unsafe {
///     hwnd.SendMessage(
///         msg::wm::GetText {
///             buffer: buf.as_mut_slice(),
///         },
///     );
/// }
///
/// println!("Text: {}", buf.to_string());
/// ```
pub struct GetText<'a> {
	pub buffer: &'a mut [u16], // can't be WString because this message can be received
}

impl<'a> MsgSend for GetText<'a> {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETTEXT,
			wparam: self.buffer.len(),
			lparam: self.buffer.as_mut_ptr() as _,
		}
	}
}

impl<'a> MsgSendRecv for GetText<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			buffer: std::slice::from_raw_parts_mut(p.lparam as _, p.wparam),
		}
	}
}

/// [`WM_GETTEXTLENGTH`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-gettextlength)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetTextLength {}

impl MsgSend for GetTextLength {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETTEXTLENGTH,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for GetTextLength {
	unsafe fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

/// [`WM_GETTITLEBARINFOEX`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-gettitlebarinfoex)
/// message parameters.
///
/// Return type: `()`.
pub struct GetTitleBarInfoEx<'a> {
	pub info: &'a mut TITLEBARINFOEX,
}

impl<'a> MsgSend for GetTitleBarInfoEx<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETTITLEBARINFOEX,
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

impl<'a> MsgSendRecv for GetTitleBarInfoEx<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			info: &mut *(p.lparam as *mut _),
		}
	}
}

/// [`WM_HELP`](https://learn.microsoft.com/en-us/windows/win32/shell/wm-help)
/// message parameters.
///
/// Return type: `()`.
pub struct Help<'a> {
	pub helpinfo: &'a HELPINFO,
}

impl<'a> MsgSend for Help<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::HELP,
			wparam: 0,
			lparam: self.helpinfo as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for Help<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			helpinfo: &mut *(p.lparam as *mut _),
		}
	}
}

/// [`WM_HSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-hscroll)
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

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::HSCROLL,
			wparam: MAKEDWORD(self.request.raw(), self.scroll_box_pos) as _,
			lparam: self.hcontrol.as_ref().map_or(0, |h| h.ptr() as _),
		}
	}
}

impl MsgSendRecv for HScroll {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			scroll_box_pos: HIWORD(p.wparam as _),
			request: co::SB_REQ::from_raw(LOWORD(p.wparam as _)),
			hcontrol: match p.lparam {
				0 => None,
				ptr => Some(HWND::from_ptr(ptr as _)),
			},
		}
	}
}

/// [`WM_INITDIALOG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
/// message parameters.
///
/// Return type: `bool`.
pub struct InitDialog {
	pub hwnd_focus: HWND,
	pub additional_data: isize,
}

impl MsgSend for InitDialog {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::INITDIALOG,
			wparam: self.hwnd_focus.ptr() as _,
			lparam: self.additional_data,
		}
	}
}

impl MsgSendRecv for InitDialog {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_focus: HWND::from_ptr(p.wparam as _),
			additional_data: p.lparam,
		}
	}
}

/// [`WM_INITMENUPOPUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-initmenupopup)
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

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::INITMENUPOPUP,
			wparam: self.hmenu.ptr() as _,
			lparam: MAKEDWORD(self.item_pos, self.is_window_menu as _) as _,
		}
	}
}

impl MsgSendRecv for InitMenuPopup {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hmenu: HMENU::from_ptr(p.wparam as _),
			item_pos: LOWORD(p.lparam as _),
			is_window_menu: HIWORD(p.lparam as _) != 0,
		}
	}
}

pub_struct_msg_char_key! { KeyDown: co::WM::KEYDOWN;
	/// [`WM_KEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keydown)
}

pub_struct_msg_char_key! { KeyUp: co::WM::KEYUP;
	/// [`WM_KEYUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keyup)
}

/// [`WM_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-killfocus)
/// message parameters.
///
/// Return type: `()`.
pub struct KillFocus {
	pub hwnd: Option<HWND>,
}

impl MsgSend for KillFocus {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::KILLFOCUS,
			wparam: self.hwnd.as_ref().map_or(0, |h| h.ptr() as _),
			lparam: 0,
		}
	}
}

impl MsgSendRecv for KillFocus {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd: match p.wparam {
				0 => None,
				ptr => Some(HWND::from_ptr(ptr as _)),
			},
		}
	}
}

pub_struct_msg_button! { LButtonDblClk: co::WM::LBUTTONDBLCLK;
	/// [`WM_LBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondblclk)
}

pub_struct_msg_button! { LButtonDown: co::WM::LBUTTONDOWN;
	/// [`WM_LBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown)
}

pub_struct_msg_button! { LButtonUp: co::WM::LBUTTONUP;
	/// [`WM_LBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttonup)
}

pub_struct_msg_button! { MButtonDblClk: co::WM::MBUTTONDBLCLK;
	/// [`WM_MBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondblclk)
}

pub_struct_msg_button! { MButtonDown: co::WM::MBUTTONDOWN;
	/// [`WM_MBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondown)
}

pub_struct_msg_button! { MButtonUp: co::WM::MBUTTONUP;
	/// [`WM_MBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttonup)
}

/// [`WM_MENUCOMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menucommand)
/// message parameters.
///
/// Return type: `()`.
pub struct MenuCommand {
	pub item_index: u32,
	pub hmenu: HMENU,
}

impl MsgSend for MenuCommand {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MENUCOMMAND,
			wparam: self.item_index as _,
			lparam: self.hmenu.ptr() as _,
		}
	}
}

impl MsgSendRecv for MenuCommand {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			item_index: p.wparam as _,
			hmenu: HMENU::from_ptr(p.lparam as _),
		}
	}
}

/// [`WM_MENUDRAG`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menudrag)
/// message parameters.
///
/// Return type: `co::MND`.
pub struct MenuDrag {
	pub position: u32,
	pub hmenu: HMENU,
}

impl MsgSend for MenuDrag {
	type RetType = co::MND;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		co::MND::from_raw(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MENUDRAG,
			wparam: self.position as _,
			lparam: self.hmenu.ptr() as _,
		}
	}
}

impl MsgSendRecv for MenuDrag {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			position: p.wparam as _,
			hmenu: HMENU::from_ptr(p.lparam as _),
		}
	}
}

/// [`WM_MENURBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menurbuttonup)
/// message parameters.
///
/// Return type: `()`.
pub struct MenuRButtonUp {
	pub position: u32,
	pub hmenu: HMENU,
}

impl MsgSend for MenuRButtonUp {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MENURBUTTONUP,
			wparam: self.position as _,
			lparam: self.hmenu.ptr() as _,
		}
	}
}

impl MsgSendRecv for MenuRButtonUp {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			position: p.wparam as _,
			hmenu: HMENU::from_ptr(p.lparam as _),
		}
	}
}

pub_struct_msg_button! { MouseHover: co::WM::MOUSEHOVER;
	/// [`WM_MOUSEHOVER`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousehover)
}

/// [`WM_MOUSEHWHEEL`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousehwheel)
/// message parameters.
///
/// Return type: `()`.
pub struct MouseHWheel {
	pub wheel_distance: i16,
	pub keys: co::MK,
	pub coords: POINT,
}

impl MsgSend for MouseHWheel {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MOUSEHWHEEL,
			wparam: MAKEDWORD(self.wheel_distance as _, self.keys.raw()) as _,
			lparam: u32::from(self.coords) as _,
		}
	}
}

impl MsgSendRecv for MouseHWheel {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			wheel_distance: LOWORD(p.wparam as _) as _,
			keys: co::MK::from_raw(HIWORD(p.wparam as _)),
			coords: POINT::from(p.lparam as u32),
		}
	}
}

pub_struct_msg_empty_handleable! { MouseLeave: co::WM::MOUSELEAVE;
	/// [`WM_MOUSELEAVE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mouseleave)
}

pub_struct_msg_button! { MouseMove: co::WM::MOUSEMOVE;
	/// [`WM_MOUSEMOVE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove)
}

/// [`WM_MOUSEWHEEL`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel)
/// message parameters.
///
/// Return type: `()`.
pub struct MouseWheel {
	pub wheel_distance: i16,
	pub keys: co::MK,
	pub coords: POINT,
}

impl MsgSend for MouseWheel {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MOUSEWHEEL,
			wparam: MAKEDWORD(self.wheel_distance as _, self.keys.raw()) as _,
			lparam: u32::from(self.coords) as _,
		}
	}
}

impl MsgSendRecv for MouseWheel {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			wheel_distance: LOWORD(p.wparam as _) as _,
			keys: co::MK::from_raw(HIWORD(p.wparam as _)),
			coords: POINT::from(p.lparam as u32),
		}
	}
}

/// [`WM_MOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-move)
/// message parameters.
///
/// Return type: `()`.
pub struct Move {
	pub coords: POINT,
}

impl MsgSend for Move {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MOVE,
			wparam: 0,
			lparam: u32::from(self.coords) as _,
		}
	}
}

impl MsgSendRecv for Move {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			coords: POINT::from(p.lparam as u32),
		}
	}
}

/// [`WM_MOVING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-moving)
/// message parameters.
///
/// Return type: `()`.
pub struct Moving<'a> {
	pub window_pos: &'a mut RECT,
}

impl<'a> MsgSend for Moving<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MOVING,
			wparam: 0,
			lparam: self.window_pos as *mut _ as _,
		}
	}
}

impl<'a> MsgSendRecv for Moving<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			window_pos: &mut *(p.lparam as *mut _),
		}
	}
}

/// [`WM_NCCALCSIZE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize)
/// message parameters.
///
/// Return type: `co::WVR`.
pub struct NcCalcSize<'a, 'b> {
	pub data: NccspRect<'a, 'b>,
}

impl<'a, 'b> MsgSend for NcCalcSize<'a, 'b> {
	type RetType = co::WVR;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		co::WVR::from_raw(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
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
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			data: match p.wparam {
				0 => NccspRect::Rect(&mut *(p.lparam as *mut _)),
				_ => NccspRect::Nccsp(&mut *(p.lparam as *mut _)),
			},
		}
	}
}

/// [`WM_NCCREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccreate)
/// message parameters.
///
/// Return type: `bool`.
pub struct NcCreate<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

impl<'a, 'b, 'c> MsgSend for NcCreate<'a, 'b, 'c> {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NCCREATE,
			wparam: 0,
			lparam: self.createstruct as *const _ as _,
		}
	}
}

impl<'a, 'b, 'c> MsgSendRecv for NcCreate<'a, 'b, 'c> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			createstruct: &*(p.lparam as *const _),
		}
	}
}

pub_struct_msg_empty_handleable! { NcDestroy: co::WM::NCDESTROY;
	/// [`WM_NCDESTROY`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-ncdestroy)
}

/// [`WM_NCHITTEST`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-nchittest)
/// message parameters.
///
/// Return type: `co::HT`.
pub struct NcHitTest{
	pub cursor_pos: POINT,
}

impl MsgSend for NcHitTest {
	type RetType = co::HT;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		co::HT::from_raw(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NCHITTEST,
			wparam: 0,
			lparam: u32::from(self.cursor_pos) as _,
		}
	}
}

impl MsgSendRecv for NcHitTest {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			cursor_pos: POINT::from(p.lparam as u32),
		}
	}
}

/// [`WM_NEXTDLGCTL`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-nextdlgctl)
/// message parameters.
///
/// Return type: `()`.
pub struct NextDlgCtl {
	pub hwnd_focus: HwndFocus,
}

impl MsgSend for NextDlgCtl {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NEXTDLGCTL,
			wparam: match &self.hwnd_focus {
				HwndFocus::Hwnd(hctl) => hctl.ptr() as _,
				HwndFocus::FocusNext(next) => if *next { 0 } else { 1 },
			},
			lparam: MAKEDWORD(match &self.hwnd_focus {
				HwndFocus::Hwnd(_) => 1,
				HwndFocus::FocusNext(_) => 0,
			}, 0) as _,
		}
	}
}

impl MsgSendRecv for NextDlgCtl {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_focus: match p.wparam {
				1 => HwndFocus::Hwnd(HWND::from_ptr(p.wparam as _)),
				_ => HwndFocus::FocusNext(p.wparam == 0),
			},
		}
	}
}

pub_struct_msg_empty_handleable! { Null: co::WM::NULL;
	/// [`WM_NULL`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-null)
}

/// [`WM_PARENTNOTIFY`](https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-parentnotify)
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

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::PARENTNOTIFY,
			wparam: MAKEDWORD(self.event.raw(), self.child_id) as _,
			lparam: self.data.as_isize(),
		}
	}
}

impl MsgSendRecv for ParentNotify {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		let event = co::WMPN::from_raw(LOWORD(p.wparam as _));
		Self {
			event,
			child_id: HIWORD(p.wparam as _),
			data: match event {
				co::WMPN::CREATE
				| co::WMPN::DESTROY => HwndPointId::Hwnd(HWND::from_ptr(p.lparam as _)),
				co::WMPN::POINTERDOWN => HwndPointId::Id(p.lparam as _),
				_ => HwndPointId::Point(POINT::from(p.lparam as u32)),
			},
		}
	}
}

/// [`WM_POWERBROADCAST`](https://learn.microsoft.com/en-us/windows/win32/power/wm-powerbroadcast)
/// message parameters.
///
/// Return type: `()`.
pub struct PowerBroadcast<'a> {
	pub event: co::PBT,
	pub data: Option<&'a POWERBROADCAST_SETTING>,
}

impl<'a> MsgSend for PowerBroadcast<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::POWERBROADCAST,
			wparam: self.event.raw() as _,
			lparam: self.data.map_or(0, |d| d as *const _ as _),
		}
	}
}

impl<'a> MsgSendRecv for PowerBroadcast<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			event: co::PBT::from_raw(p.wparam as _),
			data: if p.lparam == 0 {
				None
			} else {
				Some(&*(p.lparam as *const _))
			},
		}
	}
}

/// [`WM_QUERYOPEN`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-queryopen)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct QueryOpen {}

impl MsgSend for QueryOpen {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::QUERYOPEN,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for QueryOpen {
	unsafe fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

pub_struct_msg_button! { RButtonDblClk: co::WM::RBUTTONDBLCLK;
	/// [`WM_RBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondblclk)
}

pub_struct_msg_button! { RButtonDown: co::WM::RBUTTONDOWN;
	/// [`WM_RBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondown)
}

pub_struct_msg_button! { RButtonUp: co::WM::RBUTTONUP;
	/// [`WM_RBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttonup)
}

/// [`WM_SETCURSOR`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-setcursor)
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

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETCURSOR,
			wparam: self.hwnd.ptr() as _,
			lparam: MAKEDWORD(self.hit_test.raw(), self.mouse_msg) as _,
		}
	}
}

impl MsgSendRecv for SetCursor {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd: HWND::from_ptr(p.wparam as _),
			hit_test: co::HT::from_raw(LOWORD(p.lparam as _)),
			mouse_msg: HIWORD(p.lparam as _),
		}
	}
}

/// [`WM_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-setfocus)
/// message parameters.
///
/// Return type: `()`.
pub struct SetFocus {
	pub hwnd_losing_focus: HWND,
}

impl MsgSend for SetFocus {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETFOCUS,
			wparam: self.hwnd_losing_focus.ptr() as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for SetFocus {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_losing_focus: HWND::from_ptr(p.wparam as _),
		}
	}
}

/// [`WM_SETICON`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-seticon)
/// message parameters.
///
/// Return type: `Option<HICON>`.
pub struct SetIcon {
	pub size: co::ICON_SZ,
	pub hicon: HICON,
}

impl MsgSend for SetIcon {
	type RetType = Option<HICON>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HICON::from_ptr(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETICON,
			wparam: self.size.raw() as _,
			lparam: self.hicon.ptr() as _,
		}
	}
}

impl MsgSendRecv for SetIcon {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			size: co::ICON_SZ::from_raw(p.wparam as _),
			hicon: HICON::from_ptr(p.lparam as _),
		}
	}
}

/// [`WM_SETTEXT`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-settext)
/// message parameters.
///
/// Return type: `bool`.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, msg};
///
/// let hwnd: w::HWND; // initialized somewhere
/// # let hwnd = w::HWND::NULL;
///
/// let new_text = w::WString::from_str("some text");
///
/// unsafe {
///     hwnd.SendMessage(
///         msg::wm::SetText {
///             text: unsafe { new_text.as_ptr() },
///         },
///     );
/// }
/// ```
pub struct SetText {
	pub text: *const u16, // can't be WString because this message can be received
}

impl MsgSend for SetText {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			0 | CB_ERRSPACE | CB_ERR => false,
			_ => true,
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETTEXT,
			wparam: 0,
			lparam: self.text as _,
		}
	}
}

impl MsgSendRecv for SetText {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			text: p.lparam as _,
		}
	}
}

/// [`WM_SHOWWINDOW`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow)
/// message parameters.
///
/// Return type: `()`.
pub struct ShowWindow {
	pub being_shown: bool,
	pub status: co::SW_S,
}

impl MsgSend for ShowWindow {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SHOWWINDOW,
			wparam: self.being_shown as _,
			lparam: self.status.raw() as _,
		}
	}
}

impl MsgSendRecv for ShowWindow {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			being_shown: p.wparam != 0,
			status: co::SW_S::from_raw(p.lparam as _),
		}
	}
}

/// [`WM_SIZE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-size)
/// message parameters.
///
/// Return type: `()`.
pub struct Size {
	pub request: co::SIZE_R,
	pub client_area: SIZE,
}

impl MsgSend for Size {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SIZE,
			wparam: self.request.raw() as _,
			lparam: u32::from(self.client_area) as _,
		}
	}
}

impl MsgSendRecv for Size {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			request: co::SIZE_R::from_raw(p.wparam as _),
			client_area: SIZE::new(
				LOWORD(p.lparam as _) as _,
				HIWORD(p.lparam as _) as _,
			),
		}
	}
}

/// [`WM_SIZING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-sizing)
/// message parameters.
///
/// Return type: `()`.
pub struct Sizing<'a> {
	pub window_edge: co::WMSZ,
	pub coords: &'a mut RECT,
}

impl<'a> MsgSend for Sizing<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SIZING,
			wparam: self.window_edge.raw() as _,
			lparam: self.coords as *mut _ as _,
		}
	}
}

impl<'a> MsgSendRecv for Sizing<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			window_edge: co::WMSZ::from_raw(p.wparam as _),
			coords: &mut *(p.lparam as *mut _),
		}
	}
}

/// [`WM_STYLECHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanged)
/// message parameters.
///
/// Return type: `()`.
pub struct StyleChanged<'a> {
	pub change: co::GWL_C,
	pub stylestruct: &'a STYLESTRUCT,
}

impl<'a> MsgSend for StyleChanged<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::STYLECHANGED,
			wparam: self.change.raw() as _,
			lparam: self.stylestruct as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for StyleChanged<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			change: co::GWL_C::from_raw(p.wparam as _),
			stylestruct: &*(p.lparam as *const _),
		}
	}
}

/// [`WM_STYLECHANGING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanging)
/// message parameters.
///
/// Return type: `()`.
pub struct StyleChanging<'a> {
	pub change: co::GWL_C,
	pub stylestruct: &'a STYLESTRUCT,
}

impl<'a> MsgSend for StyleChanging<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::STYLECHANGING,
			wparam: self.change.raw() as _,
			lparam: self.stylestruct as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for StyleChanging<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			change: co::GWL_C::from_raw(p.wparam as _),
			stylestruct: &*(p.lparam as *const _),
		}
	}
}

pub_struct_msg_char_code! { SysChar: co::WM::SYSCHAR;
	/// [`WM_SYSCHAR`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syschar)
}

/// [`WM_SYSCOMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syscommand)
/// message parameters.
///
/// Return type: `()`.
pub struct SysCommand {
	pub request: co::SC,
	pub position: POINT,
}

impl MsgSend for SysCommand {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SYSCOMMAND,
			wparam: self.request.raw() as _,
			lparam: u32::from(self.position) as _,
		}
	}
}

impl MsgSendRecv for SysCommand {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			request: co::SC::from_raw(p.wparam as _),
			position: POINT::from(p.lparam as u32),
		}
	}
}

pub_struct_msg_char_code! { SysDeadChar: co::WM::SYSDEADCHAR;
	/// [`WM_SYSDEADCHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-sysdeadchar)
}

pub_struct_msg_char_key! { SysKeyDown: co::WM::SYSKEYDOWN;
	/// [`WM_SYSKEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-syskeydown)
}

pub_struct_msg_char_key! { SysKeyUp: co::WM::SYSKEYUP;
	/// [`WM_SYSKEYUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-syskeyup)
}

pub_struct_msg_empty_handleable! { ThemeChanged: co::WM::THEMECHANGED;
	/// [`WM_THEMECHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-themechanged)
}

/// [`WM_TIMER`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-timer)
/// message parameters.
///
/// Return type: `()`.
pub struct Timer {
	pub timer_id: usize,
	pub timer_proc: Option<TIMERPROC>,
}

impl MsgSend for Timer {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::TIMER,
			wparam: self.timer_id,
			lparam: self.timer_proc.map_or(0, |proc| proc as _),
		}
	}
}

impl MsgSendRecv for Timer {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			timer_id: p.wparam as _,
			timer_proc: match p.lparam {
				0 => None,
				addr => std::mem::transmute(addr),
			},
		}
	}
}

/// [`WM_UNINITMENUPOPUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-uninitmenupopup)
/// message parameters.
///
/// Return type: `()`.
pub struct UninitMenuPopup {
	pub hmenu: HMENU,
	pub which: co::MF,
}

impl MsgSend for UninitMenuPopup {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::UNINITMENUPOPUP,
			wparam: self.hmenu.ptr() as _,
			lparam: MAKEDWORD(0, self.which.raw() as _) as _,
		}
	}
}

impl MsgSendRecv for UninitMenuPopup {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hmenu: HMENU::from_ptr(p.wparam as _),
			which: co::MF::from_raw(LOWORD(p.lparam as _) as _),
		}
	}
}

/// [`WM_UNDO`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-undo)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct Undo {}

impl MsgSend for Undo {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::UNDO,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for Undo {
	unsafe fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

/// [`WM_VSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-vscroll)
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

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::VSCROLL,
			wparam: MAKEDWORD(self.request.raw(), self.scroll_box_pos) as _,
			lparam: self.hcontrol.as_ref().map_or(0, |h| h.ptr() as _),
		}
	}
}

impl MsgSendRecv for VScroll {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			scroll_box_pos: HIWORD(p.wparam as _),
			request: co::SB_REQ::from_raw(LOWORD(p.wparam as _)),
			hcontrol: match p.lparam {
				0 => None,
				ptr => Some(HWND::from_ptr(ptr as _)),
			},
		}
	}
}

/// [`WM_WINDOWPOSCHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanged)
/// message parameters.
///
/// Return type: `()`.
pub struct WindowPosChanged<'a> {
	pub windowpos: &'a WINDOWPOS,
}

impl<'a> MsgSend for WindowPosChanged<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::WINDOWPOSCHANGED,
			wparam: 0,
			lparam: self.windowpos as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WindowPosChanged<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			windowpos: &*(p.lparam as *const _),
		}
	}
}

/// [`WM_WINDOWPOSCHANGING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanging)
/// message parameters.
///
/// Return type: `()`.
pub struct WindowPosChanging<'a> {
	pub windowpos: &'a WINDOWPOS,
}

impl<'a> MsgSend for WindowPosChanging<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::WINDOWPOSCHANGING,
			wparam: 0,
			lparam: self.windowpos as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WindowPosChanging<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			windowpos: &*(p.lparam as *const _),
		}
	}
}

/// [`WM_WTSSESSION_CHANGE`](https://learn.microsoft.com/en-us/windows/win32/termserv/wm-wtssession-change)
/// message parameters.
///
/// Return type: `()`.
pub struct WtsSessionChange {
	pub state: co::WTS,
	pub session_id: u32,
}

impl MsgSend for WtsSessionChange {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::WTSSESSION_CHANGE,
			wparam: self.state.raw() as _,
			lparam: self.session_id as _,
		}
	}
}

impl MsgSendRecv for WtsSessionChange {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			state: co::WTS::from_raw(p.wparam as _),
			session_id: p.lparam as _,
		}
	}
}

pub_struct_msg_button! { XButtonDblClk: co::WM::XBUTTONDBLCLK;
	/// [`WM_XBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondblclk)
}

pub_struct_msg_button! { XButtonDown: co::WM::XBUTTONDOWN;
	/// [`WM_XBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondown)
}

pub_struct_msg_button! { XButtonUp: co::WM::XBUTTONUP;
	/// [`WM_XBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttonup)
}
