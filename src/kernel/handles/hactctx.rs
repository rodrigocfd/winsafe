#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};

handle! { HACTCTX;
	/// Handle to an
	/// [activation context](https://learn.microsoft.com/en-us/windows/win32/sbscs/activation-contexts).
	/// Originally just a `HANDLE`.
}

impl HACTCTX {
	/// [`CreateActCtx`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createactctxw)
	/// function.
	#[must_use]
	pub fn CreateActCtx(actctx: &mut ACTCTX) -> SysResult<ReleaseActCtxGuard> {
		unsafe {
			PtrRet(ffi::CreateActCtxW(pvoid(actctx)))
				.to_sysresult_handle()
				.map(|h| ReleaseActCtxGuard::new(h))
		}
	}

	/// [`ZombifyActCtx`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-zombifyactctx)
	/// function.
	pub fn ZombifyActCtx(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::ZombifyActCtx(self.ptr()) }).to_sysresult()
	}
}
