#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	GetLastError, path, SysResult, WIN32_FIND_DATA, WString,
};
use crate::prelude::Handle;

impl_handle! { HFINDFILE: "kernel";
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
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait kernel_Hfindfile: Handle {
	/// Returns an iterator over the found items, by calling
	/// [`HFINDFILE::FindFirstFile`](crate::prelude::kernel_Hfindfile::FindFirstFile),
	/// then subsequent
	/// [`HFINDFILE::FindNextFile`](crate::prelude::kernel_Hfindfile::FindNextFile),
	/// and finally freeing the resource by calling
	/// [`HFINDFILE::FindClose`](crate::prelude::kernel_Hfindfile::FindClose).
	///
	/// # Examples
	///
	/// Enumerating all TXT files in a directory:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{AnyResult, HFINDFILE};
	///
	/// for file_path in HFINDFILE::iter("C:\\Temp\\*.txt") {
	///     let file_path = file_path?;
	///     println!("File: {}", file_path);
	/// }
	///
	/// HFINDFILE::iter("C:\\Temp\\*.txt")
	///     .try_for_each(|file_path| -> AnyResult<()> {
	///         let file_path = file_path?;
	///         println!("File: {}", file_path);
	///         Ok(())
	///     })?;
	/// # Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
	/// ```
	///
	/// Collecting the TXTs into a
	/// [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html):
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HFINDFILE, SysResult};
	///
	/// let file_paths = HFINDFILE::iter("C:\\Temp\\*.txt")
	///     .collect::<SysResult<Vec<_>>>()?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn iter<'a>(
		path_and_pattern: &'a str) -> Box<dyn Iterator<Item = SysResult<String>> + 'a>
	{
		Box::new(FindFileIter::new(path_and_pattern))
	}

	/// [`FindFirstFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findfirstfilew)
	/// static method.
	///
	/// This method is rather tricky, consider using
	/// [`HFINDFILE::iter`](crate::prelude::kernel_Hfindfile::iter).
	#[must_use]
	fn FindFirstFile(
		file_name: &str,
		wfd: &mut WIN32_FIND_DATA) -> SysResult<(HfindfileGuard, bool)>
	{
		match unsafe {
			kernel::ffi::FindFirstFileW(
				WString::from_str(file_name).as_ptr(),
				wfd as *mut _ as _,
			).as_mut()
		} {
			Some(ptr) => Ok((
				HfindfileGuard { handle: HFINDFILE(ptr) }, // first file found
				true,
			)),
			None => match GetLastError() {
				co::ERROR::FILE_NOT_FOUND => Ok((
					HfindfileGuard { handle: HFINDFILE::NULL }, // not an error, first file not found
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
	/// [`HFINDFILE::iter`](crate::prelude::kernel_Hfindfile::iter).
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

//------------------------------------------------------------------------------

handle_guard! { HfindfileGuard, HFINDFILE, "kernel";
	kernel::ffi::FindClose;
	/// RAII implementation for [`HFINDFILE`](crate::HFINDFILE) which
	/// automatically calls
	/// [`FindClose`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findclose)
	/// when the object goes out of scope.
}

//------------------------------------------------------------------------------

struct FindFileIter<'a> {
	hfind: HfindfileGuard,
	wfd: WIN32_FIND_DATA,
	path_and_pattern: &'a str,
	first_pass: bool,
	no_more: bool,
}

impl<'a> Iterator for FindFileIter<'a> {
	type Item = SysResult<String>;

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

impl<'a> FindFileIter<'a> {
	fn new(path_and_pattern: &'a str) -> Self {
		Self {
			hfind: HfindfileGuard { handle: HFINDFILE::NULL },
			wfd: WIN32_FIND_DATA::default(),
			path_and_pattern,
			first_pass: true,
			no_more: false,
		}
	}
}
