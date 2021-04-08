//! Internal definitions used by the library.

use std::ffi::c_void;

use crate::aliases::WinResult;
use crate::ffi::BOOL;
use crate::funcs::GetLastError;
use crate::WString;

pub const CB_ERR: i32 = -1;
pub const CB_ERRSPACE: i32 = -2;
pub const FAPPCOMMAND_MASK: u16 = 0xf000;
pub const GDI_ERROR: u32 = 0xffff_ffff;
pub const GDT_ERROR: i32 = -1;
pub const INFINITE: u32 = 0xffff_ffff;
pub const L_MAX_URL_LENGTH: usize = 2048 + 32 + 4;
pub const LB_ERR: i32 = -1;
pub const LB_ERRSPACE: i32 = -2;
pub const LF_FACESIZE: usize = 32;
pub const MAX_LINKID_TEXT: usize = 48;

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

/// Converts a `BOOL` value to a `WinResult`. `TRUE` is `Ok(())`, `FALSE` is
/// `Err(GetLastError())`.
pub fn bool_to_winresult(expr: BOOL) -> WinResult<()> {
	match expr {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
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
