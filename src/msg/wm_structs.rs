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
pub struct WmActivate {
	pub event: co::WA,
	pub is_minimized: bool,
	pub hwnd: HWND,
}

impl Message for WmActivate {
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

impl MessageHandleable for WmActivate {
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
pub struct WmActivateApp {
	pub is_being_activated: bool,
	pub thread_id: u32,
}

impl Message for WmActivateApp {
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

impl MessageHandleable for WmActivateApp {
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
pub struct WmAppCommand {
	pub hwnd_owner: HWND,
	pub app_command: co::APPCOMMAND,
	pub u_device: co::FAPPCOMMAND,
	pub keys: co::MK,
}

impl Message for WmAppCommand {
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

impl MessageHandleable for WmAppCommand {
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

empty_msg_handleable! { WmCancelMode, co::WM::CANCELMODE,
	/// [`WM_CANCELMODE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-cancelmode)
	/// message, which has no parameters.
	///
	/// Return type: `()`.
}

empty_msg_handleable! { WmChildActivate, co::WM::CHILDACTIVATE,
	/// [`WM_CHILDACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-childactivate)
	/// message, which has no parameters.
	///
	/// Return type: `()`.
}

empty_msg_handleable! { WmClose, co::WM::CLOSE,
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
pub struct WmCommand {
	pub code: co::CMD,
	pub ctrl_id: u16,
	pub ctrl_hwnd: Option<HWND>,
}

impl Message for WmCommand {
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

impl MessageHandleable for WmCommand {
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
pub struct WmContextMenu {
	pub hwnd: HWND,
	pub cursor_pos: POINT,
}

impl Message for WmContextMenu {
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

impl MessageHandleable for WmContextMenu {
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
pub struct WmCreate<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

impl<'a, 'b, 'c> Message for WmCreate<'a, 'b, 'c> {
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

impl<'a, 'b, 'c> MessageHandleable for WmCreate<'a, 'b, 'c> {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			createstruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

//------------------------------------------------------------------------------

ctl_color_msg! { WmCtlColorBtn, co::WM::CTLCOLORBTN,
	/// [`WM_CTLCOLORBTN`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorbtn)
	/// message parameters.
	///
	/// Return type: `HBRUSH`.
}

ctl_color_msg! { WmCtlColorDlg, co::WM::CTLCOLORDLG,
	/// [`WM_CTLCOLORDLG`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-ctlcolordlg)
	/// message parameters.
	///
	/// Return type: `HBRUSH`.
}

ctl_color_msg! { WmCtlColorEdit, co::WM::CTLCOLOREDIT,
	/// [`WM_CTLCOLOREDIT`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcoloredit)
	/// message parameters.
	///
	/// Return type: `HBRUSH`.
}

ctl_color_msg! { WmCtlColorListBox, co::WM::CTLCOLORLISTBOX,
	/// [`WM_CTLCOLORLISTBOX`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorlistbox)
	/// message parameters.
	///
	/// Return type: `HBRUSH`.
}

ctl_color_msg! { WmCtlColorScrollBar, co::WM::CTLCOLORSCROLLBAR,
	/// [`WM_CTLCOLORSCROLLBAR`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorscrollbar)
	/// message parameters.
	///
	/// Return type: `HBRUSH`.
}

ctl_color_msg! { WmCtlColorStatic, co::WM::CTLCOLORSTATIC,
	/// [`WM_CTLCOLORSTATIC`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorstatic)
	/// message parameters.
	///
	/// Return type: `HBRUSH`.
}

//------------------------------------------------------------------------------

empty_msg_handleable! { WmDestroy, co::WM::DESTROY,
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
pub struct WmDropFiles {
	pub hdrop: HDROP,
}

impl Message for WmDropFiles {
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

impl MessageHandleable for WmDropFiles {
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
pub struct WmEnable {
	pub has_been_enabled: bool,
}

impl Message for WmEnable {
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

impl MessageHandleable for WmEnable {
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
pub struct WmEndSession {
	pub is_session_being_ended: bool,
	pub event: co::ENDSESSION,
}

impl Message for WmEndSession {
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

impl MessageHandleable for WmEndSession {
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
pub struct WmEnterIdle {
	pub reason: co::MSGF,
	pub handle: HwndHmenu,
}

impl Message for WmEnterIdle {
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

impl MessageHandleable for WmEnterIdle {
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

empty_msg_handleable! { WmEnterSizeMove, co::WM::ENTERSIZEMOVE,
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
pub struct WmEraseBkgnd {
	pub hdc: HDC,
}

impl Message for WmEraseBkgnd {
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

impl MessageHandleable for WmEraseBkgnd {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			hdc: HDC { ptr: p.wparam as *mut _ },
		}
	}
}

//------------------------------------------------------------------------------

empty_msg_handleable! { WmExitSizeMove, co::WM::EXITSIZEMOVE,
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
pub struct WmGetMinMaxInfo<'a> {
	pub info: &'a mut MINMAXINFO,
}

impl<'a> Message for WmGetMinMaxInfo<'a> {
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

impl<'a> MessageHandleable for WmGetMinMaxInfo<'a> {
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
pub struct WmInitDialog {
	pub hwnd_focus: HWND,
	pub additional_data: isize,
}

impl Message for WmInitDialog {
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

impl MessageHandleable for WmInitDialog {
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
pub struct WmInitMenuPopup {
	pub hmenu: HMENU,
	pub item_pos: u16,
	pub is_window_menu: bool,
}

impl Message for WmInitMenuPopup {
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

impl MessageHandleable for WmInitMenuPopup {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			hmenu: HMENU { ptr: p.wparam as *mut _ },
			item_pos: LOWORD(p.lparam as u32),
			is_window_menu: HIWORD(p.lparam as u32) != 0,
		}
	}
}

//------------------------------------------------------------------------------

button_msg! { WmLButtonDblClk, co::WM::LBUTTONDBLCLK,
	/// [`WM_LBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondblclk)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { WmLButtonDown, co::WM::LBUTTONDOWN,
	/// [`WM_LBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { WmLButtonUp, co::WM::LBUTTONUP,
	/// [`WM_LBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttonup)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { WmMButtonDblClk, co::WM::MBUTTONDBLCLK,
	/// [`WM_MBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondblclk)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { WmMButtonDown, co::WM::MBUTTONDOWN,
	/// [`WM_MBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondown)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { WmMButtonUp, co::WM::MBUTTONUP,
	/// [`WM_MBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttonup)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { WmMouseHover, co::WM::MOUSEHOVER,
	/// [`WM_MOUSEHOVER`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousehover)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { WmMouseMove, co::WM::MOUSEMOVE,
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
pub struct WmMove {
	pub coords: POINT,
}

impl Message for WmMove {
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

impl MessageHandleable for WmMove {
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
pub struct WmMoving<'a> {
	pub window_pos: &'a mut RECT,
}

impl<'a> Message for WmMoving<'a> {
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

impl<'a> MessageHandleable for WmMoving<'a> {
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
pub struct WmNcCalcSize<'a, 'b> {
	pub data: NccspRect<'a, 'b>,
}

impl<'a, 'b> Message for WmNcCalcSize<'a, 'b> {
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

impl<'a, 'b> MessageHandleable for WmNcCalcSize<'a, 'b> {
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
pub struct WmNcCreate<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

impl<'a, 'b, 'c> Message for WmNcCreate<'a, 'b, 'c> {
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

impl<'a, 'b, 'c> MessageHandleable for WmNcCreate<'a, 'b, 'c> {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			createstruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

//------------------------------------------------------------------------------

empty_msg_handleable! { WmNcDestroy, co::WM::NCDESTROY,
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
pub struct WmNcPaint {
	pub updated_hrgn: HRGN,
}

impl Message for WmNcPaint {
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

impl MessageHandleable for WmNcPaint {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			updated_hrgn: HRGN { ptr: p.wparam as *mut _ },
		}
	}
}

//------------------------------------------------------------------------------

empty_msg_handleable! { WmNull, co::WM::NULL,
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
pub struct WmNotify<'a> {
	pub nmhdr: &'a NMHDR,
}

impl<'a> Message for WmNotify<'a> {
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

impl<'a> MessageHandleable for WmNotify<'a> {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			nmhdr: unsafe { &*(p.lparam as *const NMHDR) },
		}
	}
}

impl<'a> WmNotify<'a> {
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

empty_msg_handleable! { WmPaint, co::WM::PAINT,
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
pub struct WmQueryOpen {}

impl Message for WmQueryOpen {
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

impl MessageHandleable for WmQueryOpen {
	fn from_generic_wm(_: Wm) -> Self {
		Self {}
	}
}

//------------------------------------------------------------------------------

button_msg! { WmRButtonDblClk, co::WM::RBUTTONDBLCLK,
	/// [`WM_RBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondblclk)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { WmRButtonDown, co::WM::RBUTTONDOWN,
	/// [`WM_RBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondown)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { WmRButtonUp, co::WM::RBUTTONUP,
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
pub struct WmSetFocus {
	pub hwnd_losing_focus: HWND,
}

impl Message for WmSetFocus {
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

impl MessageHandleable for WmSetFocus {
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
pub struct WmSetFont {
	pub hfont: HFONT,
	pub redraw: bool,
}

impl Message for WmSetFont {
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

impl MessageHandleable for WmSetFont {
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
pub struct WmSetIcon {
	pub size: co::ICON_SZ,
	pub hicon: HICON,
}

impl Message for WmSetIcon {
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

impl MessageHandleable for WmSetIcon {
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
pub struct WmShowWindow {
	pub being_shown: bool,
	pub status: co::SW_S,
}

impl Message for WmShowWindow {
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

impl MessageHandleable for WmShowWindow {
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
pub struct WmSize {
	pub request: co::SIZE_R,
	pub client_area: SIZE,
}

impl Message for WmSize {
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

impl MessageHandleable for WmSize {
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
pub struct WmSizing<'a> {
	pub window_edge: co::WMSZ,
	pub coords: &'a mut RECT,
}

impl<'a> Message for WmSizing<'a> {
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

impl<'a> MessageHandleable for WmSizing<'a> {
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
pub struct WmStyleChanged<'a> {
	pub change: co::GWL_C,
	pub stylestruct: WsWsex<'a>,
}

impl<'a> Message for WmStyleChanged<'a> {
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

impl<'a> MessageHandleable for WmStyleChanged<'a> {
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
pub struct WmStyleChanging<'a> {
	pub change: co::GWL_C,
	pub stylestruct: WsWsex<'a>,
}

impl<'a> Message for WmStyleChanging<'a> {
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

impl<'a> MessageHandleable for WmStyleChanging<'a> {
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

empty_msg_handleable! { WmThemeChanged, co::WM::THEMECHANGED,
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
pub struct WmTimer {
	pub timer_id: u32,
	pub timer_proc: Option<TIMERPROC>,
}

impl Message for WmTimer {
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

impl MessageHandleable for WmTimer {
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
pub struct WmWindowPosChanged<'a> {
	pub windowpos: &'a WINDOWPOS,
}

impl<'a> Message for WmWindowPosChanged<'a> {
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

impl<'a> MessageHandleable for WmWindowPosChanged<'a> {
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
pub struct WmWindowPosChanging<'a> {
	pub windowpos: &'a WINDOWPOS,
}

impl<'a> Message for WmWindowPosChanging<'a> {
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

impl<'a> MessageHandleable for WmWindowPosChanging<'a> {
	fn from_generic_wm(p: Wm) -> Self {
		Self {
			windowpos: unsafe { &*(p.lparam as *const _) },
		}
	}
}

//------------------------------------------------------------------------------

button_msg! { WmXButtonDblClk, co::WM::XBUTTONDBLCLK,
	/// [`WM_XBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondblclk)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { WmXButtonDown, co::WM::XBUTTONDOWN,
	/// [`WM_XBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondown)
	/// message parameters.
	///
	/// Return type: `()`.
}

button_msg! { WmXButtonUp, co::WM::XBUTTONUP,
	/// [`WM_XBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttonup)
	/// message parameters.
	///
	/// Return type: `()`.
}
