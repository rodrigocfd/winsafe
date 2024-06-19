#![allow(non_camel_case_types, non_snake_case)]

use crate::mf::vts::*;
use crate::prelude::*;

com_interface! { IMFSourceResolver: "fbe5a32d-a497-4b61-bb85-97b1a848a6e3";
	/// [`IMFSourceResolver`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfsourceresolver)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with
	/// [`MFCreateSourceResolver`](crate::MFCreateSourceResolver) function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let source_resolver = w::MFCreateSourceResolver()?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl mf_IMFSourceResolver for IMFSourceResolver {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFSourceResolver`](crate::IMFSourceResolver).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFSourceResolver: ole_IUnknown {

}
