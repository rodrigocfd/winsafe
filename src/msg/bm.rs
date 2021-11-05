//! Button control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-messages),
//! whose constants have [`BM` and `BCM`](crate::co::BM) prefixes.

use crate::aliases::WinResult;
use crate::co;
use crate::enums::BmpIcon;
use crate::handles::{HBITMAP, HICON};
use crate::msg::{MsgSend, WndMsg};
use crate::msg::macros::zero_as_err;
use crate::structs::{BUTTON_IMAGELIST, BUTTON_SPLITINFO, RECT, SIZE};
use crate::various::WString;

/// [`BCM_GETIDEALSIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getidealsize)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetIdealSize<'a> {
	pub size: &'a mut SIZE,
}

impl<'a> MsgSend for GetIdealSize<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::GETIDEALSIZE.into(),
			wparam: 0,
			lparam: self.size as *const _ as _,
		}
	}
}

/// [`BCM_GETIMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getimagelist)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetImageList<'a> {
	pub info: &'a mut BUTTON_IMAGELIST,
}

impl<'a> MsgSend for GetImageList<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::GETIMAGELIST.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`BCM_GETNOTE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getnote)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetNote<'a> {
	pub text: &'a mut WString,
}

impl<'a> MsgSend for GetNote<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::GETNOTE.into(),
			wparam: self.text.buffer_size(),
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`BCM_GETNOTELENGTH`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getnotelength)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetNoteLength {}

impl MsgSend for GetNoteLength {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::GETNOTELENGTH.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`BCM_GETSPLITINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getsplitinfo)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetSplitInfo<'a> {
	pub splitinfo: &'a mut BUTTON_SPLITINFO,
}

impl<'a> MsgSend for GetSplitInfo<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::GETSPLITINFO.into(),
			wparam: 0,
			lparam: self.splitinfo as *const _ as _,
		}
	}
}

/// [`BCM_GETTEXTMARGIN`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-gettextmargin)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct GetTextMargin<'a> {
	pub margins: &'a mut RECT,
}

impl<'a> MsgSend for GetTextMargin<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::GETTEXTMARGIN.into(),
			wparam: 0,
			lparam: self.margins as *const _ as _,
		}
	}
}

/// [`BCM_SETDROPDOWNSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-setdropdownstate)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetDropDownState {
	pub is_pushed: bool,
}

impl MsgSend for SetDropDownState {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETDROPDOWNSTATE.into(),
			wparam: self.is_pushed as _,
			lparam: 0,
		}
	}
}

/// [`BCM_SETIMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-setimagelist)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetImageList<'a> {
	pub info: &'a BUTTON_IMAGELIST,
}

impl<'a> MsgSend for SetImageList<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETIMAGELIST.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`BCM_SETNOTE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-setnote)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetNote<'a> {
	pub text: &'a WString,
}

impl<'a> MsgSend for SetNote<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETNOTE.into(),
			wparam: self.text.buffer_size(),
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`BCM_SETSHIELD`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-setshield)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetShield {
	pub has_elevated_icon: bool,
}

impl MsgSend for SetShield {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETSHIELD.into(),
			wparam: self.has_elevated_icon as _,
			lparam: 0,
		}
	}
}

/// [`BCM_SETSPLITINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-setsplitinfo)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetSplitInfo<'a> {
	pub splitinfo: &'a BUTTON_SPLITINFO,
}

impl<'a> MsgSend for SetSplitInfo<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETSPLITINFO.into(),
			wparam: 0,
			lparam: self.splitinfo as *const _ as _,
		}
	}
}

/// [`BCM_SETTEXTMARGIN`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-settextmargin)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct SetTextMargin<'a> {
	pub margins: &'a RECT,
}

impl<'a> MsgSend for SetTextMargin<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETTEXTMARGIN.into(),
			wparam: 0,
			lparam: self.margins as *const _ as _,
		}
	}
}

pub_struct_msg_empty! { Click, co::BM::CLICK.into(),
	/// [`BM_CLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-click)
}

/// [`BM_GETCHECK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-getcheck)
/// message parameters.
///
/// Return type: `co::BST`.
pub struct GetCheck {}

impl MsgSend for GetCheck {
	type RetType = co::BST;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::BST(v as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::GETCHECK.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`BM_GETIMAGE`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-getimage)
/// message parameters.
///
/// Return type: `WinResult<BmpIcon>`.
pub struct GetImage {
	pub img_type: co::IMAGE_TYPE,
}

impl MsgSend for GetImage {
	type RetType = WinResult<BmpIcon>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match self.img_type {
			co::IMAGE_TYPE::BITMAP => Ok(BmpIcon::Bmp(HBITMAP(v as _))),
			co::IMAGE_TYPE::ICON => Ok(BmpIcon::Icon(HICON(v as _))),
			_ => Err(co::ERROR::BAD_ARGUMENTS),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::GETIMAGE.into(),
			wparam: self.img_type.0 as _,
			lparam: 0,
		}
	}
}

/// [`BM_GETSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-getstate)
/// message, which has no parameters.
///
/// Return type: `co::BST`.
pub struct GetState {}

impl MsgSend for GetState {
	type RetType = co::BST;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::BST(v as _)
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::GETSTATE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`BM_SETCHECK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setcheck)
/// message parameters.
///
/// Return type: `()`.
pub struct SetCheck {
	pub state: co::BST,
}

impl MsgSend for SetCheck {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETCHECK.into(),
			wparam: self.state.0 as _,
			lparam: 0,
		}
	}
}

/// [`BM_SETDONTCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setdontclick)
/// message parameters.
///
/// Return type: `()`.
pub struct SetDontClick {
	pub dont_click: bool,
}

impl MsgSend for SetDontClick {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETDONTCLICK.into(),
			wparam: self.dont_click as _,
			lparam: 0,
		}
	}
}

/// [`BM_SETIMAGE`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setimage)
/// message parameters.
///
/// Return type: `WinResult<BmpIcon>`.
pub struct SetImage {
	pub image: BmpIcon,
}

impl MsgSend for SetImage {
	type RetType = WinResult<BmpIcon>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match self.image {
			BmpIcon::Bmp(_) => Ok(BmpIcon::Bmp(HBITMAP(v as _))),
			BmpIcon::Icon(_) => Ok(BmpIcon::Icon(HICON(v as _))),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETIMAGE.into(),
			wparam: match self.image {
				BmpIcon::Bmp(_) => co::IMAGE_TYPE::BITMAP.0,
				BmpIcon::Icon(_) => co::IMAGE_TYPE::ICON.0,
			} as _,
			lparam: self.image.as_isize(),
		}
	}
}

/// [`BM_SETSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setstate)
/// message parameters.
///
/// Return type: `()`.
pub struct SetState {
	pub highlight: bool,
}

impl MsgSend for SetState {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETSTATE.into(),
			wparam: self.highlight as _,
			lparam: 0,
		}
	}
}

/// [`BM_SETSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setstyle)
/// message parameters.
///
/// Return type: `()`.
pub struct SetStyle {
	pub style: co::BS,
	pub redraw: bool,
}

impl MsgSend for SetStyle {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETSTYLE.into(),
			wparam: self.style.0 as _,
			lparam: self.redraw as _,
		}
	}
}
