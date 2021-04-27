#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::funcs::{GetLastError, HIDWORD, LODWORD};
use crate::handles::HFILEMAP;
use crate::privs::{bool_to_winresult, ptr_as_opt, ref_as_pvoid};
use crate::structs::{
	BY_HANDLE_FILE_INFORMATION,
	OVERLAPPED,
	SECURITY_ATTRIBUTES,
};
use crate::WString;

handle_type! {
	/// Handle to a
	/// [file](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hfile).
	/// Originally just a `HANDLE`.
	HFILE
}

impl HFILE {
	/// [`CloseHandle`](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
	/// method.
	pub fn CloseHandle(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel32::CloseHandle(self.ptr) })
	}

	/// [`CreateFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)
	/// static method.
	///
	/// **Note:** Must be paired with a
	/// [`CloseHandle`](crate::HFILE::CloseHandle) call.
	///
	/// # Examples
	///
	/// Opening an existing file as read-only:
	///
	/// ```rust,ignore
	/// use winsafe::{co, HFILE};
	///
	/// let hfile = HFILE::CreateFile(
	///     "C:\\Temp\\something.txt",
	///     co::GENERIC::READ,
	///     co::FILE_SHARE::READ,
	///     None,
	///     co::DISPOSITION::OPEN_EXISTING,
	///     co::FILE_ATTRIBUTE::NORMAL,
	///     None,
	/// ).unwrap();
	///
	/// hfile.CloseHandle().unwrap();
	/// ```
	///
	/// Opening a file for read and write. If the file doesn't exist, create it:
	///
	/// ```rust,ignore
	/// use winsafe::{co, HFILE};
	///
	/// let hfile = w::HFILE::CreateFile(
	///     "C:\\Temp\\something.txt",
	///     co::GENERIC::READ | co::GENERIC::WRITE,
	///     co::FILE_SHARE::NONE,
	///     None,
	///     co::DISPOSITION::OPEN_ALWAYS,
	///     co::FILE_ATTRIBUTE::NORMAL,
	///     None,
	/// ).unwrap();
	///
	/// hfile.CloseHandle().unwrap();
	/// ```
	pub fn CreateFile(
		lpFileName: &str,
		dwDesiredAccess: co::GENERIC,
		dwShareMode: co::FILE_SHARE,
		lpSecurityAttributes: Option<&mut SECURITY_ATTRIBUTES>,
		dwCreationDisposition: co::DISPOSITION,
		dwFlagsAndAttributes: co::FILE_ATTRIBUTE,
		hTemplateFile: Option<HFILE>) -> WinResult<HFILE>
	{
		ptr_as_opt(
			unsafe {
				kernel32::CreateFileW(
					WString::from_str(lpFileName).as_ptr(),
					dwDesiredAccess.0,
					dwShareMode.0,
					lpSecurityAttributes.map_or(std::ptr::null_mut(), |lp| ref_as_pvoid(lp)),
					dwCreationDisposition.0,
					dwFlagsAndAttributes.0,
					hTemplateFile.map_or(std::ptr::null_mut(), |h| h.ptr),
				)
			},
		).map(|ptr| Self { ptr }).ok_or_else(|| GetLastError())
	}

	/// [`CreateFileMapping`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-createfilemappingw)
	/// method.
	///
	/// **Note:** Must be paired with a
	/// [`CloseHandle`](crate::HFILEMAP::CloseHandle) call.
	pub fn CreateFileMapping(self,
		lpFileMappingAttributes: Option<&mut SECURITY_ATTRIBUTES>,
		flProtect: co::PAGE,
		maximumSize: u64,
		lpName: Option<&str>) -> WinResult<HFILEMAP>
	{
		ptr_as_opt(
			unsafe {
				kernel32::CreateFileMappingW(
					self.ptr,
					lpFileMappingAttributes.map_or(std::ptr::null_mut(), |lp| ref_as_pvoid(lp)),
					flProtect.0,
					HIDWORD(maximumSize),
					LODWORD(maximumSize),
					lpName.map_or(std::ptr::null(), |s| WString::from_str(s).as_ptr()),
				)
			},
		).map(|ptr| HFILEMAP { ptr }).ok_or_else(|| GetLastError())
	}

	/// [`GetFileInformationByHandle`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileinformationbyhandle)
	/// method.
	pub fn GetFileInformationByHandle(self,
		lpFileInformation: &mut BY_HANDLE_FILE_INFORMATION) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				kernel32::GetFileInformationByHandle(
					self.ptr,
					ref_as_pvoid(lpFileInformation),
				)
			},
		)
	}

	/// [`GetFileSizeEx`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfilesizeex)
	/// method.
	pub fn GetFileSizeEx(self) -> WinResult<i64> {
		let mut ibuf = 0;
		match unsafe { kernel32::GetFileSizeEx(self.ptr, &mut ibuf) } {
			0 => Err(GetLastError()),
			_ => Ok(ibuf),
		}
	}

	/// [`GetFileType`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)
	/// method.
	pub fn GetFileType(self) -> WinResult<co::FILE_TYPE> {
		match co::FILE_TYPE(unsafe { kernel32::GetFileType(self.ptr) }) {
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
	/// [`UnlockFile`](crate::HFILE::UnlockFile) call.
	pub fn LockFile(self, offset: u64, numBytesToLock: u64) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				kernel32::LockFile(
					self.ptr,
					LODWORD(offset),
					HIDWORD(offset),
					LODWORD(numBytesToLock),
					HIDWORD(numBytesToLock),
				)
			},
		)
	}

	/// [`ReadFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)
	/// method.
	pub fn ReadFile(self,
		numBytesToRead: u32,
		lpOverlapped: Option<&mut OVERLAPPED>) -> WinResult<Vec<u8>>
	{
		let mut buf = vec![0; numBytesToRead as _];
		let mut bytesRead: u32 = 0;

		bool_to_winresult(
			unsafe {
				kernel32::ReadFile(
					self.ptr,
					buf.as_mut_ptr() as *mut _,
					numBytesToRead,
					&mut bytesRead,
					lpOverlapped.map_or(std::ptr::null_mut(), |lp| ref_as_pvoid(lp)),
				)
			},
		).map(|_| buf)
	}

	/// [`SetEndOfFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setendoffile)
	/// method.
	pub fn SetEndOfFile(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel32::SetEndOfFile(self.ptr) })
	}

	/// [`SetFilePointerEx`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfilepointerex)
	/// method.
	pub fn SetFilePointerEx(self,
		liDistanceToMove: i64,
		dwMoveMethod: co::FILE_STARTING_POINT) -> WinResult<i64>
	{
		let mut newOffset: i64 = 0;

		bool_to_winresult(
			unsafe {
				kernel32::SetFilePointerEx(
					self.ptr,
					liDistanceToMove,
					&mut newOffset,
					dwMoveMethod.0,
				)
			},
		).map(|_| newOffset)
	}

	/// [`UnlockFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfile)
	/// method.
	pub fn UnlockFile(self, offset: u64, numBytesToLock: u64) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				kernel32::UnlockFile(
					self.ptr,
					LODWORD(offset),
					HIDWORD(offset),
					LODWORD(numBytesToLock),
					HIDWORD(numBytesToLock),
				)
			},
		)
	}

	/// [`WriteFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)
	/// method.
	pub fn WriteFile(self,
		buffer: &[u8],
		lpOverlapped: Option<&mut OVERLAPPED>) -> WinResult<u32>
	{
		let mut bytesWritten: u32 = 0;

		bool_to_winresult(
			unsafe {
				kernel32::WriteFile(
					self.ptr,
					buffer.as_ptr() as *const _,
					buffer.len() as _,
					&mut bytesWritten,
					lpOverlapped.map_or(std::ptr::null_mut(), |lp| ref_as_pvoid(lp)),
				)
			},
		).map(|_| bytesWritten)
	}
}
