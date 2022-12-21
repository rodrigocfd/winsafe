#![allow(non_snake_case)]

use crate::advapi;
use crate::advapi::privs::{SECURITY_DESCRIPTOR_REVISION, UNLEN};
use crate::kernel::decl::{
	GetLastError, SECURITY_DESCRIPTOR, SysResult, WString,
};
use crate::kernel::privs::bool_to_sysresult;

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

/// [`GetUserName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getusernamew)
/// function.
#[must_use]
pub fn GetUserName() -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(UNLEN + 1);
	let mut sz = buf.buf_len() as u32;

	match unsafe { advapi::ffi::GetUserNameW(buf.as_mut_ptr(), &mut sz) } {
		0 => Err(GetLastError()),
		_ => Ok(buf.to_string()),
	}
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
