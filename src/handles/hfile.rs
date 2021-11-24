#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::funcs::{GetLastError, HIDWORD, LODWORD};
use crate::handles::{HandleClose, HFILEMAP};
use crate::privs::{bool_to_winresult, INVALID_HANDLE_VALUE};
use crate::structs::{
	BY_HANDLE_FILE_INFORMATION,
	OVERLAPPED,
	SECURITY_ATTRIBUTES,
};
use crate::various::WString;

/// Handle to a
/// [file](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hfile).
/// Originally just a `HANDLE`.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HFILE(pub(crate) *mut std::ffi::c_void);

impl_handle!(HFILE);
impl HandleClose for HFILE {}

impl HFILE {
	/// [`CreateFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HFILE::CloseHandle`](crate::HFILE::CloseHandle) call.
	///
	/// # Examples
	///
	/// Opening an existing file as read-only:
	///
	/// ```rust,ignore
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
	/// ```
	///
	/// Opening a file for read and write. If the file doesn't exist, create it:
	///
	/// ```rust,ignore
	/// use winsafe::{co, HFILE};
	///
	/// let (hfile, status) = HFILE::CreateFile(
	///     "C:\\Temp\\test.txt",
	///     co::GENERIC::READ | co::GENERIC::WRITE,
	///     co::FILE_SHARE::NONE,
	///     None,
	///     co::DISPOSITION::OPEN_ALWAYS,
	///     co::FILE_ATTRIBUTE::NORMAL,
	///     None,
	/// )?;
	///
	/// hfile.CloseHandle()?;
	/// ```
	pub fn CreateFile(
		file_name: &str,
		desired_access: co::GENERIC,
		share_mode: co::FILE_SHARE,
		security_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		creation_disposition: co::DISPOSITION,
		flags_and_attrs: co::FILE_ATTRIBUTE,
		hfile_template: Option<HFILE>) -> WinResult<(HFILE, co::ERROR)>
	{
		match unsafe {
			kernel32::CreateFileW(
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
			ptr => Ok((Self(ptr as _), GetLastError())),
		}
	}

	/// [`CreateFileMapping`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-createfilemappingw)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HFILEMAP::CloseHandle`](crate::HFILEMAP::CloseHandle) call.
	pub fn CreateFileMapping(self,
		mapping_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		protect: co::PAGE,
		max_size: Option<u64>,
		mapping_name: Option<&str>) -> WinResult<HFILEMAP>
	{
		unsafe {
			kernel32::CreateFileMappingW(
				self.0,
				mapping_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				protect.0,
				max_size.map_or(0, |n| HIDWORD(n)),
				max_size.map_or(0, |n| LODWORD(n)),
				mapping_name.map_or(std::ptr::null(), |lp| WString::from_str(lp).as_ptr()),
			).as_mut()
		}.map(|ptr| HFILEMAP(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetFileInformationByHandle`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileinformationbyhandle)
	/// method.
	pub fn GetFileInformationByHandle(self,
		fi: &mut BY_HANDLE_FILE_INFORMATION) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				kernel32::GetFileInformationByHandle(self.0, fi as *mut _ as _)
			},
		)
	}

	/// [`GetFileSizeEx`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfilesizeex)
	/// method.
	pub fn GetFileSizeEx(self) -> WinResult<usize> {
		let mut sz_buf = 0;
		match unsafe { kernel32::GetFileSizeEx(self.0, &mut sz_buf) } {
			0 => Err(GetLastError()),
			_ => Ok(sz_buf as _),
		}
	}

	/// [`GetFileType`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)
	/// method.
	pub fn GetFileType(self) -> WinResult<co::FILE_TYPE> {
		match co::FILE_TYPE(unsafe { kernel32::GetFileType(self.0) }) {
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
	/// [`HFILE::UnlockFile`](crate::HFILE::UnlockFile) call.
	pub fn LockFile(self, offset: u64, num_bytes_to_lock: u64) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				kernel32::LockFile(
					self.0,
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
	pub fn ReadFile(self,
		buffer: &mut [u8],
		overlapped: Option<&mut OVERLAPPED>) -> WinResult<u32>
	{
		let mut bytes_read = u32::default();
		bool_to_winresult(
			unsafe {
				kernel32::ReadFile(
					self.0,
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
	pub fn SetEndOfFile(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel32::SetEndOfFile(self.0) })
	}

	/// [`SetFilePointerEx`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfilepointerex)
	/// method.
	pub fn SetFilePointerEx(self,
		distance_to_move: i64,
		move_method: co::FILE_STARTING_POINT) -> WinResult<i64>
	{
		let mut new_offset = i64::default();

		bool_to_winresult(
			unsafe {
				kernel32::SetFilePointerEx(
					self.0,
					distance_to_move,
					&mut new_offset,
					move_method.0,
				)
			},
		).map(|_| new_offset)
	}

	/// [`UnlockFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfile)
	/// method.
	pub fn UnlockFile(self,
		offset: u64, num_bytes_to_lock: u64) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				kernel32::UnlockFile(
					self.0,
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
	pub fn WriteFile(self,
		buffer: &[u8], overlapped: Option<&mut OVERLAPPED>) -> WinResult<u32>
	{
		let mut bytes_written = u32::default();

		bool_to_winresult(
			unsafe {
				kernel32::WriteFile(
					self.0,
					buffer.as_ptr() as _,
					buffer.len() as _,
					&mut bytes_written,
					overlapped.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				)
			},
		).map(|_| bytes_written)
	}
}
