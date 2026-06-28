use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`BCM_GETIDEALSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-getidealsize)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct BcmGetIdealSize<'a> {
	pub size: &'a mut SIZE,
}

impl<'a> MsgSend for BcmGetIdealSize<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BCM::GETIDEALSIZE.into(),
			wparam: 0,
			lparam: self.size as *mut _ as _,
		}
	}
}

/// [`BCM_GETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-getimagelist)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct BcmGetImageList<'a> {
	pub info: &'a mut BUTTON_IMAGELIST,
}

impl<'a> MsgSend for BcmGetImageList<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BCM::GETIMAGELIST.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`BCM_GETNOTE`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-getnote)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct BcmGetNote<'a> {
	pub text: &'a mut WString,
}

impl<'a> MsgSend for BcmGetNote<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BCM::GETNOTE.into(),
			wparam: self.text.buf_len(),
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`BCM_GETNOTELENGTH`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-getnotelength)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct BcmGetNoteLength {}

impl MsgSend for BcmGetNoteLength {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BCM::GETNOTELENGTH.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`BCM_GETSPLITINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-getsplitinfo)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct BcmGetSplitInfo<'a> {
	pub splitinfo: &'a mut BUTTON_SPLITINFO,
}

impl<'a> MsgSend for BcmGetSplitInfo<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BCM::GETSPLITINFO.into(),
			wparam: 0,
			lparam: self.splitinfo as *mut _ as _,
		}
	}
}

/// [`BCM_GETTEXTMARGIN`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-gettextmargin)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct BcmGetTextMargin<'a> {
	pub margins: &'a mut RECT,
}

impl<'a> MsgSend for BcmGetTextMargin<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BCM::GETTEXTMARGIN.into(),
			wparam: 0,
			lparam: self.margins as *mut _ as _,
		}
	}
}

/// [`BCM_SETDROPDOWNSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-setdropdownstate)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct BcmSetDropDownState {
	pub is_pushed: bool,
}

impl MsgSend for BcmSetDropDownState {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BCM::SETDROPDOWNSTATE.into(),
			wparam: self.is_pushed as _,
			lparam: 0,
		}
	}
}

/// [`BCM_SETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-setimagelist)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct BcmSetImageList<'a> {
	pub info: &'a BUTTON_IMAGELIST,
}

impl<'a> MsgSend for BcmSetImageList<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BCM::SETIMAGELIST.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`BCM_SETNOTE`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-setnote)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct BcmSetNote {
	pub text: WString,
}

impl MsgSend for BcmSetNote {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BCM::SETNOTE.into(),
			wparam: self.text.buf_len(),
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`BCM_SETSHIELD`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-setshield)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct BcmSetShield {
	pub has_elevated_icon: bool,
}

impl MsgSend for BcmSetShield {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BCM::SETSHIELD.into(),
			wparam: self.has_elevated_icon as _,
			lparam: 0,
		}
	}
}

/// [`BCM_SETSPLITINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-setsplitinfo)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct BcmSetSplitInfo<'a> {
	pub splitinfo: &'a BUTTON_SPLITINFO,
}

impl<'a> MsgSend for BcmSetSplitInfo<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BCM::SETSPLITINFO.into(),
			wparam: 0,
			lparam: self.splitinfo as *const _ as _,
		}
	}
}

/// [`BCM_SETTEXTMARGIN`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-settextmargin)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct BcmSetTextMargin<'a> {
	pub margins: &'a RECT,
}

impl<'a> MsgSend for BcmSetTextMargin<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::BCM::SETTEXTMARGIN.into(),
			wparam: 0,
			lparam: self.margins as *const _ as _,
		}
	}
}
