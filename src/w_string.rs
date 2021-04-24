use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use crate::ffi::kernel32;

/// Stores a `Vec<u16>` buffer for an
/// [Unicode UTF-16](https://docs.microsoft.com/en-us/windows/win32/intl/unicode-in-the-windows-api)
/// wide string natively used by Windows.
///
/// Performs UTF-8 conversions and can be used as a buffer to low-level Win32
/// functions.
///
/// See an example in [`HWND::GetWindowText`](crate::HWND::GetWindowText).
pub struct WString {
	char_vec: Option<Vec<u16>>,
}

impl Default for WString {
	fn default() -> Self {
		Self { char_vec: None }
	}
}

impl WString {
	/// Creates a new UTF-16 string from an optional `&str`, and stores it
	/// internally. If `s` is null, a null pointer will be stored.
	pub fn from_opt_str(val: Option<&str>) -> WString {
		match val {
			Some(val) => Self::from_str(val),
			None => Self::default(),
		}
	}

	/// Creates a new UTF-16 string from an ordinary `&str`, and stores it
	/// internally, appending a terminating null.
	pub fn from_str(val: &str) -> WString {
		Self {
			char_vec: Some(
				OsStr::new(val)
					.encode_wide()
					.chain(std::iter::once(0))
					.collect::<Vec<u16>>(),
			),
		}
	}

	/// Creates a new UTF-16 string by copying from a non-null-terminated
	/// buffer, specifying the number of existing chars. A terminating null will
	/// be appended.
	pub fn from_wchars_count(src: *const u16, num_chars: usize) -> WString {
		if src.is_null() {
			Self::default()
		} else {
			let mut me = Self::new_alloc_buffer(num_chars + 1); // add room for terminating null
			let vec_ref = &mut me.char_vec.as_mut().unwrap();

			unsafe {
				std::ptr::copy_nonoverlapping(
					src,
					vec_ref.as_mut_ptr(),
					num_chars, // copy the exact number of chars, no terminating null
				);
			}
			me
		}
	}

	/// Creates a new UTF-16 string by copying from a null-terminated buffer.
	/// The terminating null will be appended.
	pub fn from_wchars_nullt(src: *const u16) -> WString {
		if src.is_null() {
			Self::default()
		} else {
			let num_chars = unsafe { kernel32::lstrlenW(src) as usize };
			Self::from_wchars_count(src, num_chars)
		}
	}

	/// Creates a new UTF-16 string by copying from a slice.
	pub fn from_wchars_slice(src: &[u16]) -> WString {
		Self::from_wchars_count(src.as_ptr(), src.len())
	}

	/// Creates a new UTF-16 buffer allocated with an specific length. All
	/// UTF-16 chars will be set to zero.
	pub fn new_alloc_buffer(num_chars: usize) -> WString {
		let mut me = Self::default();
		me.realloc_buffer(num_chars);
		me
	}

	/// Returns a `LPWSTR` mut pointer to the internal UTF-16 string buffer, to
	/// be passed to native Win32 functions. This is useful to receive strings.
	///
	/// # Panics
	///
	/// Panics if the buffer wasn't previously allocated. Be sure to alloc
	/// enough room, otherwise a buffer overrun may occur.
	pub unsafe fn as_mut_ptr(&mut self) -> *mut u16 {
		match self.char_vec.as_mut() {
			Some(vec_ref) => vec_ref.as_mut_ptr(),
			None => panic!("Trying to use an unallocated WString buffer."),
		}
	}

	/// Returns a `LPCWSTR` const pointer to the internal UTF-16 string buffer,
	/// to be passed to native Win32 functions.
	///
	/// **Note:** Returns a null pointer if the buffer wasn't previously
	/// allocated. Make sure the `WString` object outlives the function call,
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
	/// Panics if the buffer wasn't previously allocated. Be sure to alloc
	/// enough room, otherwise a buffer overrun may occur.
	pub fn as_mut_slice(&mut self) -> &mut [u16] {
		match self.char_vec.as_mut() {
			Some(vec_ref) => &mut vec_ref[..],
			None => panic!("Trying to use an unallocated WString buffer."),
		}
	}

	/// Returns a slice to the internal UTF-16 string buffer.
	///
	/// # Panics
	///
	/// Panics if the buffer wasn't previously allocated. Make sure the
	/// `WString` object outlives the function call, otherwise it will point to
	/// an invalid memory location.
	pub fn as_slice(&mut self) -> &[u16] {
		match self.char_vec.as_ref() {
			Some(vec_ref) => &vec_ref[..],
			None => panic!("Trying to use an unallocated WString buffer."),
		}
	}

	/// Returns the size of the allocated internal buffer.
	pub fn buffer_size(&self) -> usize {
		match self.char_vec.as_ref() {
			Some(vec_ref) => vec_ref.len(),
			None => 0, // not allocated yet
		}
	}

	/// Copies the content into the external buffer pointed by `dest`. A
	/// terminating null will always be appended.
	///
	/// # Panics
	///
	/// Panics if `dest` is null or if `len` is zero. If `len` is 1, the buffer
	/// will receive a single null char.
	pub fn copy_to_pointer(&self, dest: *mut u16, len: usize) {
		if dest.is_null() {
			panic!("Destination buffer cannot be null.");
		} else if len == 0 {
			panic!("Destination buffer cannot have zero length");
		}

		let mut dest_slice = unsafe { std::slice::from_raw_parts_mut(dest, len) };
		self.copy_to_slice(&mut dest_slice);
	}

	/// Copies the content into an external buffer. A terminating null will
	/// always be appended.
	///
	/// # Panics
	///
	/// Panics if `dest` has zero length. If length is 1, the buffer will
	/// receive a single null char.
	pub fn copy_to_slice(&self, dest: &mut [u16]) {
		if dest.is_empty() {
			panic!("Destination buffer cannot have zero length");
		}

		if let Some(vec_ref) = self.char_vec.as_ref() {
			unsafe {
				std::ptr::copy_nonoverlapping(
					vec_ref.as_ptr(),
					dest.as_mut_ptr(),
					std::cmp::min(vec_ref.len(), dest.len()), // copy the exact number of chars, no terminating null
				);
			}
			*dest.last_mut().unwrap() = 0; // terminating null
		}
	}

	/// Fills the entire buffer with zero values.
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
	/// Returns the number of `u16` characters stored in the internal buffer,
	/// not counting the terminating null.
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
