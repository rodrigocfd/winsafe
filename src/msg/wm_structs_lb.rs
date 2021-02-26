use crate::aliases::WinResult;
use crate::co;
use crate::msg::{Message, Wm};
use crate::privs::{LB_ERR, LB_ERRSPACE};
use crate::WString;

/// [`LB_ADDFILE`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-addfile)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct LbAddFile<'a> {
	pub text: &'a str,
}

impl<'a> Message for LbAddFile<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR | LB_ERRSPACE => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::LB::ADDSTRING.into(),
			wparam: 0,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LB_ADDSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-addstring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct LbAddString<'a> {
	pub text: &'a str,
}

impl<'a> Message for LbAddString<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR | LB_ERRSPACE => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::LB::ADDSTRING.into(),
			wparam: 0,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LB_DELETESTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-deletestring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct LbDeleteString {
	pub index: u32,
}

impl Message for LbDeleteString {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::LB::DELETESTRING.into(),
			wparam: self.index as usize,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LB_DIR`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-dir)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct LbDir<'a> {
	pub attributes: co::DDL,
	pub path: &'a str,
}

impl<'a> Message for LbDir<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::LB::DELETESTRING.into(),
			wparam: self.attributes.0 as usize,
			lparam: unsafe { WString::from_str(self.path).as_ptr() } as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LB_FINDSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-findstring)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct LbFindString<'a> {
	pub preceding_index: Option<u32>,
	pub text: &'a str,
}

impl<'a> Message for LbFindString<'a> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => None,
			idx => Some(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::LB::FINDSTRING.into(),
			wparam: match self.preceding_index {
				None => -1,
				Some(idx) => idx as i32,
			} as usize,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LB_FINDSTRINGEXACT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-findstringexact)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct LbFindStringExact<'a> {
	pub preceding_index: Option<u32>,
	pub text: &'a str,
}

impl<'a> Message for LbFindStringExact<'a> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => None,
			idx => Some(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::LB::FINDSTRINGEXACT.into(),
			wparam: match self.preceding_index {
				None => -1,
				Some(idx) => idx as i32,
			} as usize,
			lparam: unsafe { WString::from_str(self.text).as_ptr() } as isize,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LB_GETANCHORINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getanchorindex)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct LbGetAnchorIndex {}

impl Message for LbGetAnchorIndex {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::LB::GETANCHORINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LB_GETCARETINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getcaretindex)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct LbGetCaretIndex {}

impl Message for LbGetCaretIndex {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::LB::GETCARETINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LB_GETCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getcount)
/// message, which has no parameters.
///
/// Return type: `WinResult<u32>`.
pub struct LbGetCount {}

impl Message for LbGetCount {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::LB::GETCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LB_GETCURSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getcursel)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
pub struct LbGetCurSel {}

impl Message for LbGetCurSel {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => None,
			idx => Some(idx as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::LB::GETCURSEL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LB_GETHORIZONTALEXTENT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-gethorizontalextent)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct LbGetHorizontalExtent {}

impl Message for LbGetHorizontalExtent {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as u32
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::LB::GETHORIZONTALEXTENT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LB_GETITEMDATA`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getitemdata)
/// message parameters.
///
/// Return type: `WinResult<isize>`.
pub struct LbGetItemData {
	pub index: u32,
}

impl Message for LbGetItemData {
	type RetType = WinResult<isize>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		const LB_ERR_ISIZE: isize = LB_ERR as _;
		match v {
			LB_ERR_ISIZE => Err(co::ERROR::BAD_ARGUMENTS),
			data => Ok(data),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::LB::GETITEMDATA.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

//------------------------------------------------------------------------------

/// [`LB_GETITEMHEIGHT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getitemheight)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct LbGetItemHeight {
	pub index: Option<u32>,
}

impl Message for LbGetItemHeight {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			height => Ok(height as u32),
		}
	}

	fn as_generic_wm(&self) -> Wm {
		Wm {
			msg_id: co::LB::GETITEMHEIGHT.into(),
			wparam: self.index.unwrap_or(0) as usize,
			lparam: 0,
		}
	}
}
