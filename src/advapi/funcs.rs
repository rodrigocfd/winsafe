#![allow(non_snake_case)]

use crate::advapi::{ffi, privs::*};
use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;

/// [`AllocateAndInitializeSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-allocateandinitializesid)
/// function.
///
/// # Panics
///
/// Panics if `sub_authorities` has more than 8 elements.
///
/// # Examples
///
/// Create a well-known SID for the Everyone group:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let sid_everyone = w::AllocateAndInitializeSid(
///     &w::SID_IDENTIFIER_AUTHORITY::WORLD,
///     &[
///         co::RID::SECURITY_WORLD,
///     ],
/// )?;
///
/// // FreeSid() automatically called
/// # w::SysResult::Ok(())
/// ```
///
/// Create a SID for the BUILTIN\Administrators group:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let sid_builtin_administrators = w::AllocateAndInitializeSid(
///     &w::SID_IDENTIFIER_AUTHORITY::NT,
///     &[
///         co::RID::SECURITY_BUILTIN_DOMAIN,
///         co::RID::DOMAIN_ALIAS_ADMINS,
///     ],
/// )?;
///
/// // FreeSid() automatically called
/// # w::SysResult::Ok(())
/// ```
///
/// # Related functions
///
/// * [`ConvertSidToStringSid`](crate::ConvertSidToStringSid)
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid)
/// * [`CopySid`](crate::CopySid)
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid)
/// * [`EqualDomainSid`](crate::EqualDomainSid)
/// * [`EqualPrefixSid`](crate::EqualPrefixSid)
/// * [`EqualSid`](crate::EqualSid)
/// * [`GetLengthSid`](crate::GetLengthSid)
/// * [`GetSidLengthRequired`](crate::GetSidLengthRequired)
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid)
/// * [`IsValidSid`](crate::IsValidSid)
/// * [`IsWellKnownSid`](crate::IsWellKnownSid)
/// * [`LookupAccountName`](crate::LookupAccountName)
/// * [`LookupAccountSid`](crate::LookupAccountSid)
#[must_use]
pub fn AllocateAndInitializeSid(
	identifier_authority: &SID_IDENTIFIER_AUTHORITY,
	sub_authorities: &[co::RID],
) -> SysResult<FreeSidGuard> {
	if sub_authorities.len() > 8 {
		panic!("You must specify at most 8 sub authorities.");
	}

	let mut psid = std::ptr::null_mut() as *mut SID;
	unsafe {
		bool_to_sysresult(ffi::AllocateAndInitializeSid(
			pcvoid(identifier_authority),
			sub_authorities.len() as _,
			if sub_authorities.len() >= 1 { sub_authorities[0].raw() } else { 0 },
			if sub_authorities.len() >= 2 { sub_authorities[1].raw() } else { 0 },
			if sub_authorities.len() >= 3 { sub_authorities[2].raw() } else { 0 },
			if sub_authorities.len() >= 4 { sub_authorities[3].raw() } else { 0 },
			if sub_authorities.len() >= 5 { sub_authorities[4].raw() } else { 0 },
			if sub_authorities.len() >= 6 { sub_authorities[5].raw() } else { 0 },
			if sub_authorities.len() >= 7 { sub_authorities[6].raw() } else { 0 },
			if sub_authorities.len() >= 8 { sub_authorities[7].raw() } else { 0 },
			pvoid(&mut psid),
		))
		.map(|_| FreeSidGuard::new(psid))
	}
}

/// [`ConvertSidToStringSid`](https://learn.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertsidtostringsidw)
/// function.
///
/// You don't need to call this function directly, because [`SID`](crate::SID)
/// implements [`Display`](std::fmt::Display) and
/// [`ToString`](std::string::ToString) traits, which call it.
///
/// # Related functions
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid)
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid)
/// * [`CopySid`](crate::CopySid)
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid)
/// * [`EqualDomainSid`](crate::EqualDomainSid)
/// * [`EqualPrefixSid`](crate::EqualPrefixSid)
/// * [`EqualSid`](crate::EqualSid)
/// * [`GetLengthSid`](crate::GetLengthSid)
/// * [`GetSidLengthRequired`](crate::GetSidLengthRequired)
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid)
/// * [`IsValidSid`](crate::IsValidSid)
/// * [`IsWellKnownSid`](crate::IsWellKnownSid)
/// * [`LookupAccountName`](crate::LookupAccountName)
/// * [`LookupAccountSid`](crate::LookupAccountSid)
#[must_use]
pub fn ConvertSidToStringSid(sid: &SID) -> SysResult<String> {
	let mut pstr = std::ptr::null_mut() as *mut u16;
	bool_to_sysresult(unsafe { ffi::ConvertSidToStringSidW(pcvoid(sid), &mut pstr) })?;
	let name = unsafe { WString::from_wchars_nullt(pstr) }.to_string();
	let _ = unsafe { LocalFreeGuard::new(HLOCAL::from_ptr(pstr as _)) }; // free returned pointer
	Ok(name)
}

/// [`ConvertStringSidToSid`](https://learn.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertstringsidtosidw)
/// function.
///
/// # Related functions
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid)
/// * [`ConvertSidToStringSid`](crate::ConvertSidToStringSid)
/// * [`CopySid`](crate::CopySid)
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid)
/// * [`EqualDomainSid`](crate::EqualDomainSid)
/// * [`EqualPrefixSid`](crate::EqualPrefixSid)
/// * [`EqualSid`](crate::EqualSid)
/// * [`GetLengthSid`](crate::GetLengthSid)
/// * [`GetSidLengthRequired`](crate::GetSidLengthRequired)
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid)
/// * [`IsValidSid`](crate::IsValidSid)
/// * [`IsWellKnownSid`](crate::IsWellKnownSid)
/// * [`LookupAccountName`](crate::LookupAccountName)
/// * [`LookupAccountSid`](crate::LookupAccountSid)
#[must_use]
pub fn ConvertStringSidToSid(str_sid: &str) -> SysResult<LocalFreeSidGuard> {
	let mut pbuf = std::ptr::null_mut() as *mut u8;
	unsafe {
		bool_to_sysresult(ffi::ConvertStringSidToSidW(
			WString::from_str(str_sid).as_ptr(),
			&mut pbuf,
		))
		.map(|_| LocalFreeSidGuard::new(HLOCAL::from_ptr(pbuf as _)))
	}
}

/// [`CopySid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-copysid)
/// function.
///
/// # Related functions
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid)
/// * [`ConvertSidToStringSid`](crate::ConvertSidToStringSid)
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid)
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid)
/// * [`EqualDomainSid`](crate::EqualDomainSid)
/// * [`EqualPrefixSid`](crate::EqualPrefixSid)
/// * [`EqualSid`](crate::EqualSid)
/// * [`GetLengthSid`](crate::GetLengthSid)
/// * [`GetSidLengthRequired`](crate::GetSidLengthRequired)
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid)
/// * [`IsValidSid`](crate::IsValidSid)
/// * [`IsWellKnownSid`](crate::IsWellKnownSid)
/// * [`LookupAccountName`](crate::LookupAccountName)
/// * [`LookupAccountSid`](crate::LookupAccountSid)
#[must_use]
pub fn CopySid(src: &SID) -> SysResult<SidGuard> {
	let sid_sz = GetLengthSid(&src);
	let sid_buf = HGLOBAL::GlobalAlloc(co::GMEM::FIXED | co::GMEM::ZEROINIT, sid_sz as _)?;

	unsafe {
		bool_to_sysresult(ffi::CopySid(sid_sz, sid_buf.ptr(), pcvoid(src)))
			.map(|_| SidGuard::new(sid_buf))
	}
}

/// [`CreateWellKnownSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createwellknownsid)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let sid = w::CreateWellKnownSid(
///     co::WELL_KNOWN_SID_TYPE::LocalSystem,
///     None,
/// )?;
/// # w::SysResult::Ok(())
/// ```
///
/// # Related functions
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid)
/// * [`ConvertSidToStringSid`](crate::ConvertSidToStringSid)
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid)
/// * [`CopySid`](crate::CopySid)
/// * [`EqualDomainSid`](crate::EqualDomainSid)
/// * [`EqualPrefixSid`](crate::EqualPrefixSid)
/// * [`EqualSid`](crate::EqualSid)
/// * [`GetLengthSid`](crate::GetLengthSid)
/// * [`GetSidLengthRequired`](crate::GetSidLengthRequired)
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid)
/// * [`IsValidSid`](crate::IsValidSid)
/// * [`IsWellKnownSid`](crate::IsWellKnownSid)
/// * [`LookupAccountName`](crate::LookupAccountName)
/// * [`LookupAccountSid`](crate::LookupAccountSid)
#[must_use]
pub fn CreateWellKnownSid(
	well_known_sid: co::WELL_KNOWN_SID_TYPE,
	domain_sid: Option<&SID>,
) -> SysResult<SidGuard> {
	let mut sid_sz = 0u32;

	unsafe {
		ffi::CreateWellKnownSid(
			well_known_sid.raw(),
			pcvoid_or_null(domain_sid),
			std::ptr::null_mut(),
			&mut sid_sz, // retrieve needed buffer sizes
		);
	}
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let sid_buf = HGLOBAL::GlobalAlloc(co::GMEM::FIXED | co::GMEM::ZEROINIT, sid_sz as _)?;
	unsafe {
		bool_to_sysresult(ffi::CreateWellKnownSid(
			well_known_sid.raw(),
			pcvoid_or_null(domain_sid),
			sid_buf.ptr(),
			&mut sid_sz,
		))
		.map(|_| SidGuard::new(sid_buf))
	}
}

/// [`DecryptFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-decryptfilew)
/// function.
///
/// # Related functions
///
/// * [`EncryptFile`](crate::EncryptFile)
/// * [`EncryptionDisable`](crate::EncryptionDisable)
pub fn DecryptFile(file_name: &str) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::DecryptFileW(WString::from_str(file_name).as_ptr(), 0) })
}

/// [`EncryptFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-encryptfilew)
/// function.
///
/// # Related functions
///
/// * [`DecryptFile`](crate::DecryptFile)
/// * [`EncryptionDisable`](crate::EncryptionDisable)
pub fn EncryptFile(file_name: &str) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::EncryptFileW(WString::from_str(file_name).as_ptr()) })
}

/// [`EncryptionDisable`](https://learn.microsoft.com/en-us/windows/win32/api/winefs/nf-winefs-encryptiondisable)
/// function.
///
/// # Related functions
///
/// * [`EncryptFile`](crate::EncryptFile)
/// * [`DecryptFile`](crate::DecryptFile)
pub fn EncryptionDisable(dir_path: &str, disable: bool) -> SysResult<()> {
	bool_to_sysresult(unsafe {
		ffi::EncryptionDisable(WString::from_str(dir_path).as_ptr(), disable as _)
	})
}

/// [`EqualDomainSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-equaldomainsid)
/// function.
///
/// # Related functions
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid)
/// * [`ConvertSidToStringSid`](crate::ConvertSidToStringSid)
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid)
/// * [`CopySid`](crate::CopySid)
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid)
/// * [`EqualPrefixSid`](crate::EqualPrefixSid)
/// * [`EqualSid`](crate::EqualSid)
/// * [`GetLengthSid`](crate::GetLengthSid)
/// * [`GetSidLengthRequired`](crate::GetSidLengthRequired)
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid)
/// * [`IsValidSid`](crate::IsValidSid)
/// * [`IsWellKnownSid`](crate::IsWellKnownSid)
/// * [`LookupAccountName`](crate::LookupAccountName)
/// * [`LookupAccountSid`](crate::LookupAccountSid)
#[must_use]
pub fn EqualDomainSid(sid1: &SID, sid2: &SID) -> SysResult<bool> {
	let mut is_equal = 0;
	bool_to_sysresult(unsafe { ffi::EqualDomainSid(pcvoid(sid1), pcvoid(sid2), &mut is_equal) })
		.map(|_| is_equal != 0)
}

/// [`EqualPrefixSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-equalprefixsid)
/// function.
///
/// # Related functions
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid)
/// * [`ConvertSidToStringSid`](crate::ConvertSidToStringSid)
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid)
/// * [`CopySid`](crate::CopySid)
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid)
/// * [`EqualDomainSid`](crate::EqualDomainSid)
/// * [`EqualSid`](crate::EqualSid)
/// * [`GetLengthSid`](crate::GetLengthSid)
/// * [`GetSidLengthRequired`](crate::GetSidLengthRequired)
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid)
/// * [`IsValidSid`](crate::IsValidSid)
/// * [`IsWellKnownSid`](crate::IsWellKnownSid)
/// * [`LookupAccountName`](crate::LookupAccountName)
/// * [`LookupAccountSid`](crate::LookupAccountSid)
#[must_use]
pub fn EqualPrefixSid(sid1: &SID, sid2: &SID) -> SysResult<bool> {
	match unsafe { ffi::EqualPrefixSid(pcvoid(sid1), pcvoid(sid2)) } {
		0 => match GetLastError() {
			co::ERROR::SUCCESS => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`EqualSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-equalsid)
/// function.
///
/// You don't need to call this function directly, because [`SID`](crate::SID)
/// implements [`PartialEq`](std::cmp::PartialEq) and [`Eq`](std::cmp::Eq)
/// traits, which call it.
///
/// # Related functions
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid)
/// * [`ConvertSidToStringSid`](crate::ConvertSidToStringSid)
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid)
/// * [`CopySid`](crate::CopySid)
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid)
/// * [`EqualDomainSid`](crate::EqualDomainSid)
/// * [`EqualPrefixSid`](crate::EqualPrefixSid)
/// * [`GetLengthSid`](crate::GetLengthSid)
/// * [`GetSidLengthRequired`](crate::GetSidLengthRequired)
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid)
/// * [`IsValidSid`](crate::IsValidSid)
/// * [`IsWellKnownSid`](crate::IsWellKnownSid)
/// * [`LookupAccountName`](crate::LookupAccountName)
/// * [`LookupAccountSid`](crate::LookupAccountSid)
#[must_use]
pub fn EqualSid(sid1: &SID, sid2: &SID) -> SysResult<bool> {
	match unsafe { ffi::EqualSid(pcvoid(sid1), pcvoid(sid2)) } {
		0 => match GetLastError() {
			co::ERROR::SUCCESS => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`GetLengthSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getlengthsid)
/// function.
///
/// # Related functions
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid)
/// * [`ConvertSidToStringSid`](crate::ConvertSidToStringSid)
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid)
/// * [`CopySid`](crate::CopySid)
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid)
/// * [`EqualDomainSid`](crate::EqualDomainSid)
/// * [`EqualPrefixSid`](crate::EqualPrefixSid)
/// * [`EqualSid`](crate::EqualSid)
/// * [`GetSidLengthRequired`](crate::GetSidLengthRequired)
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid)
/// * [`IsValidSid`](crate::IsValidSid)
/// * [`IsWellKnownSid`](crate::IsWellKnownSid)
/// * [`LookupAccountName`](crate::LookupAccountName)
/// * [`LookupAccountSid`](crate::LookupAccountSid)
#[must_use]
pub fn GetLengthSid(sid: &SID) -> u32 {
	unsafe { ffi::GetLengthSid(pcvoid(sid)) }
}

/// [`GetSidLengthRequired`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getsidlengthrequired)
/// function.
///
/// # Related functions
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid)
/// * [`ConvertSidToStringSid`](crate::ConvertSidToStringSid)
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid)
/// * [`CopySid`](crate::CopySid)
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid)
/// * [`EqualDomainSid`](crate::EqualDomainSid)
/// * [`EqualPrefixSid`](crate::EqualPrefixSid)
/// * [`EqualSid`](crate::EqualSid)
/// * [`GetLengthSid`](crate::GetLengthSid)
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid)
/// * [`IsValidSid`](crate::IsValidSid)
/// * [`IsWellKnownSid`](crate::IsWellKnownSid)
/// * [`LookupAccountName`](crate::LookupAccountName)
/// * [`LookupAccountSid`](crate::LookupAccountSid)
#[must_use]
pub fn GetSidLengthRequired(sub_authority_count: u8) -> u32 {
	unsafe { ffi::GetSidLengthRequired(sub_authority_count) }
}

/// [`GetUserName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getusernamew)
/// function.
#[must_use]
pub fn GetUserName() -> SysResult<String> {
	let mut name_sz = 0u32;
	unsafe {
		ffi::GetUserNameW(std::ptr::null_mut(), &mut name_sz);
	}
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut name_buf = WString::new_alloc_buf(name_sz as _);
	bool_to_sysresult(unsafe { ffi::GetUserNameW(name_buf.as_mut_ptr(), &mut name_sz) })
		.map(|_| name_buf.to_string())
}

/// [`GetWindowsAccountDomainSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getwindowsaccountdomainsid)
/// function.
///
/// # Related functions
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid)
/// * [`ConvertSidToStringSid`](crate::ConvertSidToStringSid)
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid)
/// * [`CopySid`](crate::CopySid)
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid)
/// * [`EqualDomainSid`](crate::EqualDomainSid)
/// * [`EqualPrefixSid`](crate::EqualPrefixSid)
/// * [`EqualSid`](crate::EqualSid)
/// * [`GetLengthSid`](crate::GetLengthSid)
/// * [`GetSidLengthRequired`](crate::GetSidLengthRequired)
/// * [`IsValidSid`](crate::IsValidSid)
/// * [`IsWellKnownSid`](crate::IsWellKnownSid)
/// * [`LookupAccountName`](crate::LookupAccountName)
/// * [`LookupAccountSid`](crate::LookupAccountSid)
#[must_use]
pub fn GetWindowsAccountDomainSid(sid: &SID) -> SysResult<SidGuard> {
	let mut ad_sid_sz = 0u32;

	unsafe { ffi::GetWindowsAccountDomainSid(pcvoid(sid), std::ptr::null_mut(), &mut ad_sid_sz) };
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let ad_sid_buf = HGLOBAL::GlobalAlloc(co::GMEM::FIXED | co::GMEM::ZEROINIT, ad_sid_sz as _)?;
	unsafe {
		bool_to_sysresult(ffi::GetWindowsAccountDomainSid(
			pcvoid(sid),
			ad_sid_buf.ptr(),
			&mut ad_sid_sz,
		))
		.map(|_| SidGuard::new(ad_sid_buf))
	}
}

/// [`InitializeSecurityDescriptor`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-initializesecuritydescriptor)
/// function.
///
/// You don't need to call this function directly, because
/// [`SECURITY_DESCRIPTOR`](crate::SECURITY_DESCRIPTOR) implements the
/// [`Default`](std::default::Default) trait, which calls it.
#[must_use]
pub fn InitializeSecurityDescriptor() -> SysResult<SECURITY_DESCRIPTOR> {
	let mut sd = unsafe { std::mem::zeroed::<SECURITY_DESCRIPTOR>() };
	bool_to_sysresult(unsafe {
		ffi::InitializeSecurityDescriptor(pvoid(&mut sd), SECURITY_DESCRIPTOR_REVISION)
	})
	.map(|_| sd)
}

/// [`InitiateSystemShutdown`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-initiatesystemshutdownw)
/// function.
pub fn InitiateSystemShutdown(
	machine_name: Option<&str>,
	message: Option<&str>,
	timeout: u32,
	force_apps_closed: bool,
	reboot_after_shutdown: bool,
) -> SysResult<()> {
	bool_to_sysresult(unsafe {
		ffi::InitiateSystemShutdownW(
			WString::from_opt_str(machine_name).as_ptr(),
			WString::from_opt_str(message).as_ptr(),
			timeout,
			force_apps_closed as _,
			reboot_after_shutdown as _,
		)
	})
}

/// [`InitiateSystemShutdownEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-initiatesystemshutdownexw)
/// function.
pub fn InitiateSystemShutdownEx(
	machine_name: Option<&str>,
	message: Option<&str>,
	timeout: u32,
	force_apps_closed: bool,
	reboot_after_shutdown: bool,
	reason: Option<co::SHTDN_REASON>,
) -> SysResult<()> {
	bool_to_sysresult(unsafe {
		ffi::InitiateSystemShutdownExW(
			WString::from_opt_str(machine_name).as_ptr(),
			WString::from_opt_str(message).as_ptr(),
			timeout,
			force_apps_closed as _,
			reboot_after_shutdown as _,
			reason.unwrap_or_default().raw(),
		)
	})
}

/// [`IsValidSecurityDescriptor`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-isvalidsecuritydescriptor)
/// function.
#[must_use]
pub fn IsValidSecurityDescriptor(sd: &SECURITY_DESCRIPTOR) -> bool {
	unsafe { ffi::IsValidSecurityDescriptor(pcvoid(sd)) != 0 }
}

/// [`IsValidSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-isvalidsid)
/// function.
///
/// # Related functions
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid)
/// * [`ConvertSidToStringSid`](crate::ConvertSidToStringSid)
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid)
/// * [`CopySid`](crate::CopySid)
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid)
/// * [`EqualDomainSid`](crate::EqualDomainSid)
/// * [`EqualPrefixSid`](crate::EqualPrefixSid)
/// * [`EqualSid`](crate::EqualSid)
/// * [`GetLengthSid`](crate::GetLengthSid)
/// * [`GetSidLengthRequired`](crate::GetSidLengthRequired)
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid)
/// * [`IsWellKnownSid`](crate::IsWellKnownSid)
/// * [`LookupAccountName`](crate::LookupAccountName)
/// * [`LookupAccountSid`](crate::LookupAccountSid)
#[must_use]
pub fn IsValidSid(sid: &SID) -> SysResult<bool> {
	match unsafe { ffi::IsValidSid(pcvoid(sid)) } {
		0 => match GetLastError() {
			co::ERROR::SUCCESS => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`IsWellKnownSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-iswellknownsid)
/// function.
///
/// # Related functions
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid)
/// * [`ConvertSidToStringSid`](crate::ConvertSidToStringSid)
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid)
/// * [`CopySid`](crate::CopySid)
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid)
/// * [`EqualDomainSid`](crate::EqualDomainSid)
/// * [`EqualPrefixSid`](crate::EqualPrefixSid)
/// * [`EqualSid`](crate::EqualSid)
/// * [`GetLengthSid`](crate::GetLengthSid)
/// * [`GetSidLengthRequired`](crate::GetSidLengthRequired)
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid)
/// * [`IsValidSid`](crate::IsValidSid)
/// * [`LookupAccountName`](crate::LookupAccountName)
/// * [`LookupAccountSid`](crate::LookupAccountSid)
#[must_use]
pub fn IsWellKnownSid(sid: &SID, well_known_sid: co::WELL_KNOWN_SID_TYPE) -> bool {
	unsafe { ffi::IsWellKnownSid(pcvoid(sid), well_known_sid.raw()) != 0 }
}

/// [`LookupAccountName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupaccountnamew)
/// function.
///
/// Returns account's domain name, `SID` and type, respectively.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let user_name = w::GetUserName()?;
/// let (domain_name, sid, kind) = w::LookupAccountName(None, &user_name)?;
/// # w::SysResult::Ok(())
/// ```
///
/// # Related functions
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid)
/// * [`ConvertSidToStringSid`](crate::ConvertSidToStringSid)
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid)
/// * [`CopySid`](crate::CopySid)
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid)
/// * [`EqualDomainSid`](crate::EqualDomainSid)
/// * [`EqualPrefixSid`](crate::EqualPrefixSid)
/// * [`EqualSid`](crate::EqualSid)
/// * [`GetLengthSid`](crate::GetLengthSid)
/// * [`GetSidLengthRequired`](crate::GetSidLengthRequired)
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid)
/// * [`IsValidSid`](crate::IsValidSid)
/// * [`IsWellKnownSid`](crate::IsWellKnownSid)
/// * [`LookupAccountSid`](crate::LookupAccountSid)
#[must_use]
pub fn LookupAccountName(
	system_name: Option<&str>,
	account_name: &str,
) -> SysResult<(String, SidGuard, co::SID_NAME_USE)> {
	let mut sid_sz = 0u32;
	let mut domain_sz = 0u32;
	let mut sid_name_use = co::SID_NAME_USE::default();

	unsafe {
		ffi::LookupAccountNameW(
			WString::from_opt_str(system_name).as_ptr(),
			WString::from_str(account_name).as_ptr(),
			std::ptr::null_mut(),
			&mut sid_sz, // retrieve needed buffer sizes
			std::ptr::null_mut(),
			&mut domain_sz,
			sid_name_use.as_mut(),
		);
	}
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let sid_buf = HGLOBAL::GlobalAlloc(co::GMEM::FIXED | co::GMEM::ZEROINIT, sid_sz as _)?;
	let mut domain_buf = WString::new_alloc_buf(domain_sz as _);

	unsafe {
		bool_to_sysresult(ffi::LookupAccountNameW(
			WString::from_opt_str(system_name).as_ptr(),
			WString::from_str(account_name).as_ptr(),
			sid_buf.ptr(),
			&mut sid_sz,
			domain_buf.as_mut_ptr(),
			&mut domain_sz,
			sid_name_use.as_mut(),
		))
		.map(|_| (domain_buf.to_string(), SidGuard::new(sid_buf), sid_name_use))
	}
}

/// [`LookupAccountSid`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupaccountsidw)
/// function.
///
/// Returns account name, domain name and type, respectively.
///
/// # Related functions
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid)
/// * [`ConvertSidToStringSid`](crate::ConvertSidToStringSid)
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid)
/// * [`CopySid`](crate::CopySid)
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid)
/// * [`EqualDomainSid`](crate::EqualDomainSid)
/// * [`EqualPrefixSid`](crate::EqualPrefixSid)
/// * [`EqualSid`](crate::EqualSid)
/// * [`GetLengthSid`](crate::GetLengthSid)
/// * [`GetSidLengthRequired`](crate::GetSidLengthRequired)
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid)
/// * [`IsValidSid`](crate::IsValidSid)
/// * [`IsWellKnownSid`](crate::IsWellKnownSid)
/// * [`LookupAccountName`](crate::LookupAccountName)
#[must_use]
pub fn LookupAccountSid(
	system_name: Option<&str>,
	sid: &SID,
) -> SysResult<(String, String, co::SID_NAME_USE)> {
	let mut account_sz = 0u32;
	let mut domain_sz = 0u32;
	let mut sid_name_use = co::SID_NAME_USE::default();

	unsafe {
		ffi::LookupAccountSidW(
			WString::from_opt_str(system_name).as_ptr(),
			pcvoid(sid),
			std::ptr::null_mut(),
			&mut account_sz, // retrieve needed buffer sizes
			std::ptr::null_mut(),
			&mut domain_sz,
			sid_name_use.as_mut(),
		);
	}
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut account_buf = WString::new_alloc_buf(account_sz as _);
	let mut domain_buf = WString::new_alloc_buf(domain_sz as _);

	bool_to_sysresult(unsafe {
		ffi::LookupAccountSidW(
			WString::from_opt_str(system_name).as_ptr(),
			pcvoid(sid),
			account_buf.as_mut_ptr(),
			&mut account_sz,
			domain_buf.as_mut_ptr(),
			&mut domain_sz,
			sid_name_use.as_mut(),
		)
	})
	.map(|_| (account_buf.to_string(), domain_buf.to_string(), sid_name_use))
}

/// [`LookupPrivilegeName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegenamew)
/// function.
#[must_use]
pub fn LookupPrivilegeName(system_name: Option<&str>, luid: LUID) -> SysResult<co::SE_PRIV> {
	let mut cch_name = 0u32;

	bool_to_sysresult(unsafe {
		ffi::LookupPrivilegeNameW(
			WString::from_opt_str(system_name).as_ptr(),
			pcvoid(&luid),
			std::ptr::null_mut(),
			&mut cch_name,
		)
	})?;

	let mut buf = WString::new_alloc_buf(cch_name as _);

	bool_to_sysresult(unsafe {
		ffi::LookupPrivilegeNameW(
			WString::from_opt_str(system_name).as_ptr(),
			pcvoid(&luid),
			buf.as_mut_ptr(),
			&mut cch_name,
		)
	})
	.map(|_| co::SE_PRIV::try_from(buf.to_string().as_str()))?
}

/// [`LookupPrivilegeValue`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew)
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
/// # w::SysResult::Ok(())
/// ```
#[must_use]
pub fn LookupPrivilegeValue(system_name: Option<&str>, name: co::SE_PRIV) -> SysResult<LUID> {
	let mut luid = LUID::from_parts(0, 0);
	bool_to_sysresult(unsafe {
		ffi::LookupPrivilegeValueW(
			WString::from_opt_str(system_name).as_ptr(),
			WString::from(name).as_ptr(),
			pvoid(&mut luid),
		)
	})
	.map(|_| luid)
}

/// [`RegDisablePredefinedCache`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdisablepredefinedcache)
/// function.
pub fn RegDisablePredefinedCache() -> SysResult<()> {
	error_to_sysresult(unsafe { ffi::RegDisablePredefinedCache() })
}

/// [`RegDisablePredefinedCacheEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdisablepredefinedcacheex)
/// function.
pub fn RegDisablePredefinedCacheEx() -> SysResult<()> {
	error_to_sysresult(unsafe { ffi::RegDisablePredefinedCacheEx() })
}
