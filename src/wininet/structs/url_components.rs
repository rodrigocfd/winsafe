#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;

/// [`URL_COMPONENTS`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/ns-wininet-url_componentsw)
/// struct.
pub struct URL_COMPONENTS {
	pub scheme: String,
	pub protocol_scheme: co::INTERNET_SCHEME,
	pub host_name: String,
	pub port: u16,
	pub user_name: String,
	pub password: String,
	pub url_path: String,
	pub extra_info: String,
}

pub(in crate::wininet) struct URL_COMPONENTS_buf {
	pub raw: URL_COMPONENTS_raw,
	w_scheme: WString,
	w_host_name: WString,
	w_user_name: WString,
	w_password: WString,
	w_url_path: WString,
	w_extra_info: WString,
}

impl URL_COMPONENTS_buf {
	#[must_use]
	pub(in crate::wininet) fn new() -> Self {
		Self {
			raw: URL_COMPONENTS_raw::new(),
			w_scheme: WString::new_alloc_buf(WString::SSO_LEN),
			w_host_name: WString::new_alloc_buf(WString::SSO_LEN),
			w_user_name: WString::new_alloc_buf(WString::SSO_LEN),
			w_password: WString::new_alloc_buf(WString::SSO_LEN),
			w_url_path: WString::new_alloc_buf(WString::SSO_LEN),
			w_extra_info: WString::new_alloc_buf(WString::SSO_LEN),
		}
	}

	pub(in crate::wininet) fn set_initial_ptrs(&mut self) {
		// Separated method because buffers are initially stack-allocated.
		unsafe {
			self.raw.lpszScheme = self.w_scheme.as_mut_ptr();
			self.raw.lpszHostName = self.w_host_name.as_mut_ptr();
			self.raw.lpszUserName = self.w_user_name.as_mut_ptr();
			self.raw.lpszPassword = self.w_password.as_mut_ptr();
			self.raw.lpszUrlPath = self.w_url_path.as_mut_ptr();
			self.raw.lpszExtraInfo = self.w_extra_info.as_mut_ptr();
		}
	}

	pub(in crate::wininet) fn alloc_more_strs(&mut self) {
		const INCREASE: usize = 64;

		// There is no way to know which buffer needs more room, so we increase everyone.

		self.w_scheme = WString::new_alloc_buf(self.w_scheme.buf_len() + INCREASE); // make room
		self.raw.lpszScheme = unsafe { self.w_scheme.as_mut_ptr() }; // and reassign pointer to buffer
		self.raw.dwSchemeLength = self.w_scheme.buf_len() as _;

		self.w_host_name = WString::new_alloc_buf(self.w_host_name.buf_len() + INCREASE);
		self.raw.lpszHostName = unsafe { self.w_host_name.as_mut_ptr() };
		self.raw.dwHostNameLength = self.w_host_name.buf_len() as _;

		self.w_user_name = WString::new_alloc_buf(self.w_user_name.buf_len() + INCREASE);
		self.raw.lpszUserName = unsafe { self.w_user_name.as_mut_ptr() };
		self.raw.dwUserNameLength = self.w_user_name.buf_len() as _;

		self.w_password = WString::new_alloc_buf(self.w_password.buf_len() + INCREASE);
		self.raw.lpszPassword = unsafe { self.w_password.as_mut_ptr() };
		self.raw.dwPasswordLength = self.w_password.buf_len() as _;

		self.w_url_path = WString::new_alloc_buf(self.w_url_path.buf_len() + INCREASE);
		self.raw.lpszUrlPath = unsafe { self.w_url_path.as_mut_ptr() };
		self.raw.dwUrlPathLength = self.w_url_path.buf_len() as _;

		self.w_extra_info = WString::new_alloc_buf(self.w_extra_info.buf_len() + INCREASE);
		self.raw.lpszExtraInfo = unsafe { self.w_extra_info.as_mut_ptr() };
		self.raw.dwExtraInfoLength = self.w_extra_info.buf_len() as _;
	}

	#[must_use]
	pub(in crate::wininet) fn to_final(&self) -> URL_COMPONENTS {
		URL_COMPONENTS {
			scheme: self.w_scheme.to_string(),
			protocol_scheme: self.raw.nScheme,
			host_name: self.w_host_name.to_string(),
			port: self.raw.nPort,
			user_name: self.w_user_name.to_string(),
			password: self.w_password.to_string(),
			url_path: self.w_url_path.to_string(),
			extra_info: self.w_extra_info.to_string(),
		}
	}
}

#[repr(C)]
pub(in crate::wininet) struct URL_COMPONENTS_raw {
	dwStructSize: u32,
	pub lpszScheme: *mut u16,
	pub dwSchemeLength: u32,
	pub nScheme: co::INTERNET_SCHEME,
	pub lpszHostName: *mut u16,
	pub dwHostNameLength: u32,
	pub nPort: u16,
	pub lpszUserName: *mut u16,
	pub dwUserNameLength: u32,
	pub lpszPassword: *mut u16,
	pub dwPasswordLength: u32,
	pub lpszUrlPath: *mut u16,
	pub dwUrlPathLength: u32,
	pub lpszExtraInfo: *mut u16,
	pub dwExtraInfoLength: u32,
}

impl URL_COMPONENTS_raw {
	#[must_use]
	const fn new() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.dwStructSize = std::mem::size_of::<Self>() as _;
		obj.dwSchemeLength = WString::SSO_LEN as _; // start with stack allocation
		obj.dwHostNameLength = WString::SSO_LEN as _;
		obj.dwUserNameLength = WString::SSO_LEN as _;
		obj.dwPasswordLength = WString::SSO_LEN as _;
		obj.dwUrlPathLength = WString::SSO_LEN as _;
		obj.dwExtraInfoLength = WString::SSO_LEN as _;
		obj
	}
}
