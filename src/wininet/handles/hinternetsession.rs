#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::wininet::ffi;

handle! { HINTERNETSESSION;
	/// Handle to an
	/// [Internet session](https://learn.microsoft.com/en-us/windows/win32/wininet/appendix-a-hinternet-handles).
	///
	/// Originally just a `HINTERNET`.
}

impl HINTERNETSESSION {
	/// [`HttpOpenRequest`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-httpopenrequestw)
	/// function.
	#[must_use]
	pub fn HttpOpenRequest(
		&self,
		verb: Option<&str>,
		object_name: &str,
		version: Option<&str>,
		referrer: Option<&str>,
		accept_types: Option<&[impl AsRef<str>]>,
		flags: co::INTERNET_FLAG,
		context: usize,
	) -> SysResult<InternetCloseHandleGuard<HINTERNETREQUEST>> {
		unsafe {
			ptr_to_sysresult_handle(ffi::HttpOpenRequestW(
				self.ptr(),
				WString::from_opt_str(verb).as_ptr(),
				WString::from_str(object_name).as_ptr(),
				WString::from_opt_str(version).as_ptr(),
				WString::from_opt_str(referrer).as_ptr(),
				accept_types.map_or(std::ptr::null_mut(), |v| WString::from_str_vec(v).as_ptr()),
				flags.raw(),
				context,
			))
			.map(|h| InternetCloseHandleGuard::new(h))
		}
	}
}
