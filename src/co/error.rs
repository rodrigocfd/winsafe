#![allow(non_snake_case)]

use crate::co;
use crate::ffi::kernel32;
use crate::handles::HLOCAL;
use crate::Utf16;

const_type!(ERROR, u32,
	"A Windows
	[system error code](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes)
	retrieved by
	[`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
	function, or an
	[`HRESULT`](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a).");
impl ERROR {
	/// Returns the last error code with
	/// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror).
	pub fn GetLastError() -> ERROR {
		unsafe { Self(kernel32::GetLastError()) }
	}

	/// Returns the textual description of the system error, by calling
	/// [`FormatMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
	/// function.
	pub fn FormatMessage(&self) -> String {
		unsafe {
			let mut lpBuf: *mut u16 = std::ptr::null_mut();
			let numChars = kernel32::FormatMessageW(
				u32::from(co::FORMAT_MESSAGE::ALLOCATE_BUFFER
					| co::FORMAT_MESSAGE::FROM_SYSTEM
					| co::FORMAT_MESSAGE::IGNORE_INSERTS),
				std::ptr::null_mut(),
				self.0,
				co::LANG::NEUTRAL.MAKELANGID(co::SUBLANG::DEFAULT),
				(&mut lpBuf as *mut *mut u16) as *mut u16,
				0,
				std::ptr::null_mut(),
			);
			let text16 = Utf16::from_utf16_nchars(lpBuf, numChars as usize);
			HLOCAL::from(lpBuf).LocalFree();
			text16.to_string()
		}
	}

	/// Sets this error as the last error code, by passing it to
	/// [`SetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror).
	pub fn SetLastError(&self) {
		unsafe { kernel32::SetLastError(self.0) }
	}

	const_val!(SUCCESS, 0);
	const_val!(INVALID_FUNCTION, 1);
	const_val!(FILE_NOT_FOUND, 2);
	const_val!(PATH_NOT_FOUND, 3);

	const_val!(S_OK, 0);
	const_val!(S_FALSE, 1);

	const_val!(RPC_E_CHANGED_MODE, 0x80010106);
}