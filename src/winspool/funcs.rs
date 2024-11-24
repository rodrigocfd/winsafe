#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::winspool::ffi;

/// [`GetDefaultPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/getdefaultprinter)
/// function.
#[must_use]
pub fn GetDefaultPrinter() -> SysResult<String> {
	let mut sz = u32::default();
	unsafe { ffi::GetDefaultPrinterW(std::ptr::null_mut(), &mut sz); }
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut name_buf = WString::new_alloc_buf(sz as _);
	bool_to_sysresult(
		unsafe { ffi::GetDefaultPrinterW(name_buf.as_mut_ptr(), &mut sz) },
	).map(|_| name_buf.to_string())
}
