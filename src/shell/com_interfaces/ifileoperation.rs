#![allow(non_camel_case_types, non_snake_case)]

use crate::prelude::*;

com_interface! { IFileOperation: "947aab5f-0a5c-4c13-b4d6-4bf7836fc9f8";
	/// [`IFileOperation`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifileoperation)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let fo = w::CoCreateInstance::<w::IFileOperation>(
	///     &co::CLSID::FileOperation,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl shell_IFileOperation for IFileOperation {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IFileOperation`](crate::IFileOperation).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IFileOperation: ole_IUnknown {}
