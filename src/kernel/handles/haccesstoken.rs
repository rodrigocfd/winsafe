#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, SID, SysResult};
use crate::kernel::enums::DisabPriv;
use crate::kernel::ffi_types::BOOL;
use crate::kernel::guard::CloseHandleGuard;
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
	/// [`AdjustTokenPrivileges`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{
	///     co, DisabPriv, HPROCESS, LookupPrivilegeValue,
	///     LUID_AND_ATTRIBUTES, TOKEN_PRIVILEGES,
	/// };
	///
	/// let htoken = HPROCESS::GetCurrentProcess()
	///     .OpenProcessToken(co::TOKEN::ADJUST_PRIVILEGES | co::TOKEN::QUERY)?;
	///
	/// let luid = LookupPrivilegeValue(None, co::SE_PRIV::SHUTDOWN_NAME)?;
	///
	/// let privs = TOKEN_PRIVILEGES::new(&[
	///     LUID_AND_ATTRIBUTES::new(luid, co::SE_PRIV_ATTR::ENABLED),
	/// ]);
	///
	/// htoken.AdjustTokenPrivileges(DisabPriv::Privs(&privs))?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	fn AdjustTokenPrivileges(&self, new_state: DisabPriv) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				kernel::ffi::AdjustTokenPrivileges(
					self.ptr(),
					match new_state {
						DisabPriv::Disab => 1,
						_ => 0,
					},
					match new_state {
						DisabPriv::Privs(privs) => privs as *const _ as _,
						_ => std::ptr::null(),
					},
					0,
					std::ptr::null_mut(),
					std::ptr::null_mut(),
				)
			},
		)
	}

	/// [`CheckTokenCapability`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-checktokencapability)
	/// method.
	#[must_use]
	fn CheckTokenCapability(&self,
		capability_sid_to_check: &SID) -> SysResult<bool>
	{
		let mut has_capability: BOOL = 0;
		bool_to_sysresult(
			unsafe {
				kernel::ffi::CheckTokenCapability(
					self.ptr(),
					capability_sid_to_check as *const _ as _,
					&mut has_capability,
				)
			},
		).map(|_| has_capability != 0)
	}

	/// [`CheckTokenMembership`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-checktokenmembership)
	/// method.
	#[must_use]
	fn CheckTokenMembership(&self, sid_to_check: &SID) -> SysResult<bool> {
		let mut is_member: BOOL = 0;
		bool_to_sysresult(
			unsafe {
				kernel::ffi::CheckTokenMembership(
					self.ptr(),
					sid_to_check as *const _ as _,
					&mut is_member,
				)
			},
		).map(|_| is_member != 0)
	}

	/// [`DuplicateToken`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetoken)
	/// method.
	#[must_use]
	fn DuplicateToken(&self,
		level: co::SECURITY_IMPERSONATION,
	) -> SysResult<CloseHandleGuard<HACCESSTOKEN>>
	{
		let mut handle = HACCESSTOKEN::NULL;
		unsafe {
			bool_to_sysresult(
				kernel::ffi::DuplicateToken(
					self.ptr(),
					level.raw(),
					handle.as_mut(),
				),
			).map(|_| CloseHandleGuard::new(handle))
		}
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
		match unsafe { kernel::ffi::IsTokenRestricted(self.ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(false), // actual false
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}
