use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::advapi::ffi;
use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::prelude::*;

handle_guard! { CloseServiceHandleGuard: HSC;
	ffi::CloseServiceHandle;
	/// RAII implementation for [`HSC`](crate::HSC) which automatically calls
	/// [`CloseServiceHandle`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-closeservicehandle)
	/// when the object goes out of scope.
}

handle_guard! { CloseServiceHandleSvcGuard: HSERVICE;
	ffi::CloseServiceHandle;
	/// RAII implementation for [`HSERVICE`](crate::HSERVICE) which
	/// automatically calls
	/// [`CloseServiceHandle`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-closeservicehandle)
	/// when the object goes out of scope.
}

handle_guard! { DeregisterEventSourceGuard: HEVENTLOG;
	ffi::DeregisterEventSource;
	/// RAII implementation for [`HEVENTLOG`](crate::HEVENTLOG) which
	/// automatically calls
	/// [`DeregisterEventSource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-deregistereventsource)
	/// when the object goes out of scope.
}

/// RAII implementation for [`SID`](crate::SID), returned by
/// [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid), which
/// automatically calls
/// [`FreeSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-freesid)
/// when the object goes out of scope.
pub struct FreeSidGuard {
	psid: *mut SID,
}

impl Drop for FreeSidGuard {
	fn drop(&mut self) {
		if !self.psid.is_null() {
			unsafe {
				ffi::FreeSid(self.psid as *mut _ as _); // ignore errors
			}
		}
	}
}

impl Deref for FreeSidGuard {
	type Target = SID;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.psid }
	}
}

impl std::fmt::Display for FreeSidGuard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.deref(), f) // delegate to the underlying SID
	}
}

impl FreeSidGuard {
	/// Constructs the guard by taking ownership of the pointer.
	///
	/// # Safety
	///
	/// Be sure the pointer must be freed with
	/// [`FreeSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-freesid).
	#[must_use]
	pub const unsafe fn new(psid: *mut SID) -> Self {
		Self { psid }
	}

	/// Ejects the underlying pointer, leaving a null pointer in its place.
	///
	/// Since the internal pointer will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> *mut SID {
		std::mem::replace(&mut self.psid, std::ptr::null_mut())
	}
}

/// RAII implementation for [`HKEY`](crate::HKEY) which automatically calls
/// [`RegCloseKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey)
/// when the object goes out of scope.
pub struct RegCloseKeyGuard {
	hkey: HKEY,
}

impl Drop for RegCloseKeyGuard {
	fn drop(&mut self) {
		if let Some(h) = self.hkey.as_opt() {
			// Don't call on predefined keys, these belong to the system.
			if !self.is_predef_key() {
				unsafe {
					ffi::RegCloseKey(h.ptr()); // ignore errors
				}
			}
		}
	}
}

impl Deref for RegCloseKeyGuard {
	type Target = HKEY;

	fn deref(&self) -> &Self::Target {
		&self.hkey
	}
}

impl DerefMut for RegCloseKeyGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.hkey
	}
}

impl RegCloseKeyGuard {
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`RegCloseKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(hkey: HKEY) -> Self {
		Self { hkey }
	}

	/// Ejects the underlying handle, leaving
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsibility to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> HKEY {
		std::mem::replace(&mut self.hkey, HKEY::INVALID)
	}
}

/// RAII implementation for [`SID`](crate::SID), returned by
/// [`ConvertStringSidToSid`](crate::ConvertStringSidToSid), which automatically
/// calls
/// [`LocalFree`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
/// when the object goes out of scope.
pub struct LocalFreeSidGuard {
	pmem: LocalFreeGuard,
}

impl Deref for LocalFreeSidGuard {
	type Target = SID;

	fn deref(&self) -> &Self::Target {
		unsafe { &*(self.pmem.ptr() as *mut _) }
	}
}

impl std::fmt::Display for LocalFreeSidGuard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.deref(), f) // delegate to the underlying SID
	}
}

impl LocalFreeSidGuard {
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure the pointer is an [`HLOCAL`](crate::HLOCAL) handle pointing to a
	/// [`SID`](crate::SID) memory block.
	#[must_use]
	pub const unsafe fn new(pmem: HLOCAL) -> Self {
		Self { pmem: LocalFreeGuard::new(pmem) }
	}
}

/// RAII implementation for [`SID`](crate::SID), returned by
/// [`CopySid`](crate::CopySid),
/// [`CreateWellKnownSid`](crate::CreateWellKnownSid),
/// [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid) and
/// [`LookupAccountName`](crate::LookupAccountName), which automatically frees
/// the underlying memory block when the object goes out of scope.
pub struct SidGuard {
	ptr: GlobalFreeGuard,
}

impl Deref for SidGuard {
	type Target = SID;

	fn deref(&self) -> &Self::Target {
		unsafe { &*(self.ptr.ptr() as *const _) }
	}
}

impl std::fmt::Display for SidGuard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.deref(), f) // delegate to the underlying SID
	}
}

impl SidGuard {
	/// Constructs a new guard by taking ownership of the data.
	///
	/// # Safety
	///
	/// Be sure the data is an allocated [`SID`](crate::SID) structure.
	#[must_use]
	pub const unsafe fn new(ptr: GlobalFreeGuard) -> Self {
		Self { ptr }
	}
}

/// RAII implementation for [`TOKEN_GROUPS`](crate::TOKEN_GROUPS) which manages
/// the allocated memory.
pub struct TokenGroupsGuard<'a> {
	ptr: GlobalFreeGuard,
	_groups: PhantomData<&'a ()>,
}

impl<'a> Deref for TokenGroupsGuard<'a> {
	type Target = TOKEN_GROUPS<'a>;

	fn deref(&self) -> &Self::Target {
		unsafe { &*(self.ptr.ptr() as *const _) }
	}
}

impl<'a> DerefMut for TokenGroupsGuard<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *(self.ptr.ptr() as *mut _) }
	}
}

impl<'a> TokenGroupsGuard<'a> {
	#[must_use]
	pub(in crate::advapi) fn new(groups: &'a [SID_AND_ATTRIBUTES<'a>]) -> SysResult<Self> {
		let sz = std::mem::size_of::<TOKEN_GROUPS>() // size in bytes of the allocated struct
			- std::mem::size_of::<SID_AND_ATTRIBUTES>()
			+ (groups.len() * std::mem::size_of::<SID_AND_ATTRIBUTES>());
		let mut new_self = Self {
			ptr: HGLOBAL::GlobalAlloc(co::GMEM::FIXED | co::GMEM::ZEROINIT, sz)?,
			_groups: PhantomData,
		};
		new_self.GroupCount = groups.len() as _;
		groups
			.iter()
			.zip(new_self.Groups_mut())
			.for_each(|(src, dest)| *dest = src.clone()); // copy all SID_AND_ATTRIBUTES into struct room
		Ok(new_self)
	}
}

/// RAII implementation for [`TOKEN_PRIVILEGES`](crate::TOKEN_PRIVILEGES) which
/// manages the allocated memory.
pub struct TokenPrivilegesGuard {
	ptr: GlobalFreeGuard,
}

impl Deref for TokenPrivilegesGuard {
	type Target = TOKEN_PRIVILEGES;

	fn deref(&self) -> &Self::Target {
		unsafe { &*(self.ptr.ptr() as *const _) }
	}
}

impl DerefMut for TokenPrivilegesGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *(self.ptr.ptr() as *mut _) }
	}
}

impl TokenPrivilegesGuard {
	#[must_use]
	pub(in crate::advapi) fn new(privileges: &[LUID_AND_ATTRIBUTES]) -> SysResult<Self> {
		let sz = std::mem::size_of::<TOKEN_PRIVILEGES>() // size in bytes of the allocated struct
			- std::mem::size_of::<LUID_AND_ATTRIBUTES>()
			+ (privileges.len() * std::mem::size_of::<LUID_AND_ATTRIBUTES>());
		let mut new_self = Self {
			ptr: HGLOBAL::GlobalAlloc(co::GMEM::FIXED | co::GMEM::ZEROINIT, sz)?,
		};
		new_self.PrivilegeCount = privileges.len() as _;
		privileges
			.iter()
			.zip(new_self.Privileges_mut())
			.for_each(|(src, dest)| *dest = *src); // copy all LUID_AND_ATTRIBUTES into struct room
		Ok(new_self)
	}
}
