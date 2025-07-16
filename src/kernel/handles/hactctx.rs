#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};
use crate::prelude::*;

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
			match HACTCTX(ffi::CreateActCtxW(pvoid(actctx))) {
				HACTCTX::INVALID => Err(GetLastError()),
				handle => Ok(ReleaseActCtxGuard::new(handle)),
			}
		}
	}

	/// [`ZombifyActCtx`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-zombifyactctx)
	/// function.
	pub fn ZombifyActCtx(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::ZombifyActCtx(self.ptr()) })
	}
}
