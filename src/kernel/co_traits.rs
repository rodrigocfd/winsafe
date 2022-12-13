#![allow(non_snake_case)]

use std::{fmt, hash, ops};

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, HLOCAL, LANGID, WString};
use crate::prelude::kernel_Hlocal;

/// A system error which can be formatted with
/// [`FormatMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew).
pub trait FormattedError: Into<u32> {
	/// Returns the textual description of the system error, by calling
	/// [`FormatMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
	/// function.
	#[must_use]
	fn FormatMessage(self) -> String {
		let err_code: u32 = self.into();
		unsafe {
			let mut ptr_buf: *mut u16 = std::ptr::null_mut();
			match kernel::ffi::FormatMessageW(
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

/// A native typed constant.
///
/// If the values of this constant type can be combined as bitflags, it will
/// also implement the [`NativeBitflag`](crate::prelude::NativeBitflag) trait.
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait NativeConst: Sized
	+ Default + Clone + Copy + PartialEq + Eq + Send + hash::Hash
	+ From<Self::Raw> + Into<Self::Raw>
	+ fmt::Debug + fmt::Display
	+ fmt::LowerHex + fmt::UpperHex
	+ fmt::Binary + fmt::Octal
{
	/// The underlying type of this constant.
	type Raw;
}

/// A native typed bitflag constant.
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait NativeBitflag: NativeConst
	+ ops::BitAnd + ops::BitAndAssign
	+ ops::BitOr + ops::BitOrAssign
	+ ops::BitXor + ops::BitXorAssign
	+ ops::Not
{
	/// Tells whether other bitflag style is present.
	///
	/// Equivalent to `(val & other) != 0`.
	#[must_use]
	fn has(&self, other: Self) -> bool;
}
