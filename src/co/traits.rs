#![allow(non_snake_case)]

use std::fmt;
use std::hash::Hash;
use std::ops;

use crate::co;
use crate::ffi::kernel32;
use crate::funcs::GetLastError;
use crate::handles::HLOCAL;
use crate::structs::LANGID;
use crate::various::WString;

/// Any native Windows constant.
pub trait NativeConstant: Default + Copy + Clone + Eq + PartialEq + Hash
	+ From<Self::Concrete> + Into<Self::Concrete>
	+ fmt::LowerHex + fmt::UpperHex + fmt::Binary + fmt::Octal
	+ ops::BitAnd + ops::BitAndAssign
	+ ops::BitOr + ops::BitOrAssign
	+ ops::BitXor + ops::BitXorAssign
	+ ops::Not
{
	/// The underlying concrete type for this constant type.
	type Concrete;

	/// Tells whether other bitflag style is present.
	///
	/// Equivalent to `(val & other) != 0`.
	fn has(&self, other: Self) -> bool;
}

/// A system error which can be formatted with
/// [`FormatMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew).
pub trait FormattedError: NativeConstant + Into<u32> {
	/// Returns the textual description of the system error, by calling
	/// [`FormatMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
	/// function.
	fn FormatMessage(self) -> String {
		let err_code: u32 = self.into();
		unsafe {
			let mut ptr_buf: *mut u16 = std::ptr::null_mut();
			match kernel32::FormatMessageW(
				co::FORMAT_MESSAGE::ALLOCATE_BUFFER.0
					| co::FORMAT_MESSAGE::FROM_SYSTEM.0
					| co::FORMAT_MESSAGE::IGNORE_INSERTS.0,
				std::ptr::null(),
				err_code,
				LANGID::USER_DEFAULT.0 as _,
				(&mut ptr_buf as *mut *mut u16) as _, // pass pointer to pointer
				0,
				std::ptr::null_mut(),
			) {
				0 => format!( // never fails, returns a message instead
					"FormatMessage failed to format error {:#06x}: error {:#06x}.",
					err_code, GetLastError(),
				),
				nchars => {
					let final_str = WString::from_wchars_count(ptr_buf, nchars as _);
					match (HLOCAL(ptr_buf as _)).LocalFree() {
						Ok(()) => final_str.to_string(),
						Err(e) => format!(
							"LocalFree failed after formatting error {:#06x}: error {:#06x}.",
							err_code, e.0),
					}
				},
			}
		}
	}
}
