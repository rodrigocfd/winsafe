//! Internal definitions used by the library.

use std::ffi::c_void;

use crate::co;
use crate::funcs::GetLastError;
use crate::WString;

pub const CB_ERR: i32 = -1;
pub const CB_ERRSPACE: i32 = -2;
pub const FAPPCOMMAND_MASK: u16 = 0xf000;
pub const L_MAX_URL_LENGTH: usize = 2048 + 32 + 4;
pub const LF_FACESIZE: usize = 32;
pub const MAX_LINKID_TEXT: usize = 48;
pub const WC_DIALOG: u16 = 0x8002;

/// Transforms a raw pointer into an option, which is `None` if the pointer is
/// null.
///
/// https://stackoverflow.com/q/65144143/6923555
pub fn ptr_as_opt(ptr: *mut c_void) -> Option<*mut c_void> {
	if ptr.is_null() {
		None
	} else {
		Some(ptr)
	}
}

/// Converts a reference to a `*const c_void`.
pub fn const_void<T>(val: &T) -> *const c_void {
	val as *const T as *const _
}
/// Converts a mut reference to a `*mut c_void`.
pub fn mut_void<T>(val: &mut T) -> *mut c_void {
	val as *mut T as *mut _
}

/// Parses a null-delimited multi-string, which must terminate with two nulls.
pub fn parse_multi_z_str(src: *const u16) -> Vec<String> {
	let mut src = src;
	let mut strings = Vec::default();
	let mut i = 0;

	loop {
		if unsafe { *src.add(i) } == 0 {
			let slice = unsafe { std::slice::from_raw_parts(src, i) };
			if slice.is_empty() {
				break;
			}
			strings.push(WString::from_wchars_slice(slice).to_string());
			src = unsafe { src.add(i + 1) };
			i = 0;
		} else {
			i += 1;
		}
	}
	strings
}
