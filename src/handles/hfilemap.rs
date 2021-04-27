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
	/// [`UnmapViewOfFile`](crate::HFILEMAPADDR::UnmapViewOfFile) call.
	pub fn MapViewOfFile(self,
		desiredAccess: co::FILE_MAP,
		offset: u64,
		numberOfBytesToMap: i64) -> WinResult<HFILEMAPADDR>
	{
		unsafe {
			kernel32::MapViewOfFile(
				self.ptr,
				desiredAccess.0,
				HIDWORD(offset),
				LODWORD(offset),
				numberOfBytesToMap,
			).as_mut()
		}.map(|ptr| HFILEMAPADDR { ptr }).ok_or_else(|| GetLastError())
	}
}

//------------------------------------------------------------------------------

handle_type! {
	/// Address of a
	/// [mapped view](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile).
	/// Originally just an `LPVOID`.
	HFILEMAPADDR
}

impl HFILEMAPADDR {
	/// [`UnmapViewOfFile`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-unmapviewoffile)
	/// method.
	pub fn UnmapViewOfFile(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel32::UnmapViewOfFile(self.ptr) })
	}
}
