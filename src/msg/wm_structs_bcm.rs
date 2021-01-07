use crate::co;
use crate::funcs::GetLastError;
use crate::msg::macros::{lparam_to_ref, ref_to_lparam};
use crate::msg::{Message, Wm};
use crate::structs::SIZE;

/// [`BCM_GETIDEALSIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getidealsize)
/// message parameters.
pub struct BcmGetIdealSize<'a> {
	pub size: &'a SIZE,
}

impl<'a> Message for BcmGetIdealSize<'a> {
	type RetType = Result<(), co::ERROR>;

	fn convert_ret(v: isize) -> Result<(), co::ERROR> {
		match v {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::BCM_GETIDEALSIZE,
			wparam: 0,
			lparam: ref_to_lparam(self.size),
		}
	}

	fn from_generic_wm(p: Wm) -> Self {
		Self {
			size: lparam_to_ref(p),
		}
	}
}

//------------------------------------------------------------------------------

/// [`BM_GETCHECK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-getcheck)
/// message.
pub struct BmGetCheck {}

impl Message for BmGetCheck {
	type RetType = co::BST;

	fn convert_ret(v: isize) -> co::BST {
		co::BST::from(v as u32)
	}

	fn into_generic_wm(self) -> Wm {
		Wm {
			msg_id: co::WM::BM_GETCHECK,
			wparam: 0,
			lparam: 0,
		}
	}

	fn from_generic_wm(_: Wm) -> Self {
		Self {}
	}
}
