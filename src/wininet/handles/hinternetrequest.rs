#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
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
	/// [`HttpQueryInfo`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-httpqueryinfow)
	/// function.
	///
	/// The return types varies according to `format`:
	///
	/// * [`co::HTTP_QUERY_FLAG::NUMBER`](crate::co::HTTP_QUERY_FLAG::NUMBER) – [`HttpInfo::Number`](crate::HttpInfo::Number)
	/// * [`co::HTTP_QUERY_FLAG::NUMBER64`](crate::co::HTTP_QUERY_FLAG::NUMBER64) – [`HttpInfo::Number64`](crate::HttpInfo::Number64)
	/// * [`co::HTTP_QUERY_FLAG::SYSTEMTIME`](crate::co::HTTP_QUERY_FLAG::SYSTEMTIME) – [`HttpInfo::Time`](crate::HttpInfo::Time)
	/// * default – [`HttpInfo::Str`](crate::HttpInfo::Str)
	#[must_use]
	pub fn HttpQueryInfo(
		&self,
		info: co::HTTP_QUERY,
		format: co::HTTP_QUERY_FLAG,
	) -> SysResult<HttpInfo> {
		// Stack buffer with the size of SYSTEMTIME for our initial allocation.
		let mut static_buf = [0u8; std::mem::size_of::<SYSTEMTIME>() / std::mem::size_of::<u8>()];
		let mut buf_len = static_buf.len() as u32;
		let mut buf_slice = &mut static_buf[..];

		// Dynamic buffer, used if INSUFFICIENT_BUFFER error.
		let mut dyn_buf = Vec::<u8>::new();

		let mut index = 0u32;

		loop {
			match bool_to_sysresult(unsafe {
				ffi::HttpQueryInfoW(
					self.ptr(),
					info.raw() | format.raw(),
					buf_slice.as_mut_ptr() as _,
					&mut buf_len,
					&mut index,
				)
			}) {
				Ok(_) => break, // successful call, result is in buffer
				Err(err) => match err {
					co::ERROR::INSUFFICIENT_BUFFER => {
						buf_len += 2; // terminating null
						dyn_buf.resize(buf_len as _, 0);
						buf_slice = dyn_buf.as_mut_slice();
						continue; // call the function again, now with dyn_buf
					},
					err => return Err(err),
				},
			}
		}

		Ok(match format {
			co::HTTP_QUERY_FLAG::NUMBER => {
				HttpInfo::Number(u32::from_le_bytes(buf_slice[0..4].try_into().unwrap()))
			},
			co::HTTP_QUERY_FLAG::NUMBER64 => {
				HttpInfo::Number64(u64::from_le_bytes(buf_slice[0..8].try_into().unwrap()))
			},
			co::HTTP_QUERY_FLAG::SYSTEMTIME => {
				let arr: [u8; 16] = buf_slice.try_into().unwrap();
				let st = unsafe { std::mem::transmute::<_, SYSTEMTIME>(arr) };
				HttpInfo::Time(st)
			},
			_ => {
				let wbuf = unsafe {
					std::slice::from_raw_parts(
						buf_slice.as_ptr() as *const u16,
						buf_slice.len() / 2,
					)
				};
				HttpInfo::Str(WString::from_wchars_slice(wbuf).to_string())
			},
		})
	}

	/// [`HttpAddRequestHeaders`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-httpaddrequestheadersw)
	/// function.
	pub fn HttpAddRequestHeaders(
		&self,
		headers: &str,
		modifiers: co::HTTP_ADDREQ,
	) -> SysResult<()> {
		bool_to_sysresult(unsafe {
			ffi::HttpAddRequestHeadersW(
				self.ptr(),
				WString::from_str(headers).as_ptr(),
				headers.chars().count() as _,
				modifiers.raw(),
			)
		})
	}

	/// [`HttpSendRequest`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-httpsendrequestw)
	/// function.
	pub fn HttpSendRequest(&self, headers: Option<&str>, optional: &[u8]) -> SysResult<()> {
		bool_to_sysresult(unsafe {
			ffi::HttpSendRequestW(
				self.ptr(),
				WString::from_opt_str(headers).as_ptr(),
				headers.map_or(0, |h| h.chars().count() as _),
				vec_ptr(optional) as _,
				optional.len() as _,
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
