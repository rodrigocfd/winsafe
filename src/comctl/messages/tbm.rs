use crate::co;
use crate::comctl::decl::{IdxCbNone, ResStrs, TBADDBITMAP, TBBUTTON};
use crate::kernel::decl::SysResult;
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::privs::zero_as_err;

/// [`TB_ADDBITMAP`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-addbitmap)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct AddBitmap<'a> {
	pub num_images: u32,
	pub info: &'a TBADDBITMAP,
}

unsafe impl<'a> MsgSend for AddBitmap<'a> {
	type RetType = SysResult<u32>;

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
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct AddButtons<'a, 'b> {
	pub buttons: &'a mut [TBBUTTON<'b>],
}

unsafe impl<'a, 'b> MsgSend for AddButtons<'a, 'b> {
	type RetType = SysResult<()>;

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
/// Return type: `SysResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct AddString {
	pub texts: ResStrs,
}

unsafe impl MsgSend for AddString {
	type RetType = SysResult<u32>;

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

/// [`TB_CHANGEBITMAP`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-changebitmap)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct ChangeBitmap {
	pub btn_cmd_id: u16,
	pub image: IdxCbNone,
}

unsafe impl MsgSend for ChangeBitmap {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::CHANGEBITMAP.into(),
			wparam: self.btn_cmd_id as _,
			lparam: self.image.into(),
		}
	}
}

/// [`TB_CHECKBUTTON`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-checkbutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct CheckButton {
	pub btn_cmd_id: u16,
	pub check: bool,
}

unsafe impl MsgSend for CheckButton {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::CHECKBUTTON.into(),
			wparam: self.btn_cmd_id as _,
			lparam: self.check as _,
		}
	}
}

/// [`TB_COMMANDTOINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-commandtoindex)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct CommandToIndex {
	pub btn_cmd_id: u16,
}

unsafe impl MsgSend for CommandToIndex {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => None,
			v => Some(v as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::COMMANDTOINDEX.into(),
			wparam: self.btn_cmd_id as _,
			lparam: 0,
		}
	}
}

pub_struct_msg_empty! { Customize: co::TBM::CUSTOMIZE.into(); "comctl";
	/// [`TB_CUSTOMIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-customize)
}

/// [`TB_DELETEBUTTON`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-deletebutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct DeleteButton {
	pub btn_index: u32,
}

unsafe impl MsgSend for DeleteButton {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::CHECKBUTTON.into(),
			wparam: self.btn_index as _,
			lparam: 0,
		}
	}
}

/// [`TB_ENABLEBUTTON`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-enablebutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct EnableButton {
	pub btn_cmd_id: u16,
	pub enable: bool,
}

unsafe impl MsgSend for EnableButton {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ENABLEBUTTON.into(),
			wparam: self.btn_cmd_id as _,
			lparam: self.enable as _,
		}
	}
}
