#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::wininet::{ffi, structs::*};

/// [`InternetCanonicalizeUrl`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetcanonicalizeurlw)
/// function.
#[must_use]
pub fn InternetCanonicalizeUrl(url: &str, flags: co::ICU) -> SysResult<String> {
	let wurl = WString::from_str(url);
	let mut buf_sz = WString::SSO_LEN as u32; // start with no string heap allocation

	loop {
		let mut buf = WString::new_alloc_buf(buf_sz as _);
		match BoolRet(unsafe {
			ffi::InternetCanonicalizeUrlW(wurl.as_ptr(), buf.as_mut_ptr(), &mut buf_sz, flags.raw())
		})
		.to_sysresult()
		{
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
		match BoolRet(unsafe {
			ffi::InternetCombineUrlW(
				wbase.as_ptr(),
				wrelative.as_ptr(),
				buf.as_mut_ptr(),
				&mut buf_sz,
				flags.raw(),
			)
		})
		.to_sysresult()
		{
			Ok(_) => return Ok(buf.to_string()),
			Err(err) => match err {
				co::ERROR::INSUFFICIENT_BUFFER => continue,
				err => return Err(err),
			},
		}
	}
}

/// [`InternetCrackUrl`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetcrackurlw)
/// function.
///
/// # Related functions
///
/// * [`InternetCreateUrl`](crate::InternetCreateUrl)
#[must_use]
pub fn InternetCrackUrl(url: &str, flags: co::ICU) -> SysResult<URL_COMPONENTS> {
	let w_url = WString::from_str(url);
	let mut raw = URL_COMPONENTS_raw::new_crack();

	BoolRet(unsafe {
		ffi::InternetCrackUrlW(w_url.as_ptr(), w_url.str_len() as _, flags.raw(), pvoid(&mut raw))
	})
	.to_sysresult()
	.map(|_| URL_COMPONENTS {
		scheme: WString::from_wchars_count(raw.lpszScheme, raw.dwSchemeLength as _).to_string(),
		protocol_scheme: raw.nScheme,
		host_name: WString::from_wchars_count(raw.lpszHostName, raw.dwHostNameLength as _)
			.to_string(),
		port: raw.nPort,
		user_name: WString::from_wchars_count(raw.lpszUserName, raw.dwUserNameLength as _)
			.to_string(),
		password: WString::from_wchars_count(raw.lpszPassword, raw.dwPasswordLength as _)
			.to_string(),
		url_path: WString::from_wchars_count(raw.lpszUrlPath, raw.dwUrlPathLength as _).to_string(),
		extra_info: WString::from_wchars_count(raw.lpszExtraInfo, raw.dwExtraInfoLength as _)
			.to_string(),
	})
}

/// [`InternetCreateUrl`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetcreateurlw)
/// function.
///
/// # Related functions
///
/// * [`InternetCrackUrl`](crate::InternetCrackUrl)
#[must_use]
pub fn InternetCreateUrl(components: &URL_COMPONENTS, flags: co::ICU) -> SysResult<String> {
	let mut scheme_w = WString::from_str(&components.scheme);
	let mut host_name_w = WString::from_str(&components.host_name);
	let mut user_name_w = WString::from_str(&components.user_name);
	let mut password_w = WString::from_str(&components.password);
	let mut url_path_w = WString::from_str(&components.url_path);
	let mut extra_info_w = WString::from_str(&components.extra_info);

	let mut raw = URL_COMPONENTS_raw::new_create(&components);
	if scheme_w.is_allocated() {
		raw.lpszScheme = unsafe { scheme_w.as_mut_ptr() };
		raw.dwSchemeLength = scheme_w.str_len() as _;
	}
	if host_name_w.is_allocated() {
		raw.lpszHostName = unsafe { host_name_w.as_mut_ptr() };
		raw.dwHostNameLength = host_name_w.str_len() as _;
	}
	if user_name_w.is_allocated() {
		raw.lpszUserName = unsafe { user_name_w.as_mut_ptr() };
		raw.dwUserNameLength = user_name_w.str_len() as _;
	}
	if password_w.is_allocated() {
		raw.lpszPassword = unsafe { password_w.as_mut_ptr() };
		raw.dwPasswordLength = password_w.str_len() as _;
	}
	if url_path_w.is_allocated() {
		raw.lpszUrlPath = unsafe { url_path_w.as_mut_ptr() };
		raw.dwUrlPathLength = url_path_w.str_len() as _;
	}
	if extra_info_w.is_allocated() {
		raw.lpszExtraInfo = unsafe { extra_info_w.as_mut_ptr() };
		raw.dwExtraInfoLength = extra_info_w.str_len() as _;
	}

	let mut url_len = 0u32;
	match BoolRet(unsafe {
		ffi::InternetCreateUrlW(pcvoid(&raw), flags.raw(), std::ptr::null_mut(), &mut url_len) // first call to retrieve len
	})
	.to_sysresult()
	{
		Ok(_) => {}, // should never happen
		Err(err) => match err {
			co::ERROR::INSUFFICIENT_BUFFER => {}, // expected
			err => return Err(err),
		},
	}

	let mut buf = WString::new_alloc_buf(url_len as _);
	BoolRet(unsafe {
		ffi::InternetCreateUrlW(pcvoid(&raw), flags.raw(), buf.as_mut_ptr(), &mut url_len)
	})
	.to_sysresult()
	.map(|_| buf.to_string())
}

/// [`InternetTimeToSystemTime`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internettimetosystemtime)
/// function.
#[must_use]
pub fn InternetTimeToSystemTime(time: &str) -> SysResult<SYSTEMTIME> {
	let mut st = SYSTEMTIME::default();
	BoolRet(unsafe {
		ffi::InternetTimeToSystemTimeW(WString::from_str(time).as_ptr(), pvoid(&mut st), 0)
	})
	.to_sysresult()
	.map(|_| st)
}
