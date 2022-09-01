#![allow(non_snake_case)]

use crate::kernel::decl::{GetLastError, SysResult, WString};
use crate::kernel::ffi_types::BOOL;

pub(crate) const GMEM_INVALID_HANDLE: u32 = 0x8000;
pub(crate) const INFINITE: u32 = 0xffff_ffff;
pub(crate) const INVALID_FILE_ATTRIBUTES: i32 = -1;
pub(crate) const INVALID_HANDLE_VALUE: isize = -1;
pub(crate) const MAX_COMPUTERNAME_LENGTH: usize = 15;
pub(crate) const MAX_MODULE_NAME32: usize = 255;
pub(crate) const MAX_PATH: usize = 260;

/// [`IS_INTRESOURCE`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-is_intresource)
/// macro.
pub(crate) fn IS_INTRESOURCE(val: *const u16) -> bool {
	(val as usize >> 16) == 0
}

/// [`MAKEINTRESOURCE`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
/// macro.
pub(crate) fn MAKEINTRESOURCE(val: isize) -> *const u16 {
	val as u16 as _
}

/// If value is `FALSE`, yields `Err(GetLastError)`, otherwise `Ok()`.
pub(crate) fn bool_to_sysresult(expr: BOOL) -> SysResult<()> {
	match expr {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}

/// Converts a string to a ISO-8859-1 null-terminated byte array.
pub(crate) fn str_to_iso88591(s: &str) -> Vec<u8> {
	s.chars().map(|ch| ch as u8)
		.chain(std::iter::once(0)) // append a terminating null
		.collect()
}

/// Parses a null-delimited multi-string, which must terminate with two nulls.
pub(crate) fn parse_multi_z_str(src: *const u16) -> Vec<String> {
	let mut src = src;
	let mut strings = Vec::<String>::default();
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
