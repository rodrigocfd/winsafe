#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::user::ffi;

impl HPROCESS {
	/// [`SetUserObjectInformation`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setuserobjectinformationw)
	/// function.
	///
	/// # Safety
	///
	/// The `pv_info` type varies according to `index`. If you set it wrong,
	/// you're likely to cause a buffer overrun.
	pub unsafe fn SetUserObjectInformation<T>(
		&self,
		index: co::UOI,
		pv_info: &mut T,
	) -> SysResult<()> {
		bool_to_sysresult(unsafe {
			ffi::SetUserObjectInformationW(
				self.ptr(),
				index.raw(),
				pvoid(pv_info),
				std::mem::size_of::<T>() as _,
			)
		})
	}

	/// [`WaitForInputIdle`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-waitforinputidle)
	/// function.
	pub fn WaitForInputIdle(&self, milliseconds: u32) -> SysResult<SuccessTimeout> {
		match unsafe { ffi::WaitForInputIdle(self.ptr(), milliseconds) } {
			0 => Ok(SuccessTimeout::Success),
			0x0000_0102 => Ok(SuccessTimeout::Timeout),
			_ => Err(GetLastError()),
		}
	}
}
