//! Internal definitions used by the library.

#![allow(non_snake_case)]

use crate::aliases::{HrResult, WinResult};
use crate::co;
use crate::ffi::{BOOL, HRES};
use crate::funcs::GetLastError;
use crate::handles::HINSTANCE;
use crate::various::WString;

pub(crate) const CB_ERR: i32 = -1;
pub(crate) const CB_ERRSPACE: i32 = -2;
pub(crate) const CCHDEVICENAME: usize = 32;
pub(crate) const CCHFORMNAME: usize = 32;
pub(crate) const CCHILDREN_TITLEBAR: usize = 5;
pub(crate) const CLR_DEFAULT: u32 = 0xff00_0000;
pub(crate) const CLR_INVALID: u32 = 0xffff_ffff;
pub(crate) const DM_SPECVERSION: u16 = 0x0401;
pub(crate) const FAPPCOMMAND_MASK: u16 = 0xf000;
pub(crate) const GDI_ERROR: u32 = 0xffff_ffff;
pub(crate) const GDT_ERROR: i32 = -1;
pub(crate) const GMEM_INVALID_HANDLE: u32 = 0x8000;
pub(crate) const HINST_COMMCTRL: HINSTANCE = HINSTANCE(-1 as _);
pub(crate) const INFINITE: u32 = 0xffff_ffff;
pub(crate) const INVALID_FILE_ATTRIBUTES: i32 = -1;
pub(crate) const INVALID_HANDLE_VALUE: isize = -1;
pub(crate) const L_MAX_URL_LENGTH: usize = 2048 + 32 + 4;
pub(crate) const LB_ERR: i32 = -1;
pub(crate) const LB_ERRSPACE: i32 = -2;
pub(crate) const LF_FACESIZE: usize = 32;
pub(crate) const MAX_COMPUTERNAME_LENGTH: usize = 15;
pub(crate) const MAX_LINKID_TEXT: usize = 48;
pub(crate) const MAX_PATH: usize = 260;
pub(crate) const UNLEN: usize = 256;

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
pub(crate) fn bool_to_winresult(expr: BOOL) -> WinResult<()> {
	match expr {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}

/// If value is `S_OK` yields `Ok()`, othersize `Err(hresult)`.
pub(crate) fn ok_to_hrresult(hr: HRES) -> HrResult<()> {
	match co::HRESULT(hr) {
		co::HRESULT::S_OK => Ok(()),
		hr => Err(hr),
	}
}

/// If value is `S_OK` yields `Ok(true)`, if `S_FALSE` yields `Ok(false)`
/// othersize `Err(hresult)`.
#[allow(dead_code)]
pub(crate) fn okfalse_to_hrresult(hr: HRES) -> HrResult<bool> {
	match co::HRESULT(hr) {
		co::HRESULT::S_OK => Ok(true),
		co::HRESULT::S_FALSE => Ok(false),
		hr => Err(hr),
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
