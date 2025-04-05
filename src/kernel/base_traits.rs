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
			// This function never fails, return an informational text about the formatting error.
			Err(e) => match e {
				co::ERROR::MR_MID_NOT_FOUND => {
					"(The system cannot format this message error.)".to_owned()
				},
				e => format!(
					"The system failed to format error {:#06x} with error {:#06x}.",
					err_code, e
				),
			},
			Ok(s) => s,
		}
	}
}
