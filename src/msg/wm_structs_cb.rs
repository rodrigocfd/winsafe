use crate::aliases::WinResult;
use crate::co;
use crate::funcs_priv::{CB_ERR, CB_ERRSPACE};
use crate::funcs::GetLastError;
use crate::msg::{Message, Wm};
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
