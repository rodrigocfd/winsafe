#![allow(non_camel_case_types, non_snake_case)]

use crate::advapi::ffi;
use crate::decl::*;
use crate::kernel::privs::*;

handle! { HSERVICE;
	/// Handle to a
	/// [service](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-openservicew).
	/// Originally `SC_HANDLE`.
}

impl HSERVICE {
	/// [`DeleteService`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-deleteservice)
	/// function.
	pub fn DeleteService(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::DeleteService(self.ptr()) }).to_sysresult()
	}
}
