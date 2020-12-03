#![allow(non_snake_case)]

use crate::co;
use crate::ffi::*;
use crate::Utf16;

ty_handle!(HINSTANCE,
	"A handle to an
	[instance](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hinstance).
	This is the base address of the module in memory. Same as `HMODULE`.");

impl HINSTANCE {
	/// [`GetModuleHandle`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
	/// function.
	pub fn GetModuleHandle(lpModuleName: Option<&str>) -> Result<HINSTANCE, co::ERROR> {
		match ptr_to_opt!(unsafe {
			kernel32::GetModuleHandleW(Utf16::from_opt_str(lpModuleName).as_ptr())
		}) {
			Some(p) => Ok(HINSTANCE(p)),
			None => Err(co::ERROR::GetLastError()),
		}
	}
}