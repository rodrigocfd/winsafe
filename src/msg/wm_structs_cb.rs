use crate::aliases::WinResult;
use crate::co;
use crate::funcs::GetLastError;
use crate::msg::{Message, Wm};
use crate::privs::{CB_ERR, CB_ERRSPACE};
use crate::WString;

/// [`CB_ADDSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-addstring)
/// message parameters.
pub struct CbAddString<'a> {
	pub text: &'a str,
}

impl<'a> Message for CbAddString<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR | CB_ERRSPACE => Err(GetLastError()),
			idx => Ok(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CB_ADDSTRING,
			wparam: 0,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_DELETESTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-deletestring)
/// message parameters.
pub struct CbDeleteString {
	pub index: u32,
}

impl Message for CbDeleteString {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(GetLastError()),
			count => Ok(count as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CB_DELETESTRING,
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_GETCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getcount)
/// message parameters.
pub struct CbGetCount {}

impl Message for CbGetCount {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(GetLastError()),
			count => Ok(count as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CB_GETCOUNT,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_GETCURSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getcursel)
/// message parameters.
pub struct CbGetCurSel {}

impl Message for CbGetCurSel {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			count => Some(count as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CB_GETCURSEL,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_GETLBTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getlbtext)
/// message parameters.
pub struct CbGetLbText<'a> {
	pub index: u32,
	pub text: &'a mut WString,
}

impl<'a> Message for CbGetLbText<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(GetLastError()),
			nchars => Ok(nchars as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CB_GETLBTEXT,
			wparam: self.index as usize,
			lparam: unsafe { self.text.as_ptr() } as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_GETLBTEXTLEN`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getlbtextlen)
/// message parameters.
pub struct CbGetLbTextLen {
	pub index: u32,
}

impl Message for CbGetLbTextLen {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(GetLastError()),
			nchars => Ok(nchars as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CB_GETLBTEXTLEN,
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_GETMINVISIBLE`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-getminvisible)
/// message parameters.
pub struct CbGetMinVisible {}

impl Message for CbGetMinVisible {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CB_GETMINVISIBLE,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_GETTOPINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-gettopindex)
/// message parameters.
pub struct CbGetTopIndex {}

impl Message for CbGetTopIndex {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(GetLastError()),
			idx => Ok(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CB_GETTOPINDEX,
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_INITSTORAGE`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-initstorage)
/// message parameters.
pub struct CbInitStorage {
	pub num_items: u32,
	pub memory_bytes: u32,
}

impl Message for CbInitStorage {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERRSPACE => Err(GetLastError()),
			n_items => Ok(n_items as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CB_INITSTORAGE,
			wparam: self.num_items as usize,
			lparam: self.memory_bytes as isize,
		}
	}
}

//------------------------------------------------------------------------------

empty_msg! { CbResetContent, co::WM::CB_RESETCONTENT,
	/// [`CB_RESETCONTENT`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-resetcontent)
	/// message parameters.
}

//------------------------------------------------------------------------------

/// [`CB_SETCURSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-setcursel)
/// message parameters.
pub struct CbSetCurSel {
	pub index: Option<u32>,
}

impl Message for CbSetCurSel {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CB_SETCURSEL,
			wparam: match self.index {
				Some(index) => index as i32,
				None => -1,
			} as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_SETMINVISIBLE`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-setminvisible)
/// message parameters.
pub struct CbSetMinVisible {
	pub num_items: u32,
}

impl Message for CbSetMinVisible {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CB_SETMINVISIBLE,
			wparam: self.num_items as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_SETTOPINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-settopindex)
/// message parameters.
pub struct CbSetTopIndex {
	pub index: u32,
}

impl Message for CbSetTopIndex {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CB_SETTOPINDEX,
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`CB_SHOWDROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/cb-showdropdown)
/// message parameters.
pub struct CbShowDropDown {
	pub show: bool,
}

impl Message for CbShowDropDown {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::WM::CB_SHOWDROPDOWN,
			wparam: self.show as usize,
			lparam: 0,
		}
	}
}
