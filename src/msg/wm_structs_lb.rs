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
