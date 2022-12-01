use crate::co;
use crate::comctl::decl::{
	COLORSCHEME, HIMAGELIST, IdxCbNone, ResStrs, TBADDBITMAP, TBBUTTON,
	TBBUTTONINFO, TBINSERTMARK, TBMETRICS, TBREPLACEBITMAP,
};
use crate::comctl::privs::HINST_COMMCTRL;
use crate::kernel::decl::{HIWORD, LOWORD, MAKEDWORD, SysResult, WString};
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::decl::{COLORREF, HWND, POINT, RECT, SIZE};
use crate::user::privs::{
	minus1_as_err, minus1_as_none, zero_as_err, zero_as_none,
};

/// [`TB_ADDBITMAP`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-addbitmap)
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

/// [`TB_ADDBUTTONS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-addbuttons)
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

/// [`TB_ADDSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-addstring)
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
	/// [`TB_AUTOSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-autosize)
}

/// [`TB_BUTTONCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-buttoncount)
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

/// [`TB_BUTTONSTRUCTSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-buttonstructsize)
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

/// [`TB_CHANGEBITMAP`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-changebitmap)
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

/// [`TB_CHECKBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-checkbutton)
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

/// [`TB_COMMANDTOINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-commandtoindex)
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
		minus1_as_none(v).map(|v| v as _)
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
	/// [`TB_CUSTOMIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-customize)
}

/// [`TB_DELETEBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-deletebutton)
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

/// [`TB_ENABLEBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-enablebutton)
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

/// [`TB_GETANCHORHIGHLIGHT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getanchorhighlight)
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

/// [`TB_GETBITMAP`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getbitmap)
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

/// [`TB_GETBITMAPFLAGS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getbitmapflags)
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

/// [`TB_GETBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getbutton)
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

/// [`TB_GETBUTTONINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getbuttoninfo)
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

/// [`TB_GETBUTTONSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getbuttonsize)
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

/// [`TB_GETBUTTONTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getbuttontext)
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

/// [`TB_GETCOLORSCHEME`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getcolorscheme)
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

/// [`TB_GETDISABLEDIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getdisabledimagelist)
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

/// [`TB_GETEXTENDEDSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getextendedstyle)
/// message, which has no parameters.
///
/// Return type: `co::TBSTYLE_EX`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetExtendedStyle {}

unsafe impl MsgSend for GetExtendedStyle {
	type RetType = co::TBSTYLE_EX;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::TBSTYLE_EX(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETEXTENDEDSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETHOTIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-gethotimagelist)
/// message, which has no parameters.
///
/// Return type: `Option<HIMAGELIST>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetHotImageList {}

unsafe impl MsgSend for GetHotImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| HIMAGELIST(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETHOTIMAGELIST.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETHOTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-gethotitem)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetHotItem {}

unsafe impl MsgSend for GetHotItem {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETHOTITEM.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETIDEALSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getidealsize)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetIdealSize<'a> {
	pub get_height: bool,
	pub size: &'a mut SIZE,
}

unsafe impl<'a> MsgSend for GetIdealSize<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETIDEALSIZE.into(),
			wparam: self.get_height as _,
			lparam: self.size as *mut _ as _,
		}
	}
}

/// [`TB_GETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getimagelist)
/// message, which has no parameters.
///
/// Return type: `Option<HIMAGELIST>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetImageList {}

unsafe impl MsgSend for GetImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| HIMAGELIST(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETIMAGELIST.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETIMAGELISTCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getimagelistcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetImageListCount {}

unsafe impl MsgSend for GetImageListCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETIMAGELISTCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETINSERTMARK`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getinsertmark)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetInsertMark<'a> {
	pub info: &'a mut TBINSERTMARK,
}

unsafe impl<'a> MsgSend for GetInsertMark<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETINSERTMARK.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`TB_GETINSERTMARKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getinsertmarkcolor)
/// message, which has no parameters.
///
/// Return type: `COLORREF`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetInsertMarkColor {}

unsafe impl MsgSend for GetInsertMarkColor {
	type RetType = COLORREF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		COLORREF(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETINSERTMARKCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETITEMDROPDOWNRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getitemdropdownrect)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetItemDropdownRect<'a> {
	pub item_index: u32,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetItemDropdownRect<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETITEMDROPDOWNRECT.into(),
			wparam: self.item_index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TB_GETITEMRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getitemrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetItemRect<'a> {
	pub btn_index: u32,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetItemRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETITEMRECT.into(),
			wparam: self.btn_index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TB_GETMAXSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getmaxsize)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetMaxSize<'a> {
	pub size: &'a mut SIZE,
}

unsafe impl<'a> MsgSend for GetMaxSize<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETMAXSIZE.into(),
			wparam: 0,
			lparam: self.size as *mut _ as _,
		}
	}
}

/// [`TB_GETMETRICS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getmetrics)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetMetrics<'a> {
	pub metrics: &'a mut TBMETRICS,
}

unsafe impl<'a> MsgSend for GetMetrics<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETMETRICS.into(),
			wparam: 0,
			lparam: self.metrics as *mut _ as _,
		}
	}
}

/// [`TB_GETPADDING`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getpadding)
/// message, which has no parameters.
///
/// Return type: `(u16, u16)`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetPadding {}

unsafe impl MsgSend for GetPadding {
	type RetType = (u16, u16);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), HIWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETPADDING.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETPRESSEDIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getpressedimagelist)
/// message, which has no parameters.
///
/// Return type: `Option<HIMAGELIST>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetPressedImageList {}

unsafe impl MsgSend for GetPressedImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HIMAGELIST(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETPRESSEDIMAGELIST.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetRect<'a> {
	pub cmd_id: u16,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETRECT.into(),
			wparam: self.cmd_id as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TB_GETROWS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getrows)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetRows {}

unsafe impl MsgSend for GetRows {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETROWS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getstate)
/// message parameters.
///
/// Return type: `SysResult<co::TBSTATE>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetState {
	pub cmd_id: u16,
}

unsafe impl MsgSend for GetState {
	type RetType = SysResult<co::TBSTATE>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_err(v).map(|v| co::TBSTATE(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETSTATE.into(),
			wparam: self.cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_GETSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getstring)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetString<'a> {
	pub index: u16,
	pub text: &'a mut WString,
}

unsafe impl<'a> MsgSend for GetString<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETSTRING.into(),
			wparam: MAKEDWORD(self.text.buf_len() as _, self.index) as _,
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`TB_GETSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getstyle)
/// message, which has no parameters.
///
/// Return type: `co::BTNS`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetStyle {}

unsafe impl MsgSend for GetStyle {
	type RetType = co::BTNS;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::BTNS(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETTEXTROWS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-gettextrows)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetTextRows {}

unsafe impl MsgSend for GetTextRows {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETTEXTROWS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-gettooltips)
/// message, which has no parameters.
///
/// Return type: `Option<HWND>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetTooltips {}

unsafe impl MsgSend for GetTooltips {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| HWND(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETTOOLTIPS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getunicodeformat)
/// message, which has no parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetUnicodeFormat {}

unsafe impl MsgSend for GetUnicodeFormat {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_HIDEBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-hidebutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct HideButton {
	pub cmd_id: u16,
	pub hide: bool,
}

unsafe impl MsgSend for HideButton {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::HIDEBUTTON.into(),
			wparam: self.cmd_id as _,
			lparam: MAKEDWORD(self.hide as _, 0) as _,
		}
	}
}

/// [`TB_HITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-hittest)
/// message parameters.
///
/// Return type: `i32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct HitTest<'a> {
	pub coords: &'a mut POINT,
}

unsafe impl<'a> MsgSend for HitTest<'a> {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::HITTEST.into(),
			wparam: 0,
			lparam: self.coords as *mut _ as _,
		}
	}
}

/// [`TB_INDETERMINATE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-indeterminate)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct Indeterminate {
	pub cmd_id: u16,
	pub indeterminate: bool,
}

unsafe impl MsgSend for Indeterminate {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::INDETERMINATE.into(),
			wparam: self.cmd_id as _,
			lparam: MAKEDWORD(self.indeterminate as _, 0) as _,
		}
	}
}

/// [`TB_INSERTBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-insertbutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct InsertButton<'a, 'b> {
	pub index: u32,
	pub button: &'a TBBUTTON<'b>,
}

unsafe impl<'a, 'b> MsgSend for InsertButton<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::INSERTBUTTON.into(),
			wparam: self.index as _,
			lparam: self.button as *const _ as _,
		}
	}
}

/// [`TB_INSERTMARKHITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-insertmarkhittest)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct InsertMarkHitTest<'a, 'b> {
	pub coords: &'a POINT,
	pub info: &'b mut TBINSERTMARK,
}

unsafe impl<'a, 'b> MsgSend for InsertMarkHitTest<'a, 'b> {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::INSERTMARKHITTEST.into(),
			wparam: self.coords as *const _ as _,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`TB_ISBUTTONCHECKED`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-isbuttonchecked)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct IsButtonChecked {
	pub cmd_id: u16,
}

unsafe impl MsgSend for IsButtonChecked {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ISBUTTONCHECKED.into(),
			wparam: self.cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_ISBUTTONENABLED`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-isbuttonenabled)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct IsButtonEnabled {
	pub cmd_id: u16,
}

unsafe impl MsgSend for IsButtonEnabled {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ISBUTTONENABLED.into(),
			wparam: self.cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_ISBUTTONHIDDEN`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-isbuttonhidden)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct IsButtonHidden {
	pub cmd_id: u16,
}

unsafe impl MsgSend for IsButtonHidden {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ISBUTTONHIDDEN.into(),
			wparam: self.cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_ISBUTTONHIGHLIGHTED`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-isbuttonhighlighted)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct IsButtonHighlighted {
	pub cmd_id: u16,
}

unsafe impl MsgSend for IsButtonHighlighted {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ISBUTTONHIGHLIGHTED.into(),
			wparam: self.cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_ISBUTTONINDETERMINATE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-isbuttonindeterminate)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct IsButtonIndeterminate {
	pub cmd_id: u16,
}

unsafe impl MsgSend for IsButtonIndeterminate {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ISBUTTONINDETERMINATE.into(),
			wparam: self.cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_ISBUTTONPRESSED`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-isbuttonpressed)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct IsButtonPressed {
	pub cmd_id: u16,
}

unsafe impl MsgSend for IsButtonPressed {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ISBUTTONPRESSED.into(),
			wparam: self.cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_LOADIMAGES`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-loadimages)
/// message parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct LoadImages {
	pub img_list: co::IDB,
}

unsafe impl MsgSend for LoadImages {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::LOADIMAGES.into(),
			wparam: self.img_list.0 as _,
			lparam: HINST_COMMCTRL.0 as _,
		}
	}
}

/// [`TB_MAPACCELERATOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-mapaccelerator)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct MapAccelerator<'a> {
	pub character: char,
	pub cmd_id: &'a mut u16,
}

unsafe impl<'a> MsgSend for MapAccelerator<'a> {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::MAPACCELERATOR.into(),
			wparam: self.character as _,
			lparam: self.cmd_id as *mut _ as _,
		}
	}
}

/// [`TB_MARKBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-markbutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct MarkButton {
	pub cmd_id: u16,
	pub highlight: bool,
}

unsafe impl MsgSend for MarkButton {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::MARKBUTTON.into(),
			wparam: self.cmd_id as _,
			lparam: MAKEDWORD(self.highlight as _, 0) as _,
		}
	}
}

/// [`TB_MOVEBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-movebutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct MoveButton {
	pub btn_index: u32,
	pub dest_index: u32,
}

unsafe impl MsgSend for MoveButton {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::MOVEBUTTON.into(),
			wparam: self.btn_index as _,
			lparam: self.dest_index as _,
		}
	}
}

/// [`TB_PRESSBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-pressbutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct PressButton {
	pub cmd_id: u16,
	pub press: bool,
}

unsafe impl MsgSend for PressButton {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::PRESSBUTTON.into(),
			wparam: self.cmd_id as _,
			lparam: MAKEDWORD(self.press as _, 0) as _,
		}
	}
}

/// [`TB_REPLACEBITMAP`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-replacebitmap)
/// message parameters.
///
/// Return type: `SysResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct ReplaceBitmap<'a> {
	pub info: &'a TBREPLACEBITMAP,
}

unsafe impl<'a> MsgSend for ReplaceBitmap<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::REPLACEBITMAP.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}