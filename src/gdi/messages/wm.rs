use crate::co;
use crate::kernel::decl::{HIWORD, LOWORD, MAKEDWORD};

use crate::gdi::decl::HFONT;
use crate::msg::WndMsg;
use crate::prelude::{MsgSend, MsgSendRecv};
use crate::user::decl::HRGN;
use crate::user::privs::zero_as_none;

pub_struct_msg_ctlcolor! { CtlColorBtn: co::WM::CTLCOLORBTN; "gdi";
	/// [`WM_CTLCOLORBTN`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorbtn)
}

pub_struct_msg_ctlcolor! { CtlColorDlg: co::WM::CTLCOLORDLG; "gdi";
	/// [`WM_CTLCOLORDLG`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-ctlcolordlg)
}

pub_struct_msg_ctlcolor! { CtlColorEdit: co::WM::CTLCOLOREDIT; "gdi";
	/// [`WM_CTLCOLOREDIT`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcoloredit)
}

pub_struct_msg_ctlcolor! { CtlColorListBox: co::WM::CTLCOLORLISTBOX; "gdi";
	/// [`WM_CTLCOLORLISTBOX`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorlistbox)
}

pub_struct_msg_ctlcolor! { CtlColorScrollBar: co::WM::CTLCOLORSCROLLBAR; "gdi";
	/// [`WM_CTLCOLORSCROLLBAR`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorscrollbar)
}

pub_struct_msg_ctlcolor! { CtlColorStatic: co::WM::CTLCOLORSTATIC; "gdi";
	/// [`WM_CTLCOLORSTATIC`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorstatic)
}

pub_struct_msg_char! { DeadChar: co::WM::DEADCHAR; "gdi";
	/// [`WM_DEADCHAR`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-deadchar)
}

/// [`WM_DISPLAYCHANGE`](https://docs.microsoft.com/en-us/windows/win32/gdi/wm-displaychange)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
pub struct DisplayChange {
	pub depth_bpp: u32,
	pub horz_res: u16,
	pub vert_res: u16,
}

impl MsgSend for DisplayChange {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
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
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			depth_bpp: p.wparam as _,
			horz_res: LOWORD(p.lparam as _),
			vert_res: HIWORD(p.lparam as _),
		}
	}
}

/// [`WM_GETFONT`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-getfont)
/// message, which has no parameters.
///
/// Return type: `Option<HFONT>`.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
pub struct GetFont {}

impl MsgSend for GetFont {
	type RetType = Option<HFONT>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HFONT(p as _))
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
	fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

/// [`WM_NCPAINT`](https://docs.microsoft.com/en-us/windows/win32/gdi/wm-ncpaint)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
pub struct NcPaint {
	pub updated_hrgn: HRGN,
}

impl MsgSend for NcPaint {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NCPAINT,
			wparam: self.updated_hrgn.0 as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for NcPaint {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			updated_hrgn: HRGN(p.wparam as _),
		}
	}
}

pub_struct_msg_empty_handleable! { Paint: co::WM::PAINT; "gdi";
	/// [`WM_PAINT`](https://docs.microsoft.com/en-us/windows/win32/gdi/wm-paint)
}

/// [`WM_SETFONT`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-setfont)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
pub struct SetFont {
	pub hfont: HFONT,
	pub redraw: bool,
}

impl MsgSend for SetFont {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETFONT,
			wparam: self.hfont.0 as _,
			lparam: MAKEDWORD(self.redraw as _, 0) as _,
		}
	}
}

impl MsgSendRecv for SetFont {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hfont: HFONT(p.wparam as _),
			redraw: LOWORD(p.lparam as _) != 0,
		}
	}
}

/// [`WM_SETREDRAW`](https://docs.microsoft.com/en-us/windows/win32/gdi/wm-setredraw)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
pub struct SetRedraw {
	pub can_redraw: bool,
}

impl MsgSend for SetRedraw {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
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
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			can_redraw: p.wparam != 0,
		}
	}
}

pub_struct_msg_empty_handleable! { SyncPaint: co::WM::SYNCPAINT; "gdi";
	/// [`WM_SYNCPAINT`](https://docs.microsoft.com/en-us/windows/win32/gdi/wm-syncpaint)
}
