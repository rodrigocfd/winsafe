#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::privs::*;
use crate::wininet::ffi;

handle! { HINTERNETREQUEST;
	/// Handle to an
	/// [Internet request](https://learn.microsoft.com/en-us/windows/win32/wininet/appendix-a-hinternet-handles).
	///
	/// Originally just a `HINTERNET`.
}

impl HINTERNETREQUEST {
	/// [`HttpSendRequest`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-httpsendrequestw)
	/// function.
	pub fn HttpSendRequest(&self, headers: Option<&str>, optional: Option<&[u8]>) -> SysResult<()> {
		bool_to_sysresult(unsafe {
			ffi::HttpSendRequestW(
				self.ptr(),
				WString::from_opt_str(headers).as_ptr(),
				headers.map_or(0, |h| h.chars().count() as _),
				optional.map_or(std::ptr::null(), |p| p.as_ptr() as _),
				optional.map_or(0, |p| p.len() as _),
			)
		})
	}

	/// [`InternetReadFile`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetreadfile)
	/// function.
	///
	/// Reads at most `buffer.len()`. Returns how many bytes were actually read.
	pub fn InternetReadFile(&self, buffer: &mut [u8]) -> SysResult<u32> {
		let mut bytes_read = 0u32;
		bool_to_sysresult(unsafe {
			ffi::InternetReadFile(
				self.ptr(),
				buffer.as_mut_ptr() as _,
				buffer.len() as _,
				&mut bytes_read,
			)
		})
		.map(|_| bytes_read)
	}
}
