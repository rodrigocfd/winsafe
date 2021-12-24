#![allow(non_snake_case)]

use crate::co;
use crate::kernel::decl::{HPROCESS, WinResult};
use crate::kernel::privs::bool_to_winresult;
use crate::user;
use crate::prelude::Handle;

impl UserHprocess for HPROCESS {}

/// [`HPROCESS`](crate::HPROCESS) methods from `user` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub trait UserHprocess: Handle {
	/// [`SetUserObjectInformation`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setuserobjectinformationw)
	/// method.
	///
	/// **Note:** The `pv_info` type varies according to `index`. If you set it
	/// wrong, you're likely to cause a buffer overrun.
	unsafe fn SetUserObjectInformation<T>(self,
		index: co::UOI, pv_info: &mut T) -> WinResult<()>
	{
		bool_to_winresult(
			user::ffi::SetUserObjectInformationW(
				self.as_ptr(),
				index.0,
				pv_info as *mut _ as _,
				std::mem::size_of::<T>() as _,
			),
		)
	}
}
