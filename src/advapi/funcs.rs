#![allow(non_snake_case)]

use crate::{advapi, co};
use crate::advapi::decl::{SID, SID_wrap};
use crate::advapi::privs::SECURITY_DESCRIPTOR_REVISION;
use crate::kernel::decl::{
	GetLastError, HLOCAL, SECURITY_DESCRIPTOR, SysResult, WString,
};
use crate::kernel::ffi_types::BOOL;
use crate::kernel::privs::bool_to_sysresult;
use crate::prelude::kernel_Hlocal;

/// [`ConvertSidToStringSid`](https://learn.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertsidtostringsidw)
/// function.
/// 
/// You don't need to call this function directly, because [`SID`](crate::SID)
/// implements [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
/// and [`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html)
/// traits, which call it.
#[must_use]
pub fn ConvertSidToStringSid(sid: &SID) -> SysResult<String> {
	let mut pstr = std::ptr::null_mut() as *mut u16;
	bool_to_sysresult(
		unsafe {
			advapi::ffi::ConvertSidToStringSidW(sid as *const _ as _, &mut pstr)
		},
	)?;	
	let name = WString::from_wchars_nullt(pstr).to_string();
	HLOCAL(pstr as _).LocalFree()?;
	Ok(name)
}

/// [`ConvertStringSidToSid`](https://learn.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertstringsidtosidw)
/// function.
#[must_use]
pub fn ConvertStringSidToSid(str_sid: &str) -> SysResult<SID_wrap> {
	let mut pbuf = std::ptr::null_mut() as *mut u8;
	bool_to_sysresult(
		unsafe {
			advapi::ffi::ConvertStringSidToSidW(
				WString::from_str(str_sid).as_ptr(),
				&mut pbuf,
			)
		},
	)?;
	let pbuf_sid = unsafe { std::mem::transmute::<_, &SID>(pbuf) };
	let pbuf_slice = unsafe { std::slice::from_raw_parts(pbuf, GetLengthSid(pbuf_sid) as _) };
	let raw_sid_copied = Vec::from_iter(pbuf_slice.iter().cloned());
	HLOCAL(pbuf as _).LocalFree()?;
	Ok(SID_wrap::new(raw_sid_copied))
}

/// [`CopySid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-copysid)
/// function.
#[must_use]
pub fn CopySid(src: &SID) -> SysResult<SID_wrap> {
	let sid_sz = GetLengthSid(&src);
	let mut sid_buf = vec![0u8; sid_sz as _];

	bool_to_sysresult(
		unsafe {
			advapi::ffi::CopySid(
				sid_sz,
				sid_buf.as_mut_ptr(),
				src as *const _ as _,
			)
		},
	).map(|_| SID_wrap::new(sid_buf))
}

/// [`CreateWellKnownSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createwellknownsid)
/// function.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, CreateWellKnownSid};
/// 
/// let sid = CreateWellKnownSid(co::WELL_KNOWN_SID_TYPE::LocalSystem, None)?;
/// # Ok::<_, co::ERROR>(())
/// ```
#[must_use]
pub fn CreateWellKnownSid(
	well_known_sid: co::WELL_KNOWN_SID_TYPE,
	domain_sid: Option<&SID>,
) -> SysResult<SID_wrap>
{
	let mut sid_sz = u32::default();

	unsafe {
		advapi::ffi::CreateWellKnownSid( // retrieve needed buffer sizes
			well_known_sid.0,
			domain_sid.map_or(std::ptr::null(), |s| s as *const _ as _),
			std::ptr::null_mut(),
			&mut sid_sz,
		);
	}
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut sid_buf = vec![0u8; sid_sz as _];

	bool_to_sysresult(
		unsafe {
			advapi::ffi::CreateWellKnownSid(
				well_known_sid.0,
				domain_sid.map_or(std::ptr::null(), |s| s as *const _ as _),
				sid_buf.as_mut_ptr(),
				&mut sid_sz,
			)
		},
	).map(|_| SID_wrap::new(sid_buf))
}

/// [`DecryptFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-decryptfilew)
/// function.
pub fn DecryptFile(file_name: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			advapi::ffi::DecryptFileW(WString::from_str(file_name).as_ptr(), 0)
		},
	)
}

/// [`EncryptFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-encryptfilew)
/// function.
pub fn EncryptFile(file_name: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			advapi::ffi::EncryptFileW(WString::from_str(file_name).as_ptr())
		},
	)
}

/// [`EncryptionDisable`](https://learn.microsoft.com/en-us/windows/win32/api/winefs/nf-winefs-encryptiondisable)
/// function.
pub fn EncryptionDisable(dir_path: &str, disable: bool) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			advapi::ffi::EncryptionDisable(
				WString::from_str(dir_path).as_ptr(),
				disable as _,
			)
		},
	)
}

/// [`EqualDomainSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-equaldomainsid)
/// function.
#[must_use]
pub fn EqualDomainSid(sid1: &SID, sid2: &SID) -> SysResult<bool> {
	let mut is_equal: BOOL = 0;
	bool_to_sysresult(
		unsafe {
			advapi::ffi::EqualDomainSid(
				sid1 as *const _ as _,
				sid2 as *const _ as _,
				&mut is_equal,
			)
		},
	).map(|_| is_equal != 0)
}

/// [`EqualPrefixSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-equalprefixsid)
/// function.
#[must_use]
pub fn EqualPrefixSid(sid1: &SID, sid2: &SID) -> SysResult<bool> {
	match unsafe {
		advapi::ffi::EqualPrefixSid(sid1 as *const _ as _, sid2 as *const _ as _)
	} {
		0 => match GetLastError() {
			co::ERROR::SUCCESS => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`EqualSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-equalsid)
/// function.
#[must_use]
pub fn EqualSid(sid1: &SID, sid2: &SID) -> SysResult<bool> {
	match unsafe {
		advapi::ffi::EqualSid(sid1 as *const _ as _, sid2 as *const _ as _)
	} {
		0 => match GetLastError() {
			co::ERROR::SUCCESS => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`GetLengthSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getlengthsid)
/// function.
#[must_use]
pub fn GetLengthSid(sid: &SID) -> u32 {
	unsafe { advapi::ffi::GetLengthSid(sid as *const _ as _) }
}

/// [`GetSidLengthRequired`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getsidlengthrequired)
/// function.
#[must_use]
pub fn GetSidLengthRequired(sub_authority_count: u8) -> u32 {
	unsafe { advapi::ffi::GetSidLengthRequired(sub_authority_count) }
}

/// [`GetUserName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getusernamew)
/// function.
#[must_use]
pub fn GetUserName() -> SysResult<String> {
	let mut name_sz = u32::default();

	unsafe { advapi::ffi::GetUserNameW(std::ptr::null_mut(), &mut name_sz); }
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut name_buf = WString::new_alloc_buf(name_sz as _);

	bool_to_sysresult(
		unsafe { advapi::ffi::GetUserNameW(name_buf.as_mut_ptr(), &mut name_sz) },
	).map(|_| name_buf.to_string())
}

/// [`GetWindowsAccountDomainSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getwindowsaccountdomainsid)
/// function.
#[must_use]
pub fn GetWindowsAccountDomainSid(sid: &SID) -> SysResult<SID_wrap> {
	let mut ad_sid_sz = u32::default();

	unsafe {
		advapi::ffi::GetWindowsAccountDomainSid(
			sid as *const _ as _,
			std::ptr::null_mut(),
			&mut ad_sid_sz,
		)
	};
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut ad_sid_buf = vec![0u8; ad_sid_sz as _];

	bool_to_sysresult(
		unsafe {
			advapi::ffi::GetWindowsAccountDomainSid(
				sid as *const _ as _,
				ad_sid_buf.as_mut_ptr(),
				&mut ad_sid_sz,
			)
		},
	).map(|_| SID_wrap::new(ad_sid_buf))
}

/// [`InitializeSecurityDescriptor`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-initializesecuritydescriptor)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::InitializeSecurityDescriptor;
///
/// let security_descriptor = InitializeSecurityDescriptor()?;
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[must_use]
pub fn InitializeSecurityDescriptor() -> SysResult<SECURITY_DESCRIPTOR> {
	let mut sd = unsafe { std::mem::zeroed::<SECURITY_DESCRIPTOR>() };
	bool_to_sysresult(
		unsafe {
			advapi::ffi::InitializeSecurityDescriptor(
				&mut sd as *mut _ as _,
				SECURITY_DESCRIPTOR_REVISION,
			)
		},
	).map(|_| sd)
}

/// [`IsValidSecurityDescriptor`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-isvalidsecuritydescriptor)
/// function.
#[must_use]
pub fn IsValidSecurityDescriptor(sd: &SECURITY_DESCRIPTOR) -> bool {
	unsafe { advapi::ffi::IsValidSecurityDescriptor(sd as *const _ as _) != 0 }
}

/// [`IsValidSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-isvalidsid)
/// function.
#[must_use]
pub fn IsValidSid(sid: &SID) -> SysResult<bool> {
	match unsafe { advapi::ffi::IsValidSid(sid as *const _ as _) } {
		0 => match GetLastError() {
			co::ERROR::SUCCESS => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`IsWellKnownSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-iswellknownsid)
/// function.
#[must_use]
pub fn IsWellKnownSid(
	sid: &SID, well_known_sid: co::WELL_KNOWN_SID_TYPE) -> bool
{
	unsafe {
		advapi::ffi::IsWellKnownSid(sid as *const _ as _, well_known_sid.0) != 0
	}
}

/// [`LookupAccountName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupaccountnamew)
/// function.
/// 
/// Returns account's domain name, `SID` and type, respectively.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{GetUserName, LookupAccountName};
/// 
/// let user_name = GetUserName()?;
/// let (domain_name, sid, kind) = LookupAccountName(None, &user_name)?;
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[must_use]
pub fn LookupAccountName(
	system_name: Option<&str>,
	account_name: &str,
) -> SysResult<(String, SID_wrap, co::SID_NAME_USE)>
{
	let mut sid_sz = u32::default();
	let mut domain_sz = u32::default();
	let mut sid_name_use = co::SID_NAME_USE::User;

	unsafe {
		advapi::ffi::LookupAccountNameW( // retrieve needed buffer sizes
			WString::from_opt_str(system_name).as_ptr(),
			WString::from_str(account_name).as_ptr(),
			std::ptr::null_mut(),
			&mut sid_sz,
			std::ptr::null_mut(),
			&mut domain_sz,
			&mut sid_name_use.0,
		);
	}
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut sid_buf = vec![0u8; sid_sz as _];
	let mut domain_buf = WString::new_alloc_buf(domain_sz as _);

	bool_to_sysresult(
		unsafe {
			advapi::ffi::LookupAccountNameW(
				WString::from_opt_str(system_name).as_ptr(),
				WString::from_str(account_name).as_ptr(),
				sid_buf.as_mut_ptr(),
				&mut sid_sz,
				domain_buf.as_mut_ptr(),
				&mut domain_sz,
				&mut sid_name_use.0,
			)
		},
	).map(|_| (domain_buf.to_string(), SID_wrap::new(sid_buf), sid_name_use))
}

/// [`LookupAccountSid`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupaccountsidw)
/// function.
/// 
/// Returns account name, domain name and type, respectively.
#[must_use]
pub fn LookupAccountSid(
	system_name: Option<&str>,
	sid: &SID,
) -> SysResult<(String, String, co::SID_NAME_USE)>
{
	let mut account_sz = u32::default();
	let mut domain_sz = u32::default();
	let mut sid_name_use = co::SID_NAME_USE::User;

	unsafe {
		advapi::ffi::LookupAccountSidW( // retrieve needed buffer sizes
			WString::from_opt_str(system_name).as_ptr(),
			sid as *const _ as _,
			std::ptr::null_mut(),
			&mut account_sz,
			std::ptr::null_mut(),
			&mut domain_sz,
			&mut sid_name_use.0,
		);
	}
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut account_buf = WString::new_alloc_buf(account_sz as _);
	let mut domain_buf = WString::new_alloc_buf(domain_sz as _);

	bool_to_sysresult(
		unsafe {
			advapi::ffi::LookupAccountSidW(
				WString::from_opt_str(system_name).as_ptr(),
				sid as *const _ as _,
				account_buf.as_mut_ptr(),
				&mut account_sz,
				domain_buf.as_mut_ptr(),
				&mut domain_sz,
				&mut sid_name_use.0,
			)
		},
	).map(|_| (account_buf.to_string(), domain_buf.to_string(), sid_name_use))
}
