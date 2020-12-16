//! Internal value definitions used within the library only.

use crate::Utf16;

pub const LF_FACESIZE: usize = 0;

/// Transforms a pointer into an option, which is None if the pointer is null.
///
/// https://stackoverflow.com/q/65144143/6923555
macro_rules! ptr_as_opt {
	($ptr:expr) => {
		unsafe {
			if $ptr.is_null() {
				None
			} else {
				Some($ptr)
			}
		}
	};
}

/// Converts a reference to a const void pointer.
pub fn const_void<T>(val: &T) -> *const std::ffi::c_void {
	val as *const T as *const std::ffi::c_void
}
/// Converts a mut reference to a mut void pointer.
pub fn mut_void<T>(val: &mut T) -> *mut std::ffi::c_void {
	val as *mut T as *mut std::ffi::c_void
}

/// Parses a null-delimited multi-string, which must terminate with two nulls.
pub fn parse_multi_z_str(src: *const u16) -> Vec<String> {
	let mut src = src;
	let mut strings = Vec::new();
	let mut i = 0;

	loop {
		if unsafe { *src.add(i) } == 0 {
			let slice = unsafe { std::slice::from_raw_parts(src, i) };
			if slice.is_empty() {
				break;
			}
			strings.push(Utf16::from_utf16_slice(slice).to_string());
			src = unsafe { src.add(i + 1) };
			i = 0;
		} else {
			i += 1;
		}
	}
	strings
}