#![allow(non_camel_case_types, non_snake_case)]

use crate::advapi::ffi;
use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl HPROCESS {
	/// [`OpenProcessToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let htoken = w::HPROCESS::GetCurrentProcess()
	///     .OpenProcessToken(co::TOKEN::ADJUST_PRIVILEGES | co::TOKEN::QUERY)?;
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn OpenProcessToken(
		&self,
		desired_access: co::TOKEN,
	) -> SysResult<CloseHandleGuard<HACCESSTOKEN>> {
		let mut handle = HACCESSTOKEN::NULL;
		unsafe {
			BoolRet(ffi::OpenProcessToken(self.ptr(), desired_access.raw(), handle.as_mut()))
				.to_sysresult()
				.map(|_| CloseHandleGuard::new(handle))
		}
	}
}
