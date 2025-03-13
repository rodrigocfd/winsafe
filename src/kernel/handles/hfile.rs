#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};
use crate::prelude::*;

handle! { HFILE;
	/// Handle to a
	/// [file](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hfile).
	/// Originally just a `HANDLE`.
	///
	/// Unless you need something specific, consider using the
	/// [`File`](crate::File) high-level abstraction.
}

impl kernel_Hfile for HFILE {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HFILE`](crate::HFILE).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hfile: Handle {
	/// [`CreateFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)
	/// function.
	///
	/// The error code is also returned because it can carry information even if
	/// the file is successfully open.
	///
	/// Unless you need something specific, consider using the
	/// [`File`](crate::File) high-level abstraction.
	///
	/// # Examples
	///
	/// Opening an existing file as read-only:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let (hfile, status) = w::HFILE::CreateFile(
	///     "C:\\Temp\\test.txt",
	///     co::GENERIC::READ,
	///     Some(co::FILE_SHARE::READ),
	///     None,
	///     co::DISPOSITION::OPEN_EXISTING,
	///     co::FILE_ATTRIBUTE::NORMAL,
	///     None,
	///     None,
	///     None,
	/// )?;
	/// # w::SysResult::Ok(())
	/// ```
	///
	/// Opening a file for read and write. If the file doesn't exist, create it:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let (hfile, status) = w::HFILE::CreateFile(
	///     "C:\\Temp\\test.txt",
	///     co::GENERIC::READ | co::GENERIC::WRITE,
	///     None,
	///     None,
	///     co::DISPOSITION::OPEN_ALWAYS,
	///     co::FILE_ATTRIBUTE::NORMAL,
	///     None,
	///     None,
	///     None,
	/// )?;
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	fn CreateFile(
		file_name: &str,
		desired_access: co::GENERIC,
		share_mode: Option<co::FILE_SHARE>,
		security_attributes: Option<&mut SECURITY_ATTRIBUTES>,
		creation_disposition: co::DISPOSITION,
		attributes: co::FILE_ATTRIBUTE,
		flags: Option<co::FILE_FLAG>,
		security: Option<co::FILE_SECURITY>,
		hfile_template: Option<&HFILE>,
	) -> SysResult<(CloseHandleGuard<HFILE>, co::ERROR)> {
		unsafe {
			match HFILE(ffi::CreateFileW(
				WString::from_str(file_name).as_ptr(),
				desired_access.raw(),
				share_mode.unwrap_or_default().raw(),
				security_attributes.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				creation_disposition.raw(),
				attributes.raw()
					| flags.unwrap_or_default().raw()
					| security.map_or(0, |s| SECURITY_SQOS_PRESENT | s.raw()),
				hfile_template.map_or(std::ptr::null_mut(), |h| h.ptr()),
			) as _)
			{
				HFILE::NULL | HFILE::INVALID => Err(GetLastError()),
				handle => Ok((CloseHandleGuard::new(handle), GetLastError())),
			}
		}
	}

	/// [`CreateFileMapping`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-createfilemappingw)
	/// function.
	///
	/// Unless you need something specific, consider using the
	/// [`FileMapped`](crate::FileMapped) high-level abstraction.
	#[must_use]
	fn CreateFileMapping(
		&self,
		mapping_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		protect: co::PAGE,
		max_size: Option<u64>,
		mapping_name: Option<&str>,
	) -> SysResult<CloseHandleGuard<HFILEMAP>> {
		unsafe {
			ptr_to_sysresult_handle(ffi::CreateFileMappingFromApp(
				self.ptr(),
				mapping_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				protect.raw(),
				max_size.unwrap_or_default(),
				WString::from_opt_str(mapping_name).as_ptr(),
			))
			.map(|h| CloseHandleGuard::new(h))
		}
	}

	/// [`GetFileInformationByHandle`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileinformationbyhandle)
	/// function.
	fn GetFileInformationByHandle(&self) -> SysResult<BY_HANDLE_FILE_INFORMATION> {
		let mut fi = BY_HANDLE_FILE_INFORMATION::default();
		bool_to_sysresult(unsafe {
			ffi::GetFileInformationByHandle(self.ptr(), &mut fi as *mut _ as _)
		})
		.map(|_| fi)
	}

	/// [`GetFileSizeEx`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfilesizeex)
	/// function.
	#[must_use]
	fn GetFileSizeEx(&self) -> SysResult<u64> {
		let mut sz_buf = i64::default();
		bool_to_sysresult(unsafe { ffi::GetFileSizeEx(self.ptr(), &mut sz_buf) })
			.map(|_| sz_buf as _)
	}

	/// [`GetFileTime`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletime)
	/// function.
	///
	/// Returns, respectively:
	/// 1. creation time;
	/// 2. last access time;
	/// 3. last write time.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hfile: w::HFILE; // initialized somewhere
	/// # let hfile = w::HFILE::NULL;
	///
	/// let (creation, last_access, last_write) = hfile.GetFileTime()?;
	/// # w::SysResult::Ok(())
	/// ```
	fn GetFileTime(&self) -> SysResult<(FILETIME, FILETIME, FILETIME)> {
		let (mut creation, mut last_access, mut last_write) =
			(FILETIME::default(), FILETIME::default(), FILETIME::default());

		bool_to_sysresult(unsafe {
			ffi::GetFileTime(
				self.ptr(),
				&mut creation as *mut _ as _,
				&mut last_access as *mut _ as _,
				&mut last_write as *mut _ as _,
			)
		})
		.map(|_| (creation, last_access, last_write))
	}

	/// [`GetFileType`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)
	/// function.
	#[must_use]
	fn GetFileType(&self) -> SysResult<co::FILE_TYPE> {
		match unsafe { co::FILE_TYPE::from_raw(ffi::GetFileType(self.ptr())) } {
			co::FILE_TYPE::UNKNOWN => match GetLastError() {
				co::ERROR::SUCCESS => Ok(co::FILE_TYPE::UNKNOWN), // actual unknown type
				err => Err(err),
			},
			ty => Ok(ty),
		}
	}

	/// [`LockFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfile)
	/// function.
	///
	/// In the original C implementation, you must call
	/// [`UnlockFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-unlockfile)
	/// as a cleanup operation.
	///
	/// Here, the cleanup is performed automatically, because `LockFile` returns
	/// an [`UnlockFileGuard`](crate::guard::UnlockFileGuard), which
	/// automatically calls `UnlockFile` when the guard goes out of scope. You
	/// must, however, keep the guard alive, otherwise the cleanup will be
	/// performed right away.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hfile: w::HFILE; // initialized somewhere
	/// # let hfile = w::HFILE::NULL;
	///
	/// let total_size = hfile.GetFileSizeEx()?;
	///
	/// let _lock_guard = hfile.LockFile(0, total_size as _)?; // keep guard alive
	///
	/// // file read/write operations...
	///
	/// // UnlockFile() called automatically
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	fn LockFile(
		&self,
		offset: u64,
		num_bytes_to_lock: u64,
	) -> SysResult<UnlockFileGuard<'_, Self>> {
		unsafe {
			bool_to_sysresult(ffi::LockFile(
				self.ptr(),
				LODWORD(offset),
				HIDWORD(offset),
				LODWORD(num_bytes_to_lock),
				HIDWORD(num_bytes_to_lock),
			))
			.map(|_| UnlockFileGuard::new(self, offset, num_bytes_to_lock))
		}
	}

	/// [`ReadFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)
	/// function.
	///
	/// Reads at most `buffer.len()` bytes from the file, starting at the
	/// current file pointer offset. Returns how many bytes were actually read.
	/// The file pointer is then incremented by the number of bytes read.
	///
	/// Note that asynchronous reading – which use the
	/// [`OVERLAPPED`](crate::OVERLAPPED) struct – is not currently supported by
	/// this method, because the buffer must remain untouched until the async
	/// operation is complete, thus making the method unsound.
	fn ReadFile(&self, buffer: &mut [u8]) -> SysResult<u32> {
		let mut bytes_read = u32::default();
		bool_to_sysresult(unsafe {
			ffi::ReadFile(
				self.ptr(),
				buffer.as_mut_ptr() as _,
				buffer.len() as _,
				&mut bytes_read,
				std::ptr::null_mut(),
			)
		})
		.map(|_| bytes_read)
	}

	/// [`SetEndOfFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setendoffile)
	/// function.
	fn SetEndOfFile(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::SetEndOfFile(self.ptr()) })
	}

	/// [`SetFilePointerEx`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfilepointerex)
	/// function.
	fn SetFilePointerEx(
		&self,
		distance_to_move: i64,
		move_method: co::FILE_STARTING_POINT,
	) -> SysResult<i64> {
		let mut new_offset = i64::default();

		bool_to_sysresult(unsafe {
			ffi::SetFilePointerEx(self.ptr(), distance_to_move, &mut new_offset, move_method.raw())
		})
		.map(|_| new_offset)
	}

	/// [`SetFileTime`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfiletime)
	/// function.
	fn SetFileTime(
		&self,
		creation_time: Option<&FILETIME>,
		last_access_time: Option<&FILETIME>,
		last_write_time: Option<&FILETIME>,
	) -> SysResult<()> {
		bool_to_sysresult(unsafe {
			ffi::SetFileTime(
				self.ptr(),
				creation_time.map_or(std::ptr::null(), |p| p as *const _ as _),
				last_access_time.map_or(std::ptr::null(), |p| p as *const _ as _),
				last_write_time.map_or(std::ptr::null(), |p| p as *const _ as _),
			)
		})
	}

	/// [`WriteFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)
	/// function.
	///
	/// Returns the number of bytes written.
	///
	/// Note that asynchronous writing – which use the
	/// [`OVERLAPPED`](crate::OVERLAPPED) struct – is not currently supported by
	/// this method, because the buffer must remain untouched until the async
	/// operation is complete, thus making the method unsound.
	fn WriteFile(&self, data: &[u8]) -> SysResult<u32> {
		let mut bytes_written = u32::default();

		bool_to_sysresult(unsafe {
			ffi::WriteFile(
				self.ptr(),
				vec_ptr(data) as _,
				data.len() as _,
				&mut bytes_written,
				std::ptr::null_mut(),
			)
		})
		.map(|_| bytes_written)
	}
}
