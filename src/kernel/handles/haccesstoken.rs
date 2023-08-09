#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, ffi_types::*, privs::*};
use crate::prelude::*;

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
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Haccesstoken: Handle {
	/// [`AdjustTokenPrivileges`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let htoken = w::HPROCESS::GetCurrentProcess()
	///     .OpenProcessToken(co::TOKEN::ADJUST_PRIVILEGES | co::TOKEN::QUERY)?;
	///
	/// let luid = w::LookupPrivilegeValue(None, co::SE_PRIV::SHUTDOWN_NAME)?;
	///
	/// let privs = w::TOKEN_PRIVILEGES::new(&[
	///     w::LUID_AND_ATTRIBUTES::new(luid, co::SE_PRIV_ATTR::ENABLED),
	/// ]);
	///
	/// htoken.AdjustTokenPrivileges(w::DisabPriv::Privs(&privs))?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	fn AdjustTokenPrivileges(&self, new_state: DisabPriv) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::AdjustTokenPrivileges(
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
	/// function.
	#[must_use]
	fn CheckTokenCapability(&self,
		capability_sid_to_check: &SID) -> SysResult<bool>
	{
		let mut has_capability: BOOL = 0;
		bool_to_sysresult(
			unsafe {
				ffi::CheckTokenCapability(
					self.ptr(),
					capability_sid_to_check as *const _ as _,
					&mut has_capability,
				)
			},
		).map(|_| has_capability != 0)
	}

	/// [`CheckTokenMembership`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-checktokenmembership)
	/// function.
	#[must_use]
	fn CheckTokenMembership(&self, sid_to_check: &SID) -> SysResult<bool> {
		let mut is_member: BOOL = 0;
		bool_to_sysresult(
			unsafe {
				ffi::CheckTokenMembership(
					self.ptr(),
					sid_to_check as *const _ as _,
					&mut is_member,
				)
			},
		).map(|_| is_member != 0)
	}

	/// [`DuplicateToken`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetoken)
	/// function.
	#[must_use]
	fn DuplicateToken(&self,
		level: co::SECURITY_IMPERSONATION,
	) -> SysResult<CloseHandleGuard<HACCESSTOKEN>>
	{
		let mut handle = HACCESSTOKEN::NULL;
		unsafe {
			bool_to_sysresult(
				ffi::DuplicateToken(
					self.ptr(),
					level.raw(),
					handle.as_mut(),
				),
			).map(|_| CloseHandleGuard::new(handle))
		}
	}

	/// [`GetCurrentProcessToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocesstoken)
	/// function.
	#[must_use]
	fn GetCurrentProcessToken() -> HACCESSTOKEN {
		HACCESSTOKEN(unsafe { ffi::GetCurrentProcessToken() })
	}

	/// [`GetCurrentThreadEffectiveToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadeffectivetoken)
	/// function.
	#[must_use]
	fn GetCurrentThreadEffectiveToken() -> HACCESSTOKEN {
		HACCESSTOKEN(unsafe { ffi::GetCurrentThreadEffectiveToken() })
	}

	/// [`GetTokenInformation`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)
	/// function.
	///
	/// # Safety
	///
	/// Make sure the `information` type is the correct one, matching that in
	/// `information_class`.
	///
	/// # Examples
	///
	/// Checking if the current process has elevated privileges:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let htoken = w::HPROCESS::GetCurrentProcess()
	///     .OpenProcessToken(co::TOKEN::QUERY)?;
	///
	/// let mut elevation = w::TOKEN_ELEVATION::default();
	/// unsafe {
	///     htoken.GetTokenInformation(
	///         co::TOKEN_INFORMATION_CLASS::Elevation,
	///         &mut elevation,
	///      )?;
	/// }
	/// println!("Is elevated: {}", elevation.TokenIsElevated());
	/// # Ok::<_, co::ERROR>(())
	/// ```
	unsafe fn GetTokenInformation<T>(&self,
		information_class: co::TOKEN_INFORMATION_CLASS,
		information: &mut T,
	) -> SysResult<()>
	{
		let mut ret_len = u32::default();
		bool_to_sysresult(
			unsafe {
				ffi::GetTokenInformation(
					self.ptr(),
					information_class.raw(),
					information as *mut _ as _,
					std::mem::size_of::<T>() as _,
					&mut ret_len,
				)
			},
		)
	}

	/// [`IsTokenRestricted`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-istokenrestricted)
	/// function.
	#[must_use]
	fn IsTokenRestricted(&self) -> SysResult<bool> {
		match unsafe { ffi::IsTokenRestricted(self.ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(false), // actual false
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}
