#![allow(non_camel_case_types, non_snake_case)]

use crate::advapi::ffi;
use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

handle! { HACCESSTOKEN;
	/// Handle to an
	/// [access token](https://learn.microsoft.com/en-us/windows/win32/secgloss/a-gly).
	/// Originally just a `HANDLE`.
}

impl HACCESSTOKEN {
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
	/// ])?;
	///
	/// htoken.AdjustTokenPrivileges(w::DisabPriv::Privs(&privs))?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn AdjustTokenPrivileges(&self, new_state: DisabPriv) -> SysResult<()> {
		bool_to_sysresult(unsafe {
			ffi::AdjustTokenPrivileges(
				self.ptr(),
				match new_state {
					DisabPriv::Disab => 1,
					_ => 0,
				},
				match new_state {
					DisabPriv::Privs(privs) => pcvoid(privs),
					_ => std::ptr::null(),
				},
				0,
				std::ptr::null_mut(),
				std::ptr::null_mut(),
			)
		})
	}

	/// [`CheckTokenCapability`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-checktokencapability)
	/// function.
	#[must_use]
	pub fn CheckTokenCapability(&self, capability_sid_to_check: &SID) -> SysResult<bool> {
		let mut has_capability = 0;
		bool_to_sysresult(unsafe {
			ffi::CheckTokenCapability(
				self.ptr(),
				pcvoid(capability_sid_to_check),
				&mut has_capability,
			)
		})
		.map(|_| has_capability != 0)
	}

	/// [`CheckTokenMembership`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-checktokenmembership)
	/// function.
	#[must_use]
	pub fn CheckTokenMembership(&self, sid_to_check: &SID) -> SysResult<bool> {
		let mut is_member = 0;
		bool_to_sysresult(unsafe {
			ffi::CheckTokenMembership(self.ptr(), pcvoid(sid_to_check), &mut is_member)
		})
		.map(|_| is_member != 0)
	}

	/// [`DuplicateToken`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetoken)
	/// function.
	#[must_use]
	pub fn DuplicateToken(
		&self,
		level: co::SECURITY_IMPERSONATION,
	) -> SysResult<CloseHandleGuard<HACCESSTOKEN>> {
		let mut handle = HACCESSTOKEN::NULL;
		unsafe {
			bool_to_sysresult(ffi::DuplicateToken(self.ptr(), level.raw(), handle.as_mut()))
				.map(|_| CloseHandleGuard::new(handle))
		}
	}

	/// [`GetCurrentProcessToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocesstoken)
	/// function.
	#[must_use]
	pub fn GetCurrentProcessToken() -> HACCESSTOKEN {
		// We don't do a FFI call because there's no actual library function to call: this is an
		// inlined function defined in the processthreadsapi.h header that always returns a constant.
		// See: https://github.com/microsoft/win32metadata/issues/436
		HACCESSTOKEN(-4 as _)
	}

	/// [`GetCurrentThreadEffectiveToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadeffectivetoken)
	/// function.
	#[must_use]
	pub fn GetCurrentThreadEffectiveToken() -> HACCESSTOKEN {
		// We don't do a FFI call because there's no actual library function to call: this is an
		// inlined function defined in the processthreadsapi.h header that always returns a constant.
		// See: https://github.com/microsoft/win32metadata/issues/436
		HACCESSTOKEN(-6 as _)
	}

	/// [`GetTokenInformation`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)
	/// function.
	///
	/// The returned enum variant will correspond to the passed
	/// `information_class`.
	///
	/// # Examples
	///
	/// Retrieving the `Groups` information:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let htoken = w::HPROCESS::GetCurrentProcess()
	///     .OpenProcessToken(co::TOKEN::QUERY)?;
	///
	/// let nfo = htoken.GetTokenInformation(co::TOKEN_INFORMATION_CLASS::Groups)?;
	/// let w::TokenInfo::Groups(groups) = nfo else { unreachable!() };
	///
	/// for (idx, g) in groups.Groups().iter().enumerate() {
	///     println!("{}: {}", idx, g.Sid().unwrap());
	/// }
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn GetTokenInformation(
		&self,
		information_class: co::TOKEN_INFORMATION_CLASS,
	) -> SysResult<TokenInfo> {
		let mut num_bytes = u32::default();
		match bool_to_sysresult(unsafe {
			ffi::GetTokenInformation(
				self.ptr(),
				information_class.raw(),
				std::ptr::null_mut(),
				0,
				&mut num_bytes,
			)
		}) {
			Err(err) => match err {
				co::ERROR::INSUFFICIENT_BUFFER => {}, // all good
				err => return Err(err),
			},
			Ok(_) => return Err(co::ERROR::INVALID_PARAMETER), // should never happen
		};

		let mut buf = vec![0u8; num_bytes as usize].into_boxed_slice();

		unsafe {
			bool_to_sysresult(ffi::GetTokenInformation(
				self.ptr(),
				information_class.raw(),
				buf.as_mut_ptr() as _,
				num_bytes,
				&mut num_bytes,
			))
			.map(|_| match information_class {
				co::TOKEN_INFORMATION_CLASS::User => {
					TokenInfo::User(Box::from_raw(Box::into_raw(buf) as *mut TOKEN_USER))
				},
				co::TOKEN_INFORMATION_CLASS::Groups => {
					TokenInfo::Groups(Box::from_raw(Box::into_raw(buf) as *mut TOKEN_GROUPS))
				},
				co::TOKEN_INFORMATION_CLASS::Privileges => TokenInfo::Privileges(Box::from_raw(
					Box::into_raw(buf) as *mut TOKEN_PRIVILEGES,
				)),
				co::TOKEN_INFORMATION_CLASS::Owner => {
					TokenInfo::Owner(Box::from_raw(Box::into_raw(buf) as *mut TOKEN_OWNER))
				},
				co::TOKEN_INFORMATION_CLASS::PrimaryGroup => TokenInfo::PrimaryGroup(
					Box::from_raw(Box::into_raw(buf) as *mut TOKEN_PRIMARY_GROUP),
				),
				co::TOKEN_INFORMATION_CLASS::DefaultDacl => TokenInfo::DefaultDacl(Box::from_raw(
					Box::into_raw(buf) as *mut TOKEN_DEFAULT_DACL,
				)),
				co::TOKEN_INFORMATION_CLASS::Source => {
					TokenInfo::Source(Box::from_raw(Box::into_raw(buf) as *mut TOKEN_SOURCE))
				},
				co::TOKEN_INFORMATION_CLASS::Type => {
					TokenInfo::Type(Box::from_raw(Box::into_raw(buf) as *mut co::TOKEN_TYPE))
				},
				co::TOKEN_INFORMATION_CLASS::ImpersonationLevel => TokenInfo::ImpersonationLevel(
					Box::from_raw(Box::into_raw(buf) as *mut co::SECURITY_IMPERSONATION),
				),
				co::TOKEN_INFORMATION_CLASS::Statistics => TokenInfo::Statistics(Box::from_raw(
					Box::into_raw(buf) as *mut TOKEN_STATISTICS,
				)),
				co::TOKEN_INFORMATION_CLASS::RestrictedSids => TokenInfo::RestrictedSids(
					Box::from_raw(Box::into_raw(buf) as *mut TOKEN_GROUPS),
				),
				co::TOKEN_INFORMATION_CLASS::SessionId => {
					TokenInfo::SessionId(Box::from_raw(Box::into_raw(buf) as *mut u32))
				},
				co::TOKEN_INFORMATION_CLASS::GroupsAndPrivileges => TokenInfo::GroupsAndPrivileges(
					Box::from_raw(Box::into_raw(buf) as *mut TOKEN_GROUPS_AND_PRIVILEGES),
				),
				co::TOKEN_INFORMATION_CLASS::SandBoxInert => {
					TokenInfo::SandBoxInert(Box::from_raw(Box::into_raw(buf) as *mut u32))
				},
				co::TOKEN_INFORMATION_CLASS::Origin => {
					TokenInfo::Origin(Box::from_raw(Box::into_raw(buf) as *mut TOKEN_ORIGIN))
				},
				co::TOKEN_INFORMATION_CLASS::ElevationType => TokenInfo::ElevationType(
					Box::from_raw(Box::into_raw(buf) as *mut co::TOKEN_ELEVATION_TYPE),
				),
				co::TOKEN_INFORMATION_CLASS::LinkedToken => TokenInfo::LinkedToken(Box::from_raw(
					Box::into_raw(buf) as *mut TOKEN_LINKED_TOKEN,
				)),
				co::TOKEN_INFORMATION_CLASS::Elevation => {
					TokenInfo::Elevation(Box::from_raw(Box::into_raw(buf) as *mut TOKEN_ELEVATION))
				},
				co::TOKEN_INFORMATION_CLASS::HasRestrictions => {
					TokenInfo::HasRestrictions(Box::from_raw(Box::into_raw(buf) as *mut u32))
				},
				co::TOKEN_INFORMATION_CLASS::AccessInformation => TokenInfo::AccessInformation(
					Box::from_raw(Box::into_raw(buf) as *mut TOKEN_ACCESS_INFORMATION),
				),
				co::TOKEN_INFORMATION_CLASS::VirtualizationAllowed => {
					TokenInfo::VirtualizationAllowed(Box::from_raw(Box::into_raw(buf) as *mut u32))
				},
				co::TOKEN_INFORMATION_CLASS::VirtualizationEnabled => {
					TokenInfo::VirtualizationEnabled(Box::from_raw(Box::into_raw(buf) as *mut u32))
				},
				co::TOKEN_INFORMATION_CLASS::IntegrityLevel => TokenInfo::IntegrityLevel(
					Box::from_raw(Box::into_raw(buf) as *mut TOKEN_MANDATORY_LABEL),
				),
				co::TOKEN_INFORMATION_CLASS::UIAccess => {
					TokenInfo::UIAccess(Box::from_raw(Box::into_raw(buf) as *mut u32))
				},
				co::TOKEN_INFORMATION_CLASS::MandatoryPolicy => TokenInfo::MandatoryPolicy(
					Box::from_raw(Box::into_raw(buf) as *mut TOKEN_MANDATORY_POLICY),
				),
				co::TOKEN_INFORMATION_CLASS::LogonSid => {
					TokenInfo::LogonSid(Box::from_raw(Box::into_raw(buf) as *mut TOKEN_GROUPS))
				},
				co::TOKEN_INFORMATION_CLASS::IsAppContainer => {
					TokenInfo::IsAppContainer(Box::from_raw(Box::into_raw(buf) as *mut u32))
				},
				co::TOKEN_INFORMATION_CLASS::Capabilities => {
					TokenInfo::Capabilities(Box::from_raw(Box::into_raw(buf) as *mut TOKEN_GROUPS))
				},
				co::TOKEN_INFORMATION_CLASS::AppContainerNumber => {
					TokenInfo::AppContainerNumber(Box::from_raw(Box::into_raw(buf) as *mut u32))
				},
				co::TOKEN_INFORMATION_CLASS::DeviceClaimAttributes => {
					TokenInfo::DeviceClaimAttributes(Box::from_raw(
						Box::into_raw(buf) as *mut CLAIM_SECURITY_ATTRIBUTES_INFORMATION
					))
				},
				co::TOKEN_INFORMATION_CLASS::DeviceGroups => {
					TokenInfo::DeviceGroups(Box::from_raw(Box::into_raw(buf) as *mut TOKEN_GROUPS))
				},
				co::TOKEN_INFORMATION_CLASS::RestrictedDeviceGroups => {
					TokenInfo::RestrictedDeviceGroups(Box::from_raw(
						Box::into_raw(buf) as *mut TOKEN_GROUPS
					))
				},
				_ => {
					panic!("co::TOKEN_INFORMATION_CLASS not implemented yet: {}", information_class)
				},
			})
		}
	}

	/// [`ImpersonateLoggedOnUser`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-impersonateloggedonuser)
	/// function.
	pub fn ImpersonateLoggedOnUser(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::ImpersonateLoggedOnUser(self.ptr()) })
	}

	/// [`IsTokenRestricted`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-istokenrestricted)
	/// function.
	#[must_use]
	pub fn IsTokenRestricted(&self) -> SysResult<bool> {
		match unsafe { ffi::IsTokenRestricted(self.ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(false), // actual false
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}
