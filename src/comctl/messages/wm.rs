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

impl<'a> MsgSend for Notify<'a> {
	type RetType = isize;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
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

impl<'a> MsgSendRecv for Notify<'a> {
	unsafe fn from_generic_wm(p: WndMsg) -> Self {
		unsafe { Self { nmhdr: &mut *(p.lparam as *mut _) } }
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
		unsafe { &*(self.nmhdr as *const _ as *const _) }
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
		unsafe { &mut *(self.nmhdr as *const _ as *mut _) }
	}
}
