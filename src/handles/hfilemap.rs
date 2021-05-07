#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::funcs::{GetLastError, HIDWORD, LODWORD};
use crate::privs::bool_to_winresult;

handle_type! {
	/// Handle to a
	/// [file mapping](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-createfilemappingw).
	/// Originally just a `HANDLE`.
	HFILEMAP
}

impl HFILEMAP {
	/// [`CloseHandle`](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
	/// method.
	pub fn CloseHandle(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel32::CloseHandle(self.ptr) })
	}

	/// [`MapViewOfFile`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`UnmapViewOfFile`](crate::HFILEMAPVIEW::UnmapViewOfFile) call.
	pub fn MapViewOfFile(self,
		desiredAccess: co::FILE_MAP,
		offset: u64,
		numberOfBytesToMap: Option<i64>) -> WinResult<HFILEMAPVIEW>
	{
		unsafe {
			kernel32::MapViewOfFile(
				self.ptr,
				desiredAccess.0,
				HIDWORD(offset),
				LODWORD(offset),
				numberOfBytesToMap.unwrap_or_default(),
			).as_mut()
		}.map(|ptr| HFILEMAPVIEW { ptr })
			.ok_or_else(|| GetLastError())
	}
}

//------------------------------------------------------------------------------

handle_type! {
	/// Address of a
	/// [mapped view](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile).
	/// Originally just an `LPVOID`.
	HFILEMAPVIEW
}

impl HFILEMAPVIEW {
	/// [`UnmapViewOfFile`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-unmapviewoffile)
	/// method.
	pub fn UnmapViewOfFile(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel32::UnmapViewOfFile(self.ptr) })
	}

	/// Returns a slice representing the mapped memory. You can modify the
	/// contents.
	///
	/// **Note:** You should call this method only if the file has write access.
	pub fn as_mut_slice<'a>(self, len: usize) -> &'a mut [u8] {
		unsafe { std::slice::from_raw_parts_mut(self.ptr as _, len) }
	}

	/// Returns a slice representing the mapped memory.
	///
	/// # Examples
	///
	/// Reading the contents of a file into a string:
	///
	/// ```rust,ignore
	/// use winsafe::{co, HFILE};
	///
	/// let hfile = HFILE::CreateFile(
	///     "C:\\Temp\\test.txt",
	///     co::GENERIC::READ,
	///     co::FILE_SHARE::READ,
	///     None,
	///     co::DISPOSITION::OPEN_EXISTING,
	///     co::FILE_ATTRIBUTE::NORMAL,
	///     None,
	/// ).unwrap();
	///
	/// let hmap = hfile.CreateFileMapping(
	///     None,
	///     co::PAGE::READONLY,
	///     None,
	///     None,
	/// ).unwrap();
	///
	/// let view = hmap.MapViewOfFile(co::FILE_MAP::READ, 0, None).unwrap();
	///
	/// let slice = view.as_slice(hfile.GetFileSizeEx().unwrap());
	/// let text = std::str::from_utf8(slice).unwrap();
	///
	/// view.UnmapViewOfFile().unwrap();
	/// hmap.CloseHandle().unwrap();
	/// hfile.CloseHandle().unwrap();
	///
	/// println!("{}", text);
	/// ```
	pub fn as_slice<'a>(self, len: usize) -> &'a [u8] {
		unsafe { std::slice::from_raw_parts(self.ptr as _, len) }
	}
}
