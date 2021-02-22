use crate::aliases::WinResult;
use crate::co;
use crate::enums::BitmapIcon;
use crate::handles::{HBITMAP, HICON};
use crate::msg::{Message, Wm};
use crate::structs::{BUTTON_IMAGELIST, BUTTON_SPLITINFO, RECT, SIZE};
use crate::WString;

/// [`BCM_GETIDEALSIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getidealsize)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct BcmGetIdealSize<'a> {
	pub size: &'a mut SIZE,
}

impl<'a> Message for BcmGetIdealSize<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_GETIDEALSIZE,
			wparam: 0,
			lparam: self.size as *const _ as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_GETIMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getimagelist)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct BcmGetImageList<'a> {
	pub info: &'a mut BUTTON_IMAGELIST,
}

impl<'a> Message for BcmGetImageList<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_GETIMAGELIST,
			wparam: 0,
			lparam: self.info as *const _ as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_GETNOTE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getnote)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct BcmGetNote<'a> {
	pub text: &'a mut WString,
}

impl<'a> Message for BcmGetNote<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_GETNOTE,
			wparam: self.text.buffer_size(),
			lparam: unsafe { self.text.as_ptr() } as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_GETNOTELENGTH`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getnotelength)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct BcmGetNoteLength {}

impl Message for BcmGetNoteLength {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_GETNOTELENGTH,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_GETSPLITINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getsplitinfo)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct BcmGetSplitInfo<'a> {
	pub splitinfo: &'a mut BUTTON_SPLITINFO,
}

impl<'a> Message for BcmGetSplitInfo<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_GETSPLITINFO,
			wparam: 0,
			lparam: self.splitinfo as *const _ as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_GETTEXTMARGIN`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-gettextmargin)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct BcmGetTextMargin<'a> {
	pub margins: &'a mut RECT,
}

impl<'a> Message for BcmGetTextMargin<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_GETTEXTMARGIN,
			wparam: 0,
			lparam: self.margins as *const _ as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_SETDROPDOWNSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-setdropdownstate)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct BcmSetDropDownState {
	pub is_pushed: bool,
}

impl Message for BcmSetDropDownState {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_SETDROPDOWNSTATE,
			wparam: self.is_pushed as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_SETIMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-setimagelist)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct BcmSetImageList<'a> {
	pub info: &'a BUTTON_IMAGELIST,
}

impl<'a> Message for BcmSetImageList<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_SETIMAGELIST,
			wparam: 0,
			lparam: self.info as *const _ as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_SETNOTE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-setnote)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct BcmSetNote<'a> {
	pub text: &'a WString,
}

impl<'a> Message for BcmSetNote<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_SETNOTE,
			wparam: self.text.buffer_size(),
			lparam: unsafe { self.text.as_ptr() } as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_SETSHIELD`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-setshield)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct BcmSetShield {
	pub has_elevated_icon: bool,
}

impl Message for BcmSetShield {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_SETSHIELD,
			wparam: self.has_elevated_icon as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_SETSPLITINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-setsplitinfo)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct BcmSetSplitInfo<'a> {
	pub splitinfo: &'a BUTTON_SPLITINFO,
}

impl<'a> Message for BcmSetSplitInfo<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_SETSPLITINFO,
			wparam: 0,
			lparam: self.splitinfo as *const _ as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_SETTEXTMARGIN`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-settextmargin)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct BcmSetTextMargin<'a> {
	pub margins: &'a RECT,
}

impl<'a> Message for BcmSetTextMargin<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_SETTEXTMARGIN,
			wparam: 0,
			lparam: self.margins as *const _ as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_CLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-click)
/// message, which has no parameters.
///
/// Return type: `()`.
pub struct BmClick {}

impl Message for BmClick {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BM_CLICK,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_GETCHECK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-getcheck)
/// message parameters.
///
/// Return type: `BST`.
pub struct BmGetCheck {}

impl Message for BmGetCheck {
	type RetType = co::BST;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::BST(v as u32)
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BM_GETCHECK,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_GETIMAGE`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-getimage)
/// message parameters.
///
/// Return type: `WinResult<BitmapIcon>`.
pub struct BmGetImage {
	pub img_type: co::IMAGE_TYPE,
}

impl Message for BmGetImage {
	type RetType = WinResult<BitmapIcon>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match self.img_type {
			co::IMAGE_TYPE::BITMAP => Ok(BitmapIcon::Bitmap(HBITMAP { ptr: v as *mut _ })),
			co::IMAGE_TYPE::ICON => Ok(BitmapIcon::Icon(HICON { ptr: v as *mut _ })),
			_ => Err(co::ERROR::BAD_ARGUMENTS),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BM_GETIMAGE,
			wparam: self.img_type.0 as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_GETSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-getstate)
/// message, which has no parameters.
///
/// Return type: `BST`.
pub struct BmGetState {}

impl Message for BmGetState {
	type RetType = co::BST;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::BST(v as u32)
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BM_GETSTATE,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_SETCHECK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setcheck)
/// message parameters.
///
/// Return type: `()`.
pub struct BmSetCheck {
	pub state: co::BST,
}

impl Message for BmSetCheck {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BM_SETCHECK,
			wparam: self.state.0 as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_SETDONTCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setdontclick)
/// message parameters.
///
/// Return type: `()`.
pub struct BmSetDontClick {
	pub dont_click: bool,
}

impl Message for BmSetDontClick {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BM_SETDONTCLICK,
			wparam: self.dont_click as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_SETIMAGE`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setimage)
/// message parameters.
///
/// Return type: `WinResult<BitmapIcon>`.
pub struct BmSetImage {
	pub image: BitmapIcon,
}

impl Message for BmSetImage {
	type RetType = WinResult<BitmapIcon>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match self.image {
			BitmapIcon::Bitmap(_) => Ok(BitmapIcon::Bitmap(HBITMAP { ptr: v as *mut _ })),
			BitmapIcon::Icon(_) => Ok(BitmapIcon::Icon(HICON { ptr: v as *mut _ })),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BM_SETIMAGE,
			wparam: match self.image {
				BitmapIcon::Bitmap(_) => co::IMAGE_TYPE::BITMAP.0,
				BitmapIcon::Icon(_) => co::IMAGE_TYPE::ICON.0,
			} as usize,
			lparam: self.image.as_isize(),
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_SETSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setstate)
/// message parameters.
///
/// Return type: `()`.
pub struct BmSetState {
	pub highlight: bool,
}

impl Message for BmSetState {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BM_SETSTATE,
			wparam: self.highlight as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_SETSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setstyle)
/// message parameters.
///
/// Return type: `()`.
pub struct BmSetStyle {
	pub style: co::BS,
	pub redraw: bool,
}

impl Message for BmSetStyle {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BM_SETSTYLE,
			wparam: self.style.0 as usize,
			lparam: self.redraw as isize,
		}
	}
}
