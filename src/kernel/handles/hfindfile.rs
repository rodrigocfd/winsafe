#![allow(non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	GetLastError, path, WIN32_FIND_DATA, WinResult, WString,
};
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::Handle;

impl_handle! { HFINDFILE: "kernel";
	/// Handle to a
	/// [file search](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findfirstfilew).
	/// Originally just a `HANDLE`.
}

impl KernelHfindfile for HFINDFILE {}

/// [`HFILEMAPVIEW`](crate::HFILEMAPVIEW) methods from `kernel` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait KernelHfindfile: Handle {
	/// Returns an iterator over the found items, by calling
	/// [`HFINDFILE::FindFirstFile`](crate::prelude::KernelHfindfile::FindFirstFile),
	/// then subsequent
	/// [`HFINDFILE::FindNextFile`](crate::prelude::KernelHfindfile::FindNextFile),
	/// and finally freeing the resource by calling
	/// [`HFINDFILE::FindClose`](crate::prelude::KernelHfindfile::FindClose).
	///
	/// # Examples
	///
	/// Enumerating all TXT files in a directory:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::HFINDFILE;
	///
	/// for file_path in HFINDFILE::iter("C:\\Temp\\*.txt") {
	///     let file_path = file_path?;
	///     println!("File: {}", file_path);
	/// }
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	///
	/// Collecting the strings into a
	/// [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html):
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HFINDFILE, WinResult};
	///
	/// let file_paths = HFINDFILE::iter("")
	///     .collect::<WinResult<Vec<_>>>()?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn iter<'a>(
		path_and_pattern: &'a str) -> Box<dyn Iterator<Item = WinResult<String>> + 'a>
	{
		Box::new(HfindfileIter::new(path_and_pattern))
	}

	/// [`FindClose`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findclose)
	/// method.
	fn FindClose(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel::ffi::FindClose(self.as_ptr()) })
	}

	/// [`FindFirstFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findfirstfilew)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HFINDFILE::FindClose`](crate::prelude::KernelHfindfile::FindClose)
	/// call.
	///
	/// This method is rather tricky, consider using
	/// [`HFINDFILE::iter`](crate::prelude::KernelHfindfile::iter).
	#[must_use]
	fn FindFirstFile(
		file_name: &str,
		wfd: &mut WIN32_FIND_DATA) -> WinResult<(HFINDFILE, bool)>
	{
		match unsafe {
			kernel::ffi::FindFirstFileW(
				WString::from_str(file_name).as_ptr(),
				wfd as *mut _ as _,
			).as_mut()
		} {
			Some(ptr) => Ok((HFINDFILE(ptr), true)), // first file found
			None => match GetLastError() {
				co::ERROR::FILE_NOT_FOUND => Ok((HFINDFILE::NULL, false)), // not an error, first file not found
				err => Err(err),
			},
		}
	}

	/// [`FindNextFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findnextfilew)
	/// method.
	///
	/// This method is rather tricky, consider using
	/// [`HFINDFILE::iter`](crate::prelude::KernelHfindfile::iter).
	fn FindNextFile(self, wfd: &mut WIN32_FIND_DATA) -> WinResult<bool> {
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

//------------------------------------------------------------------------------

struct HfindfileIter<'a> {
	hfind: HFINDFILE,
	first_pass: bool,
	wfd: WIN32_FIND_DATA,
	path_and_pattern: &'a str,
	no_more: bool,
}

impl<'a> Drop for HfindfileIter<'a> {
	fn drop(&mut self) {
		self.hfind.FindClose().ok(); // ignore error
	}
}

impl<'a> Iterator for HfindfileIter<'a> {
	type Item = WinResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.no_more {
			return None;
		}

		let found = if self.first_pass {
			self.first_pass = false;
			let (hfind, found) =
				match HFINDFILE::FindFirstFile(self.path_and_pattern, &mut self.wfd) {
					Err(e) => {
						self.no_more = true; // prevent further iterations
						return Some(Err(e))
					},
					Ok((hfind, found)) => (hfind, found),
				};
			self.hfind = hfind;
			found
		} else {
			match self.hfind.FindNextFile(&mut self.wfd) {
				Err(e) => {
					self.no_more = true; // prevent further iterations
					return Some(Err(e))
				},
				Ok(found) => found,
			}
		};

		if found {
			let path_only = path::get_path(self.path_and_pattern)
				.unwrap_or("");
			Some(Ok(format!("{}\\{}", path_only, self.wfd.cFileName())))
		} else {
			None
		}
	}
}

impl<'a> HfindfileIter<'a> {
	fn new(path_and_pattern: &'a str) -> Self {
		Self {
			hfind: HFINDFILE::NULL,
			first_pass: true,
			wfd: WIN32_FIND_DATA::default(),
			path_and_pattern,
			no_more: false,
		}
	}
}
