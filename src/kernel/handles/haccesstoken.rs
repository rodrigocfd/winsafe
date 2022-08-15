#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::privs::bool_to_sysresult;
use crate::prelude::{Handle, HandleClose};

impl_handle! { HACCESSTOKEN: "kernel";
	/// Handle to an
	/// [access token](https://docs.microsoft.com/en-us/windows/win32/secgloss/a-gly).
	/// Originally just a `HANDLE`.
}

impl HandleClose for HACCESSTOKEN {}
impl kernel_Haccesstoken for HACCESSTOKEN {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HACCESSTOKEN`](crate::HACCESSTOKEN).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait kernel_Haccesstoken: Handle {
	/// [`DuplicateToken`](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetoken)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HACCESSTOKEN::CloseHandle`](crate::prelude::HandleClose::CloseHandle)
	/// call.
	#[must_use]
	fn DuplicateToken(self,
		level: co::SECURITY_IMPERSONATION) -> SysResult<HACCESSTOKEN>
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
		).map(|_| handle)
	}

	/// [`GetCurrentProcessToken`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocesstoken)
	/// static method.
	#[must_use]
	fn GetCurrentProcessToken() -> HACCESSTOKEN {
		HACCESSTOKEN(unsafe { kernel::ffi::GetCurrentProcessToken() })
	}

	/// [`GetCurrentThreadEffectiveToken`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadeffectivetoken)
	/// static method.
	#[must_use]
	fn GetCurrentThreadEffectiveToken() -> HACCESSTOKEN {
		HACCESSTOKEN(unsafe { kernel::ffi::GetCurrentThreadEffectiveToken() })
	}

	/// [`IsTokenRestricted`](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-istokenrestricted)
	/// method.
	#[must_use]
	fn IsTokenRestricted(self) -> SysResult<bool> {
		match unsafe { kernel::ffi::IsTokenRestricted(self.as_ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(false), // actual false
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}
