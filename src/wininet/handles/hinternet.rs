#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::wininet::ffi;

handle! { HINTERNET;
	/// Root
	/// [Internet](https://learn.microsoft.com/en-us/windows/win32/wininet/appendix-a-hinternet-handles)
	/// handle.
}

impl HINTERNET {
	/// [`InternetConnect`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetconnectw)
	/// function.
	#[must_use]
	pub fn InternetConnect(
		&self,
		server_name: &str,
		server_port: co::INTERNET_DEFAULT_PORT,
		user_name: Option<&str>,
		password: Option<&str>,
		service: co::INTERNET_SERVICE,
		flags: co::INTERNET_FLAG,
		context: Option<isize>,
	) -> SysResult<InternetCloseHandleGuard<HINTERNETSESSION>> {
		unsafe {
			ptr_to_sysresult_handle(ffi::InternetConnectW(
				self.ptr(),
				WString::from_str(server_name).as_ptr(),
				server_port.raw(),
				WString::from_opt_str(user_name).as_ptr(),
				WString::from_opt_str(password).as_ptr(),
				service.raw(),
				flags.raw(),
				context.unwrap_or_default(),
			))
			.map(|h| InternetCloseHandleGuard::new(h))
		}
	}

	/// [`InternetOpen`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetopenw)
	/// function.
	#[must_use]
	pub fn InternetOpen(
		agent: &str,
		access_type: co::INTERNET_OPEN_TYPE,
		proxy: Option<&str>,
		proxy_bypass: Option<&str>,
		flags: co::INTERNET_FLAG,
	) -> SysResult<InternetCloseHandleGuard<HINTERNET>> {
		unsafe {
			ptr_to_sysresult_handle(ffi::InternetOpenW(
				WString::from_str(agent).as_ptr(),
				access_type.raw(),
				WString::from_opt_str(proxy).as_ptr(),
				WString::from_opt_str(proxy_bypass).as_ptr(),
				flags.raw(),
			))
			.map(|h| InternetCloseHandleGuard::new(h))
		}
	}

	/// [`InternetOpenUrl`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetopenurlw)
	/// function.
	///
	/// It's a good idea to pass `url` through
	/// [`InternetCanonicalizeUrl`](crate::InternetCanonicalizeUrl) before
	/// calling this function.
	#[must_use]
	pub fn InternetOpenUrl(
		&self,
		url: &str,
		headers: Option<&str>,
		flags: co::INTERNET_FLAG,
		context: Option<isize>,
	) -> SysResult<InternetCloseHandleGuard<HINTERNETREQUEST>> {
		unsafe {
			ptr_to_sysresult_handle(ffi::InternetOpenUrlW(
				self.ptr(),
				WString::from_str(url).as_ptr(),
				WString::from_opt_str(headers).as_ptr(),
				headers.map_or(0, |h| h.chars().count() as _),
				flags.raw(),
				context.unwrap_or_default(),
			))
			.map(|h| InternetCloseHandleGuard::new(h))
		}
	}
}
