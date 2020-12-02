use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

/// Stores a `Vec<u16>`
/// [Windows UTF-16](https://docs.microsoft.com/en-us/windows/win32/intl/unicode-in-the-windows-api)
/// string, which can perform UTF-8 conversions and can be used as a buffer to
/// native Win32 functions.
#[derive(Default)]
pub struct Utf16(pub Option<Vec<u16>>);

impl Utf16 {
	/// Returns a `LPWSTR` mut pointer to the internal UTF-16 string buffer, to
	/// be passed to native Win32 functions. This is useful to receive strings.
	///
	/// **Note:** Will panic if the buffer wasn't previously allocated. Be sure
	/// to alloc enough room, otherwise a buffer overrun may occur.
	pub unsafe fn as_mut_ptr(&mut self) -> *mut u16 {
		match self.0.as_mut() {
			Some(buf) => buf.as_mut_ptr(),
			None => panic!("Trying to use an unallocated Utf16 buffer."),
		}
	}

	/// Returns a `LPCWSTR` const pointer to the internal UTF-16 string buffer,
	/// to be passed to native Win32 functions.
	///
	/// **Note:** Returns a null pointer if the buffer wasn't previously
	/// allocated. Make sure the `Utf16` object outlives the function call,
	/// otherwise it will point to an invalid memory location.
	pub unsafe fn as_ptr(&self) -> *const u16 {
		match self.0.as_ref() {
			Some(buf) => buf.as_ptr(),
			None => std::ptr::null(),
		}
	}

	/// Creates an UTF-16 string from an ordinary `String`, and stores it
	/// internally.
	pub fn from_str(val: &str) -> Utf16 {
		Utf16(Some(
			OsStr::new(val)
				.encode_wide()
				.chain(std::iter::once(0)) // append terminating null
				.collect::<Vec<u16>>(),
		))
	}
}