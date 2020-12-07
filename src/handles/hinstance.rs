#![allow(non_snake_case)]

use crate::co;
use crate::ffi::{kernel32, Void};
use crate::Utf16;

handle_type! {
	/// Handle to an
	/// [instance](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hinstance).
	/// Same as `HMODULE`.
	HINSTANCE
}

impl HINSTANCE {
	/// [`GetModuleHandle`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
	/// function.
	pub fn GetModuleHandle(
		lpModuleName: Option<&str>) -> Result<HINSTANCE, co::ERROR>
	{
		match ptr_to_opt!(
			kernel32::GetModuleHandleW(Utf16::from_opt_str(lpModuleName).as_ptr())
		) {
			Some(p) => Ok(HINSTANCE(p)),
			None => Err(co::ERROR::GetLastError()),
		}
	}
}