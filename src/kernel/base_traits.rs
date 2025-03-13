#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;

/// A system error which can be formatted with
/// [`FormatMessage`](crate::FormatMessage), exhibiting a description string
/// provided by the OS.
pub trait SystemError: Into<u32> {
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
			Err(err_fmt) => format!(
				"FormatMessage failed to format error {:#06x}: error {:#06x}.", // never fails, returns a message instead
				err_code, err_fmt,
			),
			Ok(s) => s,
		}
	}
}
