#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::wininet::ffi;

/// [`InternetCanonicalizeUrl`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetcanonicalizeurlw)
/// function.
#[must_use]
pub fn InternetCanonicalizeUrl(url: &str, flags: co::ICU) -> SysResult<String> {
	let wurl = WString::from_str(url);
	let mut buf_sz = WString::SSO_LEN as u32; // start with no string heap allocation

	loop {
		let mut buf = WString::new_alloc_buf(buf_sz as _);
		match bool_to_sysresult(unsafe {
			ffi::InternetCanonicalizeUrlW(wurl.as_ptr(), buf.as_mut_ptr(), &mut buf_sz, flags.raw())
		}) {
			Ok(_) => return Ok(buf.to_string()),
			Err(err) => match err {
				co::ERROR::INSUFFICIENT_BUFFER => continue,
				err => return Err(err),
			},
		}
	}
}

/// [`InternetCombineUrl`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetcombineurlw)
/// function.
#[must_use]
pub fn InternetCombineUrl(base_url: &str, relative_url: &str, flags: co::ICU) -> SysResult<String> {
	let wbase = WString::from_str(base_url);
	let wrelative = WString::from_str(relative_url);
	let mut buf_sz = WString::SSO_LEN as u32; // start with no string heap allocation

	loop {
		let mut buf = WString::new_alloc_buf(buf_sz as _);
		match bool_to_sysresult(unsafe {
			ffi::InternetCombineUrlW(
				wbase.as_ptr(),
				wrelative.as_ptr(),
				buf.as_mut_ptr(),
				&mut buf_sz,
				flags.raw(),
			)
		}) {
			Ok(_) => return Ok(buf.to_string()),
			Err(err) => match err {
				co::ERROR::INSUFFICIENT_BUFFER => continue,
				err => return Err(err),
			},
		}
	}
}

/// [`InternetTimeToSystemTime`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internettimetosystemtime)
/// function.
#[must_use]
pub fn InternetTimeToSystemTime(time: &str) -> SysResult<SYSTEMTIME> {
	let mut st = SYSTEMTIME::default();
	bool_to_sysresult(unsafe {
		ffi::InternetTimeToSystemTimeW(WString::from_str(time).as_ptr(), pvoid(&mut st), 0)
	})
	.map(|_| st)
}
