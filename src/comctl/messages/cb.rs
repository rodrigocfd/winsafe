use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`CB_GETCUEBANNER`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getcuebanner)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct CbGetCueBanner<'a> {
	pub buffer: &'a mut WString,
}

impl<'a> MsgSend for CbGetCueBanner<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 | 1 => Ok(()),
			_ => Err(co::ERROR::BAD_ARGUMENTS),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETCUEBANNER.into(),
			wparam: unsafe { self.buffer.as_mut_ptr() } as _,
			lparam: self.buffer.buf_len() as _,
		}
	}
}

/// [`CB_GETMINVISIBLE`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getminvisible)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct CbGetMinVisible {}

impl MsgSend for CbGetMinVisible {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::GETMINVISIBLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_SETCUEBANNER`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setcuebanner)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct CbSetCueBanner {
	pub text: WString,
}

impl MsgSend for CbSetCueBanner {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			1 => Ok(()),
			_ => Err(co::ERROR::BAD_ARGUMENTS),
		}
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::SETCUEBANNER.into(),
			wparam: 0,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`CB_SETMINVISIBLE`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setminvisible)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct CbSetMinVisible {
	pub num_items: u32,
}

impl MsgSend for CbSetMinVisible {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::CB::SETMINVISIBLE.into(),
			wparam: self.num_items as _,
			lparam: 0,
		}
	}
}
