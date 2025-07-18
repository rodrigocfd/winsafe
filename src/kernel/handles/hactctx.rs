#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
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
	/// [`ActivateActCtx`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-activateactctx)
	/// function.
	///
	/// Returns a cookie which uniquely identifies a specific, activated
	/// activation context.
	///
	/// Deactivation is made by
	/// [`HACTCTX::DeactivateActCtx`](crate::HACTCTX::DeactivateActCtx).
	#[must_use]
	pub fn ActivateActCtx(&self) -> SysResult<usize> {
		let mut cookie = 0usize;
		BoolRet(unsafe { ffi::ActivateActCtx(self.ptr(), &mut cookie) })
			.to_sysresult()
			.map(|_| cookie)
	}

	/// [`AddRefActCtx`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-addrefactctx)
	/// function.
	#[must_use]
	pub fn AddRefActCtx(&self) -> ReleaseActCtxGuard {
		unsafe {
			ffi::AddRefActCtx(self.ptr());
			ReleaseActCtxGuard::new(self.raw_copy())
		}
	}

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

	/// [`DeactivateActCtx`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-deactivateactctx)
	/// function.
	///
	/// Cookie is returned by
	/// [`HACTCTX::ActivateActCtx`](crate::HACTCTX::ActivateActCtx).
	pub fn DeactivateActCtx(flags: co::DEACTIVATE_ACTCTX, cookie: usize) -> SysResult<()> {
		BoolRet(unsafe { ffi::DeactivateActCtx(flags.raw(), cookie) }).to_sysresult()
	}

	/// [`GetCurrentActCtx`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getcurrentactctx)
	/// function.
	#[must_use]
	pub fn GetCurrentActCtx() -> SysResult<ReleaseActCtxGuard> {
		let mut hact = HACTCTX::NULL;
		unsafe {
			BoolRet(ffi::GetCurrentActCtx(hact.as_mut()))
				.to_sysresult()
				.map(|_| ReleaseActCtxGuard::new(hact))
		}
	}

	/// [`ZombifyActCtx`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-zombifyactctx)
	/// function.
	pub fn ZombifyActCtx(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::ZombifyActCtx(self.ptr()) }).to_sysresult()
	}
}
