#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, shell};
use crate::ole::decl::{ComPtr, HrResult, IStream};
use crate::prelude::ole_IStream;

impl shell_IStream for IStream {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IStream`](crate::IStream).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IStream: ole_IStream {
	/// [`SHCreateMemStream`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-shcreatememstream)
	/// static method.
	///
	/// # Examples
	///
	/// Loading from a `Vec`:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::IStream;
	///
	/// let raw_data: Vec<u8>; // initialized somewhere
	/// # let raw_data = Vec::<u8>::default();
	///
	/// let stream = IStream::SHCreateMemStream(&raw_data)?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn SHCreateMemStream(src: &[u8]) -> HrResult<IStream> {
		let p = unsafe {
			shell::ffi::SHCreateMemStream(src.as_ptr(), src.len() as _)
		};
		if p.is_null() {
			Err(co::HRESULT::E_OUTOFMEMORY)
		} else {
			Ok(IStream::from(ComPtr(p as _)))
		}
	}
}
