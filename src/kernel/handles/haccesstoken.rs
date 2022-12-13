#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::guard::HandleGuard;
use crate::kernel::privs::bool_to_sysresult;
use crate::prelude::Handle;

impl_handle! { HACCESSTOKEN;
	/// Handle to an
	/// [access token](https://learn.microsoft.com/en-us/windows/win32/secgloss/a-gly).
	/// Originally just a `HANDLE`.
}

impl kernel_Haccesstoken for HACCESSTOKEN {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HACCESSTOKEN`](crate::HACCESSTOKEN).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Haccesstoken: Handle {
	/// [`DuplicateToken`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetoken)
	/// method.
	#[must_use]
	fn DuplicateToken(&self,
		level: co::SECURITY_IMPERSONATION) -> SysResult<HandleGuard<HACCESSTOKEN>>
	{
		let mut handle = HACCESSTOKEN::NULL;
		bool_to_sysresult(
			unsafe {
				kernel::ffi::DuplicateToken(
					self.as_ptr(),
					level.0,
					&mut handle.0,
				)
			},
		).map(|_| HandleGuard { handle })
	}

	/// [`GetCurrentProcessToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocesstoken)
	/// static method.
	#[must_use]
	fn GetCurrentProcessToken() -> HACCESSTOKEN {
		HACCESSTOKEN(unsafe { kernel::ffi::GetCurrentProcessToken() })
	}

	/// [`GetCurrentThreadEffectiveToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadeffectivetoken)
	/// static method.
	#[must_use]
	fn GetCurrentThreadEffectiveToken() -> HACCESSTOKEN {
		HACCESSTOKEN(unsafe { kernel::ffi::GetCurrentThreadEffectiveToken() })
	}

	/// [`IsTokenRestricted`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-istokenrestricted)
	/// method.
	#[must_use]
	fn IsTokenRestricted(&self) -> SysResult<bool> {
		match unsafe { kernel::ffi::IsTokenRestricted(self.as_ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(false), // actual false
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}
