#![allow(non_camel_case_types, non_snake_case)]

use crate::co;

/// [`URL_COMPONENTS`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/ns-wininet-url_componentsw)
/// struct.
#[derive(Default, Clone)]
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
	/// Constructs the struct to be passed to `InternetCrackUrl`.
	#[must_use]
	pub(in crate::wininet) const fn new_crack() -> Self {
		// https://learn.microsoft.com/en-us/windows/win32/api/winhttp/nf-winhttp-winhttpcreateurl#examples
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.dwStructSize = std::mem::size_of::<Self>() as _;
		obj.dwSchemeLength = 0xffff_ffff; // -1
		obj.dwHostNameLength = 0xffff_ffff;
		obj.dwUserNameLength = 0xffff_ffff;
		obj.dwPasswordLength = 0xffff_ffff;
		obj.dwUrlPathLength = 0xffff_ffff;
		obj.dwExtraInfoLength = 0xffff_ffff;
		obj
	}

	/// Constructs the struct to be passed to `InternetCreateUrl`.
	#[must_use]
	pub(in crate::wininet) const fn new_create(uc: &URL_COMPONENTS) -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.dwStructSize = std::mem::size_of::<Self>() as _;
		obj.nPort = uc.port;
		obj.nScheme = uc.protocol_scheme;
		obj
	}
}
