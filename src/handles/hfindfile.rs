#![allow(non_snake_case)]

use crate::aliases::WinResult;

use crate::co;
use crate::ffi::kernel32;
use crate::funcs::GetLastError;
use crate::privs::bool_to_winresult;
use crate::structs::WIN32_FIND_DATA;
use crate::WString;

pub_struct_handle! {
	/// Handle to a
	/// [file search](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findfirstfilew).
	/// Originally just a `HANDLE`.
	HFINDFILE
}

impl HFINDFILE {
	/// [`FindClose`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findclose)
	/// method.
	pub fn FindClose(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel32::FindClose(self.ptr) })
	}

	/// [`FindFirstFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findfirstfilew)
	/// static method.
	///
	/// **Note:** Must be paired with a
	/// [`FindClose`](crate::HFINDFILE::FindClose) call.
	///
	/// # Examples
	///
	/// Enumerating all TXT files in a directory:
	///
	/// ```rust,ignore
	/// use winsafe::{HFINDFILE, WIN32_FIND_DATA};
	///
	/// let mut wfd = WIN32_FIND_DATA::default();
	///
	/// let (hfind, mut found) = HFINDFILE::FindFirstFile(
	///     "C:\\Temp\\*.txt",
	///     &mut wfd,
	/// ).unwrap();
	///
	/// if found {
	///     while found {
	///         println!("File: {}", wfd.cFileName());
	///         found = hfind.FindNextFile(&mut wfd).unwrap();
	///     }
	///
	///     hfind.FindClose().unwrap();
	/// }
	/// ```
	pub fn FindFirstFile(
		lpFileName: &str,
		lpFindFileData: &mut WIN32_FIND_DATA) -> WinResult<(HFINDFILE, bool)>
	{
		match unsafe {
			kernel32::FindFirstFileW(
				WString::from_str(lpFileName).as_ptr(),
				lpFindFileData as *mut _ as _,
			).as_mut()
		} {
			Some(ptr) => Ok((Self { ptr }, true)), // first file found
			None => match GetLastError() {
				co::ERROR::FILE_NOT_FOUND => Ok((Self::NULL, false)), // not an error, first file not found
				err => Err(err),
			},
		}
	}

	/// [`FindNextFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findnextfilew)
	/// method.
	pub fn FindNextFile(self,
		lpFindFileData: &mut WIN32_FIND_DATA) -> WinResult<bool>
	{
		match unsafe {
			kernel32::FindNextFileW(self.ptr, lpFindFileData as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false), // not an error, no further files found
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// Wrapper to [`FindFirstFile`](crate::HFINDFILE::FindFirstFile),
	/// [`FindNextFile`](crate::HFINDFILE::FindNextFile) and
	/// [`FindClose`](crate::HFINDFILE::FindClose), performing all needed
	/// operations and returning the full paths of all found files.
	///
	/// # Examples
	///
	/// Enumerating all TXT files in a directory:
	///
	/// ```rust,ignore
	/// use winsafe::HFINDFILE;
	///
	/// let files = HFINDFILE::ListAll("C:\\Temp\\*.txt").unwrap();
	///
	/// for file in files.iter() {
	///    println!("File: {}", file);
	/// }
	/// ```
	pub fn ListAll(pathAndPattern: &str) -> WinResult<Vec<String>> {
		let mut files = Vec::default();
		let mut wfd = WIN32_FIND_DATA::default();
		let (hfind, mut found) = Self::FindFirstFile(pathAndPattern, &mut wfd)?;

		let the_path = pathAndPattern.rfind('\\').map_or(
			String::default(),
			|idx| pathAndPattern.chars().take(idx).collect(),
		);

		if found {
			while found {
				files.push(format!("{}\\{}", the_path, wfd.cFileName()));
				found = hfind.FindNextFile(&mut wfd)?;
			}

			hfind.FindClose()?;
		}

		Ok(files)
	}
}
