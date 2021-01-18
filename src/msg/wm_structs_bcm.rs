use crate::aliases::WinResult;
use crate::co;
use crate::enums::BitmapIcon;
use crate::funcs::GetLastError;
use crate::handles::{HBITMAP, HICON};
use crate::msg::{Message, Wm};
use crate::msg::macros::ref_to_lp;
use crate::structs::{BUTTON_IMAGELIST, BUTTON_SPLITINFO, RECT, SIZE};
use crate::WString;

/// [`BCM_GETIDEALSIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getidealsize)
/// message parameters.
pub struct BcmGetIdealSize<'a> {
	pub size: &'a SIZE,
}

impl<'a> Message for BcmGetIdealSize<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_GETIDEALSIZE,
			wparam: 0,
			lparam: ref_to_lp(self.size),
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_GETIMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getimagelist)
/// message parameters.
pub struct BcmGetImageList<'a> {
	pub info: &'a mut BUTTON_IMAGELIST,
}

impl<'a> Message for BcmGetImageList<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_GETIMAGELIST,
			wparam: 0,
			lparam: ref_to_lp(self.info),
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_GETNOTE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getnote)
/// message parameters.
pub struct BcmGetNote<'a> {
	pub text: &'a mut WString,
}

impl<'a> Message for BcmGetNote<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
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
/// message parameters.
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
pub struct BcmGetSplitInfo<'a> {
	pub splitinfo: &'a mut BUTTON_SPLITINFO,
}

impl<'a> Message for BcmGetSplitInfo<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_GETSPLITINFO,
			wparam: 0,
			lparam: ref_to_lp(self.splitinfo),
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_GETTEXTMARGIN`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-gettextmargin)
/// message parameters.
pub struct BcmGetTextMargin<'a> {
	pub margins: &'a mut RECT,
}

impl<'a> Message for BcmGetTextMargin<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_GETTEXTMARGIN,
			wparam: 0,
			lparam: ref_to_lp(self.margins),
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_SETDROPDOWNSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-setdropdownstate)
/// message parameters.
pub struct BcmSetDropDownState {
	pub is_pushed: bool,
}

impl Message for BcmSetDropDownState {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
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
pub struct BcmSetImageList<'a> {
	pub info: &'a BUTTON_IMAGELIST,
}

impl<'a> Message for BcmSetImageList<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_SETIMAGELIST,
			wparam: 0,
			lparam: ref_to_lp(self.info),
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_SETNOTE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-setnote)
/// message parameters.
pub struct BcmSetNote<'a> {
	pub text: &'a WString,
}

impl<'a> Message for BcmSetNote<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
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
pub struct BcmSetShield {
	pub has_elevated_icon: bool,
}

impl Message for BcmSetShield {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
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
pub struct BcmSetSplitInfo<'a> {
	pub splitinfo: &'a BUTTON_SPLITINFO,
}

impl<'a> Message for BcmSetSplitInfo<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_SETSPLITINFO,
			wparam: 0,
			lparam: ref_to_lp(self.splitinfo),
		}
	}
}

//------------------------------------------------------------------------------

/// [`BCM_SETTEXTMARGIN`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-settextmargin)
/// message parameters.
pub struct BcmSetTextMargin<'a> {
	pub margins: &'a RECT,
}

impl<'a> Message for BcmSetTextMargin<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_SETTEXTMARGIN,
			wparam: 0,
			lparam: ref_to_lp(self.margins),
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_CLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-click)
/// message parameters.
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
pub struct BmGetCheck {}

impl Message for BmGetCheck {
	type RetType = co::BST;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::BST::from(v as u32)
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
pub struct BmGetImage {
	pub img_type: co::IMAGE_TYPE,
}

impl Message for BmGetImage {
	type RetType = WinResult<BitmapIcon>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match self.img_type {
			co::IMAGE_TYPE::BITMAP => Ok(BitmapIcon::Bitmap(HBITMAP { ptr: v as *mut _ })),
			co::IMAGE_TYPE::ICON => Ok(BitmapIcon::Icon(HICON { ptr: v as *mut _ })),
			_ => Err(co::ERROR::INVALID_PARAMETER),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::BM_GETIMAGE,
			wparam: u8::from(self.img_type) as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_GETSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-getstate)
/// message parameters.
pub struct BmGetState {}

impl Message for BmGetState {
	type RetType = co::BST;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::BST::from(v as u32)
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
			wparam: u32::from(self.state) as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_SETDONTCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setdontclick)
/// message parameters.
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
				BitmapIcon::Bitmap(_) => u8::from(co::IMAGE_TYPE::BITMAP),
				BitmapIcon::Icon(_) => u8::from(co::IMAGE_TYPE::ICON),
			} as usize,
			lparam: self.image.as_isize(),
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_SETSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-setstate)
/// message parameters.
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
			wparam: u32::from(self.style) as usize,
			lparam: self.redraw as isize,
		}
	}
}
