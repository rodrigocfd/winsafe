#![allow(non_snake_case)]

use std::{fmt, hash, ops};

use crate::co;
use crate::kernel::decl::{FormatMessage, LANGID};

/// A system error which can be formatted with
/// [`FormatMessage`](crate::FormatMessage).
pub trait FormattedError: Into<u32> {
	/// Returns the textual description of the system error, by calling
	/// [`FormatMessage`](crate::FormatMessage).
	/// function.
	#[must_use]
	fn FormatMessage(self) -> String {
		let err_code: u32 = self.into();
		match unsafe {
			FormatMessage(
				co::FORMAT_MESSAGE::ALLOCATE_BUFFER
					| co::FORMAT_MESSAGE::FROM_SYSTEM
					| co::FORMAT_MESSAGE::IGNORE_INSERTS,
				None,
				err_code,
				LANGID::USER_DEFAULT,
				None,
			)
		} {
			Err(err_fmt) => format!( // never fails, returns a message instead
				"FormatMessage failed to format error {:#06x}: error {:#06x}.",
				err_code, err_fmt,
			),
			Ok(s) => s,
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
	+ From<Self::Raw> + Into<Self::Raw> + AsRef<Self::Raw> + AsMut<Self::Raw>
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
