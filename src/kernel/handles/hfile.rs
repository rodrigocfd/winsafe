#![allow(non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	BY_HANDLE_FILE_INFORMATION, GetLastError, HFILEMAP, HIDWORD, LODWORD,
	OVERLAPPED, SECURITY_ATTRIBUTES, WinResult, WString,
};
use crate::kernel::privs::{bool_to_winresult, INVALID_HANDLE_VALUE};
use crate::prelude::{Handle, HandleClose};

impl_handle! { HFILE: "kernel";
	/// Handle to a
	/// [file](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hfile).
	/// Originally just a `HANDLE`.
}

impl HandleClose for HFILE {}
impl KernelHfile for HFILE {}

/// [`HFILE`](crate::HFILE) methods from `kernel` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait KernelHfile: Handle {
	/// [`CreateFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HFILE::CloseHandle`](crate::prelude::HandleClose::CloseHandle) call.
	///
	/// # Examples
	///
	/// Opening an existing file as read-only:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HFILE};
	///
	/// let (hfile, status) = HFILE::CreateFile(
	///     "C:\\Temp\\test.txt",
	///     co::GENERIC::READ,
	///     co::FILE_SHARE::READ,
	///     None,
	///     co::DISPOSITION::OPEN_EXISTING,
	///     co::FILE_ATTRIBUTE::NORMAL,
	///     None,
	/// )?;
	///
	/// hfile.CloseHandle()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	///
	/// Opening a file for read and write. If the file doesn't exist, create it:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HFILE};
	///
	/// let (hfile, status) = HFILE::CreateFile(
	///     "C:\\Temp\\test.txt",
	///     co::GENERIC::READ | co::GENERIC::WRITE,
	///     co::FILE_SHARE::NoValue,
	///     None,
	///     co::DISPOSITION::OPEN_ALWAYS,
	///     co::FILE_ATTRIBUTE::NORMAL,
	///     None,
	/// )?;
	///
	/// hfile.CloseHandle()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn CreateFile(
		file_name: &str,
		desired_access: co::GENERIC,
		share_mode: co::FILE_SHARE,
		security_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		creation_disposition: co::DISPOSITION,
		flags_and_attrs: co::FILE_ATTRIBUTE,
		hfile_template: Option<HFILE>) -> WinResult<(HFILE, co::ERROR)>
	{
		match unsafe {
			kernel::ffi::CreateFileW(
				WString::from_str(file_name).as_ptr(),
				desired_access.0,
				share_mode.0,
				security_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				creation_disposition.0,
				flags_and_attrs.0,
				hfile_template.map_or(std::ptr::null_mut(), |h| h.0),
			) as _
		} {
			INVALID_HANDLE_VALUE => Err(GetLastError()),
			ptr => Ok((HFILE(ptr as _), GetLastError())),
		}
	}

	/// [`CreateFileMapping`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-createfilemappingw)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HFILEMAP::CloseHandle`](crate::prelude::HandleClose::CloseHandle)
	/// call.
	#[must_use]
	fn CreateFileMapping(self,
		mapping_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		protect: co::PAGE,
		max_size: Option<u64>,
		mapping_name: Option<&str>) -> WinResult<HFILEMAP>
	{
		unsafe {
			kernel::ffi::CreateFileMappingFromApp(
				self.as_ptr(),
				mapping_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				protect.0,
				max_size.unwrap_or_default(),
				WString::from_opt_str(mapping_name).as_ptr(),
			).as_mut()
		}.map(|ptr| HFILEMAP(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetFileInformationByHandle`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileinformationbyhandle)
	/// method.
	fn GetFileInformationByHandle(self,
		fi: &mut BY_HANDLE_FILE_INFORMATION) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				kernel::ffi::GetFileInformationByHandle(
					self.as_ptr(), fi as *mut _ as _,
				)
			},
		)
	}

	/// [`GetFileSizeEx`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfilesizeex)
	/// method.
	#[must_use]
	fn GetFileSizeEx(self) -> WinResult<usize> {
		let mut sz_buf = 0;
		match unsafe { kernel::ffi::GetFileSizeEx(self.as_ptr(), &mut sz_buf) } {
			0 => Err(GetLastError()),
			_ => Ok(sz_buf as _),
		}
	}

	/// [`GetFileType`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)
	/// method.
	#[must_use]
	fn GetFileType(self) -> WinResult<co::FILE_TYPE> {
		match co::FILE_TYPE(unsafe { kernel::ffi::GetFileType(self.as_ptr()) }) {
			co::FILE_TYPE::UNKNOWN => match GetLastError() {
				co::ERROR::SUCCESS => Ok(co::FILE_TYPE::UNKNOWN), // actual unknown type
				err => Err(err),
			},
			ty => Ok(ty),
		}
	}

	/// [`LockFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfile)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HFILE::UnlockFile`](crate::prelude::KernelHfile::UnlockFile) call.
	fn LockFile(self, offset: u64, num_bytes_to_lock: u64) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				kernel::ffi::LockFile(
					self.as_ptr(),
					LODWORD(offset),
					HIDWORD(offset),
					LODWORD(num_bytes_to_lock),
					HIDWORD(num_bytes_to_lock),
				)
			},
		)
	}

	/// [`ReadFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)
	/// method.
	///
	/// Returns the number of bytes read.
	fn ReadFile(self,
		buffer: &mut [u8],
		overlapped: Option<&mut OVERLAPPED>) -> WinResult<u32>
	{
		let mut bytes_read = u32::default();
		bool_to_winresult(
			unsafe {
				kernel::ffi::ReadFile(
					self.as_ptr(),
					buffer.as_mut_ptr() as _,
					buffer.len() as _,
					&mut bytes_read,
					overlapped.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				)
			},
		).map(|_| bytes_read)
	}

	/// [`SetEndOfFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setendoffile)
	/// method.
	fn SetEndOfFile(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel::ffi::SetEndOfFile(self.as_ptr()) })
	}

	/// [`SetFilePointerEx`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfilepointerex)
	/// method.
	fn SetFilePointerEx(self,
		distance_to_move: i64,
		move_method: co::FILE_STARTING_POINT) -> WinResult<i64>
	{
		let mut new_offset = i64::default();

		bool_to_winresult(
			unsafe {
				kernel::ffi::SetFilePointerEx(
					self.as_ptr(),
					distance_to_move,
					&mut new_offset,
					move_method.0,
				)
			},
		).map(|_| new_offset)
	}

	/// [`UnlockFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfile)
	/// method.
	fn UnlockFile(self,
		offset: u64, num_bytes_to_lock: u64) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				kernel::ffi::UnlockFile(
					self.as_ptr(),
					LODWORD(offset),
					HIDWORD(offset),
					LODWORD(num_bytes_to_lock),
					HIDWORD(num_bytes_to_lock),
				)
			},
		)
	}

	/// [`WriteFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)
	/// method.
	///
	/// Returns the number of bytes written.
	fn WriteFile(self,
		data: &[u8], overlapped: Option<&mut OVERLAPPED>) -> WinResult<u32>
	{
		let mut bytes_written = u32::default();

		bool_to_winresult(
			unsafe {
				kernel::ffi::WriteFile(
					self.as_ptr(),
					data.as_ptr() as _,
					data.len() as _,
					&mut bytes_written,
					overlapped.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				)
			},
		).map(|_| bytes_written)
	}
}
