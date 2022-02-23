//! Toolbar control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-toolbar-control-reference-messages),
//! whose constants have [`TBM`](crate::co::TBM) prefix.

use crate::co;
use crate::comctl::decl::{ResStrs, TBADDBITMAP, TBBUTTON};
use crate::kernel::decl::WinResult;
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::privs::zero_as_err;

/// [`TB_ADDBITMAP`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-addbitmap)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct AddBitmap<'a> {
	pub num_images: u32,
	pub info: &'a TBADDBITMAP,
}

unsafe impl<'a> MsgSend for AddBitmap<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ADDBITMAP.into(),
			wparam: self.num_images as _,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`TB_ADDBUTTONS`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-addbuttons)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct AddButtons<'a, 'b> {
	pub buttons: &'a mut [TBBUTTON<'b>],
}

unsafe impl<'a, 'b> MsgSend for AddButtons<'a, 'b> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ADDBUTTONS.into(),
			wparam: self.buttons.len() as _,
			lparam: self.buttons.as_mut_ptr() as _,
		}
	}
}

/// [`TB_ADDSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-addstring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct AddString {
	pub texts: ResStrs,
}

unsafe impl MsgSend for AddString {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ADDSTRING.into(),
			wparam: match &self.texts {
				ResStrs::Res(_, hinst) => hinst.0 as _,
				ResStrs::Strs(_) => 0,
			},
			lparam: match &self.texts {
				ResStrs::Res(res, _) => res.as_ptr() as _,
				ResStrs::Strs(strs) => unsafe { strs.as_ptr() as _ },
			},
		}
	}
}

pub_struct_msg_empty! { AutoSize: co::TBM::AUTOSIZE.into(); "comctl";
	/// [`TB_AUTOSIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-autosize)
}

/// [`TB_BUTTONCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-buttoncount)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct ButtonCount {}

unsafe impl MsgSend for ButtonCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::BUTTONCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_BUTTONSTRUCTSIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-buttonstructsize)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct ButtonStructSize {
	pub size: u32,
}

unsafe impl MsgSend for ButtonStructSize {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::BUTTONSTRUCTSIZE.into(),
			wparam: self.size as _,
			lparam: 0,
		}
	}
}
