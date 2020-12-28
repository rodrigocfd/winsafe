use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use crate::ffi::kernel32;

/// Stores a `Vec<u16>` buffer for an
/// [Unicode UTF-16](https://docs.microsoft.com/en-us/windows/win32/intl/unicode-in-the-windows-api)
/// string natively used by Windows.
///
/// Performs UTF-8 conversions and can be used as a buffer to low-level Win32
/// functions.
///
/// See an example in [`HWND::GetWindowText`](crate::HWND::GetWindowText).
pub struct Utf16 {
	char_vec: Option<Vec<u16>>,
}

impl Utf16 {
	/// Creates a new UTF-16 string from an optional `&str`, and stores it
	/// internally. If `s` is null, a null pointer will be stored.
	pub fn from_opt_str(val: Option<&str>) -> Utf16 {
		match val {
			Some(val) => Self::from_str(val),
			None => Self::new(),
		}
	}

	/// Creates a new UTF-16 string from an ordinary `&str`, and stores it
	/// internally.
	pub fn from_str(val: &str) -> Utf16 {
		Self {
			char_vec: Some(
				OsStr::new(val)
					.encode_wide()
					.chain(std::iter::once(0)) // append terminating null
					.collect::<Vec<u16>>(),
			),
		}
	}

	/// Creates a new UTF-16 string by copying from a non-null-terminated buffer,
	/// specifying the number of existing chars.
	pub fn from_utf16_nchars(src: *const u16, num_chars: usize) -> Utf16 {
		if src.is_null() {
			Self::new()
		} else {
			let mut me = Self::new_alloc_buffer(num_chars + 1); // add room for terminating null
			let vec_ref = &mut me.char_vec.as_mut().unwrap();

			unsafe {
				std::ptr::copy_nonoverlapping(
					src,
					vec_ref.as_mut_ptr(),
					num_chars, // won't copy terminating null
				);
				vec_ref.set_len(num_chars + 1); // leave room for terminating null
			}
			me
		}
	}

	/// Creates a new UTF-16 string by copying from a null-terminated buffer.
	pub fn from_utf16_nullt(src: *const u16) -> Utf16 {
		if src.is_null() {
			Self::new()
		} else {
			let num_chars = unsafe { kernel32::lstrlenW(src) as usize };
			Self::from_utf16_nchars(src, num_chars)
		}
	}

	/// Creates a new UTF-16 string by copying from a slice.
	pub fn from_utf16_slice(src: &[u16]) -> Utf16 {
		Self::from_utf16_nchars(&src[0], src.len())
	}

	/// Creates a new, empty UTF-16 buffer.
	pub fn new() -> Utf16 {
		Self {
			char_vec: None,
		}
	}

	/// Creates a new UTF-16 buffer allocated with an specific length. All UTF-16
	/// chars will be set to zero.
	pub fn new_alloc_buffer(num_chars: usize) -> Utf16 {
		let mut me = Self::new();
		me.realloc_buffer(num_chars);
		me
	}

	/// Returns a `LPWSTR` mut pointer to the internal UTF-16 string buffer, to
	/// be passed to native Win32 functions. This is useful to receive strings.
	///
	/// # Panics
	///
	/// Panics if the buffer wasn't previously allocated. Be sure to alloc enough
	/// room, otherwise a buffer overrun may occur.
	pub unsafe fn as_mut_ptr(&mut self) -> *mut u16 {
		match self.char_vec.as_mut() {
			Some(vec_ref) => vec_ref.as_mut_ptr(),
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
		match self.char_vec.as_ref() {
			Some(vec_ref) => vec_ref.as_ptr(),
			None => std::ptr::null(),
		}
	}

	/// Returns a slice to the internal `u16` buffer. This is useful to receive
	/// strings.
	///
	/// # Panics
	///
	/// Panics if the buffer wasn't previously allocated. Be sure to alloc enough
	/// room, otherwise a buffer overrun may occur.
	pub fn as_mut_slice(&mut self) -> &mut [u16] {
		match self.char_vec.as_mut() {
			Some(vec_ref) => &mut vec_ref[..],
			None => panic!("Trying to use an unallocated Utf16 buffer."),
		}
	}

	/// Returns a slice to the internal UTF-16 string buffer.
	///
	/// # Panics
	///
	/// Panics if the buffer wasn't previously allocated. Make sure the `Utf16`
	/// object outlives the function call, otherwise it will point to an invalid
	/// memory location.
	pub fn as_slice(&mut self) -> &[u16] {
		match self.char_vec.as_ref() {
			Some(vec_ref) => &vec_ref[..],
			None => panic!("Trying to use an unallocated Utf16 buffer."),
		}
	}

	/// Returns the size of the allocated internal buffer.
	pub fn buffer_size(&self) -> usize {
		match self.char_vec.as_ref() {
			Some(vec_ref) => vec_ref.len(),
			None => 0, // not allocated yet
		}
	}

	/// Fills the available buffer with zero values.
	pub fn fill_with_zero(&mut self) {
		if let Some(vec_ref) = self.char_vec.as_mut() {
			for wchar in vec_ref {
				*wchar = 0;
			}
		}
	}

	/// Returns true if the internal buffer is storing a null string pointer.
	pub fn is_null(&self) -> bool {
		self.char_vec.is_none()
	}

	/// Wrapper to
	/// [`lstrlen`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lstrlenw).
	///
	/// Returns the number of `u16` characters stored in the internal buffer, not
	/// counting the terminating null.
	pub fn len(&self) -> usize {
		match self.char_vec.as_ref() {
			Some(vec_ref) => unsafe { kernel32::lstrlenW(vec_ref.as_ptr()) as usize },
			None => 0,
		}
	}

	/// Resizes the internal buffer, to be used as a buffer for native Win32
	/// functions. All UTF-16 chars will be set to zero.
	///
	/// If the new size is zero, the internal buffer is deallocated.
	///
	/// **Note:** The internal memory can move after a realloc, so if you're
	/// using the internal buffer somewhere, update it by calling `as_const_ptr`
	/// or `as_mut_buffer` again.
	pub fn realloc_buffer(&mut self, new_size: usize) {
		if new_size == 0 {
			self.char_vec = None; // dealloc
		} else {
			if self.char_vec.is_none() {
				self.char_vec = Some(Vec::default()); // create if not yet; default Vec is empty
			}

			let vec_ref = &mut self.char_vec.as_mut().unwrap();
			vec_ref.resize(new_size, 0); // fill with nulls
		}
	}

	/// Converts into `String`. An internal null pointer will simply be converted
	/// into an empty string.
	pub fn to_string(&self) -> String {
		match self.char_vec.as_ref() {
			Some(vec_ref) => {
				let slice16 = &vec_ref[..self.len()]; // without terminating null
				String::from_utf16(slice16).unwrap_or_default()
			},
			None => String::default(),
		}
	}
}