use crate::co;
use crate::msg::macros::{lparam_to_ref, ref_to_lparam};
use crate::msg::Wm;
use crate::structs::SIZE;

/// [`BCM_GETIDEALSIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcm-getidealsize)
/// message parameters.
pub struct BcmGetIdealSize<'a> {
	pub size: &'a SIZE,
}

impl<'a> From<BcmGetIdealSize<'a>> for Wm {
	fn from(p: BcmGetIdealSize) -> Self {
		Self {
			msg_id: co::WM::BCM_GETIDEALSIZE,
			wparam: 0,
			lparam: ref_to_lparam(p.size),
		}
	}
}

impl<'a> From<Wm> for BcmGetIdealSize<'a> {
	fn from(p: Wm) -> Self {
		Self {
			size: lparam_to_ref(p),
		}
	}
}

//------------------------------------------------------------------------------

empty_msg! { BmGetCheck, co::WM::BM_GETCHECK,
	/// [`BM_GETCHECK`](https://docs.microsoft.com/en-us/windows/win32/controls/bm-getcheck)
	/// message, which has no parameters.
}
