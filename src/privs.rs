//! Internal definitions used by the library.

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::{BOOL, HRESULT};
use crate::funcs::GetLastError;
use crate::WString;

pub(crate) const CB_ERR: i32 = -1;
pub(crate) const CB_ERRSPACE: i32 = -2;
pub(crate) const CCHILDREN_TITLEBAR: usize = 5;
pub(crate) const CLR_INVALID: u32 = 0xffff_ffff;
pub(crate) const FAPPCOMMAND_MASK: u16 = 0xf000;
pub(crate) const GDI_ERROR: u32 = 0xffff_ffff;
pub(crate) const GDT_ERROR: i32 = -1;
pub(crate) const GMEM_INVALID_HANDLE: u32 = 0x8000;
pub(crate) const INFINITE: u32 = 0xffff_ffff;
pub(crate) const INVALID_FILE_ATTRIBUTES: i32 = -1;
pub(crate) const INVALID_HANDLE_VALUE: isize = -1;
pub(crate) const L_MAX_URL_LENGTH: usize = 2048 + 32 + 4;
pub(crate) const LB_ERR: i32 = -1;
pub(crate) const LB_ERRSPACE: i32 = -2;
pub(crate) const LF_FACESIZE: usize = 32;
pub(crate) const MAX_LINKID_TEXT: usize = 48;
pub(crate) const MAX_PATH: usize = 260;

/// If value is FALSE, yields `Err(GetLastError)`, otherwise `Ok()`.
pub(crate) fn bool_to_winresult(expr: BOOL) -> WinResult<()> {
	match expr {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}

/// If value is `S_OK` yields `Ok()`, othersize `Err(hresult)`.
pub(crate) fn hr_to_winresult(hresult: HRESULT) -> WinResult<()> {
	match co::ERROR(hresult as _) {
		co::ERROR::S_OK => Ok(()),
		hresult => Err(hresult),
	}
}

/// If value is `S_OK` yields `Ok(true)`, if `S_FALSE` yields `Ok(false)`
/// othersize `Err(hresult)`.
pub(crate) fn hr_to_winresult_bool(hresult: HRESULT) -> WinResult<bool> {
	match co::ERROR(hresult as _) {
		co::ERROR::S_OK => Ok(true),
		co::ERROR::S_FALSE => Ok(false),
		hresult => Err(hresult),
	}
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
