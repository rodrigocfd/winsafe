use crate::co;
use crate::comctl::decl::{
	COLORSCHEME, HIMAGELIST, IdxCbNone, ResStrs, TBADDBITMAP, TBBUTTON,
	TBBUTTONINFO,
};
use crate::kernel::decl::{SysResult, WString};
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::decl::SIZE;
use crate::user::privs::{minus1_as_err, zero_as_err, zero_as_none};

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
		minus1_as_err(v).map(|v| v as _)
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
		minus1_as_err(v).map(|v| v as _)
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

/// [`TB_GETANCHORHIGHLIGHT`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-getanchorhighlight)
/// message, which has no parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetAnchorHighlight {}

unsafe impl MsgSend for GetAnchorHighlight {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETANCHORHIGHLIGHT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETBITMAP`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-getbitmap)
/// message parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetBitmap {
	pub btn_cmd_id: u16,
}

unsafe impl MsgSend for GetBitmap {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETBITMAP.into(),
			wparam: self.btn_cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_GETBITMAPFLAGS`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-getbitmapflags)
/// message, which has no parameters.
///
/// Return type: `co::TBBF`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetBitmapFlags {}

unsafe impl MsgSend for GetBitmapFlags {
	type RetType = co::TBBF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::TBBF(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETBITMAPFLAGS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETBUTTON`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-getbutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetButton<'a, 'b> {
	pub btn_index: u32,
	pub info: &'a mut TBBUTTON<'b>,
}

unsafe impl<'a, 'b> MsgSend for GetButton<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETBUTTON.into(),
			wparam: self.btn_index as _,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`TB_GETBUTTONINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-getbuttoninfo)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetButtonInfo<'a, 'b> {
	pub btn_cmd_id: u16,
	pub info: &'a mut TBBUTTONINFO<'b>,
}

unsafe impl<'a, 'b> MsgSend for GetButtonInfo<'a, 'b> {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETBUTTONINFO.into(),
			wparam: self.btn_cmd_id as _,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`TB_GETBUTTONSIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-getbuttonsize)
/// message, which has no parameters.
///
/// Return type: `SIZE`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetButtonSize {}

unsafe impl MsgSend for GetButtonSize {
	type RetType = SIZE;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		SIZE::from(v as u32)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETBUTTONSIZE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETBUTTONTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-getbuttontext)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetButtonText<'a> {
	pub btn_cmd_id: u16,
	pub text: &'a mut WString,
}

unsafe impl<'a> MsgSend for GetButtonText<'a> {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_err(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETBUTTONTEXT.into(),
			wparam: self.btn_cmd_id as _,
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`TB_GETCOLORSCHEME`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-getcolorscheme)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetColorScheme<'a> {
	pub scheme: &'a mut COLORSCHEME,
}

unsafe impl<'a> MsgSend for GetColorScheme<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETCOLORSCHEME.into(),
			wparam: 0,
			lparam: self.scheme as *mut _ as _,
		}
	}
}

/// [`TB_GETDISABLEDIMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-getdisabledimagelist)
/// message, which has no parameters.
///
/// Return type: `Option<HIMAGELIST>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetDisabledImageList {}

unsafe impl MsgSend for GetDisabledImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| HIMAGELIST(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETDISABLEDIMAGELIST.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}
