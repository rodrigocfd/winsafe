use crate::co;
use crate::comctl::decl::NMHDR;
use crate::msg::WndMsg;
use crate::prelude::{MsgSend, MsgSendRecv};

/// [`WM_NOTIFY`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-notify)
/// message parameters.
///
/// Return type: `isize`.
#[derive(Clone, Copy)]
pub struct Notify<'a> {
	pub nmhdr: &'a NMHDR,
}

unsafe impl<'a> MsgSend for Notify<'a> {
	type RetType = isize;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NOTIFY,
			wparam: self.nmhdr.hwndFrom.0 as _,
			lparam: self.nmhdr as *const _ as _,
		}
	}
}

unsafe impl<'a> MsgSendRecv for Notify<'a> {
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
