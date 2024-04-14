use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;

/// [`WM_NOTIFY`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-notify)
/// message parameters.
///
/// Return type: `isize`.
pub struct Notify<'a> {
	pub nmhdr: &'a mut NMHDR,
}

unsafe impl<'a> MsgSend for Notify<'a> {
	type RetType = isize;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NOTIFY,
			wparam: self.nmhdr.hwndFrom.ptr() as _,
			lparam: self.nmhdr as *mut _ as _,
		}
	}
}

unsafe impl<'a> MsgSendRecv for Notify<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			nmhdr: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

impl<'a> Notify<'a> {
	/// Casts the `NMHDR` reference into a derived struct.
	///
	/// # Safety
	///
	/// The casting must be done to the correct struct.
	///
	/// You should always prefer the specific notifications, which perform this
	/// conversion for you.
	pub const unsafe fn cast_nmhdr<T>(&self) -> &T {
		&*(self.nmhdr as *const _ as *const _)
	}

	/// Casts the `NMHDR` mutable reference into a derived struct.
	///
	/// # Safety
	///
	/// The casting must be done to the correct struct.
	///
	/// You should always prefer the specific notifications, which perform this
	/// conversion for you.
	pub unsafe fn cast_nmhdr_mut<T>(&self) -> &mut T {
		#[allow(invalid_reference_casting)] // https://github.com/rust-lang/rust/issues/116410
		&mut *(self.nmhdr as *const _ as *mut _)
	}
}

/// [`WM_GETHOTKEY`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-gethotkey)
/// message, which has no parameters.
///
/// Return type: `(co::VK, co::HOTKEYF)`.
pub struct GetHotKey {}

unsafe impl MsgSend for GetHotKey {
	type RetType = (co::VK, co::HOTKEYF);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe {(
			co::VK::from_raw(LOBYTE(v as _) as _),
			co::HOTKEYF::from_raw(HIBYTE(v as _) as _),
		)}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETHOTKEY,
			wparam: 0,
			lparam: 0,
		}
	}
}

unsafe impl MsgSendRecv for GetHotKey {
	fn from_generic_wm(_: WndMsg) -> Self {
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

unsafe impl MsgSend for SetHotKey {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
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

unsafe impl MsgSendRecv for SetHotKey {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			vkey_code: unsafe { co::VK::from_raw(LOWORD(p.wparam as _)) },
			modifiers: unsafe { co::HOTKEYF::from_raw(HIWORD(p.wparam as _)) },
		}
	}
}
