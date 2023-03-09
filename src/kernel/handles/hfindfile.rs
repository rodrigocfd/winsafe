#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, SysResult, WIN32_FIND_DATA, WString};
use crate::kernel::guard::FindCloseGuard;
use crate::prelude::Handle;

impl_handle! { HFINDFILE;
	/// Handle to a
	/// [file search](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findfirstfilew).
	/// Originally just a `HANDLE`.
}

impl kernel_Hfindfile for HFINDFILE {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HFINDFILE`](crate::HFINDFILE).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hfindfile: Handle {
	/// [`FindFirstFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findfirstfilew)
	/// static method.
	///
	/// This method is rather tricky, consider using
	/// [`path::dir_list`](crate::path::dir_list).
	#[must_use]
	fn FindFirstFile(
		file_name: &str,
		wfd: &mut WIN32_FIND_DATA,
	) -> SysResult<(FindCloseGuard, bool)>
	{
		match unsafe {
			kernel::ffi::FindFirstFileW(
				WString::from_str(file_name).as_ptr(),
				wfd as *mut _ as _,
			).as_mut()
		} {
			Some(ptr) => Ok((
				FindCloseGuard::new(HFINDFILE(ptr)), // first file found
				true,
			)),
			None => match GetLastError() {
				co::ERROR::FILE_NOT_FOUND => Ok((
					FindCloseGuard::new(HFINDFILE::NULL), // not an error, first file not found
					false,
				)),
				err => Err(err),
			},
		}
	}

	/// [`FindNextFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findnextfilew)
	/// method.
	///
	/// This method is rather tricky, consider using
	/// [`path::dir_list`](crate::path::dir_list).
	#[must_use]
	fn FindNextFile(&self, wfd: &mut WIN32_FIND_DATA) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::FindNextFileW(self.as_ptr(), wfd as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false), // not an error, no further files found
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}
