use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

pub_struct_msg_ctlcolor! { CtlColorBtn: co::WM::CTLCOLORBTN;
	/// [`WM_CTLCOLORBTN`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorbtn)
}

pub_struct_msg_ctlcolor! { CtlColorDlg: co::WM::CTLCOLORDLG;
	/// [`WM_CTLCOLORDLG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-ctlcolordlg)
}

pub_struct_msg_ctlcolor! { CtlColorEdit: co::WM::CTLCOLOREDIT;
	/// [`WM_CTLCOLOREDIT`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcoloredit)
}

pub_struct_msg_ctlcolor! { CtlColorListBox: co::WM::CTLCOLORLISTBOX;
	/// [`WM_CTLCOLORLISTBOX`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorlistbox)
}

pub_struct_msg_ctlcolor! { CtlColorScrollBar: co::WM::CTLCOLORSCROLLBAR;
	/// [`WM_CTLCOLORSCROLLBAR`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorscrollbar)
}

pub_struct_msg_ctlcolor! { CtlColorStatic: co::WM::CTLCOLORSTATIC;
	/// [`WM_CTLCOLORSTATIC`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorstatic)
}

pub_struct_msg_char_code! { DeadChar: co::WM::DEADCHAR;
	/// [`WM_DEADCHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-deadchar)
}

/// [`WM_DISPLAYCHANGE`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-displaychange)
/// message parameters.
///
/// Return type: `()`.
pub struct DisplayChange {
	pub depth_bpp: u32,
	pub horz_res: u16,
	pub vert_res: u16,
}

impl MsgSend for DisplayChange {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::DISPLAYCHANGE,
			wparam: self.depth_bpp as _,
			lparam: MAKEDWORD(self.horz_res, self.vert_res) as _,
		}
	}
}

impl MsgSendRecv for DisplayChange {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			depth_bpp: p.wparam as _,
			horz_res: LOWORD(p.lparam as _),
			vert_res: HIWORD(p.lparam as _),
		}
	}
}

/// [`WM_GETFONT`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-getfont)
/// message, which has no parameters.
///
/// Return type: `Option<HFONT>`.
pub struct GetFont {}

impl MsgSend for GetFont {
	type RetType = Option<HFONT>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HFONT::from_ptr(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETFONT,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for GetFont {
	unsafe fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

/// [`WM_NCPAINT`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-ncpaint)
/// message parameters.
///
/// Return type: `()`.
pub struct NcPaint {
	pub updated_hrgn: HRGN,
}

impl MsgSend for NcPaint {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NCPAINT,
			wparam: self.updated_hrgn.ptr() as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for NcPaint {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			updated_hrgn: HRGN::from_ptr(p.wparam as _),
		}
	}
}

pub_struct_msg_empty_handleable! { Paint: co::WM::PAINT;
	/// [`WM_PAINT`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-paint)
}

/// [`WM_SETFONT`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-setfont)
/// message parameters.
///
/// Return type: `()`.
pub struct SetFont {
	pub hfont: HFONT,
	pub redraw: bool,
}

impl MsgSend for SetFont {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETFONT,
			wparam: self.hfont.ptr() as _,
			lparam: MAKEDWORD(self.redraw as _, 0) as _,
		}
	}
}

impl MsgSendRecv for SetFont {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hfont: HFONT::from_ptr(p.wparam as _),
			redraw: LOWORD(p.lparam as _) != 0,
		}
	}
}

/// [`WM_SETREDRAW`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-setredraw)
/// message parameters.
///
/// Return type: `()`.
pub struct SetRedraw {
	pub can_redraw: bool,
}

impl MsgSend for SetRedraw {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETREDRAW,
			wparam: self.can_redraw as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for SetRedraw {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self { can_redraw: p.wparam != 0 }
	}
}

pub_struct_msg_empty_handleable! { SyncPaint: co::WM::SYNCPAINT;
	/// [`WM_SYNCPAINT`](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-syncpaint)
}
