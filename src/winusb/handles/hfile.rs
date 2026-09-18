#![allow(non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::winusb::ffi;

impl HFILE {
	/// [`WinUsb_Initialize`](https://learn.microsoft.com/en-us/windows/win32/api/winusb/nf-winusb-winusb_initialize)
	/// function.
	#[must_use]
	pub fn WinUsbInitialize(&self) -> SysResult<WinUsbFreeGuard> {
		let mut handle = HUSB::NULL;
		unsafe {
			BoolRet(ffi::WinUsb_Initialize(self.ptr(), handle.as_mut()))
				.to_sysresult()
				.map(|_| WinUsbFreeGuard::new(handle))
		}
	}
}
