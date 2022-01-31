#![allow(non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, WinResult};
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::{Handle, HandleClose};

impl_handle! { HACCESSTOKEN: "kernel";
	/// Handle to an
	/// [access token](https://docs.microsoft.com/en-us/windows/win32/secgloss/a-gly).
	/// Originally just a `HANDLE`.
}

impl HandleClose for HACCESSTOKEN {}
impl KernelHaccesstoken for HACCESSTOKEN {}

/// [`HACCESSTOKEN`](crate::HACCESSTOKEN) methods from `kernel` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait KernelHaccesstoken: Handle {
	/// [`DuplicateToken`](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetoken)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HACCESSTOKEN::CloseHandle`](crate::prelude::HandleClose::CloseHandle)
	/// call.
	fn DuplicateToken(self,
		level: co::SECURITY_IMPERSONATION) -> WinResult<HACCESSTOKEN>
	{
		let mut handle = HACCESSTOKEN::NULL;
		bool_to_winresult(
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
	fn GetCurrentProcessToken() -> HACCESSTOKEN {
		HACCESSTOKEN(unsafe { kernel::ffi::GetCurrentProcessToken() })
	}

	/// [`GetCurrentThreadEffectiveToken`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadeffectivetoken)
	/// static method.
	fn GetCurrentThreadEffectiveToken() -> HACCESSTOKEN {
		HACCESSTOKEN(unsafe { kernel::ffi::GetCurrentThreadEffectiveToken() })
	}

	/// [`IsTokenRestricted`](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-istokenrestricted)
	/// method.
	fn IsTokenRestricted(self) -> WinResult<bool> {
		match unsafe { kernel::ffi::IsTokenRestricted(self.as_ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(false), // actual false
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}
