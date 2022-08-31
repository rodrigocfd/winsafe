#![allow(non_snake_case)]

use crate::advapi;
use crate::advapi::privs::UNLEN;
use crate::kernel::decl::{GetLastError, SysResult, WString};
use crate::kernel::privs::bool_to_sysresult;

/// [`DecryptFile`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-decryptfilew)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "advapi")))]
pub fn DecryptFile(file_name: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			advapi::ffi::DecryptFileW(WString::from_str(file_name).as_ptr(), 0)
		},
	)
}

/// [`EncryptFile`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-encryptfilew)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "advapi")))]
pub fn EncryptFile(file_name: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			advapi::ffi::EncryptFileW(WString::from_str(file_name).as_ptr())
		},
	)
}

/// [`EncryptionDisable`](https://docs.microsoft.com/en-us/windows/win32/api/winefs/nf-winefs-encryptiondisable)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "advapi")))]
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

/// [`GetUserName`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getusernamew)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "advapi")))]
#[must_use]
pub fn GetUserName() -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(UNLEN + 1);
	let mut sz = buf.buf_len() as u32;

	match unsafe { advapi::ffi::GetUserNameW(buf.as_mut_ptr(), &mut sz) } {
		0 => Err(GetLastError()),
		_ => Ok(buf.to_string()),
	}
}
