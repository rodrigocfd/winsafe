use crate::co;
use crate::decl::*;
use crate::macros::*;
use crate::prelude::*;
use crate::user::privs::*;

/// Generic
/// [window message](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues)
/// parameters:
/// [`WPARAM`](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#wparam)
/// and
/// [`LPARAM`](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#lparam).
///
/// All message types can be converted to `Wm` via the
/// [`as_generic_wm`](crate::prelude::MsgSend::as_generic_wm) method.
///
/// Return type: `isize`.
#[derive(Clone, Copy, Debug)]
pub struct Wm {
	/// The [`co::WM`](crate::co::WM) constant that identifies the window
	/// message.
	pub msg_id: co::WM,
	/// First message parameter.
	pub wparam: usize,
	/// Second message parameter.
	pub lparam: isize,
}

impl MsgSend for Wm {
	type RetType = isize;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v
	}

	fn as_generic_wm(&mut self) -> Self {
		*self
	}
}

impl MsgSendRecv for Wm {
	unsafe fn from_generic_wm(p: Self) -> Self {
		p
	}
}

impl Wm {
	/// Creates a new `Wm` from the given message parameters.
	#[must_use]
	pub const fn new(msg_id: co::WM, wparam: usize, lparam: isize) -> Wm {
		Self { msg_id, wparam, lparam }
	}
}

/// [`WM_ACTIVATE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-activate)
/// message parameters.
///
/// Return type: `()`.
pub struct WmActivate {
	pub event: co::WA,
	pub is_minimized: bool,
	pub hwnd: HWND,
}

impl MsgSend for WmActivate {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::ACTIVATE,
			wparam: MAKEDWORD(self.event.raw(), self.is_minimized as _) as _,
			lparam: self.hwnd.ptr() as _,
		}
	}
}

impl MsgSendRecv for WmActivate {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			event: unsafe { co::WA::from_raw(LOWORD(p.wparam as _)) },
			is_minimized: HIWORD(p.wparam as _) != 0,
			hwnd: unsafe { HWND::from_ptr(p.lparam as _) },
		}
	}
}

/// [`WM_ACTIVATEAPP`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-activateapp)
/// message parameters.
///
/// Return type: `()`.
pub struct WmActivateApp {
	pub is_being_activated: bool,
	pub thread_id: u32,
}

impl MsgSend for WmActivateApp {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::ACTIVATEAPP,
			wparam: self.is_being_activated as _,
			lparam: self.thread_id as _,
		}
	}
}

impl MsgSendRecv for WmActivateApp {
	unsafe fn from_generic_wm(p: Wm) -> Self {
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
pub struct WmAppCommand {
	pub hwnd_owner: HWND,
	pub app_command: co::APPCOMMAND,
	pub u_device: co::FAPPCOMMAND,
	pub keys: co::MK,
}

impl MsgSend for WmAppCommand {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::APPCOMMAND,
			wparam: self.hwnd_owner.ptr() as _,
			lparam: MAKEDWORD(self.keys.into(), self.app_command.raw() | self.u_device.raw()) as _,
		}
	}
}

impl MsgSendRecv for WmAppCommand {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		unsafe {
			Self {
				hwnd_owner: HWND::from_ptr(p.wparam as _),
				app_command: co::APPCOMMAND::from_raw(HIWORD(p.lparam as _) & !FAPPCOMMAND_MASK),
				u_device: co::FAPPCOMMAND::from_raw(HIWORD(p.lparam as _) & FAPPCOMMAND_MASK),
				keys: co::MK::from_raw(LOWORD(p.lparam as _)),
			}
		}
	}
}

pub_struct_msg_empty_handleable! { WmCancelMode: co::WM::CANCELMODE;
	/// [`WM_CANCELMODE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-cancelmode)
}

/// [`WM_CAPTURECHANGED`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-capturechanged)
/// message parameters.
///
/// Return type: `()`.
pub struct WmCaptureChanged {
	pub hwnd_gaining_mouse: HWND,
}

impl MsgSend for WmCaptureChanged {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::CAPTURECHANGED,
			wparam: self.hwnd_gaining_mouse.ptr() as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for WmCaptureChanged {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			hwnd_gaining_mouse: unsafe { HWND::from_ptr(p.wparam as _) },
		}
	}
}

pub_struct_msg_char_code! { WmChar: co::WM::CHAR;
	/// [`WM_CHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-char)
}

pub_struct_msg_empty_handleable! { WmChildActivate: co::WM::CHILDACTIVATE;
	/// [`WM_CHILDACTIVATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-childactivate)
}

pub_struct_msg_empty_handleable! { WmClose: co::WM::CLOSE;
	/// [`WM_CLOSE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-close)
}

/// [`WM_COMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-command)
/// message parameters.
///
/// Return type: `()`.
pub struct WmCommand {
	pub event: AccelMenuCtrl,
}

impl MsgSend for WmCommand {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::COMMAND,
			wparam: match &self.event {
				AccelMenuCtrl::Accel(id) => MAKEDWORD(*id, 1) as _,
				AccelMenuCtrl::Menu(id) => MAKEDWORD(*id, 0) as _,
				AccelMenuCtrl::Ctrl { notif_code, ctrl_id, ctrl_hwnd: _ } => {
					MAKEDWORD(*ctrl_id, notif_code.raw()) as _
				},
			},
			lparam: match &self.event {
				AccelMenuCtrl::Accel(_) => co::CMD::Accel.raw() as _,
				AccelMenuCtrl::Menu(_) => co::CMD::Menu.raw() as _,
				AccelMenuCtrl::Ctrl { notif_code: _, ctrl_id: _, ctrl_hwnd } => {
					ctrl_hwnd.ptr() as _
				},
			},
		}
	}
}

impl MsgSendRecv for WmCommand {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			event: match HIWORD(p.wparam as _) {
				1 => AccelMenuCtrl::Accel(LOWORD(p.wparam as _)),
				0 => AccelMenuCtrl::Menu(LOWORD(p.wparam as _)),
				code => AccelMenuCtrl::Ctrl {
					notif_code: unsafe { co::CMD::from_raw(code) },
					ctrl_id: LOWORD(p.wparam as _),
					ctrl_hwnd: unsafe { HWND::from_ptr(p.lparam as _) },
				},
			},
		}
	}
}

/// [`WM_CONTEXTMENU`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-contextmenu)
/// message parameters.
///
/// Return type: `()`.
pub struct WmContextMenu {
	pub hwnd: HWND,
	pub cursor_pos: POINT,
}

impl MsgSend for WmContextMenu {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::CONTEXTMENU,
			wparam: self.hwnd.ptr() as _,
			lparam: u32::from(self.cursor_pos) as _,
		}
	}
}

impl MsgSendRecv for WmContextMenu {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			hwnd: unsafe { HWND::from_ptr(p.wparam as _) },
			cursor_pos: POINT::from(p.lparam as u32),
		}
	}
}

/// [`WM_CREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create)
/// message parameters.
///
/// Return type: `i32`.
pub struct WmCreate<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

impl<'a, 'b, 'c> MsgSend for WmCreate<'a, 'b, 'c> {
	type RetType = i32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::CREATE,
			wparam: 0,
			lparam: self.createstruct as *const _ as _,
		}
	}
}

impl<'a, 'b, 'c> MsgSendRecv for WmCreate<'a, 'b, 'c> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			createstruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

/// [`WM_DELETEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-deleteitem)
/// message parameters.
///
/// Return type: `()`.
pub struct WmDeleteItem<'a> {
	pub control_id: u16,
	pub deleteitemstruct: &'a DELETEITEMSTRUCT,
}

impl<'a> MsgSend for WmDeleteItem<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::DELETEITEM,
			wparam: self.control_id as _,
			lparam: self.deleteitemstruct as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WmDeleteItem<'a> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			control_id: p.wparam as _,
			deleteitemstruct: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

pub_struct_msg_empty_handleable! { WmDestroy: co::WM::DESTROY;
	/// [`WM_DESTROY`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-destroy)
}

/// [`WM_DEVICECHANGE`](https://learn.microsoft.com/en-us/windows/win32/devio/wm-devicechange)
/// message parameters.
///
/// Return type: `()`.
pub struct WmDeviceChange<'a> {
	pub event: co::DBT,
	pub data: Option<&'a DEV_BROADCAST_HDR>,
}

impl<'a> MsgSend for WmDeviceChange<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::DEVICECHANGE,
			wparam: self.event.raw() as _,
			lparam: self.data.map_or(0, |d| d as *const _ as _),
		}
	}
}

impl<'a> MsgSendRecv for WmDeviceChange<'a> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		unsafe {
			Self {
				event: co::DBT::from_raw(p.wparam as _),
				data: if p.lparam == 0 { None } else { Some(&*(p.lparam as *const _)) },
			}
		}
	}
}

/// [`WM_ENABLE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-enable)
/// message parameters.
///
/// Return type: `()`.
pub struct WmEnable {
	pub has_been_enabled: bool,
}

impl MsgSend for WmEnable {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::ENABLE,
			wparam: self.has_been_enabled as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for WmEnable {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self { has_been_enabled: p.wparam != 0 }
	}
}

/// [`WM_ENDSESSION`](https://learn.microsoft.com/en-us/windows/win32/shutdown/wm-endsession)
/// message parameters.
///
/// Return type: `()`.
pub struct WmEndSession {
	pub is_session_being_ended: bool,
	pub event: co::ENDSESSION,
}

impl MsgSend for WmEndSession {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::ENDSESSION,
			wparam: self.is_session_being_ended as _,
			lparam: self.event.raw() as _,
		}
	}
}

impl MsgSendRecv for WmEndSession {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			is_session_being_ended: p.wparam != 0,
			event: unsafe { co::ENDSESSION::from_raw(p.lparam as _) },
		}
	}
}

/// [`WM_ENTERIDLE`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-enteridle)
/// message parameters.
///
/// Return type: `()`.
pub struct WmEnterIdle {
	pub reason: co::MSGF,
	pub handle: HwndHmenu,
}

impl MsgSend for WmEnterIdle {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::ENTERIDLE,
			wparam: self.reason.raw() as _,
			lparam: self.handle.as_isize(),
		}
	}
}

impl MsgSendRecv for WmEnterIdle {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		let reason = unsafe { co::MSGF::from_raw(p.wparam as _) };
		Self {
			reason,
			handle: unsafe {
				match reason {
					co::MSGF::DIALOGBOX => HwndHmenu::Hwnd(HWND::from_ptr(p.lparam as _)),
					_ => HwndHmenu::Hmenu(HMENU::from_ptr(p.lparam as _)),
				}
			},
		}
	}
}

/// [`WM_ENTERMENULOOP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-entermenuloop)
/// message parameters.
///
/// Return type: `()`.
pub struct WmEnterMenuLoop {
	pub with_trackpopupmenu: bool,
}

impl MsgSend for WmEnterMenuLoop {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::ENTERMENULOOP,
			wparam: self.with_trackpopupmenu as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for WmEnterMenuLoop {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self { with_trackpopupmenu: p.wparam != 0 }
	}
}

pub_struct_msg_empty_handleable! { WmEnterSizeMove: co::WM::ENTERSIZEMOVE;
	/// [`WM_ENTERSIZEMOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-entersizemove)
}

/// [`WM_ERASEBKGND`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-erasebkgnd)
/// message parameters.
///
/// Return type: `i32`.
pub struct WmEraseBkgnd {
	pub hdc: HDC,
}

impl MsgSend for WmEraseBkgnd {
	type RetType = i32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::ERASEBKGND,
			wparam: self.hdc.ptr() as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for WmEraseBkgnd {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			hdc: unsafe { HDC::from_ptr(p.wparam as _) },
		}
	}
}

/// [`WM_EXITMENULOOP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-exitmenuloop)
/// message parameters.
///
/// Return type: `()`.
pub struct WmExitMenuLoop {
	pub is_shortcut: bool,
}

impl MsgSend for WmExitMenuLoop {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::EXITMENULOOP,
			wparam: self.is_shortcut as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for WmExitMenuLoop {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self { is_shortcut: p.wparam != 0 }
	}
}

pub_struct_msg_empty_handleable! { WmExitSizeMove: co::WM::EXITSIZEMOVE;
	/// [`WM_EXITSIZEMOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-exitsizemove)
}

/// [`WM_GETDLGCODE`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-getdlgcode)
/// message parameters.
///
/// Return type: `co::DLGC`.
pub struct WmGetDlgCode<'a> {
	pub vkey_code: co::VK,
	pub msg: Option<&'a mut MSG>,
	pub is_query: bool,
}

impl<'a> MsgSend for WmGetDlgCode<'a> {
	type RetType = co::DLGC;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::DLGC::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::GETDLGCODE,
			wparam: self.vkey_code.raw() as _,
			lparam: self.msg.as_mut().map_or(0, |m| m as *mut _ as _),
		}
	}
}

impl<'a> MsgSendRecv for WmGetDlgCode<'a> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			vkey_code: unsafe { co::VK::from_raw(p.wparam as _) },
			msg: match p.lparam {
				0 => None,
				ptr => Some(unsafe { &mut *(ptr as *mut _) }),
			},
			is_query: p.lparam == 0,
		}
	}
}

/// [`WM_GETHMENU`](https://learn.microsoft.com/en-us/windows/win32/winmsg/mn-gethmenu)
/// message, which has no parameters. Originally has `MN` prefix.
///
/// Return type: `Option<HMENU>`.
pub struct WmGetHMenu {}

impl MsgSend for WmGetHMenu {
	type RetType = Option<HMENU>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HMENU::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::MN_GETHMENU,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for WmGetHMenu {
	unsafe fn from_generic_wm(_: Wm) -> Self {
		Self {}
	}
}

/// [`WM_GETHOTKEY`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-gethotkey)
/// message, which has no parameters.
///
/// Return type: `(co::VK, co::HOTKEYF)`.
pub struct WmGetHotKey {}

impl MsgSend for WmGetHotKey {
	type RetType = (co::VK, co::HOTKEYF);

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe {
			(co::VK::from_raw(LOBYTE(v as _) as _), co::HOTKEYF::from_raw(HIBYTE(v as _) as _))
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::GETHOTKEY,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for WmGetHotKey {
	unsafe fn from_generic_wm(_: Wm) -> Self {
		Self {}
	}
}

/// [`WM_GETMINMAXINFO`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-getminmaxinfo)
/// message parameters.
///
/// Return type: `()`.
pub struct WmGetMinMaxInfo<'a> {
	pub info: &'a mut MINMAXINFO,
}

impl<'a> MsgSend for WmGetMinMaxInfo<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::GETMINMAXINFO,
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WmGetMinMaxInfo<'a> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			info: unsafe { &mut *(p.lparam as *mut _) },
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
/// let needed_len = unsafe { hwnd.SendMessage(msg::WmGetTextLength {}) };
/// let mut buf = w::WString::new_alloc_buf(needed_len as _);
///
/// unsafe {
///     hwnd.SendMessage(
///         msg::WmGetText {
///             buffer: buf.as_mut_slice(),
///         },
///     );
/// }
///
/// println!("Text: {}", buf.to_string());
/// ```
pub struct WmGetText<'a> {
	pub buffer: &'a mut [u16], // can't be WString because this message can be received
}

impl<'a> MsgSend for WmGetText<'a> {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::GETTEXT,
			wparam: self.buffer.len(),
			lparam: self.buffer.as_mut_ptr() as _,
		}
	}
}

impl<'a> MsgSendRecv for WmGetText<'a> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			buffer: unsafe { std::slice::from_raw_parts_mut(p.lparam as _, p.wparam) },
		}
	}
}

/// [`WM_GETTEXTLENGTH`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-gettextlength)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct WmGetTextLength {}

impl MsgSend for WmGetTextLength {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::GETTEXTLENGTH,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for WmGetTextLength {
	unsafe fn from_generic_wm(_: Wm) -> Self {
		Self {}
	}
}

/// [`WM_GETTITLEBARINFOEX`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-gettitlebarinfoex)
/// message parameters.
///
/// Return type: `()`.
pub struct WmGetTitleBarInfoEx<'a> {
	pub info: &'a mut TITLEBARINFOEX,
}

impl<'a> MsgSend for WmGetTitleBarInfoEx<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::GETTITLEBARINFOEX,
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WmGetTitleBarInfoEx<'a> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			info: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_HELP`](https://learn.microsoft.com/en-us/windows/win32/shell/wm-help)
/// message parameters.
///
/// Return type: `()`.
pub struct WmHelp<'a> {
	pub helpinfo: &'a HELPINFO,
}

impl<'a> MsgSend for WmHelp<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::HELP,
			wparam: 0,
			lparam: self.helpinfo as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WmHelp<'a> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			helpinfo: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_HSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-hscroll)
/// message parameters.
///
/// Return type: `()`.
pub struct WmHScroll {
	pub scroll_box_pos: u16,
	pub request: co::SB_REQ,
	pub hcontrol: Option<HWND>,
}

impl MsgSend for WmHScroll {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::HSCROLL,
			wparam: MAKEDWORD(self.request.raw(), self.scroll_box_pos) as _,
			lparam: self.hcontrol.as_ref().map_or(0, |h| h.ptr() as _),
		}
	}
}

impl MsgSendRecv for WmHScroll {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			scroll_box_pos: HIWORD(p.wparam as _),
			request: unsafe { co::SB_REQ::from_raw(LOWORD(p.wparam as _)) },
			hcontrol: match p.lparam {
				0 => None,
				ptr => Some(unsafe { HWND::from_ptr(ptr as _) }),
			},
		}
	}
}

/// [`WM_INITDIALOG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
/// message parameters.
///
/// Return type: `bool`.
pub struct WmInitDialog {
	pub hwnd_focus: HWND,
	pub additional_data: isize,
}

impl MsgSend for WmInitDialog {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::INITDIALOG,
			wparam: self.hwnd_focus.ptr() as _,
			lparam: self.additional_data,
		}
	}
}

impl MsgSendRecv for WmInitDialog {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			hwnd_focus: unsafe { HWND::from_ptr(p.wparam as _) },
			additional_data: p.lparam,
		}
	}
}

/// [`WM_INITMENUPOPUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-initmenupopup)
/// message parameters.
///
/// Return type: `()`.
pub struct WmInitMenuPopup {
	pub hmenu: HMENU,
	pub item_pos: u16,
	pub is_window_menu: bool,
}

impl MsgSend for WmInitMenuPopup {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::INITMENUPOPUP,
			wparam: self.hmenu.ptr() as _,
			lparam: MAKEDWORD(self.item_pos, self.is_window_menu as _) as _,
		}
	}
}

impl MsgSendRecv for WmInitMenuPopup {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			hmenu: unsafe { HMENU::from_ptr(p.wparam as _) },
			item_pos: LOWORD(p.lparam as _),
			is_window_menu: HIWORD(p.lparam as _) != 0,
		}
	}
}

pub_struct_msg_char_key! { WmKeyDown: co::WM::KEYDOWN;
	/// [`WM_KEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keydown)
}

pub_struct_msg_char_key! { WmKeyUp: co::WM::KEYUP;
	/// [`WM_KEYUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keyup)
}

/// [`WM_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-killfocus)
/// message parameters.
///
/// Return type: `()`.
pub struct WmKillFocus {
	pub hwnd: Option<HWND>,
}

impl MsgSend for WmKillFocus {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::KILLFOCUS,
			wparam: self.hwnd.as_ref().map_or(0, |h| h.ptr() as _),
			lparam: 0,
		}
	}
}

impl MsgSendRecv for WmKillFocus {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			hwnd: match p.wparam {
				0 => None,
				ptr => Some(unsafe { HWND::from_ptr(ptr as _) }),
			},
		}
	}
}

pub_struct_msg_button! { WmLButtonDblClk: co::WM::LBUTTONDBLCLK;
	/// [`WM_LBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondblclk)
}

pub_struct_msg_button! { WmLButtonDown: co::WM::LBUTTONDOWN;
	/// [`WM_LBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown)
}

pub_struct_msg_button! { WmLButtonUp: co::WM::LBUTTONUP;
	/// [`WM_LBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttonup)
}

pub_struct_msg_button! { WmMButtonDblClk: co::WM::MBUTTONDBLCLK;
	/// [`WM_MBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondblclk)
}

pub_struct_msg_button! { WmMButtonDown: co::WM::MBUTTONDOWN;
	/// [`WM_MBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondown)
}

pub_struct_msg_button! { WmMButtonUp: co::WM::MBUTTONUP;
	/// [`WM_MBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttonup)
}

/// [`WM_MENUCOMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menucommand)
/// message parameters.
///
/// Return type: `()`.
pub struct WmMenuCommand {
	pub item_index: u32,
	pub hmenu: HMENU,
}

impl MsgSend for WmMenuCommand {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::MENUCOMMAND,
			wparam: self.item_index as _,
			lparam: self.hmenu.ptr() as _,
		}
	}
}

impl MsgSendRecv for WmMenuCommand {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			item_index: p.wparam as _,
			hmenu: unsafe { HMENU::from_ptr(p.lparam as _) },
		}
	}
}

/// [`WM_MENUDRAG`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menudrag)
/// message parameters.
///
/// Return type: `co::MND`.
pub struct WmMenuDrag {
	pub position: u32,
	pub hmenu: HMENU,
}

impl MsgSend for WmMenuDrag {
	type RetType = co::MND;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::MND::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::MENUDRAG,
			wparam: self.position as _,
			lparam: self.hmenu.ptr() as _,
		}
	}
}

impl MsgSendRecv for WmMenuDrag {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			position: p.wparam as _,
			hmenu: unsafe { HMENU::from_ptr(p.lparam as _) },
		}
	}
}

/// [`WM_MENURBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menurbuttonup)
/// message parameters.
///
/// Return type: `()`.
pub struct WmMenuRButtonUp {
	pub position: u32,
	pub hmenu: HMENU,
}

impl MsgSend for WmMenuRButtonUp {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::MENURBUTTONUP,
			wparam: self.position as _,
			lparam: self.hmenu.ptr() as _,
		}
	}
}

impl MsgSendRecv for WmMenuRButtonUp {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			position: p.wparam as _,
			hmenu: unsafe { HMENU::from_ptr(p.lparam as _) },
		}
	}
}

pub_struct_msg_button! { WmMouseHover: co::WM::MOUSEHOVER;
	/// [`WM_MOUSEHOVER`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousehover)
}

/// [`WM_MOUSEHWHEEL`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousehwheel)
/// message parameters.
///
/// Return type: `()`.
pub struct WmMouseHWheel {
	pub wheel_distance: i16,
	pub keys: co::MK,
	pub coords: POINT,
}

impl MsgSend for WmMouseHWheel {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::MOUSEHWHEEL,
			wparam: MAKEDWORD(self.wheel_distance as _, self.keys.raw()) as _,
			lparam: u32::from(self.coords) as _,
		}
	}
}

impl MsgSendRecv for WmMouseHWheel {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			wheel_distance: LOWORD(p.wparam as _) as _,
			keys: unsafe { co::MK::from_raw(HIWORD(p.wparam as _)) },
			coords: POINT::from(p.lparam as u32),
		}
	}
}

pub_struct_msg_empty_handleable! { WmMouseLeave: co::WM::MOUSELEAVE;
	/// [`WM_MOUSELEAVE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mouseleave)
}

pub_struct_msg_button! { WmMouseMove: co::WM::MOUSEMOVE;
	/// [`WM_MOUSEMOVE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove)
}

/// [`WM_MOUSEWHEEL`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel)
/// message parameters.
///
/// Return type: `()`.
pub struct WmMouseWheel {
	pub wheel_distance: i16,
	pub keys: co::MK,
	pub coords: POINT,
}

impl MsgSend for WmMouseWheel {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::MOUSEWHEEL,
			wparam: MAKEDWORD(self.wheel_distance as _, self.keys.raw()) as _,
			lparam: u32::from(self.coords) as _,
		}
	}
}

impl MsgSendRecv for WmMouseWheel {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			wheel_distance: LOWORD(p.wparam as _) as _,
			keys: unsafe { co::MK::from_raw(HIWORD(p.wparam as _)) },
			coords: POINT::from(p.lparam as u32),
		}
	}
}

/// [`WM_MOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-move)
/// message parameters.
///
/// Return type: `()`.
pub struct WmMove {
	pub coords: POINT,
}

impl MsgSend for WmMove {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::MOVE,
			wparam: 0,
			lparam: u32::from(self.coords) as _,
		}
	}
}

impl MsgSendRecv for WmMove {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self { coords: POINT::from(p.lparam as u32) }
	}
}

/// [`WM_MOVING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-moving)
/// message parameters.
///
/// Return type: `()`.
pub struct WmMoving<'a> {
	pub window_pos: &'a mut RECT,
}

impl<'a> MsgSend for WmMoving<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::MOVING,
			wparam: 0,
			lparam: self.window_pos as *mut _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WmMoving<'a> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			window_pos: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_NCCALCSIZE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize)
/// message parameters.
///
/// Return type: `co::WVR`.
pub struct WmNcCalcSize<'a, 'b> {
	pub data: NccspRect<'a, 'b>,
}

impl<'a, 'b> MsgSend for WmNcCalcSize<'a, 'b> {
	type RetType = co::WVR;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::WVR::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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

impl<'a, 'b> MsgSendRecv for WmNcCalcSize<'a, 'b> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			data: unsafe {
				match p.wparam {
					0 => NccspRect::Rect(&mut *(p.lparam as *mut _)),
					_ => NccspRect::Nccsp(&mut *(p.lparam as *mut _)),
				}
			},
		}
	}
}

/// [`WM_NCCREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccreate)
/// message parameters.
///
/// Return type: `bool`.
pub struct WmNcCreate<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

impl<'a, 'b, 'c> MsgSend for WmNcCreate<'a, 'b, 'c> {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::NCCREATE,
			wparam: 0,
			lparam: self.createstruct as *const _ as _,
		}
	}
}

impl<'a, 'b, 'c> MsgSendRecv for WmNcCreate<'a, 'b, 'c> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			createstruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

pub_struct_msg_empty_handleable! { WmNcDestroy: co::WM::NCDESTROY;
	/// [`WM_NCDESTROY`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-ncdestroy)
}

/// [`WM_NCHITTEST`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-nchittest)
/// message parameters.
///
/// Return type: `co::HT`.
pub struct WmNcHitTest {
	pub cursor_pos: POINT,
}

impl MsgSend for WmNcHitTest {
	type RetType = co::HT;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::HT::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::NCHITTEST,
			wparam: 0,
			lparam: u32::from(self.cursor_pos) as _,
		}
	}
}

impl MsgSendRecv for WmNcHitTest {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self { cursor_pos: POINT::from(p.lparam as u32) }
	}
}

/// [`WM_NEXTDLGCTL`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-nextdlgctl)
/// message parameters.
///
/// Return type: `()`.
pub struct WmNextDlgCtl {
	pub hwnd_focus: HwndFocus,
}

impl MsgSend for WmNextDlgCtl {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::NEXTDLGCTL,
			wparam: match &self.hwnd_focus {
				HwndFocus::Hwnd(hctl) => hctl.ptr() as _,
				HwndFocus::FocusNext(next) => {
					if *next {
						0
					} else {
						1
					}
				},
			},
			lparam: MAKEDWORD(
				match &self.hwnd_focus {
					HwndFocus::Hwnd(_) => 1,
					HwndFocus::FocusNext(_) => 0,
				},
				0,
			) as _,
		}
	}
}

impl MsgSendRecv for WmNextDlgCtl {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			hwnd_focus: match p.wparam {
				1 => HwndFocus::Hwnd(unsafe { HWND::from_ptr(p.wparam as _) }),
				_ => HwndFocus::FocusNext(p.wparam == 0),
			},
		}
	}
}

pub_struct_msg_empty_handleable! { WmNull: co::WM::NULL;
	/// [`WM_NULL`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-null)
}

/// [`WM_PARENTNOTIFY`](https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-parentnotify)
/// message parameters.
///
/// Return type: `()`.
pub struct WmParentNotify {
	pub event: co::WMPN,
	pub child_id: u16,
	pub data: HwndPointId,
}

impl MsgSend for WmParentNotify {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::PARENTNOTIFY,
			wparam: MAKEDWORD(self.event.raw(), self.child_id) as _,
			lparam: self.data.as_isize(),
		}
	}
}

impl MsgSendRecv for WmParentNotify {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		let event = unsafe { co::WMPN::from_raw(LOWORD(p.wparam as _)) };
		Self {
			event,
			child_id: HIWORD(p.wparam as _),
			data: match event {
				co::WMPN::CREATE | co::WMPN::DESTROY => {
					HwndPointId::Hwnd(unsafe { HWND::from_ptr(p.lparam as _) })
				},
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
pub struct WmPowerBroadcast<'a> {
	pub event: co::PBT,
	pub data: Option<&'a POWERBROADCAST_SETTING>,
}

impl<'a> MsgSend for WmPowerBroadcast<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::POWERBROADCAST,
			wparam: self.event.raw() as _,
			lparam: self.data.map_or(0, |d| d as *const _ as _),
		}
	}
}

impl<'a> MsgSendRecv for WmPowerBroadcast<'a> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		unsafe {
			Self {
				event: co::PBT::from_raw(p.wparam as _),
				data: if p.lparam == 0 { None } else { Some(&*(p.lparam as *const _)) },
			}
		}
	}
}

/// [`WM_QUERYOPEN`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-queryopen)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct WmQueryOpen {}

impl MsgSend for WmQueryOpen {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::QUERYOPEN,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for WmQueryOpen {
	unsafe fn from_generic_wm(_: Wm) -> Self {
		Self {}
	}
}

pub_struct_msg_button! { WmRButtonDblClk: co::WM::RBUTTONDBLCLK;
	/// [`WM_RBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondblclk)
}

pub_struct_msg_button! { WmRButtonDown: co::WM::RBUTTONDOWN;
	/// [`WM_RBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondown)
}

pub_struct_msg_button! { WmRButtonUp: co::WM::RBUTTONUP;
	/// [`WM_RBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttonup)
}

/// [`WM_SETCURSOR`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-setcursor)
/// message parameters.
///
/// Return type: `bool`.
pub struct WmSetCursor {
	pub hwnd: HWND,
	pub hit_test: co::HT,
	pub mouse_msg: u16,
}

impl MsgSend for WmSetCursor {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::SETCURSOR,
			wparam: self.hwnd.ptr() as _,
			lparam: MAKEDWORD(self.hit_test.raw(), self.mouse_msg) as _,
		}
	}
}

impl MsgSendRecv for WmSetCursor {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			hwnd: unsafe { HWND::from_ptr(p.wparam as _) },
			hit_test: unsafe { co::HT::from_raw(LOWORD(p.lparam as _)) },
			mouse_msg: HIWORD(p.lparam as _),
		}
	}
}

/// [`WM_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-setfocus)
/// message parameters.
///
/// Return type: `()`.
pub struct WmSetFocus {
	pub hwnd_losing_focus: HWND,
}

impl MsgSend for WmSetFocus {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::SETFOCUS,
			wparam: self.hwnd_losing_focus.ptr() as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for WmSetFocus {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			hwnd_losing_focus: unsafe { HWND::from_ptr(p.wparam as _) },
		}
	}
}

/// [`WM_SETHOTKEY`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-sethotkey)
/// message parameters.
///
/// Return type: `i32`.
pub struct WmSetHotKey {
	pub vkey_code: co::VK,
	pub modifiers: co::HOTKEYF,
}

impl MsgSend for WmSetHotKey {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::SETHOTKEY,
			wparam: MAKEDWORD(self.vkey_code.raw(), self.modifiers.raw()) as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for WmSetHotKey {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		unsafe {
			Self {
				vkey_code: co::VK::from_raw(LOWORD(p.wparam as _)),
				modifiers: co::HOTKEYF::from_raw(HIWORD(p.wparam as _)),
			}
		}
	}
}

/// [`WM_SETICON`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-seticon)
/// message parameters.
///
/// Return type: `Option<HICON>`.
pub struct WmSetIcon {
	pub size: co::ICON_SZ,
	pub hicon: HICON,
}

impl MsgSend for WmSetIcon {
	type RetType = Option<HICON>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HICON::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::SETICON,
			wparam: self.size.raw() as _,
			lparam: self.hicon.ptr() as _,
		}
	}
}

impl MsgSendRecv for WmSetIcon {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		unsafe {
			Self {
				size: co::ICON_SZ::from_raw(p.wparam as _),
				hicon: HICON::from_ptr(p.lparam as _),
			}
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
///         msg::WmSetText {
///             text: unsafe { new_text.as_ptr() },
///         },
///     );
/// }
/// ```
pub struct WmSetText {
	pub text: *const u16, // can't be WString because this message can be received
}

impl MsgSend for WmSetText {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			0 | CB_ERRSPACE | CB_ERR => false,
			_ => true,
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::SETTEXT,
			wparam: 0,
			lparam: self.text as _,
		}
	}
}

impl MsgSendRecv for WmSetText {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self { text: p.lparam as _ }
	}
}

/// [`WM_SHOWWINDOW`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow)
/// message parameters.
///
/// Return type: `()`.
pub struct WmShowWindow {
	pub being_shown: bool,
	pub status: co::SW_S,
}

impl MsgSend for WmShowWindow {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::SHOWWINDOW,
			wparam: self.being_shown as _,
			lparam: self.status.raw() as _,
		}
	}
}

impl MsgSendRecv for WmShowWindow {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			being_shown: p.wparam != 0,
			status: unsafe { co::SW_S::from_raw(p.lparam as _) },
		}
	}
}

/// [`WM_SIZE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-size)
/// message parameters.
///
/// Return type: `()`.
pub struct WmSize {
	pub request: co::SIZE_R,
	pub client_area: SIZE,
}

impl MsgSend for WmSize {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::SIZE,
			wparam: self.request.raw() as _,
			lparam: u32::from(self.client_area) as _,
		}
	}
}

impl MsgSendRecv for WmSize {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			request: unsafe { co::SIZE_R::from_raw(p.wparam as _) },
			client_area: SIZE::with(LOWORD(p.lparam as _) as _, HIWORD(p.lparam as _) as _),
		}
	}
}

/// [`WM_SIZING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-sizing)
/// message parameters.
///
/// Return type: `()`.
pub struct WmSizing<'a> {
	pub window_edge: co::WMSZ,
	pub coords: &'a mut RECT,
}

impl<'a> MsgSend for WmSizing<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::SIZING,
			wparam: self.window_edge.raw() as _,
			lparam: self.coords as *mut _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WmSizing<'a> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		unsafe {
			Self {
				window_edge: co::WMSZ::from_raw(p.wparam as _),
				coords: &mut *(p.lparam as *mut _),
			}
		}
	}
}

/// [`WM_STYLECHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanged)
/// message parameters.
///
/// Return type: `()`.
pub struct WmStyleChanged<'a> {
	pub change: co::GWL_C,
	pub stylestruct: &'a STYLESTRUCT,
}

impl<'a> MsgSend for WmStyleChanged<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::STYLECHANGED,
			wparam: self.change.raw() as _,
			lparam: self.stylestruct as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WmStyleChanged<'a> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		unsafe {
			Self {
				change: co::GWL_C::from_raw(p.wparam as _),
				stylestruct: &*(p.lparam as *const _),
			}
		}
	}
}

/// [`WM_STYLECHANGING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanging)
/// message parameters.
///
/// Return type: `()`.
pub struct WmStyleChanging<'a> {
	pub change: co::GWL_C,
	pub stylestruct: &'a STYLESTRUCT,
}

impl<'a> MsgSend for WmStyleChanging<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::STYLECHANGING,
			wparam: self.change.raw() as _,
			lparam: self.stylestruct as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WmStyleChanging<'a> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		unsafe {
			Self {
				change: co::GWL_C::from_raw(p.wparam as _),
				stylestruct: &*(p.lparam as *const _),
			}
		}
	}
}

pub_struct_msg_char_code! { WmSysChar: co::WM::SYSCHAR;
	/// [`WM_SYSCHAR`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syschar)
}

/// [`WM_SYSCOMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syscommand)
/// message parameters.
///
/// Return type: `()`.
pub struct WmSysCommand {
	pub request: co::SC,
	pub position: POINT,
}

impl MsgSend for WmSysCommand {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::SYSCOMMAND,
			wparam: self.request.raw() as _,
			lparam: u32::from(self.position) as _,
		}
	}
}

impl MsgSendRecv for WmSysCommand {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			request: unsafe { co::SC::from_raw(p.wparam as _) },
			position: POINT::from(p.lparam as u32),
		}
	}
}

pub_struct_msg_char_code! { WmSysDeadChar: co::WM::SYSDEADCHAR;
	/// [`WM_SYSDEADCHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-sysdeadchar)
}

pub_struct_msg_char_key! { WmSysKeyDown: co::WM::SYSKEYDOWN;
	/// [`WM_SYSKEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-syskeydown)
}

pub_struct_msg_char_key! { WmSysKeyUp: co::WM::SYSKEYUP;
	/// [`WM_SYSKEYUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-syskeyup)
}

pub_struct_msg_empty_handleable! { WmThemeChanged: co::WM::THEMECHANGED;
	/// [`WM_THEMECHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-themechanged)
}

/// [`WM_TIMER`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-timer)
/// message parameters.
///
/// Return type: `()`.
pub struct WmTimer {
	pub timer_id: usize,
	pub timer_proc: Option<TIMERPROC>,
}

impl MsgSend for WmTimer {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::TIMER,
			wparam: self.timer_id,
			lparam: self.timer_proc.map_or(0, |proc| proc as _),
		}
	}
}

impl MsgSendRecv for WmTimer {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			timer_id: p.wparam as _,
			timer_proc: match p.lparam {
				0 => None,
				addr => unsafe { std::mem::transmute(addr) },
			},
		}
	}
}

/// [`WM_UNINITMENUPOPUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-uninitmenupopup)
/// message parameters.
///
/// Return type: `()`.
pub struct WmUninitMenuPopup {
	pub hmenu: HMENU,
	pub which: co::MF,
}

impl MsgSend for WmUninitMenuPopup {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::UNINITMENUPOPUP,
			wparam: self.hmenu.ptr() as _,
			lparam: MAKEDWORD(0, self.which.raw() as _) as _,
		}
	}
}

impl MsgSendRecv for WmUninitMenuPopup {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		unsafe {
			Self {
				hmenu: HMENU::from_ptr(p.wparam as _),
				which: co::MF::from_raw(LOWORD(p.lparam as _) as _),
			}
		}
	}
}

/// [`WM_UNDO`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-undo)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct WmUndo {}

impl MsgSend for WmUndo {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::UNDO,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for WmUndo {
	unsafe fn from_generic_wm(_: Wm) -> Self {
		Self {}
	}
}

/// [`WM_VSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-vscroll)
/// message parameters.
///
/// Return type: `()`.
pub struct WmVScroll {
	pub scroll_box_pos: u16,
	pub request: co::SB_REQ,
	pub hcontrol: Option<HWND>,
}

impl MsgSend for WmVScroll {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::VSCROLL,
			wparam: MAKEDWORD(self.request.raw(), self.scroll_box_pos) as _,
			lparam: self.hcontrol.as_ref().map_or(0, |h| h.ptr() as _),
		}
	}
}

impl MsgSendRecv for WmVScroll {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			scroll_box_pos: HIWORD(p.wparam as _),
			request: unsafe { co::SB_REQ::from_raw(LOWORD(p.wparam as _)) },
			hcontrol: match p.lparam {
				0 => None,
				ptr => Some(unsafe { HWND::from_ptr(ptr as _) }),
			},
		}
	}
}

/// [`WM_WINDOWPOSCHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanged)
/// message parameters.
///
/// Return type: `()`.
pub struct WmWindowPosChanged<'a> {
	pub windowpos: &'a WINDOWPOS,
}

impl<'a> MsgSend for WmWindowPosChanged<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::WINDOWPOSCHANGED,
			wparam: 0,
			lparam: self.windowpos as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WmWindowPosChanged<'a> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			windowpos: unsafe { &*(p.lparam as *const _) },
		}
	}
}

/// [`WM_WINDOWPOSCHANGING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanging)
/// message parameters.
///
/// Return type: `()`.
pub struct WmWindowPosChanging<'a> {
	pub windowpos: &'a WINDOWPOS,
}

impl<'a> MsgSend for WmWindowPosChanging<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::WINDOWPOSCHANGING,
			wparam: 0,
			lparam: self.windowpos as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WmWindowPosChanging<'a> {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			windowpos: unsafe { &*(p.lparam as *const _) },
		}
	}
}

/// [`WM_WTSSESSION_CHANGE`](https://learn.microsoft.com/en-us/windows/win32/termserv/wm-wtssession-change)
/// message parameters.
///
/// Return type: `()`.
pub struct WmWtsSessionChange {
	pub state: co::WTS,
	pub session_id: u32,
}

impl MsgSend for WmWtsSessionChange {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::WM::WTSSESSION_CHANGE,
			wparam: self.state.raw() as _,
			lparam: self.session_id as _,
		}
	}
}

impl MsgSendRecv for WmWtsSessionChange {
	unsafe fn from_generic_wm(p: Wm) -> Self {
		Self {
			state: unsafe { co::WTS::from_raw(p.wparam as _) },
			session_id: p.lparam as _,
		}
	}
}

pub_struct_msg_button! { WmXButtonDblClk: co::WM::XBUTTONDBLCLK;
	/// [`WM_XBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondblclk)
}

pub_struct_msg_button! { WmXButtonDown: co::WM::XBUTTONDOWN;
	/// [`WM_XBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondown)
}

pub_struct_msg_button! { WmXButtonUp: co::WM::XBUTTONUP;
	/// [`WM_XBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttonup)
}
