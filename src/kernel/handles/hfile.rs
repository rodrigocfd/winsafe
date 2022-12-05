#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	BY_HANDLE_FILE_INFORMATION, GetLastError, HFILEMAP, HIDWORD, LODWORD,
	OVERLAPPED, SECURITY_ATTRIBUTES, SysResult, WString,
};
use crate::kernel::guard::HandleGuard;
use crate::kernel::privs::bool_to_sysresult;
use crate::prelude::Handle;

impl_handle! { HFILE: "kernel";
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
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait kernel_Hfile: Handle {
	/// [`CreateFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)
	/// static method.
	///
	/// The error code is also returned because it can carry information even if
	/// the file is successfully open.
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
		hfile_template: Option<&HFILE>) -> SysResult<(HandleGuard<HFILE>, co::ERROR)>
	{
		match HFILE(unsafe {
			kernel::ffi::CreateFileW(
				WString::from_str(file_name).as_ptr(),
				desired_access.0,
				share_mode.0,
				security_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				creation_disposition.0,
				flags_and_attrs.0,
				hfile_template.map_or(std::ptr::null_mut(), |h| h.0),
			) as _
		}) {
			HFILE::NULL => Err(GetLastError()),
			handle => Ok((HandleGuard { handle }, GetLastError())),
		}
	}

	/// [`CreateFileMapping`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-createfilemappingw)
	/// method.
	#[must_use]
	fn CreateFileMapping(&self,
		mapping_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		protect: co::PAGE,
		max_size: Option<u64>,
		mapping_name: Option<&str>) -> SysResult<HandleGuard<HFILEMAP>>
	{
		unsafe {
			kernel::ffi::CreateFileMappingFromApp(
				self.as_ptr(),
				mapping_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				protect.0,
				max_size.unwrap_or_default(),
				WString::from_opt_str(mapping_name).as_ptr(),
			).as_mut()
		}.map(|ptr| HandleGuard { handle: HFILEMAP(ptr) })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetFileInformationByHandle`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileinformationbyhandle)
	/// method.
	fn GetFileInformationByHandle(&self,
		fi: &mut BY_HANDLE_FILE_INFORMATION) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				kernel::ffi::GetFileInformationByHandle(
					self.as_ptr(), fi as *mut _ as _,
				)
			},
		)
	}

	/// [`GetFileSizeEx`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfilesizeex)
	/// method.
	#[must_use]
	fn GetFileSizeEx(&self) -> SysResult<usize> {
		let mut sz_buf = 0;
		match unsafe { kernel::ffi::GetFileSizeEx(self.as_ptr(), &mut sz_buf) } {
			0 => Err(GetLastError()),
			_ => Ok(sz_buf as _),
		}
	}

	/// [`GetFileType`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)
	/// method.
	#[must_use]
	fn GetFileType(&self) -> SysResult<co::FILE_TYPE> {
		match co::FILE_TYPE(unsafe { kernel::ffi::GetFileType(self.as_ptr()) }) {
			co::FILE_TYPE::UNKNOWN => match GetLastError() {
				co::ERROR::SUCCESS => Ok(co::FILE_TYPE::UNKNOWN), // actual unknown type
				err => Err(err),
			},
			ty => Ok(ty),
		}
	}

	/// [`LockFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfile)
	/// method.
	///
	/// Note that the guard returned by this method must be kept alive while the
	/// lock is used. This is necessary because, when the guard goes out of
	/// scope, it will automatically call
	/// [`UnlockFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfile).
	///
	/// In the example below, the guard is kept alive:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::HFILE;
	///
	/// let hfile: HFILE; // initialized somewhere
	/// # let hfile = HFILE::NULL;
	///
	/// let total_size = hfile.GetFileSizeEx()?;
	///
	/// let _lock = hfile.LockFile(0, total_size as _)?; // keep the returned guard alive
	///
	/// // file read/write operations...
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn LockFile(&self,
		offset: u64,
		num_bytes_to_lock: u64) -> SysResult<HfileLockGuard<'_, Self>>
	{
		bool_to_sysresult(
			unsafe {
				kernel::ffi::LockFile(
					self.as_ptr(),
					LODWORD(offset),
					HIDWORD(offset),
					LODWORD(num_bytes_to_lock),
					HIDWORD(num_bytes_to_lock),
				)
			},
		).map(|_| HfileLockGuard { hfile: self, offset, num_bytes_to_lock })
	}

	/// [`ReadFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)
	/// method.
	///
	/// Returns the number of bytes read.
	fn ReadFile(&self,
		buffer: &mut [u8],
		overlapped: Option<&mut OVERLAPPED>) -> SysResult<u32>
	{
		let mut bytes_read = u32::default();
		bool_to_sysresult(
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

	/// [`SetEndOfFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setendoffile)
	/// method.
	fn SetEndOfFile(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { kernel::ffi::SetEndOfFile(self.as_ptr()) })
	}

	/// [`SetFilePointerEx`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfilepointerex)
	/// method.
	fn SetFilePointerEx(&self,
		distance_to_move: i64,
		move_method: co::FILE_STARTING_POINT) -> SysResult<i64>
	{
		let mut new_offset = i64::default();

		bool_to_sysresult(
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

	/// [`WriteFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)
	/// method.
	///
	/// Returns the number of bytes written.
	fn WriteFile(&self,
		data: &[u8], overlapped: Option<&mut OVERLAPPED>) -> SysResult<u32>
	{
		let mut bytes_written = u32::default();

		bool_to_sysresult(
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

//------------------------------------------------------------------------------

/// RAII implementation for the [`HFILE`](crate::HFILE) lock which automatically
/// calls
/// [`UnlockFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfile)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub struct HfileLockGuard<'a, H>
	where H: kernel_Hfile,
{
	pub(crate) hfile: &'a H,
	pub(crate) offset: u64,
	pub(crate) num_bytes_to_lock: u64,
}

impl<'a, H> Drop for HfileLockGuard<'a, H>
	where H: kernel_Hfile,
{
	fn drop(&mut self) {
		unsafe {
			kernel::ffi::UnlockFile( // ignore errors
				self.hfile.as_ptr(),
				LODWORD(self.offset),
				HIDWORD(self.offset),
				LODWORD(self.num_bytes_to_lock),
				HIDWORD(self.num_bytes_to_lock),
			);
		}
	}
}
