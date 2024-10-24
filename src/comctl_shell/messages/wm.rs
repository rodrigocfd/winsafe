use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;

/// [`WM_GETHOTKEY`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-gethotkey)
/// message, which has no parameters.
///
/// Return type: `(co::VK, co::HOTKEYF)`.
pub struct GetHotKey {}

impl MsgSend for GetHotKey {
	type RetType = (co::VK, co::HOTKEYF);

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		(
			co::VK::from_raw(LOBYTE(v as _) as _),
			co::HOTKEYF::from_raw(HIBYTE(v as _) as _),
		)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETHOTKEY,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for GetHotKey {
	unsafe fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

/// [`WM_SETHOTKEY`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-sethotkey)
/// message parameters.
///
/// Return type: `i32`.
pub struct SetHotKey {
	pub vkey_code: co::VK,
	pub modifiers: co::HOTKEYF,
}

impl MsgSend for SetHotKey {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETHOTKEY,
			wparam: MAKEDWORD(self.vkey_code.raw(), self.modifiers.raw()) as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for SetHotKey {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			vkey_code: co::VK::from_raw(LOWORD(p.wparam as _)),
			modifiers: co::HOTKEYF::from_raw(HIWORD(p.wparam as _)),
		}
	}
}
